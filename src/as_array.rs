use crate::{Cons, List, Nil};

pub trait AsArray: List {
    fn as_array() -> [usize; Self::LEN];
}

impl<const X: usize, Xs> AsArray for Cons<X, Xs>
where
    Xs: AsArray,
    [(); Self::LEN]: ,
    [(); Xs::LEN]: ,
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

impl AsArray for Nil {
    fn as_array() -> [usize; Self::LEN] {
        []
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_array() {
        assert_eq!(<list!(1, 2, 3, 4, 5) as AsArray>::as_array(), [1, 2, 3, 4, 5]);
        assert_eq!(<list!(1) as AsArray>::as_array(), [1]);
        assert_eq!(<list!() as AsArray>::as_array(), []);
    }
}