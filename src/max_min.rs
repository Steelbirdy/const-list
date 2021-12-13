use super::{List, Cons, Nil};

pub const fn max(a: usize, b: usize) -> usize {
    if a < b {
        b
    } else {
        a
    }
}

pub const fn min(a: usize, b: usize) -> usize {
    if a > b {
        b
    } else {
        a
    }
}

pub trait Max {
    const OUTPUT: usize;
}

pub trait Min {
    const OUTPUT: usize;
}

impl<const X: usize, Xs: List> Max for Cons<X, Xs>
where
    Xs: __Max<X>,
{
    const OUTPUT: usize = Xs::OUTPUT;
}

impl<const X: usize, Xs: List> Min for Cons<X, Xs>
where
    Xs: __Min<X>,
{
    const OUTPUT: usize = Xs::OUTPUT;
}

pub trait __Max<const V: usize> {
    const OUTPUT: usize;
}

impl<const V: usize> __Max<V> for Nil {
    const OUTPUT: usize = V;
}

impl<const X: usize, Xs: List, const V: usize> __Max<V> for Cons<X, Xs>
where
    Xs: __Max<{ max(X, V) }>,
{
    const OUTPUT: usize = Xs::OUTPUT;
}

pub trait __Min<const V: usize> {
    const OUTPUT: usize;
}

impl<const V: usize> __Min<V> for Nil {
    const OUTPUT: usize = V;
}

impl<const X: usize, Xs: List, const V: usize> __Min<V> for Cons<X, Xs>
where
    Xs: __Min<{ min(X, V) }>,
{
    const OUTPUT: usize = Xs::OUTPUT;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max() {
        assert_eq!(<list!(1, 2, 3, 4, 5) as Max>::OUTPUT, 5);
        assert_eq!(<list!(4) as Max>::OUTPUT, 4);
    }

    #[test]
    fn test_min() {
        assert_eq!(<list!(1, 2, 3, 4, 5) as Min>::OUTPUT, 1);
        assert_eq!(<list!(4) as Min>::OUTPUT, 4);
    }
}