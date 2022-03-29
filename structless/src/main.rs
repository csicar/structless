use std::{
    collections::{HashMap, HashSet},
    convert::Infallible,
    fmt::Display,
    fs::File,
    io::{self, BufRead, BufReader, BufWriter, Bytes, Read, Write},
    iter::Peekable,
    path::Path,
    str::FromStr,
    sync::Arc,
    thread,
    time::Duration,
};

use clap::StructOpt;

mod tree;

use tracing::{error, instrument, trace};
use tracing_subscriber::fmt::format::Writer;
use tree_sitter::{Node, Parser, Range, Tree, TreeCursor};
use tree_sitter_structless;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame, Terminal,
};

use unicode_width::UnicodeWidthStr;

fn main_interactive() -> anyhow::Result<()> {
    tracing_subscriber::fmt::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::new("trace"))
        .with_writer(File::create("out.log")?)
        .pretty()
        .init();

    let args = Args::parse();
    println!("args : {:?}", &args);

    let mut reader: BufReader<Box<dyn Read>> = {
        if args.input == "-" {
            BufReader::new(Box::new(io::stdin()))
        } else {
            let f = File::open(args.input)?;
            BufReader::new(Box::new(f))
        }
    };

    let mut source_code = String::new();
    reader.read_to_string(&mut source_code)?;

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default().title("Block").borders(Borders::ALL);
        f.render_widget(block, size);
    })?;

    // create app and run it
    let mut app = App::new(source_code)?;
    let tree = app.tree.clone();
    let node = tree.root_node();
    // let tree =.root_node();
    node.walk();
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

enum InputMode {
    Normal,
    Editing,
}

/// App holds the state of the application
struct App {
    /// Current value of the input box
    input: String,
    /// Current input mode
    input_mode: InputMode,
    /// History of recorded messages
    tree: Arc<Tree>,
    source_code: String,
    collapsed: HashSet<usize>,
    cursor: usize,
    list_state: ListState,
}

impl App {
    pub(crate) fn new(source_code: String) -> anyhow::Result<App> {
        let language = tree_sitter_structless::language();

        let mut parser = Parser::new();
        parser.set_language(language)?;

        let tree = Arc::new(parser.parse(&source_code, None).unwrap());
        let cursor = tree.root_node().id();
        Ok(App {
            input: "".to_string(),
            input_mode: InputMode::Normal,
            tree,
            source_code,
            collapsed: HashSet::new(),
            cursor,
            list_state: ListState::default(),
        })
    }
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    let tmp_tree = app.tree.clone();
    let mut tree_cursor = tmp_tree.root_node().walk();

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('e') => {
                        app.input_mode = InputMode::Editing;
                    }
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    KeyCode::Down => {
                        let flat_lines = flatten(app.tree.root_node(), 0, Arc::new(|id| {
                            app.collapsed.contains(&id)
                        }));
                    
                        let next = flat_lines.iter().skip_while(|line| {
                            line.node.id() != app.cursor
                        }).skip(1).next().map(|line| line.node.id());
                        app.cursor = next.unwrap_or(app.cursor);
                    }
                    KeyCode::Up => {
                        tree_cursor.goto_next_sibling();
                        app.cursor = tree_cursor.node().id();
                    }
                    KeyCode::Right => {
                        tree_cursor.goto_first_child();
                        app.cursor = tree_cursor.node().id();
                    }
                    KeyCode::Left => {
                        tree_cursor.goto_parent();
                        app.cursor = tree_cursor.node().id();
                    }
                    KeyCode::Char(' ') => {
                        if app.collapsed.contains(&app.cursor) {
                            app.collapsed.remove(&app.cursor);
                        } else {
                            app.collapsed.insert(app.cursor);
                        }
                    }
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Enter => {
                        // app.messages.push(app.input.drain(..).collect());
                    }
                    KeyCode::Char(c) => {
                        app.input.push(c);
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    }
                    _ => {}
                },
            }
        }
    }
}

/// Gets the range of the first child node, that is actually smaller than
/// `node`
#[instrument(fields(start=node.start_byte(), end=node.end_byte(), id=node.id()))]
fn get_smaller_child_range(node: &Node) -> Option<Range> {
    let number = node.child_count();
    if number == 0 {
        None
    } else {
        let child_start = node.child(0).unwrap();
        let child_end = node.child(number - 1).unwrap();
        trace!(?child_start, ?child_end, "children");

        if child_start.start_byte() > node.start_byte() || child_end.end_byte() < node.end_byte() {
            Some(span_ranges(child_start, child_end))
        } else {
            get_smaller_child_range(&child_start)
        }
    }
}

fn span_ranges(start: Node, end: Node) -> Range {
    Range {
        start_byte: start.start_byte(),
        start_point: start.start_position(),
        end_byte: end.end_byte(),
        end_point: end.end_position(),
    }
}

#[derive(Debug, PartialEq, Eq)]
enum LineKind {
    Start,
    End,
    Whole,
}

