use crate::mat::Mat;
#[test]
fn it_works() {
    let mat = Mat::zeroes(&[2, 2, 2]);
    println!("{:?}", mat);
    assert_eq!(Mat::zeroes(&[2, 2, 2]), Mat {
        shape: vec![2, 2, 2],
        data: vec![0, 0, 0, 0, 0, 0, 0, 0],
    });
}

#[test]
fn add_zeroes() {
    let mat1 = &Mat {
        shape: vec![3, 3],
        data: vec![1, 0, 0, 0, 1, 0, 0, 0, 1],
    };
    let mat2 = &Mat::zeroes(&[3, 3]);
    let mat3 = mat1 + mat2;
    println!("{:?}", mat3);
    assert_eq!(*mat1, mat3);
}

#[test]
fn addition() {
    let mat1 = &Mat {
        shape: vec![3, 3],
        data: vec![1, 0, 0, 0, 1, 0, 0, 0, 1],
    };
    let mat2 = &Mat {
        shape: vec![3, 3],
        data: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
    };
    let mat3 = mat1 + mat2;
    println!("{:?}", mat3);
    assert_eq!(mat3, Mat {
        shape: vec![3, 3],
        data: vec![2, 2, 3, 4, 6, 6, 7, 8, 10],
    });
}