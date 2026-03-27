/// REPL meta-commands: `:mode`, `:angle`, `:base`, `:width`, `:help`, etc.
use calc_manager::prelude::*;

use crate::eval::functions;
use crate::state::CalcState;

/// Result of processing a command.
pub enum CommandResult {
    /// Command executed successfully, display this message.
    Ok(String),
    /// Show help text.
    Help(String),
    /// Show history.
    History(String),
    /// Clear state.
    Clear,
    /// Quit the REPL.
    Quit,
    /// Not a command — treat as expression.
    NotACommand,
    /// Command parse error.
    Error(String),
}

/// Try to process input as a REPL command. Returns `NotACommand` if it isn't one.
pub fn try_command(input: &str, state: &mut CalcState) -> CommandResult {
    let trimmed = input.trim();

    // Memory commands (no colon prefix)
    match trimmed.to_lowercase().as_str() {
        "ms" => return cmd_memory_store(state),
        "mr" => return cmd_memory_recall(state),
        "mc" => return cmd_memory_clear(state),
        "m+" => return cmd_memory_add(state),
        "m-" => return cmd_memory_subtract(state),
        _ => {}
    }

    // Colon-prefixed commands
    if !trimmed.starts_with(':') {
        return CommandResult::NotACommand;
    }

    let parts: Vec<&str> = trimmed[1..].split_whitespace().collect();
    if parts.is_empty() {
        return CommandResult::Error("Empty command. Type :help for available commands.".into());
    }

    match parts[0].to_lowercase().as_str() {
        "mode" => cmd_mode(&parts[1..], state),
        "angle" => cmd_angle(&parts[1..], state),
        "base" => cmd_base(&parts[1..], state),
        "width" => cmd_width(&parts[1..], state),
        "fe" => {
            state.settings.toggle_notation();
            let mode_name = match state.settings.number_format {
                NumberFormat::Float => "Fixed",
                NumberFormat::Scientific => "Scientific",
                NumberFormat::Engineering => "Engineering",
            };
            CommandResult::Ok(format!("Notation: {mode_name}"))
        }
        "precision" => cmd_precision(&parts[1..], state),
        "history" => cmd_history(state),
        "clear" => {
            state.clear();
            CommandResult::Clear
        }
        "help" => cmd_help(&parts[1..]),
        "quit" | "exit" | "q" => CommandResult::Quit,
        other => CommandResult::Error(format!(
            "Unknown command ':{other}'. Type :help for available commands."
        )),
    }
}

fn cmd_mode(args: &[&str], state: &mut CalcState) -> CommandResult {
    if args.is_empty() {
        let name = match state.settings.mode {
            CalculatorMode::Standard => "Standard",
            CalculatorMode::Scientific => "Scientific",
            CalculatorMode::Programmer => "Programmer",
        };
        return CommandResult::Ok(format!("Current mode: {name}"));
    }
    match args[0].to_lowercase().as_str() {
        "standard" | "std" => {
            state.settings.mode = CalculatorMode::Standard;
            state.settings.radix_type = RadixType::Decimal;
            CommandResult::Ok("Mode: Standard".into())
        }
        "scientific" | "sci" => {
            state.settings.mode = CalculatorMode::Scientific;
            state.settings.radix_type = RadixType::Decimal;
            CommandResult::Ok("Mode: Scientific".into())
        }
        "programmer" | "prog" => {
            state.settings.mode = CalculatorMode::Programmer;
            CommandResult::Ok("Mode: Programmer".into())
        }
        other => CommandResult::Error(format!(
            "Unknown mode '{other}'. Options: standard, scientific, programmer"
        )),
    }
}

