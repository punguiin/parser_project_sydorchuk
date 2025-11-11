# Math expression parser

Main idea of parses that it will take string in input, extract mathematical expressions from it, calculate them, and output the results. Parser will be able to work with basic operations, trigonometrical functions, exponential functions (may be more in future)

## Code example

```rust
use anyhow::anyhow;
use math_expression_parser::*;
use pest::Parser;

fn main() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::input, "(12+34)")?
        .next()
        .ok_or_else(|| anyhow!("Failed to parse input"))?;
    dbg!(pair);
    Ok(())
}
```
