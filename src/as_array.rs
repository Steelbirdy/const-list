use crate::{List, Cons, Nil};

pub trait AsArray: List {
    fn as_array() -> [usize; Self::LEN];
}

impl<const X: usize, Xs: List> /* const */ AsArray for Cons<X, Xs>
where
    Xs: /* ~const */ AsArray,
    [(); Self::LEN]:,
    [(); Xs::LEN]:,
{
    fn as_array() -> [usize; Self::LEN] {
        let mut arr = [0; Self::LEN];
        let next_arr = Xs::as_array();

        arr[0] = X;

        let mut i = 1;
        while i < Self::LEN {
            arr[i] = next_arr[i - 1];
            i += 1;
        }

        arr
    }
}

impl /* const */ AsArray for Nil {
    fn as_array() -> [usize; Self::LEN] {
        []
    }
}