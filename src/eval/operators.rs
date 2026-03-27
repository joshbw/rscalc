/// Binary and unary operator dispatch to ratpack functions.
use calc_manager::prelude::*;
use calc_manager::ratpack::arithmetic::{add_rat, div_rat, mul_rat, rem_rat, sub_rat};
use calc_manager::ratpack::constants::RatpackConstants;
use calc_manager::ratpack::exp::pow_rat;
use calc_manager::ratpack::logic::{and_rat, lsh_rat, or_rat, rsh_rat, xor_rat};

use crate::parser::ast::BinaryOperator;

/// Maximum result magnitude (in log2 bits) for power operations.
/// Beyond this, ratpack's internal `exp_rat → rat_pow_i32(e, N)` path
/// generates mantissas with millions of digits, causing computation to hang.
/// 4096 bits ≈ ~1233 decimal digits — generous for practical calculations.
const MAX_POW_RESULT_LOG2: f64 = 4096.0;

/// Approximate a `Number` (BASEX representation) as f64.
/// Only uses the most-significant digit, so accuracy is ~31 bits.
fn num_approx_f64(n: &Number) -> f64 {
    if n.mantissa.is_empty() || n.is_zero() {
        return 0.0;
    }
    let msd = f64::from(*n.mantissa.last().unwrap_or(&0));
    let power = f64::from(n.mantissa.len() as i32 - 1 + n.exp);
    msd * f64::exp2(31.0 * power)
}

/// Approximate a `Rational` as f64 for magnitude estimation.
fn rat_approx_f64(r: &Rational) -> f64 {
    let q_f64 = num_approx_f64(r.q());
    if q_f64 == 0.0 {
        return f64::INFINITY;
    }
    let p_sign = f64::from(r.p().sign);
    num_approx_f64(r.p()) * p_sign / q_f64
}

/// Evaluate a binary operation on two Rational values.
pub fn eval_binary_op(
    op: BinaryOperator,
    left: &Rational,
    right: &Rational,
    radix: u32,
    precision: i32,
    constants: &RatpackConstants,
    word_mask: u64,
) -> Result<Rational, CalcError> {
    match op {
        BinaryOperator::Add => Ok(add_rat(left, right, precision)),
        BinaryOperator::Subtract => Ok(sub_rat(left, right, precision)),
        BinaryOperator::Multiply => Ok(mul_rat(left, right, precision)),
        BinaryOperator::Divide => div_rat(left, right, precision),
        BinaryOperator::Modulo => {
            let result = left.dup();
            let b = right.dup();
            rem_rat(&result, &b)?;
            Ok(result)
        }
        BinaryOperator::Power => {
            // Guard against impractically large exponentiation.
            // When the exponent is non-integer, ratpack uses
            // exp(y * ln(x)) → rat_pow_i32(e, N) where N = ⌊y·ln(x)⌋.
            // The constant `e` has ~256 BASEX digits, so squaring it
            // ~13 times produces millions of digits and hangs.
            // Reject if estimated |result| would exceed MAX_POW_RESULT_LOG2 bits.
            let base_f64 = rat_approx_f64(left).abs();
            let exp_f64 = rat_approx_f64(right).abs();
            if base_f64 > 0.0 && (base_f64 - 1.0).abs() > f64::EPSILON && exp_f64 > 1.0 {
                let result_log2 = exp_f64 * base_f64.log2().abs();
                if result_log2 > MAX_POW_RESULT_LOG2 || result_log2.is_nan() {
                    return Err(CalcError::Overflow);
                }
            }

            let mut result = left.dup();
            pow_rat(&mut result, right, radix, precision, constants)?;
            Ok(result)
        }
        BinaryOperator::BitwiseAnd => {
            let mut result = left.dup();
            and_rat(&mut result, right, radix, precision)?;
            Ok(result)
        }
        BinaryOperator::BitwiseOr => {
            let mut result = left.dup();
            or_rat(&mut result, right, radix, precision)?;
            Ok(result)
        }
        BinaryOperator::BitwiseXor => {
            let mut result = left.dup();
            xor_rat(&mut result, right, radix, precision)?;
            Ok(result)
        }
        BinaryOperator::BitwiseNand => {
            let mut result = left.dup();
            and_rat(&mut result, right, radix, precision)?;
            let val = result
                .to_u64(radix, precision)
                .map_err(|_| CalcError::Domain)?;
            Ok(Rational::from_u64((!val) & word_mask))
        }
        BinaryOperator::BitwiseNor => {
            let mut result = left.dup();
            or_rat(&mut result, right, radix, precision)?;
            let val = result
                .to_u64(radix, precision)
                .map_err(|_| CalcError::Domain)?;
            Ok(Rational::from_u64((!val) & word_mask))
        }
        BinaryOperator::ShiftLeft => {
            let mut result = left.dup();
            lsh_rat(&mut result, right, radix, precision)?;
            Ok(result)
        }
        BinaryOperator::ShiftRight => {
            let mut result = left.dup();
            rsh_rat(&mut result, right, radix, precision)?;
            Ok(result)
        }
        BinaryOperator::LogicalShiftRight => {
            // Convert to unsigned integer, shift, convert back
            let val = left
                .to_u64(radix, precision)
                .map_err(|_| CalcError::Domain)?;
            let masked = val & word_mask;
            let shift = right
                .to_u64(radix, precision)
                .map_err(|_| CalcError::Domain)?;
            let result = if shift >= 64 { 0 } else { masked >> shift };
            Ok(Rational::from_u64(result))
        }
    }
}
