use anyhow::anyhow;
use parser_project_sydorchuk::*;
use pest::Parser;

fn main() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::input, "(12+34)")?
        .next()
        .ok_or_else(|| anyhow!("Failed to parse input"))?;
    dbg!(pair);
    Ok(())
}