fn cmd_angle(args: &[&str], state: &mut CalcState) -> CommandResult {
    if args.is_empty() {
        let name = match state.settings.angle_type {
            AngleType::Degrees => "Degrees",
            AngleType::Radians => "Radians",
            AngleType::Gradians => "Gradians",
        };
        return CommandResult::Ok(format!("Angle unit: {name}"));
    }
    match args[0].to_lowercase().as_str() {
        "deg" | "degrees" => {
            state.settings.angle_type = AngleType::Degrees;
            CommandResult::Ok("Angle unit: Degrees".into())
        }
        "rad" | "radians" => {
            state.settings.angle_type = AngleType::Radians;
            CommandResult::Ok("Angle unit: Radians".into())
        }
        "grad" | "gradians" => {
            state.settings.angle_type = AngleType::Gradians;
            CommandResult::Ok("Angle unit: Gradians".into())
        }
        other => CommandResult::Error(format!(
            "Unknown angle unit '{other}'. Options: deg, rad, grad"
        )),
    }
}

fn cmd_base(args: &[&str], state: &mut CalcState) -> CommandResult {
    if args.is_empty() {
        let name = match state.settings.radix_type {
            RadixType::Decimal => "Decimal",
            RadixType::Hex => "Hexadecimal",
            RadixType::Octal => "Octal",
            RadixType::Binary => "Binary",
        };
        return CommandResult::Ok(format!("Display base: {name}"));
    }
    match args[0].to_lowercase().as_str() {
        "dec" | "decimal" | "10" => {
            state.settings.radix_type = RadixType::Decimal;
            CommandResult::Ok("Base: Decimal".into())
        }
        "hex" | "hexadecimal" | "16" => {
            state.settings.radix_type = RadixType::Hex;
            CommandResult::Ok("Base: Hexadecimal".into())
        }
        "oct" | "octal" | "8" => {
            state.settings.radix_type = RadixType::Octal;
            CommandResult::Ok("Base: Octal".into())
        }
        "bin" | "binary" | "2" => {
            state.settings.radix_type = RadixType::Binary;
            CommandResult::Ok("Base: Binary".into())
        }
        other => CommandResult::Error(format!(
            "Unknown base '{other}'. Options: dec, hex, oct, bin"
        )),
    }
}

fn cmd_width(args: &[&str], state: &mut CalcState) -> CommandResult {
    if args.is_empty() {
        let name = match state.settings.num_width {
            NumWidth::QWord => "QWord (64-bit)",
            NumWidth::DWord => "DWord (32-bit)",
            NumWidth::Word => "Word (16-bit)",
            NumWidth::Byte => "Byte (8-bit)",
        };
        return CommandResult::Ok(format!("Word width: {name}"));
    }
    match args[0].to_lowercase().as_str() {
        "qword" | "64" => {
            state.settings.num_width = NumWidth::QWord;
            CommandResult::Ok("Width: QWord (64-bit)".into())
        }
        "dword" | "32" => {
            state.settings.num_width = NumWidth::DWord;
            CommandResult::Ok("Width: DWord (32-bit)".into())
        }
        "word" | "16" => {
            state.settings.num_width = NumWidth::Word;
            CommandResult::Ok("Width: Word (16-bit)".into())
        }
        "byte" | "8" => {
            state.settings.num_width = NumWidth::Byte;
            CommandResult::Ok("Width: Byte (8-bit)".into())
        }
        other => CommandResult::Error(format!(
            "Unknown width '{other}'. Options: qword, dword, word, byte"
        )),
    }
}

fn cmd_precision(args: &[&str], state: &mut CalcState) -> CommandResult {
    if args.is_empty() {
        let p = state.settings.precision();
        return CommandResult::Ok(format!("Precision: {p} digits"));
    }
    match args[0].parse::<i32>() {
        Ok(n) if n > 0 && n <= 128 => {
            state.settings.display_precision = Some(n);
            CommandResult::Ok(format!("Precision: {n} digits"))
        }
        Ok(_) => CommandResult::Error("Precision must be between 1 and 128".into()),
        Err(_) => CommandResult::Error("Invalid precision value".into()),
    }
}

fn cmd_history(state: &CalcState) -> CommandResult {
    if state.history.is_empty() {
        return CommandResult::History("No history yet.".into());
    }
    let mut lines = Vec::new();
    for (i, entry) in state.history.entries().iter().enumerate() {
        lines.push(format!(
            "  [{}] {} = {}",
            i + 1,
            entry.expression,
            entry.result
        ));
    }
    CommandResult::History(lines.join("\n"))
}

