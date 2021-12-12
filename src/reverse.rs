use super::{Cons, List, Nil};

pub trait Reverse: List {
    type Output: List;
}

impl Reverse for Nil {
    type Output = Nil;
}

impl<const X: usize, Xs: Reverse> Reverse for Cons<X, Xs> {
    type Output = <Xs::Output as List>::PushBack<X>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        assert_lists_eq!(<list!(1, 2, 3, 4) as Reverse>::Output, list!(4, 3, 2, 1));
        assert_lists_eq!(<list!(1) as Reverse>::Output, list!(1));
    }
}
