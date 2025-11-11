use math_expression_parser::{Expr, eval_expr, parse_and_eval, parse_expression};

fn approx_eq(a: f64, b: f64, eps: f64) -> bool {
    (a - b).abs() <= eps
}

#[test]
fn parse_num_integer() -> anyhow::Result<()> {
    let e = parse_expression("123")?;
    assert!(matches!(e, Expr::Num(n) if (n - 123.0).abs() < 1e-12));
    Ok(())
}

#[test]
fn parse_num_decimal() -> anyhow::Result<()> {
    let e = parse_expression("3.14")?;
    assert!(matches!(e, Expr::Num(n) if (n - 3.14).abs() < 1e-12));
    Ok(())
}

#[test]
fn parse_plus_and_eval() -> anyhow::Result<()> {
    let e = parse_expression("(12+34)")?;
    assert!(matches!(e, Expr::Add(_, _)));
    let v = eval_expr(&e)?;
    assert_eq!(v, 46.0);
    Ok(())
}

#[test]
fn parse_minus_and_eval() -> anyhow::Result<()> {
    let e = parse_expression("(5-3)")?;
    assert!(matches!(e, Expr::Sub(_, _)));
    let v = eval_expr(&e)?;
    assert_eq!(v, 2.0);
    Ok(())
}

#[test]
fn parse_multiply_and_eval() -> anyhow::Result<()> {
    let e = parse_expression("(2*3)")?;
    assert!(matches!(e, Expr::Mul(_, _)));
    let v = eval_expr(&e)?;
    assert_eq!(v, 6.0);
    Ok(())
}

#[test]
fn parse_divide_and_eval() -> anyhow::Result<()> {
    let e = parse_expression("(10/2)")?;
    assert!(matches!(e, Expr::Div(_, _)));
    let v = eval_expr(&e)?;
    assert_eq!(v, 5.0);
    Ok(())
}

#[test]
fn divide_by_zero_error() {
    let res = parse_and_eval("(1/0)");
    assert!(res.is_err());
}

#[test]
fn parse_sin_and_eval() -> anyhow::Result<()> {
    let e = parse_expression("sin(0)")?;
    assert!(matches!(e, Expr::Sin(_)));
    let v = eval_expr(&e)?;
    assert!(approx_eq(v, 0.0, 1e-12));
    Ok(())
}

#[test]
fn parse_cos_and_eval() -> anyhow::Result<()> {
    let e = parse_expression("cos(0)")?;
    assert!(matches!(e, Expr::Cos(_)));
    let v = eval_expr(&e)?;
    assert!(approx_eq(v, 1.0, 1e-12));
    Ok(())
}

#[test]
fn parse_tan_and_eval() -> anyhow::Result<()> {
    let e = parse_expression("tan(0)")?;
    assert!(matches!(e, Expr::Tan(_)));
    let v = eval_expr(&e)?;
    assert!(approx_eq(v, 0.0, 1e-12));
    Ok(())
}

#[test]
fn parse_exp_and_eval() -> anyhow::Result<()> {
    let e = parse_expression("exp(1)")?;
    assert!(matches!(e, Expr::Exp(_)));
    let v = eval_expr(&e)?;
    assert!(approx_eq(v, std::f64::consts::E, 1e-12));
    Ok(())
}

#[test]
fn parse_pow_and_eval() -> anyhow::Result<()> {
    let e = parse_expression("pow(2,3)")?;
    assert!(matches!(e, Expr::Pow(_, _)));
    let v = eval_expr(&e)?;
    assert!(approx_eq(v, 8.0, 1e-12));
    Ok(())
}

#[test]
fn parse_root_and_eval() -> anyhow::Result<()> {
    let e = parse_expression("root(27,3)")?;
    assert!(matches!(e, Expr::Root(_, _)));
    let v = eval_expr(&e)?;
    assert!(approx_eq(v, 3.0, 1e-12));
    Ok(())
}

#[test]
fn parse_log_and_eval() -> anyhow::Result<()> {
    let e = parse_expression("log(8,2)")?;
    assert!(matches!(e, Expr::Log(_, _)));
    let v = eval_expr(&e)?;
    assert!(approx_eq(v, 3.0, 1e-12));
    Ok(())
}

#[test]
fn parse_ln_and_eval() -> anyhow::Result<()> {
    let e = parse_expression("ln(2.718281828459045)")?;
    assert!(matches!(e, Expr::Ln(_)));
    let v = eval_expr(&e)?;
    assert!(approx_eq(v, 1.0, 1e-12));
    Ok(())
}

#[test]
fn nested_expression_eval() -> anyhow::Result<()> {
    // ((1+2)*(3+4)) == 3 * 7 == 21
    let v = parse_and_eval("((1+2)*(3+4))")?;
    assert_eq!(v, 21.0);
    Ok(())
}

#[test]
fn invalid_ln_and_log_errors() {
    // ln of non-positive is an error
    assert!(parse_and_eval("ln(0)").is_err());
    assert!(parse_and_eval("ln(-1)").is_err());
    // log with invalid base or value
    assert!(parse_and_eval("log(0,2)").is_err());
    assert!(parse_and_eval("log(8,1)").is_err());
}
