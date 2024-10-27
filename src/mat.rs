use std::ops::{Add, Index, Mul};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Mat {
    pub shape: Vec<u8>,
    pub data: Vec<u8>,
}

impl Mat {
    pub fn zeroes(shape: &[u8]) -> Mat {
        let size = shape.iter().fold(1usize, |a, e| a * *e as usize);
        Mat {
            shape: Vec::from(shape),
            data: vec![0; size],
        }
    }
}
impl Add for &Mat {
    type Output = Mat;

    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.shape, rhs.shape);

        Mat {
            shape: self.shape.clone(),
            data: self.data.iter().zip(&rhs.data).map(|(a, b)| a + b).collect(),
        }
    }
}


impl Mul for &Mat {
    type Output = Mat;

    fn mul(self, rhs: Self) -> Self::Output {
        assert!(self.shape.len() <= 2 && rhs.shape.len() <= 2 && self.shape[self.shape.len() - 1] == rhs.shape[0]);
        Mat {
            shape: vec![0],
            data: vec![0]
        }
    }
}