use super::{Cons, List, Nil};

pub trait ListIndex<const I: usize>: List {
    const OUTPUT: Option<usize>;
}

impl<const I: usize> ListIndex<I> for Nil {
    const OUTPUT: Option<usize> = None;
}

impl<const X: usize, Xs: List, const I: usize> ListIndex<I> for Cons<X, Xs>
where
    Xs: ListIndex<{ I.saturating_sub(1) }>,
{
    const OUTPUT: Option<usize> = {
        if I == 0 {
            Some(X)
        } else {
            Xs::OUTPUT
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index() {
        assert_eq!(<list!(1, 4, 5, 8) as ListIndex<2>>::OUTPUT, Some(5));
        assert_eq!(<list!(1, 4, 5, 8) as ListIndex<5>>::OUTPUT, None);
    }
}
