use super::{List, Cons, Nil};

pub trait Remove<const I: usize> {
    type Output: List;
}

impl<const X: usize, Xs: List, const I: usize> Remove<I> for Cons<X, Xs>
where
    Xs: __Remove<{ I == 0 }, { I.saturating_sub(1) }, X>,
{
    type Output = Xs::Output;
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

pub trait RemoveValue<const V: usize> {
    type Output: List;
}

impl<const V: usize> RemoveValue<V> for Nil {
    type Output = Nil;
}

impl<const X: usize, Xs: List, const V: usize> RemoveValue<V> for Cons<X, Xs>
where
    Xs: __RemoveValue<{ X == V }, V, X>,
{
    type Output = Xs::Output;
}

pub trait __RemoveValue<const DONE: bool, const V: usize, const STORE: usize> {
    type Output: List;
}

impl<const V: usize, const STORE: usize> __RemoveValue<true, V, STORE> for Nil {
    type Output = Nil;
}

impl<const V: usize, const STORE: usize> __RemoveValue<false, V, STORE> for Nil {
    type Output = Cons<STORE, Nil>;
}

impl<const X: usize, Xs: List, const V: usize, const STORE: usize> __RemoveValue<true, V, STORE> for Cons<X, Xs> {
    type Output = Self;
}

impl<const X: usize, Xs: List, const V: usize, const STORE: usize> __RemoveValue<false, V, STORE> for Cons<X, Xs>
where
    Xs: __RemoveValue<{ X == V }, V, X>,
{
    type Output = Cons<STORE, Xs::Output>;
}

pub trait RemoveValueAll<const V: usize> {
    type Output: List;
}

impl<const V: usize> RemoveValueAll<V> for Nil {
    type Output = Nil;
}

impl<const X: usize, Xs: List, const V: usize> RemoveValueAll<V> for Cons<X, Xs>
where
    Xs: __RemoveValueAll<{ X == V }, V, X>,
{
    type Output = <Xs as __RemoveValueAll<{ X == V }, V, X>>::Output;
}

pub trait __RemoveValueAll<const FOUND: bool, const V: usize, const STORE: usize> {
    type Output: List;
}

impl<const V: usize, const STORE: usize> __RemoveValueAll<true, V, STORE> for Nil {
    type Output = Nil;
}

impl<const X: usize, Xs: List, const V: usize, const STORE: usize> __RemoveValueAll<true, V, STORE> for Cons<X, Xs>
where
    Xs: __RemoveValueAll<{ X == V }, V, X>,
{
    type Output = Xs::Output;
}

impl<const V: usize, const STORE: usize> __RemoveValueAll<false, V, STORE> for Nil {
    type Output = Cons<STORE, Nil>;
}

impl<const X: usize, Xs: List, const V: usize, const STORE: usize> __RemoveValueAll<false, V, STORE> for Cons<X, Xs>
where
    Xs: __RemoveValueAll<{ X == V }, V, X>,
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

    #[test]
    fn test_remove_value() {
        assert_lists_eq!(<list!(3, 5, 2, 4) as RemoveValue<2>>::Output, list!(3, 5, 4));
        assert_lists_eq!(<list!(3, 5, 2, 4) as RemoveValue<1>>::Output, list!(3, 5, 2, 4));
    }

    #[test]
    fn test_remove_all_value() {
        assert_lists_eq!(<list!(3, 5, 2, 4, 3, 5, 2, 4, 3, 5) as RemoveValueAll<5>>::Output, list!(3, 2, 4, 3, 2, 4, 3));
        assert_lists_eq!(<list!(1, 2, 3) as RemoveValueAll<4>>::Output, list!(1, 2, 3));
        assert_lists_eq!(<list!(1) as RemoveValueAll<1>>::Output, list!());
        assert_lists_eq!(<list!() as RemoveValueAll<7>>::Output, list!());
    }
}