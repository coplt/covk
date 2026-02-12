use crate::*;
use alloc::boxed::Box;
use core::any::TypeId;
use core::ffi::{c_char, c_void};
use core::sync::atomic::Ordering;
use covk_sys::PFN_vkVoidFunction;

#[repr(C)]
#[derive(Debug)]
pub struct HndMetaAny {
    pub count: AtomicUsize,
    pub p_drop_meta: unsafe fn(NonNull<HndMetaAny>),
    pub dep_type: TypeId,
    pub p_drop_hnd: NonNull<()>,
}

impl HndMetaAny {
    pub unsafe fn cast<T>(&self) -> &T {
        unsafe { core::mem::transmute(self) }
    }
}

#[repr(transparent)]
#[derive(Debug)]
pub struct HndMetaObject(pub NonNull<HndMetaAny>);

impl Deref for HndMetaObject {
    type Target = HndMetaAny;

    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref() }
    }
}

impl Clone for HndMetaObject {
    fn clone(&self) -> Self {
        self.count.fetch_add(1, Ordering::Relaxed);
        Self(self.0)
    }
}

impl Drop for HndMetaObject {
    fn drop(&mut self) {
        if 1 == self.count.fetch_sub(1, Ordering::Release) {
            core::sync::atomic::fence(Ordering::Acquire);
            unsafe { (self.p_drop_meta)(self.0) }
        }
    }
}
unsafe impl Send for HndMetaObject {}
unsafe impl Sync for HndMetaObject {}

#[repr(C)]
#[derive(Debug)]
pub struct HndMetaCommands<T: CommandSource> {
    pub base: HndMetaAny,
    pub hnd: T::Handle,
    pub commands: T::Commands,
}

impl<T: CommandSource> Deref for HndMetaCommands<T> {
    type Target = HndMetaAny;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

/// The command table with ref count
#[repr(transparent)]
pub struct Commands<T: CommandSource>(pub(crate) NonNull<HndMetaCommands<T>>);

impl<T: CommandSource> Commands<T> {
    /// Get the raw handle
    pub fn hnd(&self) -> T::Handle {
        unsafe { self.0.as_ref().hnd }
    }
}

impl<T: CommandSource> core::fmt::Debug for Commands<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let ptr = self.0.as_ptr();
        f.write_fmt(format_args!(
            "Commands<{}>({:p})",
            core::any::type_name::<T>(),
            ptr
        ))
    }
}
impl<T: CommandSource> core::fmt::Pointer for Commands<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let ptr = self.0.as_ptr();
        f.write_fmt(format_args!("{:p}", ptr))
    }
}

impl<T: CommandSource> Deref for Commands<T> {
    type Target = T::Commands;

    fn deref(&self) -> &Self::Target {
        unsafe { &self.0.as_ref().commands }
    }
}

impl<T: CommandSource> Clone for Commands<T> {
    fn clone(&self) -> Self {
        unsafe {
            self.0.as_ref().base.count.fetch_add(1, Ordering::Relaxed);
        }
        Self(self.0)
    }
}

impl<T: CommandSource> Drop for Commands<T> {
    fn drop(&mut self) {
        let this = unsafe { &self.0.as_ref().base };
        if 1 == this.count.fetch_sub(1, Ordering::Release) {
            core::sync::atomic::fence(Ordering::Acquire);
            unsafe { (this.p_drop_meta)(self.0.cast()) }
        }
    }
}

unsafe impl<T: CommandSource> Send for Commands<T> {}
unsafe impl<T: CommandSource> Sync for Commands<T> {}

#[repr(C)]
pub(crate) struct HndInner<T: Handle, Dep = <T as Handle>::DefaultDep> {
    pub base: AnyHndInner<T>,
    pub dep: Dep,
}

impl<T: Handle, Dep> Deref for HndInner<T, Dep> {
    type Target = AnyHndInner<T>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<T: Handle, Dep> HndInner<T, Dep> {
    pub fn new(hnd: T, dep: Dep) -> NonNull<Self> {
        unsafe {
            NonNull::new_unchecked(Box::leak(Box::new(Self {
                base: AnyHndInner {
                    raw: hnd,
                    count: AtomicUsize::new(1),
                },
                dep,
            })))
        }
    }
}

/// Safe vulkan handles
#[repr(C)]
pub struct Hnd<T: Handle, Dep = <T as Handle>::DefaultDep> {
    pub(crate) data: NonNull<HndInner<T, Dep>>,
    pub(crate) meta: HndMetaObject,
}

impl<T: Handle, Dep> Hnd<T, Dep> {
    pub(crate) fn new(hnd: T, dep: Dep, meta: HndMetaObject) -> Self {
        Self {
            data: HndInner::new(hnd, dep),
            meta,
        }
    }
}

impl<T: Handle, Dep> Hnd<T, Dep> {
    /// Get the raw hnd
    pub fn raw(&self) -> T {
        self.data().raw
    }
    /// Get the dep reference
    pub fn dep(&self) -> &Dep {
        &self.data().dep
    }
}

impl<T: Handle, Dep> Hnd<T, Dep> {
    /// Erase the dep type, return `AnyHnd<T>`
    pub fn upcast(&self) -> &AnyHnd<T> {
        unsafe { core::mem::transmute(self) }
    }
}

impl<T: Handle, Dep> Hnd<T, Dep> {
    fn data(&self) -> &HndInner<T, Dep> {
        unsafe { self.data.as_ref() }
    }

    unsafe fn p_drop(&self) -> &unsafe fn(&Hnd<T, Dep>) {
        unsafe { core::mem::transmute(self.meta.p_drop_hnd) }
    }

