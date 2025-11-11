use anyhow::Result;
use math_expression_parser::parse_and_eval;

fn main() -> Result<()> {
    let input = "(((exp(1)/pow(2,3))-sin(log(100, 10)))+(cos(ln(7.5))*tan(45)))";
    let result = parse_and_eval(input)?;
    println!("{} = {}", input, result);
    Ok(())
}
