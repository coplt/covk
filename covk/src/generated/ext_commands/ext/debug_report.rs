// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_EXT_debug_report` InstanceCommands
#[derive(Debug, Clone, Copy)]
pub struct Instance(pub sys::ext::debug_report::InstanceCommands);

impl Instance {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::ext::debug_report::InstanceCommands::load(get) })
    }
}

impl Instance {
    /// ```c
    /// VkResult vkCreateDebugReportCallbackEXT(VkInstance instance, VkDebugReportCallbackCreateInfoEXT const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkDebugReportCallbackEXT* pCallback)
    /// ```
    pub unsafe fn create_debug_report_callback(
        &self,
        instance: vk::Instance,
        create_info: &DebugReportCallbackCreateInfoEXT,
    ) -> crate::Result<vk::DebugReportCallbackEXT> {
        unsafe {
            let mut _v: Option<vk::DebugReportCallbackEXT> = Default::default();
            let _r = self.0.CreateDebugReportCallbackEXT(
                instance.abi(), 
                create_info.abi(), 
                Default::default(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| _v)
        }
    }
    /// ```c
    /// void vkDebugReportMessageEXT(VkInstance instance, VkDebugReportFlagsEXT flags, VkDebugReportObjectTypeEXT objectType, uint64_t object, size_t location, int32_t messageCode, char const* pLayerPrefix, char const* pMessage)
    /// ```
    pub unsafe fn debug_report_message(
        &self,
        instance: vk::Instance,
        flags: DebugReportFlagsEXT,
        object_type: DebugReportObjectTypeEXT,
        object: uint64_t,
        location: size_t,
        message_code: int32_t,
        layer_prefix: &::core::ffi::CStr,
        message: &::core::ffi::CStr,
    ) -> () {
        unsafe {
            self.0.DebugReportMessageEXT(
                instance.abi(), 
                flags.abi(), 
                object_type.abi(), 
                object.abi(), 
                location.abi(), 
                message_code.abi(), 
                layer_prefix.abi(), 
                message.abi(), 
            );
        }
    }
    /// ```c
    /// void vkDestroyDebugReportCallbackEXT(VkInstance instance, VkDebugReportCallbackEXT callback, VkAllocationCallbacks const* pAllocator)
    /// ```
    pub unsafe fn destroy_debug_report_callback(
        &self,
        instance: vk::Instance,
        callback: Option<vk::DebugReportCallbackEXT>,
    ) -> () {
        unsafe {
            self.0.DestroyDebugReportCallbackEXT(
                instance.abi(), 
                callback.abi(), 
                Default::default(), 
            );
        }
    }
}

impl crate::CommandScope<vk::Instance> for vk::extensions::ext::debug_report {
    type Commands = Instance;
}

/// Instance object
pub trait ExtDebugReportInstance {
    fn raw(&self) -> vk::Instance;
    fn commands(&self) -> &Instance;

    /// ```c
    /// VkResult vkCreateDebugReportCallbackEXT(VkInstance instance, VkDebugReportCallbackCreateInfoEXT const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkDebugReportCallbackEXT* pCallback)
    /// ```
    unsafe fn create_debug_report_callback(
        &self,
        create_info: &DebugReportCallbackCreateInfoEXT,
    ) -> crate::Result<vk::DebugReportCallbackEXT> {
        unsafe {
            self.commands().create_debug_report_callback(
                self.raw(),
                create_info,
            )
        }
    }
    /// ```c
    /// void vkDebugReportMessageEXT(VkInstance instance, VkDebugReportFlagsEXT flags, VkDebugReportObjectTypeEXT objectType, uint64_t object, size_t location, int32_t messageCode, char const* pLayerPrefix, char const* pMessage)
    /// ```
    unsafe fn debug_report_message(
        &self,
        flags: DebugReportFlagsEXT,
        object_type: DebugReportObjectTypeEXT,
        object: uint64_t,
        location: size_t,
        message_code: int32_t,
        layer_prefix: &::core::ffi::CStr,
        message: &::core::ffi::CStr,
    ) -> () {
        unsafe {
            self.commands().debug_report_message(
                self.raw(),
                flags,
                object_type,
                object,
                location,
                message_code,
                layer_prefix,
                message,
            )
        }
    }
    /// ```c
    /// void vkDestroyDebugReportCallbackEXT(VkInstance instance, VkDebugReportCallbackEXT callback, VkAllocationCallbacks const* pAllocator)
    /// ```
    unsafe fn destroy_debug_report_callback(
        &self,
        callback: Option<vk::DebugReportCallbackEXT>,
    ) -> () {
        unsafe {
            self.commands().destroy_debug_report_callback(
                self.raw(),
                callback,
            )
        }
    }
}

impl crate::HndScope<vk::Instance> for vk::extensions::ext::debug_report {
    type Impl = _hs_Instance::Instance;
}

mod _hs_Instance {
    use super::*;
    #[derive(Debug)]
    pub struct Instance(pub(crate) ::alloc::sync::Arc<super::Instance>, pub(crate) crate::hnd::Instance<vk::core>);

