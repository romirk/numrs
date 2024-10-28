use crate::mat;
use crate::mat::{Element, Mat2};

#[test]
fn sanity() {
    assert_eq!(Mat2::I(2), mat![1, 0; 0, 1]);
}

#[test]
fn index() {
    let mat = mat![1, 0; 0, 1];
    assert_eq!(mat[1], [0 as Element, 1]);
    assert_eq!(mat[0][1], 0 as Element);
}

#[test]
fn transpose() {
    let mat1 = Mat2::I(2);
    let mat2 = Mat2::I(2);
    let mat3 = mat![
        1, 2;
        3, 4;
        5, 6
    ];
    assert_eq!(mat1, mat2.T());
    assert_eq!(mat3[[0, 1]], mat3.T()[[1, 0]]);
}

#[test]
fn macro_rules() {
    let mat = mat![
        1.0, 0.0, 0.0;
        0.0, 1.0, 0.0;
        0.0, 0.0, 1.0
    ];
    assert_eq!(mat, Mat2::I(3));
}

#[allow(non_snake_case)]
#[test]
fn matmul() {
    let A = &mat![
         6, 2, 4;
        -1, 4, 3;
        -2, 9, 3
    ];
    let B = &mat![
         4;
        -2;
         1
    ];
    let C: Mat2 = mat![
        24;
        -9;
        -23
    ];

    assert_eq!(A * B, C);
}

#[test]
fn iter() {
    let rows = [[1 as Element, 0, 0], [0, 1, 0], [0, 0, 1]];
    let mat =  Mat2::I(3);
    for (i, e) in mat.pairs() {
        assert_eq!(e, rows[i[0]][i[1]]);
    }
    for (i, row) in mat.T().iter().enumerate() {
        assert_eq!(row, rows[i]);
    }
}