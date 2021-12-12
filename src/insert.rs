use super::{List, Cons, Nil};

pub trait Insert<const I: usize, const V: usize>: List {
    type Output: List;
}

impl<L, const I: usize, const V: usize> Insert<I, V> for L
where
    L: __Insert<false, I, V>,
{
    type Output = <L as __Insert<false, I, V>>::Output;
}

pub trait __Insert<const DONE: bool, const I: usize, const V: usize>: List {
    type Output: List;
}

impl<const I: usize, const V: usize> __Insert<true, I, V> for Nil {
    type Output = Nil;
}

impl<const V: usize> __Insert<false, 0, V> for Nil {
    type Output = Cons<V, Nil>;
}

impl<const X: usize, Xs: List, const I: usize, const V: usize> __Insert<true, I, V> for Cons<X, Xs> {
    type Output = Cons<V, Self>;
}

pub const fn if_else<T: Copy>(pred: bool, a: T, b: T) -> T {
    if pred {
        a
    } else {
        b
    }
}

impl<const X: usize, Xs, const I: usize, const V: usize> __Insert<false, I, V> for Cons<X, Xs>
where
    Xs: __Insert<{ I == 0 }, { I.saturating_sub(1) }, { if_else(I == 0, X, V) }>,
    [(); if_else(I == 0, V, X)]: ,
{
    type Output = Cons<{ if_else(I == 0, V, X) }, <Xs as __Insert<{ I == 0 }, { I.saturating_sub(1) }, { if_else(I == 0, X, V) }>>::Output>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        assert_lists_eq!(<list!(1, 2, 3, 4) as Insert<2, 5>>::Output, list!(1, 2, 5, 3, 4));
        assert_lists_eq!(<list!(1, 2, 3, 4) as Insert<0, 2>>::Output, list!(2, 1, 2, 3, 4));
        assert_lists_eq!(<list!(1, 2, 3, 4) as Insert<4, 8>>::Output, list!(1, 2, 3, 4, 8));

        // This fails at compile time, showing that attempting to insert an item out of bounds fails.
        // assert_lists_eq!(<list!(1, 2, 3, 4) as Insert<5, 2>>::Output, list!());
    }
}