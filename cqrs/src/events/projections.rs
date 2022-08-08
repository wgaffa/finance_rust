use std::marker::PhantomData;

pub struct Projection<S, E, F>
where
    F: Fn(S, &E) -> S,
{
    init: S,
    update: F,
    _phantom: PhantomData<E>,
}

impl<S, E, F> Projection<S, E, F>
where
    F: Fn(S, &E) -> S,
{
    pub fn new(init: S, update: F) -> Self {
        Self { init, update, _phantom: PhantomData }
    }

    pub fn project<'a, I>(&'a self, iter: I) -> S
    where
        I: Iterator<Item = &'a E>,
        S: Clone
    {
        iter.fold(self.init.clone(), &self.update)
    }
}
