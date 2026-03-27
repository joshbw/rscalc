# Copilot Coding Instructions for rscalc

## Project Overview

rscalc is a cross-platform CLI calculator using the rsCalcManager engine (ratpack module) for arbitrary-precision rational arithmetic.

## Architecture

- `src/parser/` — Pratt parser: lexer → tokens → AST
- `src/eval/` — AST walker: operators, functions, display formatting
- `src/state/` — Calculator state: settings, memory, history
- `src/commands.rs` — REPL meta-commands (`:mode`, `:help`, etc.)
- `src/repl.rs` — Interactive reedline-based REPL loop
- `src/main.rs` — CLI entry point with clap argument parsing
- `calc_manager/` — Git submodule (rsCalcManager)

## Key Conventions

### Using ratpack

- All math goes through `calc_manager::ratpack` — never use f64 for calculations
- Most ratpack functions take `&mut Rational` and modify in place
- Use `.dup()` to clone a Rational before mutation
- `RatpackConstants::new(radix, precision)` must be created per-evaluation
- `rat_to_string(val, format, radix, precision)` returns `CalcResult<String>`
- Angle-aware trig functions end in `_angle_rat` (e.g., `sin_angle_rat`)
- `atanh_rat` takes 3 args (no radix), unlike most other trig functions

### Number Parsing

- Unprefixed numbers are always decimal (base 10)
- `0x`/`0o`/`0b` prefixes set the input base
- The `:base` command only affects output display, not input parsing

### Modes

- Standard: 16-digit precision, basic arithmetic
- Scientific: 32-digit precision, full function set, angle modes
- Programmer: 64-digit precision, integer-only, bitwise ops, `^` = XOR

### Testing

- Unit tests: `cargo test` (in-module tests for parser, evaluator)
- Integration tests: `tests/integration/` using `assert_cmd`
- All tests must pass on Windows, macOS, and Linux

### Code Style

- Follow existing clippy lint configuration in `Cargo.toml`
- Prefer `map_err(|e| format!("{e}"))` for CalcError → String conversion
- Use `colored` crate for terminal output (green=status, cyan=results, red=errors)
