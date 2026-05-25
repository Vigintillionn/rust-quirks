use std::cell::UnsafeCell;
use std::marker::PhantomData;

type InvariantBrand<'id> = PhantomData<fn(&'id ()) -> &'id ()>;

/// The global permission slip for the graph.
pub struct GhostToken<'id> {
    _brand: InvariantBrand<'id>,
}

/// The data wrapper.
pub struct GhostCell<'id, T: ?Sized> {
    _brand: InvariantBrand<'id>,
    value: UnsafeCell<T>,
}

impl<'id> GhostToken<'id> {
    pub fn new<F, R>(f: F) -> R
    where
        F: for<'new_id> FnOnce(GhostToken<'new_id>) -> R,
    {
        f(GhostToken {
            _brand: PhantomData,
        })
    }

    /// Mutably borrows two distinctly different cells at the same time.
    pub fn borrow_mut2<'a, T, U>(
        &'a mut self,
        c1: &'a GhostCell<'id, T>,
        c2: &'a GhostCell<'id, U>,
    ) -> (&'a mut T, &'a mut U) {
        assert!(
            !std::ptr::eq(c1 as *const _ as *const (), c2 as *const _ as *const ()),
            "Cannot mutably borrow the same cell twice!"
        );

        // SAFETY: We hold the exclusive token, so no other thread or code
        // can be accessing this graph. We proved c1 and c2 are distinct
        // memory addresses. Therefore, it is safe to hand out two `&mut`s.
        unsafe { (&mut *c1.value.get(), &mut *c2.value.get()) }
    }
}

impl<'id, T> GhostCell<'id, T> {
    pub fn new(value: T) -> Self {
        Self {
            _brand: PhantomData,
            value: UnsafeCell::new(value),
        }
    }

    pub fn borrow<'a>(&'a self, _token: &'a GhostToken<'id>) -> &'a T {
        // SAFETY: We require `&GhostToken`, which proves no one currently
        // holds a `&mut GhostToken`. Therefore, no one is currently mutating
        // the graph. It is safe to hand out a read-only reference.
        unsafe { &*self.value.get() }
    }

    pub fn borrow_mut<'a>(&'a self, _token: &'a mut GhostToken<'id>) -> &'a mut T {
        // SAFETY: Rust's borrow checker guarantees that `&mut GhostToken`
        // is strictly exclusive. Since the user must pass it here, we statically
        // know that NO OTHER PART OF THE PROGRAM can currently be borrowing
        // ANY cell branded with this `'id`, mutably or immutably.
        // Thus, we have exclusive access to this `UnsafeCell`.
        unsafe { &mut *self.value.get() }
    }
}
