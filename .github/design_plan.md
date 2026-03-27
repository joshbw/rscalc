# rscalc — Design Plan

A cross-platform command-line calculator powered by Microsoft's arbitrary-precision rational arithmetic engine, ported to Rust.

## 1. Project Overview

**rscalc** is an interactive CLI calculator that brings the full mathematical power of Microsoft's Windows Calculator to the terminal. It uses the [rsCalcManager](https://github.com/joshbw/rsCalcManager) crate — a Rust port of the CalcManager engine — providing arbitrary-precision rational arithmetic, scientific functions, programmer-mode bitwise operations, and base conversions.

### Goals
- Expose the **full range of math functionality** in rsCalcManager through an intuitive command-line interface
- Emulate the **three modes** of Windows Calculator: Standard, Scientific, and Programmer
- Provide a natural **expression-based** interface rather than button-press emulation
- Build for **Windows, macOS, and Linux** with CI for all three
- Keep the crate as a **git submodule** so it can be co-developed alongside the CLI

### Non-Goals (v1)
- Full TUI with button layout (future enhancement)
- Unit conversion (rsCalcManager's unit_converter is still a stub)
- Graphing capabilities

---

## 2. Architecture

```
rscalc/
├── .github/
│   ├── workflows/
│   │   └── ci.yml              # GitHub Actions: build + test (Win/Mac/Linux)
│   ├── copilot-instructions.md # Copilot coding guidelines
│   └── design_plan.md          # This file
├── calc_manager/               # Git submodule → rsCalcManager
├── src/
│   ├── main.rs                 # Entry point, CLI argument handling
│   ├── repl.rs                 # Interactive REPL loop
│   ├── parser/
│   │   ├── mod.rs              # Expression parser (Pratt parser / precedence climbing)
│   │   ├── lexer.rs            # Tokenizer: numbers, operators, functions, identifiers
│   │   ├── ast.rs              # AST node types
│   │   └── tokens.rs           # Token enum definitions
│   ├── eval/
│   │   ├── mod.rs              # Expression evaluator (walks AST, calls ratpack)
│   │   ├── functions.rs        # Function registry (sin, cos, ln, etc.)
│   │   ├── operators.rs        # Binary/unary operator dispatch
│   │   └── display.rs          # Result formatting (decimal, hex, scientific, etc.)
│   ├── state/
│   │   ├── mod.rs              # Calculator state management
│   │   ├── memory.rs           # Memory operations (MS, MR, MC, M+, M-)
│   │   ├── history.rs          # Calculation history
│   │   └── settings.rs         # Mode, angle type, radix, word width
│   └── commands.rs             # REPL meta-commands (mode, help, history, etc.)
├── tests/
│   ├── integration/
│   │   ├── arithmetic.rs       # Basic math tests
│   │   ├── scientific.rs       # Trig, log, exp tests
│   │   ├── programmer.rs       # Bitwise, base conversion tests
│   │   └── repl.rs             # End-to-end REPL interaction tests
│   └── snapshots/              # Snapshot tests for formatted output
├── Cargo.toml
├── README.md
└── LICENSE
```

### Dependency on rsCalcManager

Since the `engine` and `manager` modules in rsCalcManager are stubs (the `process_command()` method is a TODO), rscalc will work directly with the **ratpack** module which is fully ported and tested (373 tests). This gives us:

- Full control over expression parsing and evaluation
- Natural expression syntax instead of button-press command sequences
- No dependency on unfinished engine code

When the engine/manager modules are completed in rsCalcManager, rscalc could optionally switch to using them, but the direct-ratpack approach is actually more natural for a CLI.

---

## 3. CLI Interface Design

### 3.1 Expression Syntax (Primary Interface)

rscalc uses a natural mathematical expression syntax. The REPL reads expressions, evaluates them, and displays results.

```
rscalc> 2 + 3
  5

rscalc> sin(pi / 6)
  0.5

rscalc> 2 ^ 10
  1024

rscalc> (1 + sqrt(5)) / 2
  1.6180339887498948482...

rscalc> 100!
  9.332621544394415268e+157

rscalc> ans * 2
  1.866524308878883054e+158
```

### 3.2 Operator Precedence

Standard mathematical precedence (highest to lowest):

| Precedence | Operators | Description |
|------------|-----------|-------------|
| 1 (highest) | `()` | Parentheses / function calls |
| 2 | `!` (postfix) | Factorial |
| 3 | unary `-`, `~` | Negation, bitwise NOT |
| 4 | `**` or `^` | Exponentiation (right-associative) |
| 5 | `*`, `/`, `%`, `mod` | Multiplication, division, modulo |
| 6 | `+`, `-` | Addition, subtraction |
| 7 | `<<`, `>>`, `>>>` | Bit shifts (left, arithmetic right, logical right) |
| 8 | `&` | Bitwise AND |
| 9 | `xor` | Bitwise XOR |
| 10 (lowest) | `\|` | Bitwise OR |

> **Note on `^`:** In Standard and Scientific modes, `^` means exponentiation. In Programmer mode, `^` means bitwise XOR (matching C convention) and `**` is used for exponentiation. This mirrors the mental model a programmer would expect.

### 3.3 Available Functions

#### Trigonometric (affected by angle mode: DEG/RAD/GRAD)
| Function | Description | Inverse |
|----------|-------------|---------|
| `sin(x)` | Sine | `asin(x)` |
| `cos(x)` | Cosine | `acos(x)` |
| `tan(x)` | Tangent | `atan(x)` |
| `sec(x)` | Secant | `asec(x)` |
| `csc(x)` | Cosecant | `acsc(x)` |
| `cot(x)` | Cotangent | `acot(x)` |

#### Hyperbolic
| Function | Description | Inverse |
|----------|-------------|---------|
| `sinh(x)` | Hyperbolic sine | `asinh(x)` |
| `cosh(x)` | Hyperbolic cosine | `acosh(x)` |
| `tanh(x)` | Hyperbolic tangent | `atanh(x)` |
| `sech(x)` | Hyperbolic secant | `asech(x)` |
| `csch(x)` | Hyperbolic cosecant | `acsch(x)` |
| `coth(x)` | Hyperbolic cotangent | `acoth(x)` |

#### Exponential & Logarithmic
| Function | Description |
|----------|-------------|
| `exp(x)` | e^x |
| `ln(x)` | Natural logarithm |
| `log(x)` | Base-10 logarithm |
| `log(x, b)` | Logarithm base b |
| `pow10(x)` | 10^x |
| `pow2(x)` | 2^x |

#### Roots & Powers
| Function | Description |
|----------|-------------|
| `sqrt(x)` | Square root |
| `cbrt(x)` | Cube root |
| `root(x, n)` | nth root of x |
| `sqr(x)` | x² |
| `cube(x)` | x³ |
| `x!` | Factorial (postfix, via gamma function) |

#### Other
| Function | Description |
|----------|-------------|
| `abs(x)` | Absolute value |
| `floor(x)` | Floor |
| `ceil(x)` | Ceiling |
| `trunc(x)` | Truncate (integer part) |
| `recip(x)` | 1/x (reciprocal) |
| `dms(x)` | Degrees to DMS |
| `rand()` | Random number [0, 1) |

#### Constants
| Name | Value |
|------|-------|
| `pi`, `π` | 3.14159265358979... |
| `e` | 2.71828182845904... |
| `ans` | Last computed result |

### 3.4 REPL Commands

Meta-commands start with a colon `:` or are standalone keywords to distinguish them from expressions:

```
:mode standard          Switch to Standard mode (default)
:mode scientific        Switch to Scientific mode (32-digit precision)
:mode sci               Alias for scientific
:mode programmer        Switch to Programmer mode
:mode prog              Alias for programmer

:angle deg              Set angle unit to degrees (default)
:angle rad              Set angle unit to radians
:angle grad             Set angle unit to gradians

:base hex               Set display base to hexadecimal
:base dec               Set display base to decimal (default)
:base oct               Set display base to octal
:base bin               Set display base to binary

:width qword            Set integer width to 64-bit (default)
:width dword            Set integer width to 32-bit
:width word             Set integer width to 16-bit
:width byte             Set integer width to 8-bit

:fe                     Toggle fixed/scientific notation
:precision <n>          Set display precision (number of digits)

ms                      Memory store (save current result)
mr                      Memory recall
mc                      Memory clear
m+                      Memory add
m-                      Memory subtract

:history                Show calculation history
:clear                  Clear calculator state
:help                   Show help
:help <topic>           Show help on a specific topic (e.g., :help trig)
:quit / :exit / Ctrl+D  Exit
```

### 3.5 Programmer Mode Specifics

In Programmer mode, additional syntax is available:

```
rscalc [PROG]> 0xFF
  255 (0xFF, 0o377, 0b11111111)

rscalc [PROG]> 255 & 0xF0
  240 (0xF0, 0o360, 0b11110000)

rscalc [PROG]> 1 << 10
  1024 (0x400)

rscalc [PROG]> ~0xFF :byte
  0 (0x00)
```

Features:
- Integer-only arithmetic (no decimals)
- Hex (`0x`), octal (`0o`), binary (`0b`) literals
- Results show in the current base with optional multi-base display
- Bitwise operators: `&` (AND), `|` (OR), `^` (XOR), `~` (NOT), `nand`, `nor`
- Shift operators: `<<` (left), `>>` (arithmetic right), `>>>` (logical right)
- Rotate: `rol(x)`, `ror(x)`, `rolc(x)`, `rorc(x)`
- Word width constrains results (wrapping/truncation)

### 3.6 Prompt Design

The prompt shows the current mode and relevant state:

```
rscalc>                     # Standard mode (default, clean)
rscalc [SCI DEG]>           # Scientific mode, degrees
rscalc [SCI RAD]>           # Scientific mode, radians
rscalc [PROG HEX QWORD]>   # Programmer mode, hex display, 64-bit
```

### 3.7 Error Handling

Errors are displayed inline with clear messages:

```
rscalc> 1 / 0
  Error: Cannot divide by zero

rscalc> sqrt(-1)
  Error: Invalid input (domain error)

rscalc> asin(2)
  Error: Invalid input (domain error)

rscalc> unknown_func(5)
  Error: Unknown function 'unknown_func'. Type :help for available functions.
```

### 3.8 Command-Line Arguments

```
Usage: rscalc [OPTIONS] [EXPRESSION]

Arguments:
  [EXPRESSION]  Evaluate expression and exit (non-interactive mode)

Options:
  -m, --mode <MODE>     Start in mode: standard, scientific, programmer
  -a, --angle <ANGLE>   Angle unit: deg, rad, grad
  -b, --base <BASE>     Number base: dec, hex, oct, bin
  -p, --precision <N>   Display precision (digits)
  -e, --expr <EXPR>     Evaluate expression and exit (alternative to positional)
  -h, --help            Show help
  -V, --version         Show version
```

Examples:
```bash
$ rscalc "2 + 3"
5

$ rscalc -m sci "sin(pi/4)"
0.70710678118654752440

$ rscalc -m prog -b hex "255 & 0xF0"
0xF0
```

---

## 4. Implementation Plan

### Phase 1: Project Foundation
1. **Initialize Cargo project** — `cargo init` with binary target
2. **Add rsCalcManager as git submodule** at `calc_manager/`
3. **Configure Cargo.toml** — path dependency on submodule, add runtime dependencies:
   - `reedline` — modern readline with syntax highlighting, completions, history
   - `clap` (with `derive` feature) — CLI argument parsing
   - `colored` — terminal color output
4. **Verify build** — ensure `calc_manager` ratpack compiles and tests pass
5. **Create `.gitignore`** — standard Rust ignores

### Phase 2: Expression Parser
6. **Lexer/tokenizer** (`src/parser/lexer.rs`) — tokenize input into numbers (dec, hex, oct, bin), operators, function names, parentheses, identifiers
7. **AST types** (`src/parser/ast.rs`) — define expression tree: `Number`, `UnaryOp`, `BinaryOp`, `FunctionCall`, `Identifier`, `Factorial`
8. **Pratt parser** (`src/parser/mod.rs`) — precedence-climbing parser for mathematical expressions with configurable operator precedence based on mode
9. **Token definitions** (`src/parser/tokens.rs`) — `Token` enum with all token variants

### Phase 3: Evaluator
10. **Operator dispatch** (`src/eval/operators.rs`) — map AST binary/unary ops to `ratpack` functions (add_rat, sub_rat, mul_rat, div_rat, etc.)
11. **Function registry** (`src/eval/functions.rs`) — map function names (sin, cos, ln, etc.) to ratpack calls with proper angle conversion
12. **Expression evaluator** (`src/eval/mod.rs`) — recursive AST walker that produces `Rational` results
13. **Result formatter** (`src/eval/display.rs`) — convert `Rational` to display string using `ratpack::conv`, respecting base, precision, and notation settings

### Phase 4: Calculator State
14. **Settings** (`src/state/settings.rs`) — mode, angle type, radix, word width, precision, notation preference
15. **Memory** (`src/state/memory.rs`) — memory store/recall/clear/add/subtract with Vec<Rational> for multiple slots
16. **History** (`src/state/history.rs`) — ordered list of (expression_string, result) pairs
17. **State coordinator** (`src/state/mod.rs`) — unified `CalculatorState` struct

### Phase 5: REPL & Commands
18. **REPL command parser** (`src/commands.rs`) — parse `:mode`, `:angle`, `:base`, `:width`, `:help`, `:history`, etc.
19. **REPL loop** (`src/repl.rs`) — reedline-based interactive loop with prompt, expression evaluation, command handling, error display
20. **Help system** — contextual help text for modes, functions, operators
21. **Tab completion** — complete function names, commands, constants

### Phase 6: CLI Entry Point
22. **Argument parsing** (`src/main.rs`) — clap-based CLI with `--mode`, `--angle`, `--base`, `--expr`, positional expression
23. **Non-interactive mode** — evaluate single expression from args and exit
24. **Interactive mode** — launch REPL when no expression given

### Phase 7: Testing
25. **Arithmetic integration tests** — basic math operations
26. **Scientific function tests** — trig, log, exp against known values
27. **Programmer mode tests** — bitwise ops, base conversions, word widths
28. **Parser edge case tests** — operator precedence, nested parens, error recovery
29. **REPL command tests** — mode switching, memory operations
30. **Snapshot tests** — formatted output for regression testing

### Phase 8: CI/CD & Documentation
31. **GitHub Actions workflow** (`.github/workflows/ci.yml`) — matrix build for Windows, macOS, Linux; run tests, clippy, rustfmt
32. **README.md** — project description, installation, usage examples, feature list
33. **Copilot instructions** (`.github/copilot-instructions.md`) — coding conventions, architecture overview, contribution guidelines

---

## 5. Rust Dependencies

| Crate | Purpose | Notes |
|-------|---------|-------|
| `calc_manager` | Math engine | Local path dep via git submodule |
| `reedline` | REPL readline | History, completion, syntax highlighting |
| `clap` | CLI argument parsing | With `derive` feature for declarative args |
| `colored` | Terminal colors | Cross-platform ANSI color support |

Dev dependencies:
| Crate | Purpose |
|-------|---------|
| `assert_cmd` | Integration testing of CLI binary |
| `predicates` | Assertion helpers for test output |

### Why reedline over rustyline?
- Written in pure Rust (no C bindings)
- Better Windows support (important for cross-platform goal)
- Built-in syntax highlighting and menu completion
- Active development (Nushell project)

---

## 6. GitHub Actions CI

```yaml
# .github/workflows/ci.yml
name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  build:
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4
        with:
          submodules: recursive
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy, rustfmt
      - run: cargo fmt --check
      - run: cargo clippy -- -D warnings
      - run: cargo build --release
      - run: cargo test
```

---

## 7. Key Design Decisions

### Direct ratpack vs. engine/manager
The rsCalcManager `engine` and `manager` modules are stubs (`process_command()` is a TODO). Rather than waiting for those to be completed, rscalc calls ratpack functions directly. This is actually a better fit for a CLI because:
- Expression parsing is more natural than button-command sequences
- We control the evaluation pipeline
- No impedance mismatch between "buttons" and "expressions"

### Pratt parser for expressions
A Pratt parser (precedence climbing) is the standard approach for expression evaluation. It handles:
- Operator precedence and associativity
- Prefix/postfix unary operators
- Function calls with arguments
- Mode-dependent operator semantics (e.g., `^` as power vs. XOR)

### Mode-aware behavior
The calculator's behavior changes based on mode:
- **Standard**: Basic arithmetic, 16-digit precision
- **Scientific**: Full function set, 32-digit precision, angle mode selector
- **Programmer**: Integer-only, bitwise ops, base display, word width constraints

### Memory model
Following calc.exe, memory is a single value (not a stack):
- `ms` stores the current result
- `mr` recalls it
- `m+`/`m-` add/subtract from it
- `mc` clears it

---

## 8. Future Enhancements (Post-v1)

- **TUI mode** with ratatui for a visual calculator layout
- **Unit conversion** when rsCalcManager's unit_converter module is completed
- **Expression history navigation** — reuse/edit previous expressions
- **Plugin system** for custom functions
- **Persistent history** across sessions (saved to disk)
- **Configuration file** for default settings
- **Copy/paste integration** with system clipboard
- **Multiple memory slots** (`m1`, `m2`, etc.)
- **Variable assignment** (`x = 42`, `y = x * 2`)
