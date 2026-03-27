/// Function registry: maps function names to ratpack calls.
use calc_manager::prelude::*;
use calc_manager::ratpack::arithmetic::{add_rat, div_rat, mul_rat, sub_rat};
use calc_manager::ratpack::constants::RatpackConstants;
use calc_manager::ratpack::exp::{exp_rat, log_rat, log10_rat, pow_rat, root_rat};
use calc_manager::ratpack::itrans::{
    acos_angle_rat, acosh_rat, asin_angle_rat, asinh_rat, atan_angle_rat, atanh_rat,
};
use calc_manager::ratpack::support::{frac_rat, int_rat};
use calc_manager::ratpack::trans::{
    cos_angle_rat, cosh_rat, sin_angle_rat, sinh_rat, tan_angle_rat, tanh_rat,
};

/// Evaluate a named function call.
#[allow(clippy::too_many_lines)]
pub fn eval_function(
    name: &str,
    args: &[Rational],
    radix: u32,
    precision: i32,
    angle_type: AngleType,
    constants: &RatpackConstants,
) -> Result<Rational, String> {
    match name {
        // Trig functions (angle-mode aware)
        "sin" => {
            check_args(name, args, 1)?;
            let mut x = args[0].dup();
            sin_angle_rat(&mut x, angle_type, radix, precision, constants)
                .map_err(|e| format!("{e}"))?;
            Ok(x)
        }
        "cos" => {
            check_args(name, args, 1)?;
            let mut x = args[0].dup();
            cos_angle_rat(&mut x, angle_type, radix, precision, constants)
                .map_err(|e| format!("{e}"))?;
            Ok(x)
        }
        "tan" => {
            check_args(name, args, 1)?;
            let mut x = args[0].dup();
            tan_angle_rat(&mut x, angle_type, radix, precision, constants)
                .map_err(|e| format!("{e}"))?;
            Ok(x)
        }
        // Inverse trig
        "asin" => {
            check_args(name, args, 1)?;
            let mut x = args[0].dup();
            asin_angle_rat(&mut x, angle_type, radix, precision, constants)
                .map_err(|e| format!("{e}"))?;
            Ok(x)
        }
        "acos" => {
            check_args(name, args, 1)?;
            let mut x = args[0].dup();
            acos_angle_rat(&mut x, angle_type, radix, precision, constants)
                .map_err(|e| format!("{e}"))?;
            Ok(x)
        }
        "atan" => {
            check_args(name, args, 1)?;
            let mut x = args[0].dup();
            atan_angle_rat(&mut x, angle_type, radix, precision, constants)
                .map_err(|e| format!("{e}"))?;
            Ok(x)
        }
        // Reciprocal trig (derived from core trig)
        "sec" => {
            check_args(name, args, 1)?;
            let mut x = args[0].dup();
            cos_angle_rat(&mut x, angle_type, radix, precision, constants)
                .map_err(|e| format!("{e}"))?;
            let one = Rational::one();
            div_rat(&one, &x, precision).map_err(|e| format!("{e}"))
        }
        "csc" => {
            check_args(name, args, 1)?;
            let mut x = args[0].dup();
            sin_angle_rat(&mut x, angle_type, radix, precision, constants)
                .map_err(|e| format!("{e}"))?;
            let one = Rational::one();
            div_rat(&one, &x, precision).map_err(|e| format!("{e}"))
        }
        "cot" => {
            check_args(name, args, 1)?;
            let mut x = args[0].dup();
            tan_angle_rat(&mut x, angle_type, radix, precision, constants)
                .map_err(|e| format!("{e}"))?;
            let one = Rational::one();
            div_rat(&one, &x, precision).map_err(|e| format!("{e}"))
        }
        // Inverse reciprocal trig: asec(x) = acos(1/x)
        "asec" => {
            check_args(name, args, 1)?;
            let one = Rational::one();
            let mut x = div_rat(&one, &args[0], precision).map_err(|e| format!("{e}"))?;
            acos_angle_rat(&mut x, angle_type, radix, precision, constants)
                .map_err(|e| format!("{e}"))?;
            Ok(x)
        }
        "acsc" => {
            check_args(name, args, 1)?;
            let one = Rational::one();
            let mut x = div_rat(&one, &args[0], precision).map_err(|e| format!("{e}"))?;
            asin_angle_rat(&mut x, angle_type, radix, precision, constants)
                .map_err(|e| format!("{e}"))?;
            Ok(x)
        }
        "acot" => {
            check_args(name, args, 1)?;
            let one = Rational::one();
            let mut x = div_rat(&one, &args[0], precision).map_err(|e| format!("{e}"))?;
            atan_angle_rat(&mut x, angle_type, radix, precision, constants)
                .map_err(|e| format!("{e}"))?;
            Ok(x)
        }
        // Hyperbolic functions
        "sinh" => {
            check_args(name, args, 1)?;
            let mut x = args[0].dup();
            sinh_rat(&mut x, radix, precision, constants).map_err(|e| format!("{e}"))?;
            Ok(x)
        }
        "cosh" => {
            check_args(name, args, 1)?;
            let mut x = args[0].dup();
            cosh_rat(&mut x, radix, precision, constants).map_err(|e| format!("{e}"))?;
            Ok(x)
        }
        "tanh" => {
            check_args(name, args, 1)?;
            let mut x = args[0].dup();
            tanh_rat(&mut x, radix, precision, constants).map_err(|e| format!("{e}"))?;
            Ok(x)
        }
        // Inverse hyperbolic
        "asinh" => {
            check_args(name, args, 1)?;
            let mut x = args[0].dup();
            asinh_rat(&mut x, radix, precision, constants).map_err(|e| format!("{e}"))?;
            Ok(x)
        }
        "acosh" => {
            check_args(name, args, 1)?;
            let mut x = args[0].dup();
            acosh_rat(&mut x, radix, precision, constants).map_err(|e| format!("{e}"))?;
            Ok(x)
        }
        "atanh" => {
            check_args(name, args, 1)?;
            let mut x = args[0].dup();
            atanh_rat(&mut x, precision, constants).map_err(|e| format!("{e}"))?;
            Ok(x)
        }
        // Reciprocal hyperbolic: sech, csch, coth
        "sech" => {
            check_args(name, args, 1)?;
            let mut x = args[0].dup();
            cosh_rat(&mut x, radix, precision, constants).map_err(|e| format!("{e}"))?;
            let one = Rational::one();
            div_rat(&one, &x, precision).map_err(|e| format!("{e}"))
        }
        "csch" => {
            check_args(name, args, 1)?;
            let mut x = args[0].dup();
            sinh_rat(&mut x, radix, precision, constants).map_err(|e| format!("{e}"))?;
            let one = Rational::one();
            div_rat(&one, &x, precision).map_err(|e| format!("{e}"))
        }
        "coth" => {
            check_args(name, args, 1)?;
            let mut x = args[0].dup();
            tanh_rat(&mut x, radix, precision, constants).map_err(|e| format!("{e}"))?;
            let one = Rational::one();
            div_rat(&one, &x, precision).map_err(|e| format!("{e}"))
        }
        // Inverse reciprocal hyperbolic
        "asech" => {
            check_args(name, args, 1)?;
            let one = Rational::one();
            let mut x = div_rat(&one, &args[0], precision).map_err(|e| format!("{e}"))?;
            acosh_rat(&mut x, radix, precision, constants).map_err(|e| format!("{e}"))?;
            Ok(x)
        }
        "acsch" => {
            check_args(name, args, 1)?;
            let one = Rational::one();
            let mut x = div_rat(&one, &args[0], precision).map_err(|e| format!("{e}"))?;
            asinh_rat(&mut x, radix, precision, constants).map_err(|e| format!("{e}"))?;
            Ok(x)
        }
        "acoth" => {
            check_args(name, args, 1)?;
            let one = Rational::one();
            let mut x = div_rat(&one, &args[0], precision).map_err(|e| format!("{e}"))?;
            atanh_rat(&mut x, precision, constants).map_err(|e| format!("{e}"))?;
            Ok(x)
        }
        // Exponential & logarithmic
        "exp" => {
            check_args(name, args, 1)?;
            let mut x = args[0].dup();
            exp_rat(&mut x, radix, precision, constants).map_err(|e| format!("{e}"))?;
            Ok(x)
        }
        "ln" => {
            check_args(name, args, 1)?;
            let mut x = args[0].dup();
            log_rat(&mut x, precision, constants).map_err(|e| format!("{e}"))?;
            Ok(x)
        }
        "log" => {
            // log(x) = log10, log(x, b) = log_b(x) = ln(x)/ln(b)
            match args.len() {
                1 => {
                    let mut x = args[0].dup();
                    log10_rat(&mut x, precision, constants).map_err(|e| format!("{e}"))?;
                    Ok(x)
                }
                2 => {
                    let mut lnx = args[0].dup();
                    log_rat(&mut lnx, precision, constants).map_err(|e| format!("{e}"))?;
                    let mut lnb = args[1].dup();
                    log_rat(&mut lnb, precision, constants).map_err(|e| format!("{e}"))?;
                    div_rat(&lnx, &lnb, precision).map_err(|e| format!("{e}"))
                }
                n => Err(format!("log() expects 1 or 2 arguments, got {n}")),
            }
        }
        "pow10" => {
            check_args(name, args, 1)?;
            let mut x = Rational::from_i32(10);
            pow_rat(&mut x, &args[0], radix, precision, constants).map_err(|e| format!("{e}"))?;
            Ok(x)
        }
        "pow2" => {
            check_args(name, args, 1)?;
            let mut x = Rational::from_i32(2);
            pow_rat(&mut x, &args[0], radix, precision, constants).map_err(|e| format!("{e}"))?;
            Ok(x)
        }
        // Roots & powers
        "sqrt" => {
            check_args(name, args, 1)?;
            let two = Rational::from_i32(2);
            let mut x = args[0].dup();
            root_rat(&mut x, &two, radix, precision, constants).map_err(|e| format!("{e}"))?;
            Ok(x)
        }
        "cbrt" => {
            check_args(name, args, 1)?;
            let three = Rational::from_i32(3);
            let mut x = args[0].dup();
            root_rat(&mut x, &three, radix, precision, constants).map_err(|e| format!("{e}"))?;
            Ok(x)
        }
        "root" => {
            check_args(name, args, 2)?;
            let mut x = args[0].dup();
            root_rat(&mut x, &args[1], radix, precision, constants).map_err(|e| format!("{e}"))?;
            Ok(x)
        }
        "sqr" => {
            check_args(name, args, 1)?;
            Ok(mul_rat(&args[0], &args[0], precision))
        }
        "cube" => {
            check_args(name, args, 1)?;
            let sq = mul_rat(&args[0], &args[0], precision);
            Ok(mul_rat(&sq, &args[0], precision))
        }
        // Other
        "abs" => {
            check_args(name, args, 1)?;
            Ok(args[0].abs())
        }
        "floor" => {
            check_args(name, args, 1)?;
            let mut x = args[0].dup();
            // floor = integer part, but adjusting for negative: if frac != 0 and negative, subtract 1
            let mut frac = x.dup();
            frac_rat(&mut frac, radix, precision).map_err(|e| format!("{e}"))?;
            int_rat(&mut x, radix, precision).map_err(|e| format!("{e}"))?;
            if !frac.is_zero() && args[0].sign() < 0 {
                x = sub_rat(&x, &Rational::one(), precision);
            }
            Ok(x)
        }
        "ceil" => {
            check_args(name, args, 1)?;
            let mut x = args[0].dup();
            let mut frac = x.dup();
            frac_rat(&mut frac, radix, precision).map_err(|e| format!("{e}"))?;
            int_rat(&mut x, radix, precision).map_err(|e| format!("{e}"))?;
            if !frac.is_zero() && args[0].sign() > 0 {
                x = add_rat(&x, &Rational::one(), precision);
            }
            Ok(x)
        }
        "trunc" => {
            check_args(name, args, 1)?;
            let mut x = args[0].dup();
            int_rat(&mut x, radix, precision).map_err(|e| format!("{e}"))?;
            Ok(x)
        }
        "recip" => {
            check_args(name, args, 1)?;
            let one = Rational::one();
            div_rat(&one, &args[0], precision).map_err(|e| format!("{e}"))
        }
        "rand" => {
            if !args.is_empty() {
                return Err("rand() takes no arguments".to_string());
            }
            // Simple random using std — generates a value in [0, 1)
            let val = pseudo_random_rational(precision);
            Ok(val)
        }
        _ => Err(format!(
            "Unknown function '{name}'. Type :help for available functions."
        )),
    }
}

