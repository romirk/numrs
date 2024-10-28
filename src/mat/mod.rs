mod iter;
mod macros;

use iter::MatIterator2;
use std::fmt::{Display, Formatter};
use std::intrinsics::unlikely;
use std::ops::{Add, Index, IndexMut, Mul};

pub type Element = f32;

#[derive(Debug, Clone)]
pub struct Mat2 {
    pub shape: (isize, isize),
    pub data: Box<[Element]>,
    pub(crate) row_major: bool,
}

impl Mat2 {

    pub fn new(shape: (isize, isize), data: Box<[Element]>) -> Self {
        Self {
            shape,
            data,
             row_major: true
        }
    }
    #[inline]
    fn idx2d_internal(&self, index: &[isize; 2]) -> usize {
        (if self.row_major { index[0] * (self.shape.0) + index[1] } else { index[1] * self.shape.0 + index[0] }) as usize
    }
    #[inline]
    fn validate_shape(shape: &(isize, isize)) {
        assert!(shape.0 > 0 && shape.1 > 0, "Dimension must be positive");
    }

    pub(crate) fn zeroes(shape: (isize, isize)) -> Self {
        Self::validate_shape(&shape);
        Self {
            shape,
            data: vec![0.0; shape.0 as usize * shape.1 as usize].into_boxed_slice(),
            row_major: true,
        }
    }

    pub(crate) fn identity(shape: (isize, isize)) -> Self {
        Self::validate_shape(&shape);
        assert_eq!(shape.0, shape.1, "Identity matrices must be squares");
        let mut data = vec![0.0; shape.0 as usize * shape.1 as usize].into_boxed_slice();
        for i in 0..shape.0 {
            let pos = (i * shape.0 + i) as usize;
            data[pos] = 1.0;
        }
        Self {
            shape,
            data,
            row_major: true,
        }
    }

    pub fn transpose(&self) -> Self {
        Self {
            shape: (self.shape.1, self.shape.0),
            data: self.data.clone(),
            row_major: !self.row_major,
        }
    }

    pub fn solidify(self) -> Self {
        if self.row_major { self } else {
            Self {
                shape: self.shape,
                data: MatIterator2::from(&self).collect(),
                row_major: true,
            }
        }
    }

    fn naive_mul(&self, rhs: &Mat2) -> Mat2 {
        let mut result = Mat2::zeroes((self.shape.0, rhs.shape.1));
        for i in 0..self.shape.0 {
            for j in 0..rhs.shape.1 {
                let mut sum = 0.0;
                for k in 0..self.shape.1 {
                    sum += self[[i, k]] * rhs[[k, j]];
                }
                result[[i, j]] = sum;
            }
        }
        result
    }
}


impl PartialEq<Self> for Mat2 {
    fn eq(&self, other: &Self) -> bool {
        if self.shape != other.shape { return false; }
        if self.row_major == other.row_major {
            self.data == other.data
        } else {
            MatIterator2::from(self).zip(MatIterator2::from(other)).all(|(x, y)| x == y)
        }
    }
}


impl Eq for Mat2 {}

impl Index<usize> for Mat2 {
    type Output = [Element];

    fn index(&self, index: usize) -> &Self::Output {
        if unlikely(index >= self.shape.0 as usize) { panic!("Index out of bounds"); }
        let cols = self.shape.1 as usize;
        let left = index * cols;
        let right = (index + 1) * cols;
        if self.row_major {
            self.data[left..right].as_ref()
        } else {
            panic!("cannot get row from column major matrix (try calling .solidify())");
        }
    }
}

impl Index<[isize; 2]> for Mat2 {
    type Output = Element;

    fn index(&self, index: [isize; 2]) -> &Self::Output {
        &self.data[self.idx2d_internal(&index)]
    }
}
impl IndexMut<[isize; 2]> for Mat2 {
    fn index_mut(&mut self, index: [isize; 2]) -> &mut Self::Output {
        &mut self.data[self.idx2d_internal(&index)]
    }
}

impl Add for &Mat2 {
    type Output = Mat2;

    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.shape, rhs.shape, "Incompatible matrix dimensions: {:?} vs {:?}", self.shape, rhs.shape);
        Mat2 {
            shape: self.shape,
            row_major: true,
            data: MatIterator2::from(self).zip(MatIterator2::from(rhs)).map(|(x, y)| x + y).collect(),
        }
    }
}

impl Mul for &Mat2 {
    type Output = Mat2;

    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.shape.1, rhs.shape.0, "Incompatible matrix dimensions: {:?} x {:?}", self.shape, rhs.shape);
        self.naive_mul(rhs)
    }
}

impl Display for Mat2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (i, e) in MatIterator2::from(self).enumerate() {
            if i % self.shape.1 as usize == 0 {
                write!(f, "\n")?;
            }
            write!(f, "{} ", e)?;
        }
        Ok(())
    }
}