fn cmd_memory_store(state: &mut CalcState) -> CommandResult {
    match &state.last_result {
        Some(val) => {
            state.memory.store(val);
            CommandResult::Ok("Memory stored".into())
        }
        None => CommandResult::Error("No result to store".into()),
    }
}

fn cmd_memory_recall(state: &CalcState) -> CommandResult {
    match state.memory.recall() {
        Some(val) => {
            let s = crate::eval::display::format_result(
                val,
                state.settings.radix_type,
                state.settings.precision(),
                state.settings.number_format,
            );
            CommandResult::Ok(format!("Memory: {s}"))
        }
        None => CommandResult::Error("Memory is empty".into()),
    }
}

fn cmd_memory_clear(state: &mut CalcState) -> CommandResult {
    state.memory.clear();
    CommandResult::Ok("Memory cleared".into())
}

fn cmd_memory_add(state: &mut CalcState) -> CommandResult {
    match &state.last_result {
        Some(val) => {
            let precision = state.settings.precision();
            let val = val.dup();
            state.memory.add(&val, precision);
            CommandResult::Ok("Memory updated (M+)".into())
        }
        None => CommandResult::Error("No result to add".into()),
    }
}

fn cmd_memory_subtract(state: &mut CalcState) -> CommandResult {
    match &state.last_result {
        Some(val) => {
            let precision = state.settings.precision();
            let val = val.dup();
            state.memory.subtract(&val, precision);
            CommandResult::Ok("Memory updated (M-)".into())
        }
        None => CommandResult::Error("No result to subtract".into()),
    }
}

fn cmd_help(args: &[&str]) -> CommandResult {
    if args.is_empty() {
        return CommandResult::Help(general_help());
    }
    match args[0].to_lowercase().as_str() {
        "trig" => CommandResult::Help(trig_help()),
        "hyper" | "hyperbolic" => CommandResult::Help(hyperbolic_help()),
        "exp" | "log" => CommandResult::Help(exp_log_help()),
        "prog" | "programmer" => CommandResult::Help(programmer_help()),
        "commands" | "cmd" => CommandResult::Help(commands_help()),
        "functions" | "func" => CommandResult::Help(all_functions_help()),
        _ => CommandResult::Help(general_help()),
    }
}

fn general_help() -> String {
    format!(
        r"rscalc — CLI Calculator

  Type mathematical expressions to evaluate them.
  Examples:  2 + 3    sin(pi/6)    2^10    sqrt(2)    5!

Commands:
  :mode <standard|scientific|programmer>  Switch calculator mode
  :angle <deg|rad|grad>                   Set angle unit
  :base <dec|hex|oct|bin>                 Set display base
  :width <qword|dword|word|byte>          Set integer width
  :fe                                     Toggle fixed/scientific notation
  :precision <n>                          Set display precision
  :history                                Show calculation history
  :clear                                  Clear calculator state
  :help <topic>                           Help on: trig, hyper, exp, prog, functions, commands
  :quit                                   Exit

Memory:  ms (store)  mr (recall)  mc (clear)  m+ (add)  m- (subtract)
Constants:  pi (π)  e  ans (last result)

Available functions: {}
",
        functions::function_names().join(", ")
    )
}

fn trig_help() -> String {
    r"Trigonometric Functions (affected by :angle setting):
  sin(x)   cos(x)   tan(x)     — basic trig
  asin(x)  acos(x)  atan(x)    — inverse trig
  sec(x)   csc(x)   cot(x)     — reciprocal trig
  asec(x)  acsc(x)  acot(x)    — inverse reciprocal trig

  Examples:
    sin(pi/6)         → 0.5  (with :angle rad)
    sin(30)           → 0.5  (with :angle deg)
    asin(0.5)         → 30   (with :angle deg)
"
    .to_string()
}

