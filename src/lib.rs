#![allow(incomplete_features)]
#![feature(generic_const_exprs, generic_associated_types, const_trait_impl)]

mod as_array;
mod contains;

pub struct Cons<const X: usize, Xs: List>(std::marker::PhantomData<Xs>);
pub struct Nil;

pub trait List {
    const LEN: usize;
    const SUM: usize;

    type PushFront<const A: usize>: List;
    type PushBack<const Z: usize>: List;

    type Reversed: List;
}

impl<const X: usize, Xs: List> List for Cons<X, Xs> {
    const LEN: usize = 1 + Xs::LEN;
    const SUM: usize = X + Xs::SUM;

    type PushFront<const A: usize> = Cons<A, Self>;
    type PushBack<const Z: usize> = Cons<X, Xs::PushBack<Z>>;

    type Reversed = <Xs::Reversed as List>::PushBack<X>;
}

impl List for Nil {
    const LEN: usize = 0;
    const SUM: usize = 0;

    type PushFront<const A: usize> = Cons<A, Nil>;
    type PushBack<const Z: usize> = Cons<Z, Nil>;

    type Reversed = Nil;
}