    impl Clone for Instance {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Instance<vk::extensions::ext::debug_report> {
        pub unsafe fn new(base: &crate::hnd::Instance<vk::core>) -> Self {
            unsafe {
                Self(Instance(
                   ::alloc::sync::Arc::new(super::Instance::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Instance<vk::core>> for vk::extensions::ext::debug_report {
        type Output = crate::hnd::Instance<vk::extensions::ext::debug_report>;
        unsafe fn make(target: &crate::hnd::Instance<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Instance::<vk::extensions::ext::debug_report>::new(target) }
        }
    }

    impl crate::hnd::Instance<vk::extensions::ext::debug_report> {
        pub fn raw(&self) -> vk::Instance { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Instance<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Instance<vk::extensions::ext::debug_report> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Instance({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Instance<vk::extensions::ext::debug_report> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Instance<vk::extensions::ext::debug_report> {
        type Target = super::Instance;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::ExtDebugReportInstance for crate::hnd::Instance<vk::extensions::ext::debug_report> {
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &super::Instance { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::ext::debug_report, vk::Instance> for crate::hnd::Instance<vk::extensions::ext::debug_report> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { self.commands() }
    }
}

impl<O: crate::HndCtx<vk::extensions::ext::debug_report, vk::Instance>> crate::Owner<vk::DebugReportCallbackEXT, vk::extensions::ext::debug_report> for O {
    fn drop(&mut self, raw: vk::DebugReportCallbackEXT) {
        unsafe {
            self.commands().destroy_debug_report_callback(self.raw(), Some(raw))
        }
    }
}
impl<O> crate::HndCtx<vk::extensions::ext::debug_report, vk::Instance> for ::alloc::sync::Arc<crate::Unique<vk::DebugReportCallbackEXT, O, vk::extensions::ext::debug_report>>
    where O: crate::HndCtx<vk::extensions::ext::debug_report, vk::Instance> + Send + Sync + 'static,
{
    type Ctx = Self;
    fn ctx(&self) -> Self::Ctx { self.clone() }
    fn raw(&self) -> vk::Instance { self.owner.raw() }
    fn commands(&self) -> &::alloc::sync::Arc<Instance> { self.owner.commands() }
}

impl crate::HndScope<vk::DebugReportCallbackEXT> for vk::extensions::ext::debug_report {
    type Impl = _hs_DebugReportCallbackEXT::DebugReportCallbackEXT;
}


mod _hs_DebugReportCallbackEXT {
    use super::*;

    #[repr(transparent)]
    #[derive(Debug)]
    pub struct DebugReportCallbackEXT(pub(crate) crate::handle::Hnd<vk::DebugReportCallbackEXT, ::alloc::sync::Arc<super::Instance>>);

    impl Clone for DebugReportCallbackEXT {
        fn clone(&self) -> Self { Self(self.0.clone()) }
    }

    #[derive(Debug, Clone, Copy)]
    struct Inst;
    impl<Ctx, Dep> crate::handle::HndInst<vk::DebugReportCallbackEXT, ::alloc::sync::Arc<super::Instance>, (Ctx, Dep)> for Inst
        where Ctx: crate::HndCtx<vk::extensions::ext::debug_report, vk::Instance>,
    {
        fn drop(this: &mut crate::handle::HndData<vk::DebugReportCallbackEXT, ::alloc::sync::Arc<super::Instance>, (Ctx, Dep)>) {
            unsafe {
                this.scope.destroy_debug_report_callback(this.dep.0.raw(), Some(this.raw))
            }
        }
    }

    impl crate::hnd::DebugReportCallbackEXT<vk::extensions::ext::debug_report>
    {
        pub unsafe fn new(ctx: &impl crate::HndCtx<vk::extensions::ext::debug_report, vk::Instance>, raw: vk::DebugReportCallbackEXT) -> Self {
            unsafe { Self::new_with(ctx, raw, || ()) }
        }
        pub unsafe fn new_with<Dep: Send + Sync + 'static>(ctx: &impl crate::HndCtx<vk::extensions::ext::debug_report, vk::Instance>, raw: vk::DebugReportCallbackEXT, dep: impl FnOnce() -> Dep) -> Self {
            Self(DebugReportCallbackEXT(crate::handle::Hnd::new(
                ctx.commands().clone(),
                raw,
                (ctx.ctx(), dep()),
                Inst,
            )))
        }
    }

    impl crate::hnd::DebugReportCallbackEXT<vk::extensions::ext::debug_report> {
        pub fn raw(&self) -> vk::DebugReportCallbackEXT { self.0.0.raw }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { &self.0.0.scope }
    }

    impl ::core::fmt::Debug for crate::hnd::DebugReportCallbackEXT<vk::extensions::ext::debug_report> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("DebugReportCallbackEXT({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::DebugReportCallbackEXT<vk::extensions::ext::debug_report> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl<Ctx> crate::MakeHnd<Ctx, vk::extensions::ext::debug_report> for vk::DebugReportCallbackEXT
        where Ctx: crate::HndCtx<vk::extensions::ext::debug_report, vk::Instance>,
    {
        type Output = crate::hnd::DebugReportCallbackEXT<vk::extensions::ext::debug_report>;
        unsafe fn hnd_with<Dep: Send + Sync + 'static>(self, ctx: &Ctx, dep: impl FnOnce() -> Dep) -> Self::Output {
            unsafe { crate::hnd::DebugReportCallbackEXT::<vk::extensions::ext::debug_report>::new_with(ctx, self, dep) }
        }
    }
}
