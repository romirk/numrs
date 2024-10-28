use crate::mat::Shape;
use std::num::NonZero;

/// Guarantees generated indices are within the given shape
pub struct IndexIterator {
    shape: Shape,
    i: usize,
}

impl IndexIterator {
    pub const fn row_size(&self) -> usize {
        self.shape.1
    }

    pub const fn len(&self) -> usize {
        self.shape.0 * self.shape.1
    }
}
impl Iterator for IndexIterator {
    type Item = [usize; 2];

    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.i;
        if pos >= self.len() {
            None
        } else {
            self.i += 1;
            let r = pos / self.shape.1;
            let c = pos % self.shape.1;
            Some([r, c])
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let rem = self.len() - self.i;
        (rem, Some(rem))
    }

    fn advance_by(&mut self, n: usize) -> Result<(), NonZero<usize>> {
        let jump = self.i + n;
        if jump >= self.len() {
            return Err(NonZero::new(jump - self.len() + 1).unwrap());
        }
        self.i = jump;
        Ok(())
    }
}

impl From<Shape> for IndexIterator {
    fn from(shape: Shape) -> Self {
        Self { shape, i: 0 }
    }
}