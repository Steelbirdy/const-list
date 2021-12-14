#[macro_export]
macro_rules! list {
    ($first:expr $(, $x:expr)* $(,)?) => {
        $crate::Cons<$first, list!($($x),*)>
    };
    () => {
        $crate::Nil
    };
}

macro_rules! assert_lists_eq {
    ($l1:ty, $l2:ty) => {
        assert_eq!(
            <$l1 as $crate::as_array::AsArray>::as_array(),
            <$l2 as $crate::as_array::AsArray>::as_array()
        );
    };
}
