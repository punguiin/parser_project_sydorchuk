use pest_derive::Parser;

#[derive(Parser, Debug)]
#[grammar = "grammar.pest"]
pub struct Grammar;
