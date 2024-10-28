#[macro_export]
macro_rules! mat {
    ( $( $x:expr ),* ) => { {
        let row_vector = Box::new([$($x),*]);
        $crate::mat::Mat2::new((1, row_vector.len()) , row_vector)
    } };
    ( $($( $x:expr ),+);+ ) => {
        $crate::mat::Mat2::from([$([$(($x) as $crate::mat::Element),+]),+])
    }
}