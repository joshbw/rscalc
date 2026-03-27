/// Binary and unary operator dispatch to ratpack functions.

use calc_manager::prelude::*;
use calc_manager::ratpack::arithmetic::{
    add_rat, div_rat, mul_rat, rem_rat, sub_rat,
};
use calc_manager::ratpack::constants::RatpackConstants;
use calc_manager::ratpack::exp::pow_rat;
use calc_manager::ratpack::logic::{and_rat, lsh_rat, or_rat, rsh_rat, xor_rat};

use crate::parser::ast::BinaryOperator;

/// Evaluate a binary operation on two Rational values.
pub fn eval_binary_op(
    op: BinaryOperator,
    left: &Rational,
    right: &Rational,
    radix: u32,
    precision: i32,
    constants: &RatpackConstants,
) -> Result<Rational, CalcError> {
    match op {
        BinaryOperator::Add => Ok(add_rat(left, right, precision)),
        BinaryOperator::Subtract => Ok(sub_rat(left, right, precision)),
        BinaryOperator::Multiply => Ok(mul_rat(left, right, precision)),
        BinaryOperator::Divide => div_rat(left, right, precision),
        BinaryOperator::Modulo => {
            let mut result = left.dup();
            let b = right.dup();
            rem_rat(&mut result, &b)?;
            Ok(result)
        }
        BinaryOperator::Power => {
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
        BinaryOperator::ShiftLeft => {
            let mut result = left.dup();
            lsh_rat(&mut result, right, radix, precision)?;
            Ok(result)
        }
        BinaryOperator::ShiftRight | BinaryOperator::LogicalShiftRight => {
            let mut result = left.dup();
            rsh_rat(&mut result, right, radix, precision)?;
            Ok(result)
        }
    }
}