fn check_args(name: &str, args: &[Rational], expected: usize) -> Result<(), String> {
    if args.len() == expected {
        Ok(())
    } else {
        Err(format!(
            "{name}() expects {expected} argument{}, got {}",
            if expected == 1 { "" } else { "s" },
            args.len()
        ))
    }
}

/// Generate a pseudo-random Rational in [0, 1) using std random.
fn pseudo_random_rational(precision: i32) -> Rational {
    use std::time::SystemTime;
    let seed = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos();

    // LCG-based simple random
    let a: u64 = 6_364_136_223_846_793_005;
    let c: u64 = 1_442_695_040_888_963_407;
    let val = u64::from(seed);
    let val = val.wrapping_mul(a).wrapping_add(c);

    // Create fraction val / 2^32 to get [0, 1)
    let num = Rational::from_u64(val >> 32);
    let denom = Rational::from_u64(1u64 << 32);
    calc_manager::ratpack::arithmetic::div_rat(&num, &denom, precision)
        .unwrap_or_else(|_| Rational::zero())
}

/// List of all known function names, for tab-completion and help.
pub fn function_names() -> &'static [&'static str] {
    &[
        "sin", "cos", "tan", "asin", "acos", "atan", "sec", "csc", "cot", "asec", "acsc", "acot",
        "sinh", "cosh", "tanh", "asinh", "acosh", "atanh", "sech", "csch", "coth", "asech",
        "acsch", "acoth", "exp", "ln", "log", "pow10", "pow2", "sqrt", "cbrt", "root", "sqr",
        "cube", "abs", "floor", "ceil", "trunc", "recip", "rand",
    ]
}
