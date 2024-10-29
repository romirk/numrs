use super::iter::{ElementIterator, IndexIterator, IndexedElementIterator};
use super::{Element, Iter, Shape};
use crate::mat::ops::Op;
use std::fmt::{Display, Formatter};
use std::intrinsics::unlikely;
use std::ops::{Add, AddAssign, Index, IndexMut, Mul};

#[derive(Debug, Clone)]
pub struct Mat2 {
    shape: Shape,
    data: Box<[Element]>,
    row_major: bool,
    ops: Option<Box<Op>>,
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
            ops: Some(Box::from(Op::Finalize)),
        }
    }

    #[inline]
    fn new_from_arrays<const N: usize, const M: usize>(array: [[Element; N]; M]) -> Self
    // TODO figure out what this `where` clause is
    where
        [(); M * N]:,
    {
        let data = array.iter().flatten().map(|e| *e).collect();
        Self {
            shape: (M, N),
            data,
            row_major: true,
            ops: Some(Box::from(Op::Finalize)),
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
            ops: Some(Box::from(Op::Finalize)),
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
            ops: Some(Box::from(Op::Finalize)),
        }
    }

    pub fn reshape(mut self, shape: Shape) -> Self {
        Self::validate(&shape, &self.data);
        self.shape = shape;
        self
    }

    #[allow(non_snake_case)]
    pub fn T(mut self) -> Self {
        self.row_major = false;
        self.shape = (self.shape.1, self.shape.0);
        self
    }

    fn reorder(&mut self) -> &Self {
        if !self.row_major {
            self.data = self.elements().collect();
            self.row_major = true;
        }
        self
    }

    pub fn finalize(&mut self) -> &Self {
        let Some(ob) = self.ops.take() else {
            return self;
        };
        let op = *ob;
        match op {
            Op::Finalize => self.reorder(),
            Op::Transpose(op) => {
                self.ops = Some(op);
                self.finalize();
                self.row_major = false;
                self.shape = (self.shape.1, self.shape.0);
                self.reorder()
            }
            Op::Multiply(mut rhs, op) => {
                self.ops = Some(op);
                let result = self.finalize().mul(&rhs.finalize());
                self.data = result.data;
                self.row_major = true;
                self.shape = result.shape;
                self
            }
            Op::Add(mut rhs, op) => {
                self.ops = Some(op);
                let result = self.finalize() + rhs.finalize();
                self.data = result.data;
                self.row_major = true;
                self.shape = result.shape;
                self
            }
        }
    }

    fn naive_mul(&self, rhs: &Mat2) -> Mat2 {
        let n = self.shape.0;
        let m = self.shape.1;
        let p = rhs.shape.1;

        let mut result = Mat2::zeroes((n, p));
        for i in 0..n {
            for j in 0..p {
                result[[i, j]] = 0.0 as Element;
                for k in 0..m {
                    result[[i, j]] += self[[i, k]] * rhs[[k, j]];
                }
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

    // pub fn iter(mut self) -> Iter {
    //     self.into_iter()
    // }
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

impl Add for Mat2 {
    type Output = Mat2;

    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.shape, rhs.shape, "Incompatible matrix dimensions: {:?} vs {:?}", self.shape, rhs.shape);
        Mat2 {
            shape: self.shape,
            row_major: true,
            data: self.elements().zip(rhs.elements()).map(|(x, y)| x + y).collect(),
            ops: Some(Box::from(Op::Finalize)),
        }
    }
}

impl Add for &Mat2 {
    type Output = Mat2;

    fn add(self, rhs: Self) -> Self::Output {
        self.add(rhs)
    }
}

impl AddAssign for Mat2 {
    fn add_assign(&mut self, rhs: Self) {
        assert_eq!(self.shape, rhs.shape, "Incompatible matrix dimensions: {:?} vs {:?}", self.shape, rhs.shape);
        self.data = self.elements().zip(rhs.elements()).map(|(x, y)| x + y).collect();
        self.row_major = true;
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
        for (i, e) in self.elements().enumerate() {
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