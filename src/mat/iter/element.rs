use crate::mat::mat2::Mat2;
use crate::mat::LocIterator::{ColumnIterator, RowIterator};
use crate::mat::{Element, LocIterator};
use std::num::NonZero;

pub struct IndexedElementIterator<'a> {
    data: &'a [Element],
    iter: LocIterator,
}

pub struct ElementIterator<'a> {
    iter: IndexedElementIterator<'a>,
}

impl<'a> Iterator for IndexedElementIterator<'a> {
    type Item = ([usize; 2], Element);

    fn next(&mut self) -> Option<Self::Item> {
        let Some(idx) = self.iter.next() else { return None; };
        let e = self.data[idx];
        let row_size = self.iter.row_size();
        let coords = [idx / row_size, idx % row_size];
        Some((coords, e))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    fn advance_by(&mut self, n: usize) -> Result<(), NonZero<usize>> {
        self.iter.advance_by(n)
    }
}

impl<'a> Iterator for ElementIterator<'a> {
    type Item = Element;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((_, e)) = self.iter.next() { Some(e) } else { None }
    }

    fn size_hint(&self) -> (usize, Option<usize>) { self.iter.size_hint() }
    fn advance_by(&mut self, n: usize) -> Result<(), NonZero<usize>> { self.iter.advance_by(n) }
}

impl<'a> From<&'a Mat2> for IndexedElementIterator<'a> {
    fn from(mat: &'a Mat2) -> Self {
        Self {
            data: mat.raw().as_ref(),
            iter: if mat.row_major() {
                RowIterator(mat.shape(), 0)
            } else {
                ColumnIterator(mat.shape(), 0)
            },
        }
    }
}

impl<'a> From<&'a Mat2> for ElementIterator<'a> {
    fn from(mat: &'a Mat2) -> Self {
        Self { iter: IndexedElementIterator::from(mat) }
    }
}


impl<'a> Into<ElementIterator<'a>> for IndexedElementIterator<'a> {
    fn into(self) -> ElementIterator<'a> {
        ElementIterator { iter: self }
    }
}