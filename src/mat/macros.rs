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
    ( $( $x0:expr ),* ; $($( $x:expr ),*);* ) => { {
        let mut rows = 1isize;
        let cols = count!($($x0)*);
        let mut vec = Vec::with_capacity(cols * 2);

        vec.extend([$( ($x0) as crate::mat::Element, )*]);
        $(
            rows += 1isize;
            vec.extend([$( ($x) as crate::mat::Element, )*]);
        )*

        crate::mat::Mat2::new((rows, cols as isize), vec.into_boxed_slice())
    } }
}