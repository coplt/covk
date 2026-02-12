use crate::{vk, CoreDevice};
use core::ops::Deref;

/// Unique handle owner, responsible for dropping the handle when it goes out of scope
pub trait Owner<T: Copy, Scope> {
    fn drop(&mut self, raw: T);
}

/// Unique raii wrapper for vulkan handles
#[derive(Debug)]
pub struct Unique<T: Copy, O: Owner<T, S>, S = vk::core> {
    pub raw: T,
    pub owner: O,
    _p: ::core::marker::PhantomData<*const S>,
}

unsafe impl<T: Copy + Send, O: Owner<T, S> + Send, S> Send for Unique<T, O, S> {}
unsafe impl<T: Copy + Sync, O: Owner<T, S> + Sync, S> Sync for Unique<T, O, S> {}

impl<T: Copy, O: Owner<T, S>, S> Unique<T, O, S> {
    pub fn new(raw: T, owner: O) -> Self {
        Self {
            raw,
            owner,
            _p: ::core::marker::PhantomData,
        }
    }
}

impl<T: Copy, O: Owner<T, S>, S> Drop for Unique<T, O, S> {
    fn drop(&mut self) {
        self.owner.drop(self.raw);
    }
}

impl<T: Copy, O: Owner<T, S>, S> Deref for Unique<T, O, S> {
    type Target = O;

    fn deref(&self) -> &Self::Target {
        &self.owner
    }
}

/// Make unique
pub trait MakeUnique<O, S> {
    /// The unique
    type Output;
    /// Make a unique from a raw handle and an owner
    fn unique(self, owner: O) -> Self::Output;
}

impl<T: Copy, O: Owner<T, S>, S> MakeUnique<O, S> for T {
    type Output = Unique<T, O, S>;

    fn unique(self, owner: O) -> Self::Output {
        Unique::new(self, owner)
    }
}
