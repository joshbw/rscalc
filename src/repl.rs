/// Interactive REPL loop using reedline.
use colored::Colorize;
use reedline::{DefaultCompleter, DefaultPrompt, DefaultPromptSegment, Reedline, Signal};

use calc_manager::prelude::*;

use crate::commands::{self, CommandResult};
use crate::eval;
use crate::eval::display;
use crate::parser::Parser;
use crate::state::CalcState;

/// Build the REPL prompt string based on current state.
fn build_prompt(state: &CalcState) -> DefaultPrompt {
    let mode_str = match state.settings.mode {
        CalculatorMode::Standard => {
            return DefaultPrompt::new(
                DefaultPromptSegment::Basic("rscalc".to_string()),
                DefaultPromptSegment::Empty,
            );
        }
        CalculatorMode::Scientific => {
            let angle = match state.settings.angle_type {
                AngleType::Degrees => "DEG",
                AngleType::Radians => "RAD",
                AngleType::Gradians => "GRAD",
            };
            format!("SCI {angle}")
        }
        CalculatorMode::Programmer => {
            let base = match state.settings.radix_type {
                RadixType::Decimal => "DEC",
                RadixType::Hex => "HEX",
                RadixType::Octal => "OCT",
                RadixType::Binary => "BIN",
            };
            let width = match state.settings.num_width {
                NumWidth::QWord => "QWORD",
                NumWidth::DWord => "DWORD",
                NumWidth::Word => "WORD",
                NumWidth::Byte => "BYTE",
            };
            format!("{base} {width}")
        }
    };

    DefaultPrompt::new(
        DefaultPromptSegment::Basic(format!("rscalc [{mode_str}]")),
        DefaultPromptSegment::Empty,
    )
}

/// Run the interactive REPL.
pub fn run_repl(state: &mut CalcState) {
    // Pre-compute mathematical constants for the current mode so the first
    // evaluation doesn't pay the transcendental-computation cost.
    state.warm_constants();

    // Set up tab completion
    let completer = Box::new(DefaultCompleter::new(commands::completable_names()));

    let mut line_editor = Reedline::create().with_completer(completer);

    loop {
        let prompt = build_prompt(state);
        match line_editor.read_line(&prompt) {
            Ok(Signal::Success(line)) => {
                let input = line.trim();
                if input.is_empty() {
                    continue;
                }
                process_input(input, state);
            }
            Ok(Signal::CtrlD | Signal::CtrlC) => {
                break;
            }
            Err(err) => {
                eprintln!("{}", format!("Error: {err}").red());
                break;
            }
        }
    }
}

/// Process a single line of input (used by both REPL and non-interactive mode).
pub fn process_input(input: &str, state: &mut CalcState) {
    // Try as command first
    match commands::try_command(input, state) {
        CommandResult::Ok(msg) => {
            println!("  {}", msg.green());
        }
        CommandResult::Help(text) | CommandResult::History(text) => {
            println!("{text}");
        }
        CommandResult::Clear => {
            println!("  {}", "Cleared.".green());
        }
        CommandResult::Quit => {
            std::process::exit(0);
        }
        CommandResult::Error(msg) => {
            println!("  {}", format!("Error: {msg}").red());
        }
        CommandResult::NotACommand => {
            evaluate_expression(input, state);
        }
    }
}

/// Evaluate a mathematical expression and display the result.
fn evaluate_expression(input: &str, state: &mut CalcState) {
    // Parse
    let expr = match Parser::parse(input, state.settings.mode) {
        Ok(e) => e,
        Err(msg) => {
            println!("  {}", format!("Error: {msg}").red());
            return;
        }
    };

    // Evaluate
    let result = match eval::evaluate(&expr, state) {
        Ok(r) => r,
        Err(msg) => {
            println!("  {}", format!("Error: {msg}").red());
            return;
        }
    };

    // Format and display
    let display_str = if state.settings.mode == CalculatorMode::Programmer {
        display::format_programmer_result(
            &result,
            state.settings.radix_type,
            state.settings.precision(),
        )
    } else {
        display::format_result(
            &result,
            state.settings.radix_type,
            state.settings.precision(),
            state.settings.number_format,
        )
    };

    println!("  {}", display_str.cyan());

    // Update state
    state.history.push(input.to_string(), display_str);
    state.last_result = Some(result);
}