    fn add_ref(&self) {
        self.data().count.fetch_add(1, Ordering::Relaxed);
    }
    fn release(&self) {
        if 1 == self.data().count.fetch_sub(1, Ordering::Release) {
            core::sync::atomic::fence(Ordering::Acquire);
            unsafe {
                (self.p_drop())(self);
            }
        }
    }
}

impl<T: Handle, Dep> Clone for Hnd<T, Dep> {
    fn clone(&self) -> Self {
        self.add_ref();
        Self {
            data: self.data,
            meta: self.meta.clone(),
        }
    }
}

impl<T: Handle, Dep> Drop for Hnd<T, Dep> {
    fn drop(&mut self) {
        self.release();
    }
}

unsafe impl<T: Handle, Dep> Send for Hnd<T, Dep> {}
unsafe impl<T: Handle, Dep> Sync for Hnd<T, Dep> {}

#[repr(C)]
pub(crate) struct AnyHndInner<T: Handle> {
    pub raw: T,
    pub count: AtomicUsize,
}

/// Safe vulkan handles and erase the dep type
#[repr(C)]
pub struct AnyHnd<T: Handle> {
    pub(crate) data: NonNull<AnyHndInner<T>>,
    pub(crate) meta: HndMetaObject,
}

impl<T: Handle> AnyHnd<T> {
    /// Get the raw hnd
    pub fn raw(&self) -> T {
        self.data().raw
    }
}

impl<T: Handle> AnyHnd<T> {
    /// Check the dep type is `Dep`
    pub fn is_dep<Dep: 'static>(&self) -> bool {
        unsafe { TypeId::of::<Dep>() == self.meta.dep_type }
    }

    /// Try to downcast to `Hnd<T, Dep>`, return `None` if the dep type is not `Dep`
    pub fn downcast<Dep: 'static>(&self) -> Option<&Hnd<T, Dep>> {
        unsafe {
            if self.is_dep::<Dep>() {
                Some(core::mem::transmute(self))
            } else {
                None
            }
        }
    }
}

impl<T: Handle> AnyHnd<T> {
    fn data(&self) -> &AnyHndInner<T> {
        unsafe { self.data.as_ref() }
    }

    unsafe fn p_drop(&self) -> &unsafe fn(&AnyHnd<T>) {
        unsafe { core::mem::transmute(self.meta.p_drop_hnd) }
    }

    fn add_ref(&self) {
        self.data().count.fetch_add(1, Ordering::Relaxed);
    }
    unsafe fn release(&self) {
        if 1 == self.data().count.fetch_sub(1, Ordering::Release) {
            core::sync::atomic::fence(Ordering::Acquire);
            unsafe {
                (self.p_drop())(self);
            }
        }
    }
}

impl<T: Handle> Clone for AnyHnd<T> {
    fn clone(&self) -> Self {
        self.add_ref();
        Self {
            data: self.data,
            meta: self.meta.clone(),
        }
    }
}

impl<T: Handle> Drop for AnyHnd<T> {
    fn drop(&mut self) {
        unsafe { self.release() };
    }
}

unsafe impl<T: Handle> Send for AnyHnd<T> {}
unsafe impl<T: Handle> Sync for AnyHnd<T> {}

pub(crate) struct ExtInner<T> {
    pub target: T,
    pub meta: HndMetaObject,
}

pub struct Ext<T>(pub(crate) Arc<ExtInner<T>>);

impl<T> Clone for Ext<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

unsafe impl<T> Send for Ext<T> {}
unsafe impl<T> Sync for Ext<T> {}

/// Utils type for ffi return a handle
#[derive(Debug, Clone, Copy)]
pub struct HndRet<'a, T, Dep> {
    raw: T,
    dep: &'a Dep,
}

impl<'a, T, Dep> HndRet<'a, T, Dep> {
    /// Create the `HndRet`
    pub fn new(raw: T, dep: &'a Dep) -> Self {
        Self { raw, dep }
    }
}

impl<'a, T: Copy, Dep> HndRet<'a, T, Dep> {
    /// Get the raw handle
    pub fn raw(&self) -> T {
        self.raw
    }
}

impl<'a, T: Handle + MakeHnd<&'a Dep>, Dep: Clone + 'static> HndRet<'a, T, Dep> {
    /// Make the safe handle
    pub fn hnd(&self) -> T::Output<Dep> {
        unsafe { self.raw.make_hnd(self.dep, || self.dep.clone()) }
    }
}

impl<'a, T: Handle + MakeHnd<&'a Dep>, Dep> HndRet<'a, T, Dep> {
    /// Make the safe handle with custom dep
    pub unsafe fn hnd_with<D: 'static>(&self, dep: impl FnOnce() -> D) -> T::Output<D> {
        unsafe { self.raw.make_hnd(self.dep, dep) }
    }
}

impl<T: Handle, Dep> ObjectType for Hnd<T, Dep> {
    const TYPE: vk::ObjectType = T::TYPE;
}

impl<T: Handle> ObjectType for AnyHnd<T> {
    const TYPE: vk::ObjectType = T::TYPE;
}

/// Marker trait for vulkan handles
pub trait Handle:
    ObjectType + Clone + Copy + Eq + Ord + Hash + core::fmt::Debug + core::fmt::Pointer
{
    /// Default dep type
    type DefaultDep;
    /// The hnd type
    type Hnd<Dep>;
    /// The any hnd type
    type AnyHnd;
}


impl<F: FnMut(&CStr) -> Option<ProcAddr>> ProcAddrLoader for F {
    unsafe fn load(&mut self, name: &CStr) -> Option<ProcAddr> {
        self(name)
    }
}
