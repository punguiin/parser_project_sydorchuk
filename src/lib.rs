use anyhow::anyhow;
use pest::Parser;
use pest::iterators::Pair;
use pest_derive::Parser;
use std::fs::OpenOptions;
use std::io::Write;

// Parser struct is generated from grammar.pest file
#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct Grammar;

#[derive(Debug, Clone)]
/// The enum represents numbers, binary operations (add, sub, mul, div, pow, root, log),
/// and unary functions (sin, cos, tan, exp, ln).
pub enum Expr {
    /// A numeric literal.
    Num(f64),
    /// Addition: left + right
    Add(Box<Expr>, Box<Expr>),
    /// Subtraction: left - right
    Sub(Box<Expr>, Box<Expr>),
    /// Multiplication: left * right
    Mul(Box<Expr>, Box<Expr>),
    /// Division: left / right
    Div(Box<Expr>, Box<Expr>),
    /// Sine function: sin(x)
    Sin(Box<Expr>),
    /// Cosine function: cos(x)
    Cos(Box<Expr>),
    /// Tangent function: tan(x)
    Tan(Box<Expr>),
    /// Exponential function: exp(x) = e^x
    Exp(Box<Expr>),
    /// Natural logarithm: ln(x)
    Ln(Box<Expr>),
    /// Power: base ^ exponent
    Pow(Box<Expr>, Box<Expr>),
    /// Root: nth_root(value) (degree is second argument)
    Root(Box<Expr>, Box<Expr>),
    /// Logarithm with custom base: log(value, base)
    Log(Box<Expr>, Box<Expr>),
}

/// This function walks the parse tree produced by Pest and converts rules
/// into the corresponding `Expr` variants. It returns an error for unexpected
/// or malformed input.
fn build_expr(pair: Pair<Rule>) -> anyhow::Result<Expr> {
    match pair.as_rule() {
        Rule::input | Rule::expression => {
            let mut inner = pair.into_inner();
            if let Some(p) = inner.next() {
                build_expr(p)
            } else {
                Err(anyhow!("Empty expression"))
            }
        }
        Rule::num => {
            let s = pair.as_str();
            s.parse::<f64>()
                .map(Expr::Num)
                .map_err(|e| anyhow!("Failed to parse number '{}': {}", s, e))
        }
        Rule::plus
        | Rule::minus
        | Rule::multiply
        | Rule::divide
        | Rule::pow
        | Rule::log
        | Rule::root => {
            let mut inner = pair.clone().into_inner();
            let left = inner.next().ok_or_else(|| anyhow!("Missing left"))?;
            let right = inner.next().ok_or_else(|| anyhow!("Missing right"))?;
            let l = build_expr(left)?;
            let r = build_expr(right)?;
            match pair.as_rule() {
                Rule::plus => Ok(Expr::Add(Box::new(l), Box::new(r))),
                Rule::minus => Ok(Expr::Sub(Box::new(l), Box::new(r))),
                Rule::multiply => Ok(Expr::Mul(Box::new(l), Box::new(r))),
                Rule::divide => Ok(Expr::Div(Box::new(l), Box::new(r))),
                Rule::pow => Ok(Expr::Pow(Box::new(l), Box::new(r))),
                Rule::log => Ok(Expr::Log(Box::new(l), Box::new(r))),
                Rule::root => Ok(Expr::Root(Box::new(l), Box::new(r))),
                _ => unreachable!(),
            }
        }
        Rule::sin | Rule::cos | Rule::tan | Rule::exp | Rule::ln => {
            let mut inner = pair.clone().into_inner();
            let v = inner.next().ok_or_else(|| anyhow!("Missing argument"))?;
            let expr = build_expr(v)?;
            match pair.as_rule() {
                Rule::sin => Ok(Expr::Sin(Box::new(expr))),
                Rule::cos => Ok(Expr::Cos(Box::new(expr))),
                Rule::tan => Ok(Expr::Tan(Box::new(expr))),
                Rule::exp => Ok(Expr::Exp(Box::new(expr))),
                Rule::ln => Ok(Expr::Ln(Box::new(expr))),
                _ => unreachable!(),
            }
        }
        _ => Err(anyhow!("Unexpected rule: {:?}", pair.as_rule())),
    }
}

/// This performs runtime checks (division by zero, invalid arguments for ln/log/root)
/// and returns descriptive errors via `anyhow` when evaluation cannot proceed.
fn eval(e: &Expr) -> anyhow::Result<f64> {
    use Expr::*;
    match e {
        Num(n) => Ok(*n),
        Add(a, b) => Ok(eval(a)? + eval(b)?),
        Sub(a, b) => Ok(eval(a)? - eval(b)?),
        Mul(a, b) => Ok(eval(a)? * eval(b)?),
        Div(a, b) => {
            let rv = eval(b)?;
            if rv == 0.0 {
                Err(anyhow!("Division by zero"))
            } else {
                Ok(eval(a)? / rv)
            }
        }
        Sin(x) => Ok(eval(x)?.sin()),
        Cos(x) => Ok(eval(x)?.cos()),
        Tan(x) => Ok(eval(x)?.tan()),
        Exp(x) => Ok(eval(x)?.exp()),
        Pow(a, b) => Ok(eval(a)?.powf(eval(b)?)),
        Log(value, base) => {
            let v = eval(value)?;
            let b = eval(base)?;
            if v <= 0.0 || b <= 0.0 || b == 1.0 {
                Err(anyhow!(
                    "Invalid arguments for log(value, base): value={} base={}",
                    v,
                    b
                ))
            } else {
                Ok(v.ln() / b.ln())
            }
        }
        Ln(x) => {
            let v = eval(x)?;
            if v <= 0.0 {
                Err(anyhow!("Invalid argument for ln: {}", v))
            } else {
                Ok(v.ln())
            }
        }
        Root(value, degree) => {
            let deg = eval(degree)?;
            if deg == 0.0 {
                Err(anyhow!("Root degree cannot be zero"))
            } else {
                Ok(eval(value)?.powf(1.0 / deg))
            }
        }
    }
}

/// Parse the input string into an `Expr` AST.
///
/// Returns a descriptive error if parsing fails.
pub fn parse_expression(input: &str) -> anyhow::Result<Expr> {
    let pair = Grammar::parse(Rule::input, input)?
        .next()
        .ok_or_else(|| anyhow!("Failed to parse input"))?;
    build_expr(pair)
}

/// Evaluate a previously parsed `Expr`.
///
/// Returns the numeric result or an error if evaluation fails.
pub fn eval_expr(expr: &Expr) -> anyhow::Result<f64> {
    eval(expr)
}

/// Convenience: parse the input, evaluate it, and append the result to `res.txt`.
///
/// The function returns the computed value or an error. The output file is opened
/// in append mode and created if it does not exist.
pub fn parse_and_eval(input: &str) -> anyhow::Result<f64> {
    let e = parse_expression(input)?;
    let res = eval_expr(&e)?;

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("res.txt")
        .map_err(|e| anyhow!("Failed to open res.txt: {}", e))?;
    writeln!(file, "({}) = {}", input.trim(), res)
        .map_err(|e| anyhow!("Failed to write to res.txt: {}", e))?;

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_sum() -> anyhow::Result<()> {
        let e = parse_expression("(12+34)")?;
        assert!(matches!(e, Expr::Add(_, _)));
        Ok(())
    }
}
