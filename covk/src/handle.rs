use crate::*;
use alloc::boxed::Box;
use core::any::TypeId;
use core::fmt::{Debug, Display, Pointer};
use core::hash::{Hash, Hasher};
use core::ops::Deref;
use core::ptr::NonNull;
use core::sync::atomic::{AtomicUsize, Ordering};

#[repr(C)]
#[derive(Debug)]
pub struct HndDataBase<T, S> {
    pub p_drop: unsafe fn(NonNull<HndDataBase<T, S>>),
    pub count: AtomicUsize,
    pub raw: T,
    pub scope: S,
}

#[repr(C)]
#[derive(Debug)]
pub struct HndData<T, S, Dep> {
    pub base: HndDataBase<T, S>,
    pub dep: Dep,
}

impl<T, S, Dep> Deref for HndData<T, S, Dep> {
    type Target = HndDataBase<T, S>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

pub trait HndInst<T, S, Dep> {
    fn drop(this: &mut HndData<T, S, Dep>);
}

#[repr(transparent)]
pub struct Hnd<T, S>(NonNull<HndDataBase<T, S>>);

impl<T, S> Hnd<T, S> {
    pub fn new<Dep: Send + Sync + 'static, Inst: HndInst<T, S, Dep>>(
        scope: S,
        raw: T,
        dep: Dep,
        inst: Inst,
    ) -> Self {
        unsafe {
            Self(
                NonNull::new_unchecked(Box::leak(Box::new(HndData::<T, S, Dep> {
                    base: HndDataBase {
                        p_drop: Self::drop::<Dep, Inst>,
                        count: AtomicUsize::new(1),
                        scope,
                        raw,
                    },
                    dep,
                })))
                .cast::<HndDataBase<T, S>>(),
            )
        }
    }

    unsafe fn drop<Dep, Inst: HndInst<T, S, Dep>>(this: NonNull<HndDataBase<T, S>>) {
        unsafe {
            let mut this = Box::from_raw(this.cast::<HndData<T, S, Dep>>().as_ptr());
            Inst::drop(&mut this);
            drop(this);
        }
    }
}

impl<T, S> Deref for Hnd<T, S> {
    type Target = HndDataBase<T, S>;
    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref() }
    }
}

impl<T, S> Hnd<T, S> {
    fn add_ref(&self) {
        unsafe {
            self.count.fetch_add(1, Ordering::Relaxed);
        }
    }
    fn release(&self) {
        unsafe {
            if 1 == self.count.fetch_sub(1, Ordering::Release) {
                core::sync::atomic::fence(Ordering::Acquire);
                unsafe {
                    (self.p_drop)(self.0);
                }
            }
        }
    }
}

impl<T, S> Clone for Hnd<T, S> {
    fn clone(&self) -> Self {
        self.add_ref();
        Self(self.0)
    }
}

unsafe impl<T: Send, S: Send> Send for Hnd<T, S> {}
unsafe impl<T: Sync, S: Sync> Sync for Hnd<T, S> {}

impl<T: Debug, S> Debug for Hnd<T, S> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.raw.fmt(f)
    }
}

impl<T: Pointer, S> Pointer for Hnd<T, S> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.raw.fmt(f)
    }
}

impl<T: Display, S> Display for Hnd<T, S> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.raw.fmt(f)
    }
}

impl<T: PartialEq, S1, S2> PartialEq<Hnd<T, S2>> for Hnd<T, S1> {
    fn eq(&self, other: &Hnd<T, S2>) -> bool {
        self.raw.eq(&other.raw)
    }
    fn ne(&self, other: &Hnd<T, S2>) -> bool {
        self.raw.ne(&other.raw)
    }
}

impl<T: Eq, S> Eq for Hnd<T, S> {}

impl<T: Hash, S> Hash for Hnd<T, S> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.raw.hash(state)
    }
}

impl<T: PartialOrd, S1, S2> PartialOrd<Hnd<T, S2>> for Hnd<T, S1> {
    fn partial_cmp(&self, other: &Hnd<T, S2>) -> Option<core::cmp::Ordering> {
        self.raw.partial_cmp(&other.raw)
    }

    fn lt(&self, other: &Hnd<T, S2>) -> bool {
        self.raw.lt(&other.raw)
    }

    fn le(&self, other: &Hnd<T, S2>) -> bool {
        self.raw.le(&other.raw)
    }

    fn gt(&self, other: &Hnd<T, S2>) -> bool {
        self.raw.gt(&other.raw)
    }

    fn ge(&self, other: &Hnd<T, S2>) -> bool {
        self.raw.ge(&other.raw)
    }
}

impl<T: Ord, S> Ord for Hnd<T, S> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.raw.cmp(&other.raw)
    }
}

impl<T, S> Drop for Hnd<T, S> {
    fn drop(&mut self) {
        self.release();
    }
}
