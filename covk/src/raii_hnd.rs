use alloc::sync::Arc;

/// Mark the command scope
pub trait CommandScope<Raw: Copy> {
    /// the commands type
    type Commands;
}

/// Mark the hnd ctx
pub trait HndCtx<Scope: CommandScope<Raw>, Raw: Copy> {
    /// The ctx type
    type Ctx: Send + Sync + 'static + HndCtx<Scope, Raw>;
    /// Get ctx
    fn ctx(&self) -> Self::Ctx;
    /// Get raw handle
    fn raw(&self) -> Raw;
    /// Get commands
    fn commands(&self) -> &Arc<Scope::Commands>;
}

/// Make hnd
pub trait MakeHnd<Ctx, S>: Sized {
    type Output;
    /// Make the raii hnd
    unsafe fn hnd(self, ctx: &Ctx) -> Self::Output {
        unsafe { self.hnd_with(ctx, || ()) }
    }
    /// Make the raii hnd with dep
    unsafe fn hnd_with<Dep: Send + Sync + 'static>(
        self,
        ctx: &Ctx,
        dep: impl FnOnce() -> Dep,
    ) -> Self::Output;
}

/// Mark the extension
pub trait Extension<Target>: Sized {
    /// Make ext output type
    type Output;
    /// Make ext
    unsafe fn make(target: &Target) -> Self::Output;
}

/// Make ext
pub trait MakeExt: Sized {
    /// Make ext
    unsafe fn ext<Ext: Extension<Self>>(&self) -> Ext::Output {
        unsafe { Ext::make(self) }
    }
}

mod _hnd_scope {
    use crate::RawHandle;

    pub trait HndScope<Target: RawHandle> {
        type Impl;
    }
}

pub(crate) use _hnd_scope::*;