#[derive(Debug)]
struct Line<'a> {
    node: Node<'a>,
    indent: usize,
    kind: LineKind,
}

fn flatten<F>(node: Node, indent: usize, should_collapse: Arc<F>) -> Vec<Line>
where
    F: Fn(usize) -> bool,
{
    use LineKind::*;
    if should_collapse(node.id()) {
        vec![Line {
            node,
            indent,
            kind: Whole,
        }]
    } else {
        let mut lines = Vec::new();

        lines.push(Line {
            node,
            indent,
            kind: Start,
        });
        for child in node.children(&mut node.walk()) {
            lines.append(&mut flatten(child, indent + 1, should_collapse.clone()))
        }
        lines.push(Line {
            node,
            indent,
            kind: End,
        });
        lines
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(f.size());

    let (msg, style) = match app.input_mode {
        InputMode::Normal => (
            vec![
                Span::raw("Press "),
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit, "),
                Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to start editing."),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Editing => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop editing, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to record the message"),
            ],
            Style::default(),
        ),
    };
    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[0]);

    let input = Paragraph::new(app.input.as_ref())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(input, chunks[1]);
    match app.input_mode {
        InputMode::Normal =>
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            {}

        InputMode::Editing => {
            // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
            f.set_cursor(
                // Put cursor past the end of the input text
                chunks[2].x + app.input.width() as u16 + 1,
                // Move one line down, from the border to the input line
                chunks[2].y + 1,
            )
        }
    }

    // let messages: Vec<ListItem> = app
    //     .messages
    //     .iter()
    //     .enumerate()
    //     .map(|(i, m)| {
    //         let content = vec![Spans::from(Span::raw(format!("{}: {}", i, m)))];
    //         ListItem::new(content)
    //     })
    //     .collect();
    // let messages =
    //     List::new(messages).block(Block::default().borders(Borders::ALL).title("Messages"));

    let node_start = app.tree.root_node().byte_range().start;
    let child_range = get_smaller_child_range(&app.tree.root_node())
        .map(|it| it.start_byte)
        .unwrap_or(0);
    let r = &app.source_code[node_start..child_range];
    trace!("{}, {}, {}", r, node_start, child_range);

    let flat_lines = flatten(app.tree.root_node(), 0, Arc::new(|id| {
        app.collapsed.contains(&id)
    }));

    flat_lines.iter().enumerate().for_each(|(index, line)| {
        if line.node.id() == app.cursor && line.kind == LineKind::Start {
            app.list_state.select(Some(index));
        }
    });

    let items: Vec<_> = flat_lines
        .iter()
        .map(|line| {
            let start = line.node.start_byte();
            let end = line.node.end_byte();
            ListItem::new(Spans::from(vec![
                Span::raw(format!("{}{:?}", "  ".repeat(line.indent), line.node)),
                Span::raw("   "),
                Span::styled(
                    app.source_code[start..end].to_string(),
                    Style::default().fg(Color::Gray).add_modifier(Modifier::DIM),
                ),
            ]))
        })
        .collect();
    // let items = [
    //     ListItem::new(r),
    //     ListItem::new("Item 2"),
    //     ListItem::new("Item 3"),
    // ];
    let list = List::new(items)
        .block(Block::default().title("List").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>");
    f.render_stateful_widget(list, chunks[2], &mut app.list_state);

    // let spans: Vec<Spans> = vec![
    //     Span::raw("This is a line \n").into(),
    //     Span::styled("This is a line   \n", Style::default().fg(Color::Red)).into(),
    //     Span::styled("This is a line\n", Style::default().bg(Color::Blue)).into(),
    // ]
    // .into();

    // let block = Block::default().borders(Borders::ALL);

    // let paragraph = Paragraph::new(Text::from(spans))
    //     .block(block.clone().title("Center, wrap"))
    //     .alignment(Alignment::Center)
    //     .wrap(Wrap { trim: false })
    //     .scroll((0, 0));
    // f.render_widget(paragraph, chunks[2]);
}

/// Simple program to greet a person
#[derive(clap::Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// File to parse
    #[clap(short, long, value_hint=clap::ValueHint::FilePath)]
    input: String,
}

fn main_parse() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    println!("args : {:?}", &args);

    let mut reader: BufReader<Box<dyn Read>> = {
        if args.input == "-" {
            BufReader::new(Box::new(io::stdin()))
        } else {
            let f = File::open(args.input)?;
            BufReader::new(Box::new(f))
        }
    };

    let language = tree_sitter_structless::language();

    let mut parser = Parser::new();
    parser.set_language(language)?;

    let mut source_code = String::new();
    reader.read_to_string(&mut source_code)?;

    let tree = parser.parse(source_code, None).unwrap();

    // io::stdout().write_all(tree.root_node().to_sexp())?;
    println!("{:?}", tree.root_node().to_sexp());
    Ok(())
}

fn main() -> Result<(), anyhow::Error> {
    main_interactive()?;
    Ok(())
}
