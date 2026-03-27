/// Expression evaluator: walks the AST and produces Rational results.
pub mod display;
pub mod functions;
pub mod operators;

use calc_manager::prelude::*;
use calc_manager::ratpack::constants::RatpackConstants;
use calc_manager::ratpack::conv::string_to_rat;
use calc_manager::ratpack::fact::fact_rat;

use crate::parser::ast::{Expr, UnaryOperator};
use crate::state::CalcState;

/// Evaluate an expression AST node against the current calculator state.
pub fn evaluate(expr: &Expr, state: &CalcState) -> Result<Rational, String> {
    let radix = state.settings.radix_type.to_radix();
    let precision = state.settings.precision();
    let constants = RatpackConstants::new(radix, precision);

    eval_node(expr, state, radix, precision, &constants)
}

fn eval_node(
    expr: &Expr,
    state: &CalcState,
    radix: u32,
    precision: i32,
    constants: &RatpackConstants,
) -> Result<Rational, String> {
    match expr {
        Expr::Number(s) => parse_number(s, radix, precision),
        Expr::Ident(name) => resolve_ident(name, state, constants),
        Expr::UnaryOp { op, operand } => {
            let val = eval_node(operand, state, radix, precision, constants)?;
            eval_unary(*op, &val, radix, precision, state)
        }
        Expr::BinaryOp { op, left, right } => {
            let l = eval_node(left, state, radix, precision, constants)?;
            let r = eval_node(right, state, radix, precision, constants)?;
            operators::eval_binary_op(
                *op,
                &l,
                &r,
                radix,
                precision,
                constants,
                state.settings.word_mask(),
            )
            .map_err(|e| format!("{e}"))
        }
        Expr::Factorial(inner) => {
            let mut val = eval_node(inner, state, radix, precision, constants)?;
            fact_rat(&mut val, radix, precision, constants).map_err(|e| format!("{e}"))?;
            Ok(val)
        }
        Expr::FunctionCall { name, args } => {
            let eval_args: Result<Vec<Rational>, String> = args
                .iter()
                .map(|a| eval_node(a, state, radix, precision, constants))
                .collect();
            let eval_args = eval_args?;
            functions::eval_function(
                name,
                &eval_args,
                radix,
                precision,
                state.settings.angle_type,
                constants,
            )
        }
    }
}

/// Parse a number literal string into a Rational.
/// Prefixed numbers (0x, 0o, 0b) are parsed in their respective base.
/// Unprefixed numbers are always parsed as decimal (base 10).
fn parse_number(s: &str, _default_radix: u32, precision: i32) -> Result<Rational, String> {
    // Handle prefixed literals
    if let Some(hex) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        return string_to_rat(false, hex, false, "0", 16, precision)
            .map_err(|e| format!("Invalid hex number: {e}"));
    }
    if let Some(oct) = s.strip_prefix("0o").or_else(|| s.strip_prefix("0O")) {
        return string_to_rat(false, oct, false, "0", 8, precision)
            .map_err(|e| format!("Invalid octal number: {e}"));
    }
    if let Some(bin) = s.strip_prefix("0b").or_else(|| s.strip_prefix("0B")) {
        return string_to_rat(false, bin, false, "0", 2, precision)
            .map_err(|e| format!("Invalid binary number: {e}"));
    }

    // Handle scientific notation: split into mantissa and exponent
    if let Some(e_pos) = s.find(['e', 'E']) {
        let mantissa = &s[..e_pos];
        let exp_str = &s[e_pos + 1..];
        let (exp_neg, exp_digits) = if let Some(stripped) = exp_str.strip_prefix('-') {
            (true, stripped)
        } else if let Some(stripped) = exp_str.strip_prefix('+') {
            (false, stripped)
        } else {
            (false, exp_str)
        };
        let (mant_neg, mant_digits) = if let Some(stripped) = mantissa.strip_prefix('-') {
            (true, stripped)
        } else {
            (false, mantissa)
        };
        return string_to_rat(mant_neg, mant_digits, exp_neg, exp_digits, 10, precision)
            .map_err(|e| format!("Invalid number '{s}': {e}"));
    }

    // Plain decimal number
    let (neg, digits) = if let Some(stripped) = s.strip_prefix('-') {
        (true, stripped)
    } else {
        (false, s)
    };
    string_to_rat(neg, digits, false, "0", 10, precision)
        .map_err(|e| format!("Invalid number '{s}': {e}"))
}

/// Resolve an identifier (constant name or `ans`).
fn resolve_ident(
    name: &str,
    state: &CalcState,
    constants: &RatpackConstants,
) -> Result<Rational, String> {
    match name {
        "pi" => Ok(constants.pi.dup()),
        "e" => Ok(constants.rat_exp.dup()),
        "ans" => state
            .last_result
            .as_ref()
            .map(Rational::dup)
            .ok_or_else(|| "No previous result (ans)".to_string()),
        _ => Err(format!(
            "Unknown identifier '{name}'. Type :help for available constants."
        )),
    }
}

/// Evaluate a unary operation.
fn eval_unary(
    op: UnaryOperator,
    val: &Rational,
    radix: u32,
    precision: i32,
    state: &CalcState,
) -> Result<Rational, String> {
    match op {
        UnaryOperator::Negate => Ok(val.negate()),
        UnaryOperator::BitwiseNot => {
            // Bitwise NOT depends on word width
            let mask = state.settings.word_mask();
            let val_u64 = val
                .to_u64(radix, precision)
                .map_err(|e| format!("Cannot apply bitwise NOT: {e}"))?;
            let result = (!val_u64) & mask;
            Ok(Rational::from_u64(result))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::CalcState;

    fn eval(input: &str) -> Rational {
        let state = CalcState::new();
        let expr = crate::parser::Parser::parse(input, state.settings.mode).unwrap();
        evaluate(&expr, &state).unwrap()
    }

    fn eval_to_f64(input: &str) -> f64 {
        let result = eval(input);
        let s = calc_manager::ratpack::conv::rat_to_string(&result, NumberFormat::Float, 10, 16)
            .unwrap();
        s.trim().parse::<f64>().unwrap()
    }

    #[test]
    fn test_addition() {
        let v = eval_to_f64("2 + 3");
        assert!((v - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_multiplication() {
        let v = eval_to_f64("6 * 7");
        assert!((v - 42.0).abs() < 1e-10);
    }

    #[test]
    fn test_division() {
        let v = eval_to_f64("10 / 4");
        assert!((v - 2.5).abs() < 1e-10);
    }

    #[test]
    fn test_power() {
        let v = eval_to_f64("2 ^ 10");
        assert!((v - 1024.0).abs() < 1e-10);
    }

    #[test]
    fn test_negation() {
        let v = eval_to_f64("-5");
        assert!((v - (-5.0)).abs() < 1e-10);
    }

    #[test]
    fn test_pi_constant() {
        let v = eval_to_f64("pi");
        assert!((v - std::f64::consts::PI).abs() < 1e-10);
    }

    #[test]
    fn test_sqrt() {
        let v = eval_to_f64("sqrt(4)");
        assert!((v - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_factorial() {
        let v = eval_to_f64("5!");
        assert!((v - 120.0).abs() < 1e-10);
    }

    #[test]
    fn test_complex_expression() {
        let v = eval_to_f64("(2 + 3) * 4");
        assert!((v - 20.0).abs() < 1e-10);
    }

    #[test]
    fn test_divide_by_zero() {
        let state = CalcState::new();
        let expr = crate::parser::Parser::parse("1 / 0", state.settings.mode).unwrap();
        assert!(evaluate(&expr, &state).is_err());
    }
}
