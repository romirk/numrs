use crate::mat::Element;
use std::num::NonZero;

mod element;
mod idx;
mod loc;

pub use element::{ElementIterator, IndexedElementIterator};
pub use idx::IndexIterator;
pub use loc::{LocIterator};

use super::Mat2;

pub struct Iter<'a> {
    mat: &'a Mat2,
    row: usize,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a [Element];
    fn next(&mut self) -> Option<Self::Item> {
        if self.row >= self.mat.shape().0 {
            return None;
        }

        // IMPORTANT: assumes row major
        let r = &self.mat[self.row];
        self.row += 1;
        Some(r)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.row >= self.mat.shape().0 {
            (0, Some(0))
        } else {
            let rem = self.mat.shape().0 - self.row;
            (rem, Some(rem))
        }
    }

    fn advance_by(&mut self, n: usize) -> Result<(), NonZero<usize>> {
        let jump = self.row + n;
        if jump >= self.mat.shape().0 {
            self.row = self.mat.shape().0;
            return Err(NonZero::new(jump - self.mat.shape().0 + 1).unwrap());
        }
        self.row = jump;
        Ok(())
    }
}

impl<'a> From<&'a Mat2> for Iter<'a> {
    fn from(mat: &'a Mat2) -> Self {
        Self { mat, row: 0 }
    }
}