#[macro_export]
macro_rules! count {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + count!($($xs)*));
}

#[macro_export]
macro_rules! mat {
    ( $( $x:expr ),* ) => { {
        let __mat__data__ = vec![$($x),*].into_boxed_slice();
        $crate::mat::Mat2::new((1, __mat__data__.len() as usize) , __mat__data__)
    } };
    ( $( $x0:expr ),* ; $($( $x:expr ),*);* ) => {
        $crate::mat::Mat2::new(
            (
                (1usize + (count!($([$($x,)*])*) as usize)),
                (count!($($x0)*)) as usize
            ),
            vec![$( ($x0) as $crate::mat::Element, )* $($(($x) as $crate::mat::Element, )*)*].into_boxed_slice()
        )
    }
}