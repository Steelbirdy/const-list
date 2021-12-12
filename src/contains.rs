use super::{Cons, List, Nil};

pub trait Contains<const X: usize>: List {
    const CONTAINS: bool;
}

impl<const X: usize, Xs: Contains<K>, const K: usize> Contains<K> for Cons<X, Xs> {
    const CONTAINS: bool = { X == K || Xs::CONTAINS };
}

impl<const K: usize> Contains<K> for Nil {
    const CONTAINS: bool = false;
}

pub trait ContainsAll<L: List>: List {
    const CONTAINS_ALL: bool;
}

impl<const X: usize, Xs: List> ContainsAll<Nil> for Cons<X, Xs> {
    const CONTAINS_ALL: bool = true;
}

impl<const X: usize, Xs: List, const K: usize, L: List> ContainsAll<Cons<K, L>> for Cons<X, Xs>
where
    Self: Contains<K> + ContainsAll<L>,
{
    const CONTAINS_ALL: bool =
        { <Self as Contains<K>>::CONTAINS && <Self as ContainsAll<L>>::CONTAINS_ALL };
}

impl ContainsAll<Nil> for Nil {
    const CONTAINS_ALL: bool = true;
}

impl<const K: usize, L: List> ContainsAll<Cons<K, L>> for Nil {
    const CONTAINS_ALL: bool = false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains() {
        assert!(<list!(1, 2, 3) as Contains<2>>::CONTAINS);
        assert!(!<list!(1, 2, 3) as Contains<4>>::CONTAINS);
    }

    #[test]
    fn test_contains_all() {
        assert!(<list!(2, 4, 6, 8, 10) as ContainsAll<list!(4, 8, 2, 6)>>::CONTAINS_ALL);
        assert!(!<list!(2, 4, 6, 8, 10) as ContainsAll<list!(4, 8, 2, 6, 1)>>::CONTAINS_ALL);
    }
}