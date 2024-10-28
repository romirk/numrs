#[macro_export]
macro_rules! mat {
    ( $($( $x:expr ),+);+$(;)? ) => {
        $crate::mat::Mat2::from([$([$(($x) as $crate::mat::Element),+]),+])
    }
}