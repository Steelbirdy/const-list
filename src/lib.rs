#![allow(incomplete_features)]
#![feature(generic_const_exprs, generic_associated_types, const_trait_impl, const_fn_trait_bound)]

#[macro_use]
mod macros;

mod as_array;
mod contains;
mod index;
mod reverse;
mod insert;
mod index_iter;

pub use as_array::AsArray;
pub use contains::{Contains, ContainsAll};
pub use index::ListIndex;
pub use reverse::Reverse;
pub use insert::Insert;

pub struct Cons<const X: usize, Xs: List>(std::marker::PhantomData<Xs>);
pub struct Nil;

pub trait List {
    const LEN: usize;
    const SUM: usize;
    const PRODUCT: usize;

    type PushFront<const A: usize>: List;
    type PushBack<const Z: usize>: List;

    type Reversed: List;
    type Concat<L: List>: List;
}

impl<const X: usize, Xs: List> List for Cons<X, Xs> {
    const LEN: usize = 1 + Xs::LEN;
    const SUM: usize = X + Xs::SUM;
    const PRODUCT: usize = X * Xs::PRODUCT;

    type PushFront<const A: usize> = Cons<A, Self>;
    type PushBack<const Z: usize> = Cons<X, Xs::PushBack<Z>>;

    type Reversed = <Xs::Reversed as List>::PushBack<X>;
    type Concat<L: List> = Cons<X, Xs::Concat<L>>;
}

impl List for Nil {
    const LEN: usize = 0;
    const SUM: usize = 0;
    const PRODUCT: usize = 1;

    type PushFront<const A: usize> = Cons<A, Nil>;
    type PushBack<const Z: usize> = Cons<Z, Nil>;

    type Reversed = Nil;
    type Concat<L: List> = L;
}
