/// Abstract Syntax Tree node types for parsed expressions.

/// A parsed expression.
#[derive(Debug, Clone)]
pub enum Expr {
    /// Numeric literal, stored as the raw string for ratpack parsing.
    Number(String),

    /// A named constant or the `ans` keyword.
    Ident(String),

    /// Unary operation (prefix): `-x`, `~x`
    UnaryOp {
        op: UnaryOperator,
        operand: Box<Expr>,
    },

    /// Binary operation: `a + b`, `a * b`, etc.
    BinaryOp {
        op: BinaryOperator,
        left: Box<Expr>,
        right: Box<Expr>,
    },

    /// Postfix factorial: `x!`
    Factorial(Box<Expr>),

    /// Function call: `sin(x)`, `log(x, b)`, `rand()`
    FunctionCall {
        name: String,
        args: Vec<Expr>,
    },
}

/// Unary (prefix) operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOperator {
    /// Arithmetic negation `-`
    Negate,
    /// Bitwise NOT `~`
    BitwiseNot,
}

/// Binary operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    ShiftLeft,
    ShiftRight,
    LogicalShiftRight,
}

impl std::fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryOperator::Negate => write!(f, "-"),
            UnaryOperator::BitwiseNot => write!(f, "~"),
        }
    }
}

impl std::fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOperator::Add => write!(f, "+"),
            BinaryOperator::Subtract => write!(f, "-"),
            BinaryOperator::Multiply => write!(f, "*"),
            BinaryOperator::Divide => write!(f, "/"),
            BinaryOperator::Modulo => write!(f, "%"),
            BinaryOperator::Power => write!(f, "^"),
            BinaryOperator::BitwiseAnd => write!(f, "&"),
            BinaryOperator::BitwiseOr => write!(f, "|"),
            BinaryOperator::BitwiseXor => write!(f, "xor"),
            BinaryOperator::ShiftLeft => write!(f, "<<"),
            BinaryOperator::ShiftRight => write!(f, ">>"),
            BinaryOperator::LogicalShiftRight => write!(f, ">>>"),
        }
    }
}
