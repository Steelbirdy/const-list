use super::{List, Cons, Nil};

pub trait IndexIter {
    type Output: List;
}

impl IndexIter for Nil {
    type Output = Nil;
}

impl<const X: usize, Xs: List> IndexIter for Cons<X, Xs>
where
    Xs: IndexIter,
    [(); Self::LEN - 1]: ,
{
    type Output = <Xs::Output as List>::PushBack<{ Self::LEN - 1 }>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enumerate() {
        assert_lists_eq!(<list!(4, 6, 2, 1) as IndexIter>::Output, list!(0, 1, 2, 3));
        assert_lists_eq!(<list!(5) as IndexIter>::Output, list!(0));
        assert_lists_eq!(<list!() as IndexIter>::Output, list!());
    }
}