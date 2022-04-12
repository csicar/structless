use std::{
    cmp::{max, min},
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
use tree_sitter::{Language, Node, Parser, Range, Tree, TreeCursor};
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

    let language = match args.language {
        LanguageArgument::Rust => tree_sitter_rust::language(),
        LanguageArgument::Json => tree_sitter_json::language(),
        LanguageArgument::Unknown => tree_sitter_structless::language(),
    };

    // create app and run it
    let app = App::new(source_code, language)?;
    let tree = app.tree.clone();
    let node = tree.root_node();
    // let tree =.root_node();
    node.walk();
    run_app(&mut terminal, app)?;

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

#[derive(Debug, PartialEq, Eq)]
enum ViewMode {
    Tree,
    Text,
}

#[derive(Debug)]
enum InputMode {
    Normal,
    Editing,
}

/// App holds the state of the application
#[derive(Debug)]
struct App {
    /// Current value of the input box
    search_term: String,
    /// Current input mode
    input_mode: InputMode,
    /// History of recorded messages
    tree: Arc<Tree>,
    source_code: String,

    /// Tree controls
    collapsed: HashSet<usize>,
    line_index: usize,
    list_state: ListState,
    view_mode: ViewMode,
}

impl App {
    pub(crate) fn new(source_code: String, language: Language) -> anyhow::Result<App> {
        let mut parser = Parser::new();
        parser.set_language(language)?;

        let tree = Arc::new(parser.parse(&source_code, None).unwrap());
        // let cursor = tree.root_node().id();
        Ok(App {
            search_term: "".to_string(),
            input_mode: InputMode::Normal,
            tree,
            source_code,
            collapsed: HashSet::new(),
            line_index: 0,
            list_state: ListState::default(),
            view_mode: ViewMode::Tree,
        })
    }
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    let mut f = flatten(app.tree.root_node(), 0, Arc::new(|id| false));

    f.iter().for_each(|l| {
        app.collapsed.insert(l.node.id());
    });
    drop(f);

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let flat_lines = flatten(
            app.tree.root_node(),
            0,
            Arc::new(|id| app.collapsed.contains(&id)),
        );
        let selected_node = flat_lines[app.line_index].node.id();

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('/') => {
                        app.input_mode = InputMode::Editing;
                    }
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    KeyCode::Down | KeyCode::Char('s') => {
                        app.line_index = min(app.line_index + 1, flat_lines.len() - 1);
                    }
                    KeyCode::Up | KeyCode::Char('w') => {
                        app.line_index = max(app.line_index, 1) - 1;
                    }
                    KeyCode::Right | KeyCode::Char('d') => {
                        // already expanded ? => expand children
                        if !app.collapsed.contains(&selected_node) {
                            get_node_decedents(&flat_lines[app.line_index].node)
                                .iter()
                                .for_each(|decedent| {
                                    app.collapsed.remove(&decedent.id());
                                })
                        } else {
                            app.collapsed.remove(&selected_node);
                        }
                    }
                    KeyCode::Left | KeyCode::Char('a') => {
                        // already collapsed ? => collapse children
                        if app.collapsed.contains(&selected_node) {
                            get_node_decedents(&flat_lines[app.line_index].node)
                                .iter()
                                .for_each(|decedent| {
                                    app.collapsed.insert(decedent.id());
                                })
                        } else {
                            app.collapsed.insert(selected_node);
                        }
                    }
                    KeyCode::Enter => {
                        app.view_mode = if app.view_mode == ViewMode::Text {
                            ViewMode::Tree
                        } else {
                            ViewMode::Text
                        };
                    }
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Enter => {
                        // app.messages.push(app.input.drain(..).collect());
                    }
                    KeyCode::Char(c) => {
                        app.search_term.push(c);
                    }
                    KeyCode::Backspace => {
                        app.search_term.pop();
                    }
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    }
                    _ => {}
                },
            }
        }
        trace!(?app, "new state: ");
    }
}

