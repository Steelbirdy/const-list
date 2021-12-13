use super::{List, Cons, Nil};

pub trait Remove<const I: usize> {
    type Output: List;
}

impl<const X: usize, Xs: List, const I: usize> Remove<I> for Cons<X, Xs>
where
    Xs: __Remove<{ I == 0 }, { I.saturating_sub(1) }, X>,
{
    type Output = <Xs as __Remove<{ I == 0 }, { I.saturating_sub(1) }, X>>::Output;
}

pub trait __Remove<const DONE: bool, const I: usize, const STORE: usize> {
    type Output: List;
}

impl<const I: usize, const STORE: usize> __Remove<true, I, STORE> for Nil {
    type Output = Nil;
}

impl<const X: usize, Xs: List, const I: usize, const STORE: usize> __Remove<true, I, STORE> for Cons<X, Xs> {
    type Output = Self;
}

impl<const X: usize, Xs: List, const I: usize, const STORE: usize> __Remove<false, I, STORE> for Cons<X, Xs>
where
    Xs: __Remove<{ I == 0 }, { I.saturating_sub(1) }, X>,
{
    type Output = Cons<STORE, Xs::Output>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove() {
        assert_lists_eq!(<list!(3, 5, 2, 4) as Remove<3>>::Output, list!(3, 5, 2));
        assert_lists_eq!(<list!(3, 2, 1) as Remove<0>>::Output, list!(2, 1));
        assert_lists_eq!(<list!(1, 2, 3) as Remove<1>>::Output, list!(1, 3));
        assert_lists_eq!(<list!(2) as Remove<0>>::Output, list!());
    }
}