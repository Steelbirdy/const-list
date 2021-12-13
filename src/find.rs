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
}