/// Pratt parser (precedence climbing) for mathematical expressions.
///
/// Supports mode-dependent operator semantics: in Standard/Scientific mode,
/// `^` means exponentiation; in Programmer mode, `^` means XOR and `**` is
/// used for exponentiation.
pub mod ast;
pub mod lexer;
pub mod tokens;

use ast::{BinaryOperator, Expr, UnaryOperator};
use tokens::Token;

use calc_manager::prelude::CalculatorMode;

/// Maximum recursion depth for expression parsing.
const MAX_DEPTH: usize = 256;

/// Parser state wrapping a token stream.
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    mode: CalculatorMode,
    depth: usize,
}

/// Parse result type.
pub type ParseResult = Result<Expr, String>;

impl Parser {
    pub fn new(tokens: Vec<Token>, mode: CalculatorMode) -> Self {
        Self {
            tokens,
            pos: 0,
            mode,
            depth: 0,
        }
    }

    /// Parse a complete expression from the token stream.
    pub fn parse(input: &str, mode: CalculatorMode) -> ParseResult {
        let mut lexer = lexer::Lexer::new(input);
        let tokens = lexer.tokenize()?;
        let mut parser = Parser::new(tokens, mode);
        let expr = parser.parse_expr(0)?;
        if !parser.current().is_eof() {
            let tok = parser.current();
            // A leftover Number starting with '.' likely means the shell ate
            // a '^' character (CMD uses ^ as its escape character).
            if let Token::Number(s) = tok
                && s.starts_with('.')
            {
                return Err(format!(
                    "Unexpected token: {tok}\n\
                     Hint: your shell may have consumed a '^' operator. \
                     Try quoting the expression, e.g.: rscalc -e \"...\"",
                ));
            }
            return Err(format!("Unexpected token: {tok}"));
        }
        Ok(expr)
    }

