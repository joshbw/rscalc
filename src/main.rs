mod commands;
mod eval;
mod parser;
mod repl;
mod state;

use clap::Parser;

use calc_manager::prelude::*;
use state::CalcState;

/// rscalc — A cross-platform CLI calculator powered by arbitrary-precision rational arithmetic.
#[derive(Parser)]
#[command(name = "rscalc", version, about, after_help = "\
INTERACTIVE MODE:
  Run without arguments to enter the REPL. In interactive mode, use these
  commands to change settings on the fly:

    :mode <standard|scientific|programmer>  Switch calculator mode
    :angle <deg|rad|grad>                   Set angle unit (trig functions)
    :base <dec|hex|oct|bin>                 Set display base
    :width <qword|dword|word|byte>          Set integer width (programmer)
    :fe                                     Toggle fixed/scientific notation
    :precision <n>                          Set display precision (1-128)
    :history                                Show calculation history
    :help [topic]                           Help on: trig, hyper, exp, prog, functions
    :quit                                   Exit

  Memory: ms (store) mr (recall) mc (clear) m+ (add) m- (subtract)
  Tab completion is available for commands and function names.")]
struct Cli {
    /// Evaluate expression and exit (positional)
    #[arg(value_name = "EXPRESSION")]
    expression: Option<String>,

    /// Start in mode: standard, scientific, programmer
    #[arg(short, long, value_name = "MODE")]
    mode: Option<String>,

    /// Angle unit: deg, rad, grad
    #[arg(short, long, value_name = "ANGLE")]
    angle: Option<String>,

    /// Number base: dec, hex, oct, bin
    #[arg(short, long, value_name = "BASE")]
    base: Option<String>,

    /// Display precision (digits)
    #[arg(short, long, value_name = "N")]
    precision: Option<i32>,

    /// Evaluate expression and exit (alternative to positional)
    #[arg(short, long, value_name = "EXPR", allow_hyphen_values = true)]
    expr: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let mut state = CalcState::new();

    // Apply CLI options
    if let Some(mode) = &cli.mode {
        match mode.to_lowercase().as_str() {
            "standard" | "std" => state.settings.mode = CalculatorMode::Standard,
            "scientific" | "sci" => state.settings.mode = CalculatorMode::Scientific,
            "programmer" | "prog" => state.settings.mode = CalculatorMode::Programmer,
            _ => {
                eprintln!("Unknown mode '{mode}'. Options: standard, scientific, programmer");
                std::process::exit(1);
            }
        }
    }

    if let Some(angle) = &cli.angle {
        match angle.to_lowercase().as_str() {
            "deg" | "degrees" => state.settings.angle_type = AngleType::Degrees,
            "rad" | "radians" => state.settings.angle_type = AngleType::Radians,
            "grad" | "gradians" => state.settings.angle_type = AngleType::Gradians,
            _ => {
                eprintln!("Unknown angle unit '{angle}'. Options: deg, rad, grad");
                std::process::exit(1);
            }
        }
    }

    if let Some(base) = &cli.base {
        match base.to_lowercase().as_str() {
            "dec" | "decimal" => state.settings.radix_type = RadixType::Decimal,
            "hex" | "hexadecimal" => state.settings.radix_type = RadixType::Hex,
            "oct" | "octal" => state.settings.radix_type = RadixType::Octal,
            "bin" | "binary" => state.settings.radix_type = RadixType::Binary,
            _ => {
                eprintln!("Unknown base '{base}'. Options: dec, hex, oct, bin");
                std::process::exit(1);
            }
        }
    }

    if let Some(p) = cli.precision {
        if p > 0 && p <= 128 {
            state.settings.display_precision = Some(p);
        } else {
            eprintln!("Precision must be between 1 and 128");
            std::process::exit(1);
        }
    }

    // Determine expression (positional or --expr flag)
    let expression = cli.expression.or(cli.expr);

    if let Some(expr) = expression {
        // Non-interactive mode: evaluate and exit
        run_oneshot(&expr, &mut state);
    } else {
        // Interactive mode: launch REPL
        repl::run_repl(&mut state);
    }
}

/// Evaluate a single expression and print the result (non-interactive).
fn run_oneshot(input: &str, state: &mut CalcState) {
    let expr = match crate::parser::Parser::parse(input, state.settings.mode) {
        Ok(e) => e,
        Err(msg) => {
            eprintln!("Error: {msg}");
            std::process::exit(1);
        }
    };

    let result = match eval::evaluate(&expr, state) {
        Ok(r) => r,
        Err(msg) => {
            eprintln!("Error: {msg}");
            std::process::exit(1);
        }
    };

    let display_str = if state.settings.mode == CalculatorMode::Programmer {
        eval::display::format_programmer_result(
            &result,
            state.settings.radix_type,
            state.settings.precision(),
        )
    } else {
        eval::display::format_result(
            &result,
            state.settings.radix_type,
            state.settings.precision(),
            state.settings.number_format,
        )
    };

    println!("{display_str}");
}
