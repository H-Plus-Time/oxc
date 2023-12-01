use oxc_ast::ast::*;
use oxc_syntax::{
    operator::{BinaryOperator, LogicalOperator},
    precedence::{GetPrecedence, Precedence},
};

use crate::{Doc, Format, Prettier};

#[derive(Clone, Copy)]
pub enum BinaryishLeft<'a, 'b> {
    Expression(&'b Expression<'a>),
    PrivateIdentifier(&'b PrivateIdentifier),
}

impl<'a, 'b> From<&'b Expression<'a>> for BinaryishLeft<'a, 'b> {
    fn from(e: &'b Expression<'a>) -> Self {
        Self::Expression(e)
    }
}

impl<'a, 'b> From<&'b PrivateIdentifier> for BinaryishLeft<'a, 'b> {
    fn from(e: &'b PrivateIdentifier) -> Self {
        Self::PrivateIdentifier(e)
    }
}

impl<'a, 'b> BinaryishLeft<'a, 'b> {
    pub fn operator(&self) -> Option<BinaryishOperator> {
        match self {
            Self::Expression(Expression::BinaryExpression(e)) => {
                Some(BinaryishOperator::BinaryOperator(e.operator))
            }
            Self::Expression(Expression::LogicalExpression(e)) => {
                Some(BinaryishOperator::LogicalOperator(e.operator))
            }
            _ => None,
        }
    }
}

impl<'a, 'b> Format<'a> for BinaryishLeft<'a, 'b> {
    fn format(&self, p: &mut Prettier<'a>) -> Doc<'a> {
        match self {
            Self::Expression(expr) => expr.format(p),
            Self::PrivateIdentifier(ident) => ident.format(p),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BinaryishOperator {
    BinaryOperator(BinaryOperator),
    LogicalOperator(LogicalOperator),
}

impl From<BinaryOperator> for BinaryishOperator {
    fn from(op: BinaryOperator) -> Self {
        Self::BinaryOperator(op)
    }
}

impl From<LogicalOperator> for BinaryishOperator {
    fn from(op: LogicalOperator) -> Self {
        Self::LogicalOperator(op)
    }
}

impl GetPrecedence for BinaryishOperator {
    fn precedence(&self) -> Precedence {
        match self {
            Self::BinaryOperator(op) => op.precedence(),
            Self::LogicalOperator(op) => op.precedence(),
        }
    }
}

impl BinaryishOperator {
    pub fn is_binary(self) -> bool {
        matches!(self, Self::BinaryOperator(_))
    }

    pub fn should_flatten(self, parent_op: Self) -> bool {
        if self.precedence() != parent_op.precedence() {
            return false;
        }

        if matches!(parent_op, Self::BinaryOperator(op) if op == BinaryOperator::Exponential) {
            return false;
        }

        if matches!(parent_op, Self::BinaryOperator(op) if op.is_equality())
            && matches!(self, Self::BinaryOperator(op) if op.is_equality())
        {
            return false;
        }

        if self != parent_op
            && matches!(parent_op, Self::BinaryOperator(op) if op.is_multiplicative())
            && matches!(self, Self::BinaryOperator(op) if op.is_multiplicative())
        {
            return false;
        }

        if matches!(parent_op, Self::BinaryOperator(op) if op.is_bitshift())
            && matches!(self, Self::BinaryOperator(op) if op.is_bitshift())
        {
            return false;
        }

        true
    }
}

impl BinaryishOperator {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BinaryOperator(op) => op.as_str(),
            Self::LogicalOperator(op) => op.as_str(),
        }
    }
}