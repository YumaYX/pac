use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod node;
mod builder;
mod printer;

use builder::build_tree_with_trigger;
use printer::print_parent_children;

pub fn load_file<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let lines = io::BufReader::new(file)
        .lines()
        .collect::<Result<_, _>>()?;

    Ok(lines)
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("usage: pac <file> <trigger-text> <keyword>");
        std::process::exit(1);
    }

    let filename = &args[1];
    let trigger = &args[2];
    let keyword = &args[3];

    let lines = load_file(filename)?;
    let tree = build_tree_with_trigger(&lines, trigger);

    print_parent_children(&tree, keyword);

    Ok(())
}
