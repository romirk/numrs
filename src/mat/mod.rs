mod iter;
mod macros;
mod mat2;
mod ops;

pub use iter::{Iter, ColumnIterator, RowIterator};
pub use mat2::Mat2;

pub type Element = i32;
pub type Shape = (usize, usize);
