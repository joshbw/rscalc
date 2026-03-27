# rscalc

A cross-platform command-line calculator powered by arbitrary-precision rational arithmetic.

**rscalc** brings the full mathematical power of Microsoft's Windows Calculator to the terminal, using the [rsCalcManager](https://github.com/joshbw/rsCalcManager) engine — a Rust port of CalcManager providing arbitrary-precision rational arithmetic, scientific functions, and programmer-mode bitwise operations.

## Features

- **Three calculator modes**: Standard, Scientific, and Programmer
- **Arbitrary-precision arithmetic** via rational numbers (no floating-point errors)
- **40+ mathematical functions**: trig, hyperbolic, logarithmic, roots, and more
- **Programmer mode**: hex/oct/bin literals, bitwise ops, multi-base display
- **Interactive REPL** with tab completion, history, and colored output
- **Non-interactive mode** for scripting: `rscalc "2 + 3"` → `5`
- **Cross-platform**: Windows, macOS, and Linux

## Installation

```bash
git clone --recurse-submodules https://github.com/joshbw/rscalc.git
cd rscalc
cargo build --release
```

The binary will be at `target/release/rscalc` (or `rscalc.exe` on Windows).

## Usage

### Interactive Mode

```bash
$ rscalc
rscalc> 2 + 3
  5
rscalc> sin(pi / 6)
  0.5
rscalc> 2 ^ 10
  1024
rscalc> sqrt(2)
  1.414213562373095
rscalc> 5!
  120
```

### Non-Interactive Mode

```bash
$ rscalc "2 + 3"
5

$ rscalc -m sci "sin(30)"
0.5

$ rscalc -m prog -b hex "255 & 0xF0"
0xF0 (240, 0o360, 0b11110000)
```

### Command-Line Options

```
Usage: rscalc [OPTIONS] [EXPRESSION]

Arguments:
  [EXPRESSION]  Evaluate expression and exit

Options:
  -m, --mode <MODE>       Start in mode: standard, scientific, programmer
  -a, --angle <ANGLE>     Angle unit: deg, rad, grad
  -b, --base <BASE>       Number base: dec, hex, oct, bin
  -p, --precision <N>     Display precision (digits)
  -e, --expr <EXPR>       Evaluate expression and exit (alternative to positional)
  -h, --help              Show help
  -V, --version           Show version
```

## Calculator Modes

### Standard Mode (default)

Basic arithmetic with 16-digit precision.

```
rscalc> 2 + 3 * 4
  14
rscalc> (1 + sqrt(5)) / 2
  1.618033988749895
```

### Scientific Mode

Full function set with 32-digit precision and angle mode selection.

```
rscalc> :mode sci
rscalc [SCI DEG]> sin(30)
  0.5
rscalc [SCI DEG]> :angle rad
rscalc [SCI RAD]> sin(pi / 6)
  0.5
rscalc [SCI RAD]> ln(e)
  1
```

### Programmer Mode

Integer arithmetic, bitwise operations, and multi-base display.

```
rscalc> :mode prog
rscalc [PROG]> 0xFF & 0xF0
  240 (0xF0, 0o360, 0b11110000)
rscalc [PROG]> 1 << 10
  1024 (0x400, 0o2000, 0b10000000000)
rscalc [PROG]> :base hex
rscalc [PROG HEX QWORD]> 255
  0xFF (255, 0o377, 0b11111111)
```

Note: In Programmer mode, `^` is bitwise XOR (use `**` for exponentiation).

## REPL Commands

| Command | Description |
|---------|-------------|
| `:mode standard\|sci\|prog` | Switch calculator mode |
| `:angle deg\|rad\|grad` | Set angle unit |
| `:base dec\|hex\|oct\|bin` | Set display base |
| `:width qword\|dword\|word\|byte` | Set integer width |
| `:fe` | Toggle fixed/scientific notation |
| `:precision <n>` | Set display precision |
| `:history` | Show calculation history |
| `:clear` | Clear calculator state |
| `:help [topic]` | Show help (topics: trig, hyper, exp, prog, functions) |
| `:quit` | Exit |

Memory commands: `ms` (store), `mr` (recall), `mc` (clear), `m+` (add), `m-` (subtract)

## Operators

| Precedence | Operators | Description |
|------------|-----------|-------------|
| Highest | `()` | Parentheses / function calls |
| | `!` (postfix) | Factorial |
| | `-`, `~` | Negation, bitwise NOT |
| | `**`, `^` | Exponentiation (right-associative) |
| | `*`, `/`, `%`, `mod` | Multiplication, division, modulo |
| | `+`, `-` | Addition, subtraction |
| | `<<`, `>>`, `>>>` | Bit shifts |
| | `&` | Bitwise AND |
| | `xor` | Bitwise XOR |
| Lowest | `\|` | Bitwise OR |

## Functions

**Trig:** sin, cos, tan, asin, acos, atan, sec, csc, cot, asec, acsc, acot
**Hyperbolic:** sinh, cosh, tanh, asinh, acosh, atanh, sech, csch, coth, asech, acsch, acoth
**Exp/Log:** exp, ln, log, pow10, pow2
**Roots:** sqrt, cbrt, root(x, n)
**Powers:** sqr, cube, x! (factorial)
**Other:** abs, floor, ceil, trunc, recip, rand

**Constants:** `pi` (π), `e`, `ans` (last result)

## Building from Source

Requirements: Rust 1.80+ (2024 edition)

```bash
git clone --recurse-submodules https://github.com/joshbw/rscalc.git
cd rscalc
cargo build --release
cargo test
```

## License

MIT