fn get_node_decedents<'a, 'b>(node: &'b Node<'a>) -> Vec<Node<'a>> {
    let mut cursor = node.walk();

    node.children(&mut cursor)
        .into_iter()
        .flat_map(|child| get_node_decedents(&child).into_iter().chain([child]))
        .collect()
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
    if should_collapse(node.id()) || node.child_count() == 0 {
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

fn trim_string(s: &str, max_length: usize) -> String {
    if (s.len() > max_length) {
        let trimmed = &s[0..max_length - 1];
        format!("{}…", trimmed)
    } else {
        " ".repeat(max_length - s.len()) + s
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
                Span::styled("/", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to search."),
                Span::raw(format!("{}", app.line_index)),
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

    let input = Paragraph::new(app.search_term.as_ref())
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
                chunks[1].x + app.search_term.width() as u16 + 1,
                // Move one line down, from the border to the input line
                chunks[1].y + 1,
            )
        }
    }

    let node_start = app.tree.root_node().byte_range().start;
    let child_range = get_smaller_child_range(&app.tree.root_node())
        .map(|it| it.start_byte)
        .unwrap_or(0);
    let r = &app.source_code[node_start..child_range];
    trace!("{}, {}, {}", r, node_start, child_range);

    let flat_lines = flatten(
        app.tree.root_node(),
        0,
        Arc::new(|id| app.collapsed.contains(&id)),
    );

    app.list_state.select(Some(app.line_index));

    match app.view_mode {
        ViewMode::Tree => {
            let items: Vec<_> = flat_lines
                .iter()
                .filter(|line| {
                    if app.search_term.is_empty() {
                        true
                    } else {
                        line.node
                            .utf8_text(app.source_code.as_bytes())
                            .unwrap()
                            .contains(&app.search_term)
                    }
                })
                .map(|line| {
                    let start = line.node.start_byte();
                    let end = line.node.end_byte();
                    let node_selected = line.node.id() == flat_lines[app.line_index].node.id();

                    if line.kind == LineKind::End {
                        ListItem::new(Spans::from(vec![Span::styled(
                            format!("{}// end {}", " ".repeat(line.indent), line.node.kind()),
                            if node_selected {
                                Style::default().fg(Color::Red)
                            } else {
                                Style::default().fg(Color::DarkGray)
                            },
                        )]))
                    } else {
                        ListItem::new(Spans::from(vec![
                            Span::raw(" ".repeat(line.indent)),
                            Span::styled(
                                trim_string(line.node.kind(), 15),
                                Style::default().fg(Color::DarkGray).add_modifier(
                                    if node_selected {
                                        Modifier::empty()
                                    } else {
                                        Modifier::DIM
                                    },
                                ),
                            ),
                            Span::raw("   "),
                            Span::styled(
                                app.source_code[start..end].to_string(),
                                Style::default().fg(Color::Cyan),
                            ),
                        ]))
                    }
                })
                .collect();
            let list = List::new(items)
                .block(Block::default().title("Tree").borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                .highlight_style(
                    Style::default()
                        .add_modifier(Modifier::ITALIC)
                        .bg(Color::Rgb(30, 30, 30)),
                )
                .highlight_symbol(">>");
            f.render_stateful_widget(list, chunks[2], &mut app.list_state);
        }
        ViewMode::Text => {
            let block = Block::default().borders(Borders::ALL);
            let selected_node = flat_lines[app.line_index].node;
            let range = selected_node.start_byte()..selected_node.end_byte();

            let paragraph = Paragraph::new(Text::raw(app.source_code[range].to_string()))
                .block(block.clone().title("Source Code"))
                .wrap(Wrap { trim: false })
                .scroll((0, 0));
            f.render_widget(paragraph, chunks[2]);
        }
    }
}

/// Structure-Aware Less
#[derive(clap::Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// File to parse
    #[clap(short, long, value_hint=clap::ValueHint::FilePath)]
    input: String,

    /// Language to use for parsing the file
    #[clap(arg_enum, default_value = "unknown")]
    language: LanguageArgument,
}

#[derive(clap::ArgEnum, Clone, Debug)]
enum LanguageArgument {
    Rust,
    Json,
    Unknown,
}

fn main() -> Result<(), anyhow::Error> {
    main_interactive()?;
    Ok(())
}
