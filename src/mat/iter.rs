use super::{Element, Mat2};

pub struct MatIterator2<'a> {
    mat: &'a Mat2,
    i: usize,
}

impl<'a> Iterator for MatIterator2<'a> {
    type Item = Element;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.mat.data.len() { return None; }
        if self.mat.row_major {
            let pos = self.i.clone();
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

impl<'a> From<&'a Mat2> for MatIterator2<'a> {
    fn from(value: &'a Mat2) -> Self {
        MatIterator2 {
            mat: value,
            i: 0,
        }
    }
}