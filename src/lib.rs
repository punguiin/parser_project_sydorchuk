use anyhow::anyhow;
use pest::Parser;
use pest::iterators::Pair;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct Grammar;

#[derive(Debug, Clone)]
pub enum Expr {
    Num(f64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Sin(Box<Expr>),
    Cos(Box<Expr>),
    Tan(Box<Expr>),
    Exp(Box<Expr>),
    Ln(Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),
    Root(Box<Expr>, Box<Expr>),
    Log(Box<Expr>, Box<Expr>),
}

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

pub fn parse_expression(input: &str) -> anyhow::Result<Expr> {
    let pair = Grammar::parse(Rule::input, input)?
        .next()
        .ok_or_else(|| anyhow!("Failed to parse input"))?;
    build_expr(pair)
}

pub fn eval_expr(expr: &Expr) -> anyhow::Result<f64> {
    eval(expr)
}

pub fn parse_and_eval(input: &str) -> anyhow::Result<f64> {
    let e = parse_expression(input)?;
    eval_expr(&e)
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
