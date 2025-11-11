use clap::{Arg, Command, builder::PathBufValueParser};
use math_expression_parser::parse_and_eval;
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    let matches = Command::new("Math Expression parser")
        .version("1.0")
        .about("Parses and evaluates mathematical expressions from a file")
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .help("Path to the input file containing the mathematical expression")
                .required(true)
                .value_parser(PathBufValueParser::new()),
        )
        .get_matches();
    let file_path: &PathBuf = matches.get_one("file").expect("No file path provided");
    let input = std::fs::read_to_string(file_path)?;
    let lines: Vec<&str> = input.lines().collect();
    for input in lines {
        let result = parse_and_eval(&input)?;
        println!("{} = {}", input, result);
    }
    Ok(())
}
