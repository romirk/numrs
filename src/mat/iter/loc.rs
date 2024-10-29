use crate::mat::Shape;

pub enum LocIterator {
    RowIterator(Shape, usize),
    ColumnIterator(Shape, usize),
}

impl LocIterator {
    pub(crate) fn row_size(&self) -> usize {
        match self {
            LocIterator::RowIterator(shape, _) | LocIterator::ColumnIterator(shape, _) => shape.1
        }
    }
}

impl Iterator for LocIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            LocIterator::ColumnIterator(shape, ref mut i) => {
                if *i >= shape.0 * shape.1 {
                    return None;
                }

                let row = *i % shape.1;
                let col = *i / shape.1;
                *i += 1;
                Some(row * shape.0 + col)
            },
            LocIterator::RowIterator(shape, ref mut i) => {
                if *i >= shape.0 * shape.1 {
                    return None;
                }
                let pos = *i;
                *i += 1;
                Some(pos)
            }
        }
    }
}