    fn current(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::Eof)
    }

    fn advance(&mut self) -> Token {
        let tok = self.current().clone();
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
        tok
    }

    fn expect(&mut self, expected: &Token) -> Result<(), String> {
        if self.current() == expected {
            self.advance();
            Ok(())
        } else {
            Err(format!(
                "Expected '{}', found '{}'",
                expected,
                self.current()
            ))
        }
    }

    /// Main Pratt parsing entry: parse expression with minimum binding power.
    fn parse_expr(&mut self, min_bp: u8) -> ParseResult {
        self.depth += 1;
        if self.depth > MAX_DEPTH {
            return Err("Expression too deeply nested".to_string());
        }
        let result = self.parse_expr_inner(min_bp);
        self.depth -= 1;
        result
    }

    fn parse_expr_inner(&mut self, min_bp: u8) -> ParseResult {
        let mut lhs = self.parse_prefix()?;

        loop {
            // Handle postfix factorial
            if *self.current() == Token::Bang {
                let ((), post_bp) = Self::postfix_bp();
                if post_bp < min_bp {
                    break;
                }
                self.advance();
                lhs = Expr::Factorial(Box::new(lhs));
                continue;
            }

            let Some((l_bp, r_bp)) = self.infix_bp(self.current()) else {
                break;
            };
            if l_bp < min_bp {
                break;
            }

            let op_token = self.advance();
            let rhs = self.parse_expr(r_bp)?;
            let op = self.token_to_binop(&op_token)?;
            lhs = Expr::BinaryOp {
                op,
                left: Box::new(lhs),
                right: Box::new(rhs),
            };
        }

        Ok(lhs)
    }

    /// Parse a prefix expression: number, identifier/function, unary op, or parenthesized group.
    fn parse_prefix(&mut self) -> ParseResult {
        match self.current().clone() {
            Token::Number(s) => {
                self.advance();
                Ok(Expr::Number(s))
            }
            Token::Ident(name) => {
                self.advance();
                // Check for function call
                if *self.current() == Token::LParen {
                    self.advance(); // consume '('
                    let args = self.parse_arg_list()?;
                    self.expect(&Token::RParen)?;
                    Ok(Expr::FunctionCall { name, args })
                } else {
                    Ok(Expr::Ident(name))
                }
            }
            Token::Minus => {
                self.advance();
                let ((), r_bp) = Self::prefix_bp();
                let operand = self.parse_expr(r_bp)?;
                Ok(Expr::UnaryOp {
                    op: UnaryOperator::Negate,
                    operand: Box::new(operand),
                })
            }
            Token::Tilde => {
                self.advance();
                let ((), r_bp) = Self::prefix_bp();
                let operand = self.parse_expr(r_bp)?;
                Ok(Expr::UnaryOp {
                    op: UnaryOperator::BitwiseNot,
                    operand: Box::new(operand),
                })
            }
            Token::LParen => {
                self.advance();
                let expr = self.parse_expr(0)?;
                self.expect(&Token::RParen)?;
                Ok(expr)
            }
            tok => Err(format!("Unexpected token: {tok}")),
        }
    }

    fn parse_arg_list(&mut self) -> Result<Vec<Expr>, String> {
        let mut args = Vec::new();
        if *self.current() == Token::RParen {
            return Ok(args);
        }
        args.push(self.parse_expr(0)?);
        while *self.current() == Token::Comma {
            self.advance();
            args.push(self.parse_expr(0)?);
        }
        Ok(args)
    }

    /// Prefix binding power (unary `-` and `~`).
    fn prefix_bp() -> ((), u8) {
        // Precedence 3 in the design: higher than exponentiation
        ((), 17)
    }

    /// Postfix binding power (factorial `!`).
    fn postfix_bp() -> ((), u8) {
        // Precedence 2 in the design: highest after parentheses
        ((), 19)
    }

    /// Infix binding power. Returns (`left_bp`, `right_bp`).
    /// Left-associative: `left_bp` < `right_bp`.
    /// Right-associative: `left_bp` > `right_bp`.
    fn infix_bp(&self, tok: &Token) -> Option<(u8, u8)> {
        match tok {
            Token::Pipe => Some((1, 2)),
            // In Standard/Scientific: Caret is handled below in the power section
            // Ident("xor") is handled specially
            Token::Ampersand => Some((5, 6)),
            Token::ShiftLeft | Token::ShiftRight | Token::LogicalShiftRight => Some((7, 8)),
            Token::Plus | Token::Minus => Some((9, 10)),
            Token::Star | Token::Slash | Token::Percent => Some((11, 12)),
            // Exponentiation: right-associative
            Token::DoubleStar => Some((16, 15)),
            Token::Caret => {
                if self.mode == CalculatorMode::Programmer {
                    // XOR precedence (between AND and OR)
                    Some((3, 4))
                } else {
                    // Exponentiation, right-associative
                    Some((16, 15))
                }
            }
            // Handle keyword operators like `xor`, `nand`, `nor`
            Token::Ident(name) => match name.as_str() {
                "xor" => Some((3, 4)),
                "nand" => Some((5, 6)),
                "nor" => Some((1, 2)),
                _ => None,
            },
            _ => None,
        }
    }

    fn token_to_binop(&self, tok: &Token) -> Result<BinaryOperator, String> {
        match tok {
            Token::Plus => Ok(BinaryOperator::Add),
            Token::Minus => Ok(BinaryOperator::Subtract),
            Token::Star => Ok(BinaryOperator::Multiply),
            Token::Slash => Ok(BinaryOperator::Divide),
            Token::Percent => Ok(BinaryOperator::Modulo),
            Token::DoubleStar => Ok(BinaryOperator::Power),
            Token::Caret => {
                if self.mode == CalculatorMode::Programmer {
                    Ok(BinaryOperator::BitwiseXor)
                } else {
                    Ok(BinaryOperator::Power)
                }
            }
            Token::Ampersand => Ok(BinaryOperator::BitwiseAnd),
            Token::Pipe => Ok(BinaryOperator::BitwiseOr),
            Token::ShiftLeft => Ok(BinaryOperator::ShiftLeft),
            Token::ShiftRight => Ok(BinaryOperator::ShiftRight),
            Token::LogicalShiftRight => Ok(BinaryOperator::LogicalShiftRight),
            Token::Ident(name) => match name.as_str() {
                "xor" => Ok(BinaryOperator::BitwiseXor),
                "nand" => Ok(BinaryOperator::BitwiseNand),
                "nor" => Ok(BinaryOperator::BitwiseNor),
                _ => Err(format!("Unknown binary operator: {name}")),
            },
            _ => Err(format!("Expected binary operator, found: {tok}")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ast::{BinaryOperator, Expr, UnaryOperator};

    fn parse(input: &str) -> Expr {
        Parser::parse(input, CalculatorMode::Standard).unwrap()
    }

    fn parse_prog(input: &str) -> Expr {
        Parser::parse(input, CalculatorMode::Programmer).unwrap()
    }

    #[test]
    fn test_simple_addition() {
        match parse("2 + 3") {
            Expr::BinaryOp {
                op: BinaryOperator::Add,
                left,
                right,
            } => {
                assert!(matches!(*left, Expr::Number(ref s) if s == "2"));
                assert!(matches!(*right, Expr::Number(ref s) if s == "3"));
            }
            other => panic!("Expected BinaryOp Add, got {other:?}"),
        }
    }

    #[test]
    fn test_precedence_mul_over_add() {
        // 2 + 3 * 4 should parse as 2 + (3 * 4)
        match parse("2 + 3 * 4") {
            Expr::BinaryOp {
                op: BinaryOperator::Add,
                right,
                ..
            } => {
                assert!(matches!(
                    *right,
                    Expr::BinaryOp {
                        op: BinaryOperator::Multiply,
                        ..
                    }
                ));
            }
            other => panic!("Expected Add at top, got {other:?}"),
        }
    }

    #[test]
    fn test_power_right_associative() {
        // 2 ^ 3 ^ 4 should parse as 2 ^ (3 ^ 4)
        match parse("2 ^ 3 ^ 4") {
            Expr::BinaryOp {
                op: BinaryOperator::Power,
                right,
                ..
            } => {
                assert!(matches!(
                    *right,
                    Expr::BinaryOp {
                        op: BinaryOperator::Power,
                        ..
                    }
                ));
            }
            other => panic!("Expected Power at top with Power on right, got {other:?}"),
        }
    }

    #[test]
    fn test_caret_is_xor_in_programmer() {
        match parse_prog("5 ^ 3") {
            Expr::BinaryOp {
                op: BinaryOperator::BitwiseXor,
                ..
            } => {}
            other => panic!("Expected BitwiseXor in programmer mode, got {other:?}"),
        }
    }

    #[test]
    fn test_unary_negation() {
        match parse("-5") {
            Expr::UnaryOp {
                op: UnaryOperator::Negate,
                operand,
            } => {
                assert!(matches!(*operand, Expr::Number(ref s) if s == "5"));
            }
            other => panic!("Expected UnaryOp Negate, got {other:?}"),
        }
    }

    #[test]
    fn test_factorial() {
        match parse("5!") {
            Expr::Factorial(inner) => {
                assert!(matches!(*inner, Expr::Number(ref s) if s == "5"));
            }
            other => panic!("Expected Factorial, got {other:?}"),
        }
    }

    #[test]
    fn test_function_call() {
        match parse("sin(3.14)") {
            Expr::FunctionCall { name, args } => {
                assert_eq!(name, "sin");
                assert_eq!(args.len(), 1);
            }
            other => panic!("Expected FunctionCall, got {other:?}"),
        }
    }

    #[test]
    fn test_function_two_args() {
        match parse("log(100, 10)") {
            Expr::FunctionCall { name, args } => {
                assert_eq!(name, "log");
                assert_eq!(args.len(), 2);
            }
            other => panic!("Expected FunctionCall with 2 args, got {other:?}"),
        }
    }

    #[test]
    fn test_nested_parens() {
        // ((2 + 3)) should parse fine
        let expr = parse("((2 + 3))");
        assert!(matches!(
            expr,
            Expr::BinaryOp {
                op: BinaryOperator::Add,
                ..
            }
        ));
    }

    #[test]
    fn test_complex_expression() {
        // sin(pi / 6) + 2 * 3 should parse
        let expr = parse("sin(pi / 6) + 2 * 3");
        assert!(matches!(
            expr,
            Expr::BinaryOp {
                op: BinaryOperator::Add,
                ..
            }
        ));
    }

    #[test]
    fn test_identifier() {
        match parse("ans") {
            Expr::Ident(name) => assert_eq!(name, "ans"),
            other => panic!("Expected Ident, got {other:?}"),
        }
    }

    #[test]
    fn test_zero_arg_function() {
        match parse("rand()") {
            Expr::FunctionCall { name, args } => {
                assert_eq!(name, "rand");
                assert!(args.is_empty());
            }
            other => panic!("Expected FunctionCall with 0 args, got {other:?}"),
        }
    }

    #[test]
    fn test_error_unexpected_token() {
        assert!(Parser::parse("2 +", CalculatorMode::Standard).is_err());
    }

    #[test]
    fn test_error_unmatched_paren() {
        assert!(Parser::parse("(2 + 3", CalculatorMode::Standard).is_err());
    }
}
