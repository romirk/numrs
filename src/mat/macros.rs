#[macro_export]
macro_rules! count {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + count!($($xs)*));
}

#[macro_export]
macro_rules! mat {
    ( $( $x:expr ),* ) => { {
        let data = vec![$($x),*].into_boxed_slice();
        crate::mat::Mat2 {shape: (1, data.len() as isize) , data, row_major: true }
    } };
    ( $( $x0:expr ),* ; $($( $x:expr ),*);* ) => {
        crate::mat::Mat2::new(
            (
                (1isize $(+ 1isize + (if false { [$($x,)*][0] as isize } else { 0 }) )*),
                (count!($($x0)*)) as isize
            ),
            vec![$( ($x0) as crate::mat::Element, )* $($(($x) as crate::mat::Element, )*)*].into_boxed_slice()
        )
    }
}