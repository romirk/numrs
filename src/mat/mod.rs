mod iter;
mod macros;

pub use iter::Iter;

use iter::{ElementIterator, IndexIterator, IndexedElementIterator};
use std::fmt::{Display, Formatter};
use std::intrinsics::unlikely;
use std::ops::{Add, Index, IndexMut, Mul};

pub type Element = i32;
pub type Shape = (usize, usize);

#[derive(Debug, Clone)]
pub struct Mat2 {
    shape: Shape,
    data: Box<[Element]>,
    row_major: bool,
}

impl Mat2 {
    fn validate(shape: &Shape, data: &[Element]) {
        if shape.0 < 1 || shape.1 < 1 || data.len() != shape.0 * shape.1 {
            panic!("Invalid matrix dimensions or data size");
        }
    }
    #[inline]
    pub fn new(shape: Shape, data: Box<[Element]>) -> Self {
        Self::validate(&shape, &data);
        Self {
            shape,
            data,
            row_major: true,
        }
    }

    #[inline]
    fn new_from_arrays<const N: usize, const M: usize>(array: [[Element; N]; M]) -> Self
    // TODO figure out what this `where` clause is
    where
        [(); M * N]:,
    {
        let mut data = Box::new([0.0 as Element; M * N]);
        for i in 0..M {
            data[i * N..(i + 1) * N].copy_from_slice(array[i].as_ref());
        }
        Self {
            shape: (M, N),
            data,
            row_major: true,
        }
    }

    pub fn shape(&self) -> Shape {
        self.shape
    }

    pub fn row_major(&self) -> bool {
        self.row_major
    }

    pub fn raw(&self) -> &[Element] {
        &self.data
    }

    #[inline]
    pub fn idx2loc(index: &[usize; 2], row_size: usize) -> usize {
        index[0] * row_size + index[1]
    }
    #[inline]
    fn validate_shape(shape: &Shape) {
        assert!(shape.0 > 0 && shape.1 > 0, "Dimension must be positive");
    }

    pub(crate) fn zeroes(shape: Shape) -> Self {
        Self::validate_shape(&shape);
        Self {
            shape,
            data: vec![0.0 as Element; shape.0 * shape.1].into_boxed_slice(),
            row_major: true,
        }
    }

    #[allow(non_snake_case)]
    pub fn I(size: usize) -> Self {
        let shape = (size, size);
        Self::validate_shape(&shape);
        let mut data = vec![0.0 as Element; shape.0 * shape.1].into_boxed_slice();
        for i in 0..shape.0 {
            let pos = i * shape.0 + i;
            data[pos] = 1.0 as Element;
        }
        Self {
            shape,
            data,
            row_major: true,
        }
    }

    #[allow(non_snake_case)]
    pub fn T(&self) -> Self {
        Self {
            shape: (self.shape.1, self.shape.0),
            data: self.data.clone(),
            row_major: !self.row_major,
        }
    }

    pub fn finalize(&mut self) -> &Self{
        if !self.row_major {
            self.data = self.elements().collect();
            self.row_major = true;
        }
        self
    }

    fn naive_mul(&self, rhs: &Mat2) -> Mat2 {
        let n = self.shape.0;
        let m = self.shape.1;
        let p = rhs.shape.1;

        let mut result = Mat2::zeroes((n, p));
        for i in 0..n {
            for j in 0..p {
                let mut sum = 0.0 as Element;
                for k in 0..m {
                    sum += self[[i, k]] * rhs[[k, j]];
                }
                result[[i, j]] = sum;
            }
        }
        result
    }

    pub fn elements(&self) -> ElementIterator {
        ElementIterator::from(self)
    }
    pub fn indices(&self) -> IndexIterator {
        IndexIterator::from(self.shape)
    }
    pub fn pairs(&self) -> IndexedElementIterator {
        IndexedElementIterator::from(self)
    }

    pub fn iter(&mut self) -> Iter {
        if self.row_major {
            self.into_iter()
        } else {
            self.finalize().into_iter()
        }
    }
}


impl PartialEq<Self> for Mat2 {
    fn eq(&self, other: &Self) -> bool {
        if self.shape != other.shape { return false; }
        if self.row_major == other.row_major {
            self.data == other.data
        } else {
            ElementIterator::from(self).zip(ElementIterator::from(other)).all(|(x, y)| x == y)
        }
    }
}


impl Eq for Mat2 {}

impl Index<usize> for Mat2 {
    type Output = [Element];

    fn index(&self, index: usize) -> &Self::Output {
        if unlikely(index >= self.shape.0) { panic!("Index out of bounds"); }
        let cols = self.shape.1;
        let left = index * cols;
        let right = (index + 1) * cols;
        if !self.row_major {
            panic!("cannot get row from column major matrix (try calling .solidify())");
        }
        self.data[left..right].as_ref()
    }
}

impl Index<[usize; 2]> for Mat2 {
    type Output = Element;

    fn index(&self, mut index: [usize; 2]) -> &Self::Output {
        if self.row_major {
            &self.data[Self::idx2loc(&index, self.shape.1)]
        } else {
            index.reverse();
            &self.data[Self::idx2loc(&index, self.shape.0)]
        }
    }
}
impl IndexMut<[usize; 2]> for Mat2 {
    fn index_mut(&mut self, mut index: [usize; 2]) -> &mut Self::Output {
        if self.row_major {
            &mut self.data[Self::idx2loc(&index, self.shape.1)]
        } else {
            index.reverse();
            &mut self.data[Self::idx2loc(&index, self.shape.0)]
        }
    }
}

impl Add for &Mat2 {
    type Output = Mat2;

    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.shape, rhs.shape, "Incompatible matrix dimensions: {:?} vs {:?}", self.shape, rhs.shape);
        Mat2 {
            shape: self.shape,
            row_major: true,
            data: ElementIterator::from(self).zip(ElementIterator::from(rhs)).map(|(x, y)| x + y).collect(),
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
        for (i, e) in ElementIterator::from(self).enumerate() {
            if i % self.shape.1 == 0 {
                write!(f, "\n")?;
            }
            write!(f, "{} ", e)?;
        }
        Ok(())
    }
}

impl<const N: usize, const M: usize> From<[[Element; N]; M]> for Mat2
where
    [(); M * N]:,
{
    fn from(value: [[Element; N]; M]) -> Self {
        Self::new_from_arrays(value)
    }
}

impl<'a> IntoIterator for &'a Mat2 {
    type Item = <Iter<'a> as Iterator>::Item;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        if !self.row_major {
            panic!("Column major matrices cannot be iterated over -- call .finalize() first.");
        }
        Iter::from(self)
    }
}
