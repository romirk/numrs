use crate::mat;
use crate::mat::{Element, Mat2};

#[test]
fn sanity() {
    let mat = mat![1, 0; 0, 1];
    assert_eq!(Mat2::identity((2, 2)), mat);
}

#[test]
fn index() {
    let mat = mat![1, 0; 0, 1];
    assert_eq!(mat[1], [0 as Element, 1 as Element]);
    assert_eq!(mat[0][1], 0 as Element);
}

#[test]
fn transpose() {
    let mat1 = Mat2::identity((2, 2));
    let mat2 = Mat2::identity((2, 2));
    let mat3 = mat![
        1, 2;
        3, 4;
        5, 6
    ];
    assert_eq!(mat1, mat2.transpose());
    assert_eq!(mat1, mat1.transpose());
    let t = mat3.transpose();
    assert_ne!(mat3, t);
}

#[test]
fn macro_rules() {
    let mat = mat![
        1.0, 0.0, 0.0;
        0.0, 1.0, 0.0;
        0.0, 0.0, 1.0
    ];
    assert_eq!(mat, Mat2::identity((3, 3)));
}