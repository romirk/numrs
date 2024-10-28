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
        // we always know how many elements are remaining
        if self.i >= self.len() {
            (0, Some(0))
        } else {
            let rem = self.len() - self.i;
            (rem, Some(rem))
        }
    }

    fn advance_by(&mut self, n: usize) -> Result<(), NonZero<usize>> {
        let jump = self.i + n;
        if jump >= self.len() {
            self.i = self.len();
            return Err(NonZero::new(jump - self.len() + 1).unwrap());
        }
        self.i = jump;
        Ok(())
    }

    /// Gets the largest index of the matrix. This means the index of the bottom-right corner.
    fn max(self) -> Option<Self::Item>
    where
        Self: Sized,
        Self::Item: Ord,
    {
        Some([self.shape.0 - 1, self.shape.1 - 1])
    }
}

impl From<Shape> for IndexIterator {
    fn from(shape: Shape) -> Self {
        Self { shape, i: 0 }
    }
}