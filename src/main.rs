use clap::{Arg, Command, builder::PathBufValueParser};
use math_expression_parser::parse_and_eval;
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    let matches = Command::new("Math Expression parser")
        .version("1.0")
        .author("Nazar Sydorchuk <n.sydorchuk@ukma.edu.ua>")
        .about("Parses and evaluates mathematical expressions from a file")
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .help("Path to the input file containing the mathematical expression")
                .value_parser(PathBufValueParser::new()),
        )
        .arg(
            Arg::new("credits")
                .short('c')
                .long("credits")
                .help("Show credits and exit")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    if matches.get_flag("credits") {
        println!("Math Expression parser v1.0");
        println!("Author: Nazar Sydorchuk <n.sydorchuk@ukma.edu.ua>");
        return Ok(());
    }

    let file_path: &PathBuf = matches.get_one("file").expect("No file path provided");
    let input = std::fs::read_to_string(file_path)?;
    let lines: Vec<&str> = input.lines().collect();
    for input in lines {
        parse_and_eval(input)?;
    }
    println!("Successfully evaluated expressions.");
    println!("Results have been written to res.txt");
    Ok(())
}
