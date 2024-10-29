use super::Mat2;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub enum Op {
    Finalize,
    Transpose(Box<Op>),
    Multiply(Mat2, Box<Op>),
    Add(Mat2, Box<Op>),
}

impl Display for Op {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Finalize => { write!(f, "Finalize") }
            Op::Transpose(op) => { write!(f, "Transpose({})", *op) }
            Op::Multiply(mat, op) => { write!(f, "Multiply({}, {})", *mat, *op) }
            Op::Add(mat, op) => { write!(f, "Add({}, {})", *mat, *op) }
        }
    }
}