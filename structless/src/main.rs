use std::{
    convert::Infallible,
    fmt::Display,
    fs::File,
    io::{self, BufRead, BufReader, BufWriter, Bytes, Read, Write},
    iter::Peekable,
    path::Path,
    str::FromStr, thread, time::Duration,
};

mod delimiter;
use clap::StructOpt;
use delimiter::Delimiter;

use tree_sitter::Parser;
use tree_sitter_structless;

use tui::{
    backend::CrosstermBackend,
    widgets::{Widget, Block, Borders},
    layout::{Layout, Constraint, Direction},
    Terminal
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

fn main2() -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default()
            .title("Block")
            .borders(Borders::ALL);
        f.render_widget(block, size);
    })?;

    thread::sleep(Duration::from_millis(5000));

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

/// Simple program to greet a person
#[derive(clap::Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// File to parse
    #[clap(short, long, value_hint=clap::ValueHint::FilePath)]
    input: String,

    /// Set of matching delimiters
    #[clap(long, default_value = "(<=>)")]
    delimiter: Vec<Delimiter>,
}

fn main() -> Result<(), anyhow::Error> {
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

    let tree = parser.parse(source_code, None);

    // io::stdout().write_all(tree.root_node().to_sexp())?;
    println!("{:?}", tree);
    Ok(())
}

// // A ::= ( Txt A Txt ) |  | Txt

// enum SyntaxTree {
//     Delimited {
//         delimiter: Delimiter,
//         children: Vec<SyntaxTree>,
//     },
//     Text {
//         text: Vec<u8>,
//     },
// }

// fn is_delimiter<'a>(delimiter: &'a Vec<Delimiter>, byte: u8) -> Option<&'a Delimiter> {
//     delimiter.iter().find(|&delim| delim.start as u8 == byte)
// }

// fn parse_delimited<R: Read>(
//     delimiters: Vec<Delimiter>,
//     end: &Delimiter,
//     input: &mut Peekable<Bytes<R>>,
// ) -> Result<SyntaxTree, anyhow::Error> {
//     input.next();
//     let next = input.peek();
//     match next {
//         Some(&n) => {
//             let next = n?;

//         },
//         None => todo!(),
//     }
//     let before = parse_text(&delimiters, )
// }

// fn parse_text<R: Read>(
//     delimiters: Vec<Delimiter>,
//     end: u8,
//     input: &mut Peekable<Bytes<R>>,
// ) -> Result<SyntaxTree, anyhow::Error> {
//     let mut text = Vec::new();

//     while let Some(&n) = input.peek() {
//         let next = n?;
//         if is_delimiter(&delimiters, next).is_some() {
//             break;
//         }
//         if next == end {
//             break;
//         }
//         input.next();
//         text.push(next);
//     }

//     Ok(SyntaxTree::Text { text })
// }

// fn parse<R: Read>(
//     delimiters: Vec<Delimiter>,
//     end: u8,
//     input: &mut Peekable<Bytes<R>>,
// ) -> Result<SyntaxTree, anyhow::Error> {
//     let before: Vec<u8> = Vec::new();

//     let mut asd = match input.peek() {
//         Some(&next) => {
//             let byte = next?;
//             if byte == end {
//                 input.next();
//                 return Ok(SyntaxTree::Text { text: Vec::new() });
//             } else {
//                 if let Some(delim) = is_delimiter(&delimiters, byte) {
//                     parse_delimited(delimiters, delim, input)?
//                 } else {
//                     parse_text(delimiters, end, input)?
//                 }
//             }
//         }
//         None => return Ok(SyntaxTree::Text { text: Vec::new() }),
//     };

//     let mut curr_tree = SyntaxTree::Text { text: Vec::new() };
//     for r in input {
//         let byte = r?;
//         if byte == end {
//             return Ok(curr_tree);
//         }

//         let a = delimiter.iter().find(|&delim| delim.start as u8 == byte);
//         if let Some(delimiter) = a {
//         } else {
//         }
//     }

//     todo!()
// }
