use super::{List, Cons, Nil};

pub trait Find<const V: usize> {
    const INDEX: Option<usize>;
}

impl<const V: usize> Find<V> for Nil {
    const INDEX: Option<usize> = None;
}

impl<const X: usize, Xs: List, const V: usize> Find<V> for Cons<X, Xs>
where
    Self: __Find<0, V>,
{
    const INDEX: Option<usize> = <Self as __Find<0, V>>::INDEX;
}

pub trait __Find<const I: usize, const V: usize> {
    const INDEX: Option<usize>;
}

impl<const I: usize, const V: usize> __Find<I, V> for Nil {
    const INDEX: Option<usize> = None;
}

impl<const X: usize, Xs: List, const I: usize, const V: usize> __Find<I, V> for Cons<X, Xs>
where
    Xs: __Find<{ I + 1 }, V>,
{
    const INDEX: Option<usize> = {
        if X == V {
            Some(I)
        } else {
            Xs::INDEX
        }
    };
}

pub trait FindAll<const V: usize> {
    type Output: List;
}

impl<const V: usize> FindAll<V> for Nil {
    type Output = Nil;
}

impl<const X: usize, Xs: List, const V: usize> FindAll<V> for Cons<X, Xs>
where
    Self: __FindAll<false, 0, V>,
{
    type Output = <Self as __FindAll<false, 0, V>>::Output;
}

pub trait __FindAll<const FOUND: bool, const I: usize, const V: usize> {
    type Output: List;
}

impl<L: List, const I: usize, const V: usize> __FindAll<true, I, V> for L
where
    L: __FindAll<false, { I + 1 }, V>,
{
    type Output = Cons<I, L::Output>;
}

impl<const I: usize, const V: usize> __FindAll<false, I, V> for Nil {
    type Output = Nil;
}

impl<const X: usize, Xs: List, const I: usize, const V: usize> __FindAll<false, I, V> for Cons<X, Xs>
where
    Xs: __FindAll<{ X == V }, { I + (X != V) as usize }, V>,
{
    type Output = Xs::Output;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find() {
        assert_eq!(<list!(1, 2, 3, 4, 5) as Find<3>>::INDEX, Some(2));
        assert_eq!(<list!(2) as Find<2>>::INDEX, Some(0));
        assert_eq!(<list!() as Find<5>>::INDEX, None);
        assert_eq!(<list!(1, 2, 3, 4, 5) as Find<6>>::INDEX, None);
    }

    #[test]
    fn test_find_all() {
        assert_lists_eq!(<list!(1, 2, 3, 4, 5, 1, 2, 3, 4) as FindAll<1>>::Output, list!(0, 5));
    }
}