// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_EXT_validation_cache` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::ext::validation_cache::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::ext::validation_cache::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// VkResult vkCreateValidationCacheEXT(VkDevice device, VkValidationCacheCreateInfoEXT const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkValidationCacheEXT* pValidationCache)
    /// ```
    pub unsafe fn create_validation_cache(
        &self,
        device: vk::Device,
        create_info: &ValidationCacheCreateInfoEXT,
    ) -> crate::Result<vk::ValidationCacheEXT> {
        unsafe {
            let mut _v: Option<vk::ValidationCacheEXT> = Default::default();
            let _r = self.0.CreateValidationCacheEXT(
                device.abi(), 
                create_info.abi(), 
                Default::default(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| _v)
        }
    }
    /// ```c
    /// void vkDestroyValidationCacheEXT(VkDevice device, VkValidationCacheEXT validationCache, VkAllocationCallbacks const* pAllocator)
    /// ```
    pub unsafe fn destroy_validation_cache(
        &self,
        device: vk::Device,
        validation_cache: Option<vk::ValidationCacheEXT>,
    ) -> () {
        unsafe {
            self.0.DestroyValidationCacheEXT(
                device.abi(), 
                validation_cache.abi(), 
                Default::default(), 
            );
        }
    }
    /// ```c
    /// VkResult vkGetValidationCacheDataEXT(VkDevice device, VkValidationCacheEXT validationCache, size_t* pDataSize, void* pData)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    pub unsafe fn get_validation_cache_data(
        &self,
        device: vk::Device,
        validation_cache: vk::ValidationCacheEXT,
        data_size: *mut size_t,
        data: *mut void,
    ) -> crate::Result<Result> {
        unsafe {
            let _r = self.0.GetValidationCacheDataEXT(
                device.abi(), 
                validation_cache.abi(), 
                data_size.abi(), 
                data.abi(), 
            ).vk();
            _r.result(|| Some(_r))
        }
    }
    /// ```c
    /// VkResult vkMergeValidationCachesEXT(VkDevice device, VkValidationCacheEXT dstCache, uint32_t srcCacheCount, VkValidationCacheEXT const* pSrcCaches)
    /// ```
    pub unsafe fn merge_validation_caches(
        &self,
        device: vk::Device,
        dst_cache: vk::ValidationCacheEXT,
        src_caches: &[vk::ValidationCacheEXT],
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.MergeValidationCachesEXT(
                device.abi(), 
                dst_cache.abi(), 
                src_caches.len() as _, 
                src_caches.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::ext::validation_cache {
    type Commands = Device;
}

/// Device object
pub trait ExtValidationCacheDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkCreateValidationCacheEXT(VkDevice device, VkValidationCacheCreateInfoEXT const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkValidationCacheEXT* pValidationCache)
    /// ```
    unsafe fn create_validation_cache(
        &self,
        create_info: &ValidationCacheCreateInfoEXT,
    ) -> crate::Result<vk::ValidationCacheEXT> {
        unsafe {
            self.commands().create_validation_cache(
                self.raw(),
                create_info,
            )
        }
    }
    /// ```c
    /// void vkDestroyValidationCacheEXT(VkDevice device, VkValidationCacheEXT validationCache, VkAllocationCallbacks const* pAllocator)
    /// ```
    unsafe fn destroy_validation_cache(
        &self,
        validation_cache: Option<vk::ValidationCacheEXT>,
    ) -> () {
        unsafe {
            self.commands().destroy_validation_cache(
                self.raw(),
                validation_cache,
            )
        }
    }
    /// ```c
    /// VkResult vkGetValidationCacheDataEXT(VkDevice device, VkValidationCacheEXT validationCache, size_t* pDataSize, void* pData)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    unsafe fn get_validation_cache_data(
        &self,
        validation_cache: vk::ValidationCacheEXT,
        data_size: *mut size_t,
        data: *mut void,
    ) -> crate::Result<Result> {
        unsafe {
            self.commands().get_validation_cache_data(
                self.raw(),
                validation_cache,
                data_size,
                data,
            )
        }
    }
    /// ```c
    /// VkResult vkMergeValidationCachesEXT(VkDevice device, VkValidationCacheEXT dstCache, uint32_t srcCacheCount, VkValidationCacheEXT const* pSrcCaches)
    /// ```
    unsafe fn merge_validation_caches(
        &self,
        dst_cache: vk::ValidationCacheEXT,
        src_caches: &[vk::ValidationCacheEXT],
    ) -> crate::Result<()> {
        unsafe {
            self.commands().merge_validation_caches(
                self.raw(),
                dst_cache,
                src_caches,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::ext::validation_cache {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::ext::validation_cache> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::ext::validation_cache {
        type Output = crate::hnd::Device<vk::extensions::ext::validation_cache>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::ext::validation_cache>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::ext::validation_cache> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::ext::validation_cache> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::ext::validation_cache> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::ext::validation_cache> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::ExtValidationCacheDevice for crate::hnd::Device<vk::extensions::ext::validation_cache> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::ext::validation_cache, vk::Device> for crate::hnd::Device<vk::extensions::ext::validation_cache> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

impl<O: crate::HndCtx<vk::extensions::ext::validation_cache, vk::Device>> crate::Owner<vk::ValidationCacheEXT, vk::extensions::ext::validation_cache> for O {
    fn drop(&mut self, raw: vk::ValidationCacheEXT) {
        unsafe {
            self.commands().destroy_validation_cache(self.raw(), Some(raw))
        }
    }
}
impl<O> crate::HndCtx<vk::extensions::ext::validation_cache, vk::Device> for ::alloc::sync::Arc<crate::Unique<vk::ValidationCacheEXT, O, vk::extensions::ext::validation_cache>>
    where O: crate::HndCtx<vk::extensions::ext::validation_cache, vk::Device> + Send + Sync + 'static,
{
    type Ctx = Self;
    fn ctx(&self) -> Self::Ctx { self.clone() }
    fn raw(&self) -> vk::Device { self.owner.raw() }
    fn commands(&self) -> &::alloc::sync::Arc<Device> { self.owner.commands() }
}

impl crate::HndScope<vk::ValidationCacheEXT> for vk::extensions::ext::validation_cache {
    type Impl = _hs_ValidationCacheEXT::ValidationCacheEXT;
}


mod _hs_ValidationCacheEXT {
    use super::*;

    #[repr(transparent)]
    #[derive(Debug)]
    pub struct ValidationCacheEXT(pub(crate) crate::handle::Hnd<vk::ValidationCacheEXT, ::alloc::sync::Arc<super::Device>>);

    impl Clone for ValidationCacheEXT {
        fn clone(&self) -> Self { Self(self.0.clone()) }
    }

    #[derive(Debug, Clone, Copy)]
    struct Inst;
    impl<Ctx, Dep> crate::handle::HndInst<vk::ValidationCacheEXT, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)> for Inst
        where Ctx: crate::HndCtx<vk::extensions::ext::validation_cache, vk::Device>,
    {
        fn drop(this: &mut crate::handle::HndData<vk::ValidationCacheEXT, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)>) {
            unsafe {
                this.scope.destroy_validation_cache(this.dep.0.raw(), Some(this.raw))
            }
        }
    }

    impl crate::hnd::ValidationCacheEXT<vk::extensions::ext::validation_cache>
    {
        pub unsafe fn new(ctx: &impl crate::HndCtx<vk::extensions::ext::validation_cache, vk::Device>, raw: vk::ValidationCacheEXT) -> Self {
            unsafe { Self::new_with(ctx, raw, || ()) }
        }
        pub unsafe fn new_with<Dep: Send + Sync + 'static>(ctx: &impl crate::HndCtx<vk::extensions::ext::validation_cache, vk::Device>, raw: vk::ValidationCacheEXT, dep: impl FnOnce() -> Dep) -> Self {
            Self(ValidationCacheEXT(crate::handle::Hnd::new(
                ctx.commands().clone(),
                raw,
                (ctx.ctx(), dep()),
                Inst,
            )))
        }
    }

    impl crate::hnd::ValidationCacheEXT<vk::extensions::ext::validation_cache> {
        pub fn raw(&self) -> vk::ValidationCacheEXT { self.0.0.raw }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0.scope }
    }

    impl ::core::fmt::Debug for crate::hnd::ValidationCacheEXT<vk::extensions::ext::validation_cache> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("ValidationCacheEXT({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::ValidationCacheEXT<vk::extensions::ext::validation_cache> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl<Ctx> crate::MakeHnd<Ctx, vk::extensions::ext::validation_cache> for vk::ValidationCacheEXT
        where Ctx: crate::HndCtx<vk::extensions::ext::validation_cache, vk::Device>,
    {
        type Output = crate::hnd::ValidationCacheEXT<vk::extensions::ext::validation_cache>;
        unsafe fn hnd_with<Dep: Send + Sync + 'static>(self, ctx: &Ctx, dep: impl FnOnce() -> Dep) -> Self::Output {
            unsafe { crate::hnd::ValidationCacheEXT::<vk::extensions::ext::validation_cache>::new_with(ctx, self, dep) }
        }
    }
}
