use crate::mat::iter::IndexIterator;
use crate::mat::{Element, Mat2};
use std::num::NonZero;

pub struct IndexedElementIterator<'a> {
    data: &'a [Element],
    idxs: IndexIterator,
}

pub struct ElementIterator<'a> {
    iter: IndexedElementIterator<'a>,
}

impl<'a> Iterator for IndexedElementIterator<'a> {
    type Item = ([usize; 2], Element);

    fn next(&mut self) -> Option<Self::Item> {
        let Some(idx) = self.idxs.next() else { return None; };
        Some((idx, self.data[Mat2::idx2loc(&idx, self.idxs.row_size())]))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.idxs.size_hint()
    }

    fn advance_by(&mut self, n: usize) -> Result<(), NonZero<usize>> {
        self.idxs.advance_by(n)
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
        Self { data: mat.data.as_ref(), idxs: IndexIterator::from(mat.shape) }
    }
}

impl<'a> From<&'a Mat2> for ElementIterator<'a> {
    fn from(mat: &'a Mat2) -> Self {
        Self { iter: IndexedElementIterator::from(mat) }
    }
}

impl Into<IndexIterator> for IndexedElementIterator<'_> {
    fn into(self) -> IndexIterator {
        self.idxs
    }
}