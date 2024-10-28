use crate::count;
use crate::mat;
use crate::mat::Mat2;

#[test]
fn sanity() {
    let mat = Mat2 {
        shape: (2, 2),
        data: vec![1.0, 0.0, 0.0, 1.0].into_boxed_slice(),
        row_major: true,
    };
    assert_eq!(Mat2::identity((2, 2)), mat);
}

#[test]
fn index() {
    let mat = Mat2 {
        shape: (2, 2),
        data: vec![1.0, 0.0, 0.0, 1.0].into_boxed_slice(),
        row_major: true,
    };

    assert_eq!(mat[1], [0.0, 1.0]);
    assert_eq!(mat[0][1], 0.0);
}

#[test]
fn transpose() {
    let mat1 = Mat2::identity((2, 2));
    let mat2 = Mat2::identity((2, 2));
    let mat3 = Mat2 {
        data: vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0].into_boxed_slice(),
        shape: (3, 2),
        row_major: true,
    };
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