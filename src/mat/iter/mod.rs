use crate::mat::{Element, Mat2};

mod element;
mod idx;

pub use element::{ElementIterator, IndexedElementIterator};
pub use idx::IndexIterator;

pub struct Iter<'a> {
    mat: &'a Mat2,
    row: usize,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a [Element];
    fn next(&mut self) -> Option<Self::Item> {
        if self.row >= self.mat.shape.0 {
            return None;
        }

        // IMPORTANT: assumes row major
        let r = &self.mat[self.row];
        self.row += 1;
        Some(r)
    }
}

impl<'a> From<&'a Mat2> for Iter<'a> {
    fn from(mat: &'a Mat2) -> Self {
        Self { mat, row: 0 }
    }
}