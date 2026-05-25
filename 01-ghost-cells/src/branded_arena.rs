use std::marker::PhantomData;

/// The invariant brand strictly prevents the compiler from merging lifetimes.
type InvariantBrand<'id> = PhantomData<fn(&'id ()) -> &'id ()>;

pub struct Arena<'id, T> {
    data: Vec<T>,
    _brand: InvariantBrand<'id>,
}

#[derive(Clone, Copy)]
pub struct Index<'id> {
    index: usize,
    _brand: InvariantBrand<'id>,
}

impl<'id, T> Arena<'id, T> {
    /// Generative closure to mint a unique lifetime for this Arena instance.
    pub fn new<F, R>(f: F) -> R
    where
        F: for<'new_id> FnOnce(Arena<'new_id, T>) -> R,
    {
        let arena = Arena {
            data: Vec::new(),
            _brand: PhantomData,
        };
        f(arena)
    }

    pub fn insert(&mut self, val: T) -> Index<'id> {
        let index = self.data.len();
        self.data.push(val);
        Index {
            index,
            _brand: PhantomData,
        }
    }

    pub fn get(&self, idx: Index<'id>) -> &T {
        &self.data[idx.index]
    }
}
