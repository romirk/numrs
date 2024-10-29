use crate::mat::Shape;

pub(crate) trait LocIterator: Iterator<Item = usize> {
    fn row_size(&self) -> usize;
    fn len(&self) -> usize;
}
pub struct ColumnIterator {
    shape: Shape,
    i: usize,
}

pub struct RowIterator {
    shape: Shape,
    i: usize,
}

impl Iterator for ColumnIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.len() {
            return None;
        }

        let row = self.i % self.shape.1;
        let col = self.i / self.shape.1;
        self.i += 1;
        Some(row * self.shape.0 + col)
    }
}

impl Iterator for RowIterator {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.len() {
            return None;
        }
        let pos = self.i;
        self.i += 1;
        Some(pos)
    }
}

impl From<Shape> for ColumnIterator {
    fn from(shape: Shape) -> Self {
        Self { shape, i: 0 }
    }
}

impl From<Shape> for RowIterator {
    fn from(shape: Shape) -> Self {
        Self { shape, i: 0 }
    }
}

impl LocIterator for ColumnIterator {
    fn row_size(&self) -> usize {
        self.shape.1
    }
    fn len(&self) -> usize {
        self.shape.0 * self.shape.1
    }
}
impl LocIterator for RowIterator {
    fn row_size(&self) -> usize {
        self.shape.1
    }
    fn len(&self) -> usize {
        self.shape.0 * self.shape.1
    }
}
