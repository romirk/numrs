use crate::mat::{Element, Mat2};

pub struct ElementIterator<'a> {
    mat: &'a Mat2,
    i: usize,
}

impl<'a> Iterator for ElementIterator<'a> {
    type Item = Element;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.mat.data.len() { return None; }
        if self.mat.row_major {
            let pos = self.i;
            self.i += 1;
            Some(self.mat.data[pos])
        } else {
            let row = self.i / self.mat.shape.1;
            let col = self.i % self.mat.shape.1;
            self.i += 1;
            Some(self.mat.data[col * self.mat.shape.0 + row])
        }
    }
}

impl<'a> From<&'a Mat2> for ElementIterator<'a> {
    fn from(mat: &'a Mat2) -> Self {
        Self { mat, i: 0 }
    }
}