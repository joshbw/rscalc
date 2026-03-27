/// Lexer: tokenizes input strings into a stream of `Token`s.
use super::tokens::Token;

/// Maximum length for any single token (numeric literal or identifier).
const MAX_TOKEN_LEN: usize = 10_000;

/// Maximum length for identifiers (function/constant names).
const MAX_IDENT_LEN: usize = 256;

pub struct Lexer {
    chars: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            chars: input.chars().collect(),
            pos: 0,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        loop {
            let tok = self.next_token()?;
            if tok.is_eof() {
                tokens.push(tok);
                break;
            }
            tokens.push(tok);
        }
        Ok(tokens)
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.chars.get(self.pos).copied();
        if ch.is_some() {
            self.pos += 1;
        }
        ch
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn next_token(&mut self) -> Result<Token, String> {
        self.skip_whitespace();

        let Some(ch) = self.peek() else {
            return Ok(Token::Eof);
        };

        match ch {
            '0'..='9' | '.' => self.read_number(),
            'a'..='z' | 'A'..='Z' | '_' | '\u{03C0}' => self.read_ident_or_keyword(),
            '+' => {
                self.advance();
                Ok(Token::Plus)
            }
            '-' => {
                self.advance();
                Ok(Token::Minus)
            }
            '*' => {
                self.advance();
                if self.peek() == Some('*') {
                    self.advance();
                    Ok(Token::DoubleStar)
                } else {
                    Ok(Token::Star)
                }
            }
            '/' => {
                self.advance();
                Ok(Token::Slash)
            }
            '%' => {
                self.advance();
                Ok(Token::Percent)
            }
            '^' => {
                self.advance();
                Ok(Token::Caret)
            }
            '&' => {
                self.advance();
                Ok(Token::Ampersand)
            }
            '|' => {
                self.advance();
                Ok(Token::Pipe)
            }
            '~' => {
                self.advance();
                Ok(Token::Tilde)
            }
            '<' => {
                self.advance();
                if self.peek() == Some('<') {
                    self.advance();
                    Ok(Token::ShiftLeft)
                } else {
                    Err("Unexpected '<'. Did you mean '<<'?".to_string())
                }
            }
            '>' => {
                self.advance();
                if self.peek() == Some('>') {
                    self.advance();
                    if self.peek() == Some('>') {
                        self.advance();
                        Ok(Token::LogicalShiftRight)
                    } else {
                        Ok(Token::ShiftRight)
                    }
                } else {
                    Err("Unexpected '>'. Did you mean '>>'?".to_string())
                }
            }
            '!' => {
                self.advance();
                Ok(Token::Bang)
            }
            '(' => {
                self.advance();
                Ok(Token::LParen)
            }
            ')' => {
                self.advance();
                Ok(Token::RParen)
            }
            ',' => {
                self.advance();
                Ok(Token::Comma)
            }
            _ => Err(format!("Unexpected character: '{ch}'")),
        }
    }

    fn read_number(&mut self) -> Result<Token, String> {
        let mut s = String::new();

        // Check for hex/octal/binary prefix
        if self.peek() == Some('0') {
            s.push('0');
            self.advance();
            match self.peek() {
                Some('x' | 'X') => {
                    s.push(self.advance().unwrap());
                    return self.read_hex_digits(s);
                }
                Some('o' | 'O') => {
                    s.push(self.advance().unwrap());
                    return self.read_octal_digits(s);
                }
                Some('b' | 'B') => {
                    // Distinguish binary prefix from hex digit in decimal context
                    let next_after = self.chars.get(self.pos + 1).copied();
                    if matches!(next_after, Some('0' | '1'))
                        || !next_after.is_some_and(char::is_alphanumeric)
                    {
                        s.push(self.advance().unwrap());
                        return self.read_binary_digits(s);
                    }
                    // Fall through to decimal
                }
                _ => {}
            }
        }

        // Decimal number (may include fractional part and exponent)
        self.read_decimal_digits(&mut s)?;

        // Fractional part
        if self.peek() == Some('.') {
            s.push('.');
            self.advance();
            self.read_decimal_digits(&mut s)?;
        }

        // Exponent part
        if matches!(self.peek(), Some('e' | 'E')) {
            s.push(self.advance().unwrap());
            if matches!(self.peek(), Some('+' | '-')) {
                s.push(self.advance().unwrap());
            }
            self.read_decimal_digits(&mut s)?;
        }

        Ok(Token::Number(s))
    }