fn hyperbolic_help() -> String {
    r"Hyperbolic Functions:
  sinh(x)   cosh(x)   tanh(x)     — basic hyperbolic
  asinh(x)  acosh(x)  atanh(x)    — inverse hyperbolic
  sech(x)   csch(x)   coth(x)     — reciprocal hyperbolic
  asech(x)  acsch(x)  acoth(x)    — inverse reciprocal hyperbolic
"
    .to_string()
}

fn exp_log_help() -> String {
    r"Exponential & Logarithmic Functions:
  exp(x)       — e^x
  ln(x)        — natural logarithm
  log(x)       — base-10 logarithm
  log(x, b)    — logarithm base b
  pow10(x)     — 10^x
  pow2(x)      — 2^x
  sqrt(x)      — square root
  cbrt(x)      — cube root
  root(x, n)   — nth root of x
  sqr(x)       — x²
  cube(x)      — x³
"
    .to_string()
}

fn programmer_help() -> String {
    r"Programmer Mode (:mode programmer):
  Operators: & (AND)  | (OR)  ^ (XOR)  ~ (NOT)  << (shift left)  >> (shift right)
  Literals:  0xFF (hex)  0o77 (octal)  0b1010 (binary)
  Commands:  :base hex/dec/oct/bin    :width qword/dword/word/byte

  Note: In programmer mode, ^ is XOR (use ** for exponentiation)

  Example:
    :mode prog
    :base hex
    0xFF & 0xF0    → 0xF0
    1 << 10        → 0x400
    ~0xFF :byte    → 0x00
"
    .to_string()
}

fn commands_help() -> String {
    r"REPL Commands:
  :mode standard|sci|prog    Switch calculator mode
  :angle deg|rad|grad        Set angle unit (affects trig functions)
  :base dec|hex|oct|bin      Set display base (programmer mode)
  :width qword|dword|word|byte  Set integer width (programmer mode)
  :fe                        Toggle fixed/scientific notation
  :precision <n>             Set display precision (1-128 digits)
  :history                   Show calculation history
  :clear                     Clear all state
  :help [topic]              Show help
  :quit                      Exit rscalc

  Memory: ms  mr  mc  m+  m-
"
    .to_string()
}

fn all_functions_help() -> String {
    r"All Available Functions:

  Trig:        sin cos tan asin acos atan sec csc cot asec acsc acot
  Hyperbolic:  sinh cosh tanh asinh acosh atanh sech csch coth asech acsch acoth
  Exp/Log:     exp ln log pow10 pow2
  Roots:       sqrt cbrt root
  Powers:      sqr cube (also: x! for factorial, x^y or x**y)
  Other:       abs floor ceil trunc recip rand

  Constants:   pi (π)  e  ans
"
    .to_string()
}

/// Get all completable command/function names for tab completion.
pub fn completable_names() -> Vec<String> {
    let mut names: Vec<String> = Vec::new();

    // Commands
    for cmd in &[
        ":mode",
        ":angle",
        ":base",
        ":width",
        ":fe",
        ":precision",
        ":history",
        ":clear",
        ":help",
        ":quit",
        ":exit",
    ] {
        names.push((*cmd).to_string());
    }

    // Mode values
    for val in &["standard", "scientific", "programmer", "std", "sci", "prog"] {
        names.push((*val).to_string());
    }

    // Angle values
    for val in &["deg", "rad", "grad"] {
        names.push((*val).to_string());
    }

    // Base values
    for val in &["dec", "hex", "oct", "bin"] {
        names.push((*val).to_string());
    }

    // Width values
    for val in &["qword", "dword", "word", "byte"] {
        names.push((*val).to_string());
    }

    // Memory commands
    for cmd in &["ms", "mr", "mc", "m+", "m-"] {
        names.push((*cmd).to_string());
    }

    // Function names
    for name in functions::function_names() {
        names.push((*name).to_string());
    }

    // Constants
    names.push("pi".to_string());
    names.push("e".to_string());
    names.push("ans".to_string());

    names
}
