use std::env::{args, Args};
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug)]
struct View {
    /// Path to the file to read
    path: PathBuf,
    /// Offset to read file from
    offset: Option<usize>,
    /// Limit to stop reading file
    limit: Option<usize>,
}

fn parse(args: Args) -> View {
    if args.len() < 2 {
        // Faltan Argumentos
        todo!("Print HELP Text");
    }

    let mut view = View {
        path: PathBuf::default(),
        offset: None,
        limit: None,
    };

    for (idx, arg) in args.enumerate() {
        if idx == 0 {
            // Path to the current executable is skipped
            continue;
        }

        if idx == 1 {
            // This must be the path to the file to read
            let path = PathBuf::from_str(&arg).unwrap();
            view.path = path;
        }
    }

    view
}

fn main() {
    let arguments = args();
    let view = parse(arguments);

    println!("{:#?}", view);
}