    fn read_decimal_digits(&mut self, s: &mut String) -> Result<(), String> {
        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() || ch == '_' {
                if ch != '_' {
                    s.push(ch);
                }
                self.advance();
                if s.len() > MAX_TOKEN_LEN {
                    return Err("Numeric literal too long".to_string());
                }
            } else {
                break;
            }
        }
        Ok(())
    }

    fn read_hex_digits(&mut self, mut s: String) -> Result<Token, String> {
        let start_len = s.len();
        while let Some(ch) = self.peek() {
            if ch.is_ascii_hexdigit() || ch == '_' {
                if ch != '_' {
                    s.push(ch);
                }
                self.advance();
                if s.len() > MAX_TOKEN_LEN {
                    return Err("Numeric literal too long".to_string());
                }
            } else {
                break;
            }
        }
        if s.len() == start_len {
            return Err("Expected hex digits after '0x'".to_string());
        }
        Ok(Token::Number(s))
    }

    fn read_octal_digits(&mut self, mut s: String) -> Result<Token, String> {
        let start_len = s.len();
        while let Some(ch) = self.peek() {
            if ('0'..='7').contains(&ch) || ch == '_' {
                if ch != '_' {
                    s.push(ch);
                }
                self.advance();
                if s.len() > MAX_TOKEN_LEN {
                    return Err("Numeric literal too long".to_string());
                }
            } else {
                break;
            }
        }
        if s.len() == start_len {
            return Err("Expected octal digits after '0o'".to_string());
        }
        Ok(Token::Number(s))
    }

    fn read_binary_digits(&mut self, mut s: String) -> Result<Token, String> {
        let start_len = s.len();
        while let Some(ch) = self.peek() {
            if ch == '0' || ch == '1' || ch == '_' {
                if ch != '_' {
                    s.push(ch);
                }
                self.advance();
                if s.len() > MAX_TOKEN_LEN {
                    return Err("Numeric literal too long".to_string());
                }
            } else {
                break;
            }
        }
        if s.len() == start_len {
            return Err("Expected binary digits after '0b'".to_string());
        }
        Ok(Token::Number(s))
    }

    fn read_ident_or_keyword(&mut self) -> Result<Token, String> {
        let mut s = String::new();

        // Handle π as a single-character identifier
        if self.peek() == Some('\u{03C0}') {
            self.advance();
            return Ok(Token::Ident("pi".to_string()));
        }

        while let Some(ch) = self.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                s.push(ch);
                self.advance();
                if s.len() > MAX_IDENT_LEN {
                    return Err("Identifier too long".to_string());
                }
            } else {
                break;
            }
        }

        // Map keyword operators to tokens
        match s.as_str() {
            "mod" => Ok(Token::Percent),
            // `xor` stays as Ident so the parser can handle it in all modes
            _ => Ok(Token::Ident(s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lex(input: &str) -> Vec<Token> {
        Lexer::new(input).tokenize().unwrap()
    }

    #[test]
    fn test_basic_arithmetic() {
        let tokens = lex("2 + 3");
        assert_eq!(
            tokens,
            vec![
                Token::Number("2".into()),
                Token::Plus,
                Token::Number("3".into()),
                Token::Eof,
            ]
        );
    }

    #[test]
    fn test_hex_literal() {
        let tokens = lex("0xFF");
        assert_eq!(tokens, vec![Token::Number("0xFF".into()), Token::Eof]);
    }

    #[test]
    fn test_binary_literal() {
        let tokens = lex("0b1010");
        assert_eq!(tokens, vec![Token::Number("0b1010".into()), Token::Eof]);
    }

    #[test]
    fn test_octal_literal() {
        let tokens = lex("0o77");
        assert_eq!(tokens, vec![Token::Number("0o77".into()), Token::Eof]);
    }

    #[test]
    fn test_function_call() {
        let tokens = lex("sin(3.14)");
        assert_eq!(
            tokens,
            vec![
                Token::Ident("sin".into()),
                Token::LParen,
                Token::Number("3.14".into()),
                Token::RParen,
                Token::Eof,
            ]
        );
    }

    #[test]
    fn test_double_star() {
        let tokens = lex("2 ** 10");
        assert_eq!(
            tokens,
            vec![
                Token::Number("2".into()),
                Token::DoubleStar,
                Token::Number("10".into()),
                Token::Eof,
            ]
        );
    }

    #[test]
    fn test_shift_operators() {
        let tokens = lex("1 << 10 >> 2 >>> 1");
        assert_eq!(
            tokens,
            vec![
                Token::Number("1".into()),
                Token::ShiftLeft,
                Token::Number("10".into()),
                Token::ShiftRight,
                Token::Number("2".into()),
                Token::LogicalShiftRight,
                Token::Number("1".into()),
                Token::Eof,
            ]
        );
    }

    #[test]
    fn test_factorial() {
        let tokens = lex("5!");
        assert_eq!(
            tokens,
            vec![Token::Number("5".into()), Token::Bang, Token::Eof]
        );
    }

    #[test]
    fn test_scientific_notation() {
        let tokens = lex("1.5e10");
        assert_eq!(tokens, vec![Token::Number("1.5e10".into()), Token::Eof]);
    }

    #[test]
    fn test_pi_unicode() {
        let tokens = lex("π");
        assert_eq!(tokens, vec![Token::Ident("pi".into()), Token::Eof]);
    }

    #[test]
    fn test_mod_keyword() {
        let tokens = lex("10 mod 3");
        assert_eq!(
            tokens,
            vec![
                Token::Number("10".into()),
                Token::Percent,
                Token::Number("3".into()),
                Token::Eof,
            ]
        );
    }

    #[test]
    fn test_underscore_separator() {
        let tokens = lex("1_000_000");
        assert_eq!(tokens, vec![Token::Number("1000000".into()), Token::Eof]);
    }
}
