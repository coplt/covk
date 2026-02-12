// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_KHR_sampler_ycbcr_conversion` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::khr::sampler_ycbcr_conversion::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::khr::sampler_ycbcr_conversion::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// VkResult vkCreateSamplerYcbcrConversion(VkDevice device, VkSamplerYcbcrConversionCreateInfo const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkSamplerYcbcrConversion* pYcbcrConversion)
    /// ```
    pub unsafe fn create_sampler_ycbcr_conversion(
        &self,
        device: vk::Device,
        create_info: &SamplerYcbcrConversionCreateInfo,
    ) -> crate::Result<vk::SamplerYcbcrConversion> {
        unsafe {
            let mut _v: Option<vk::SamplerYcbcrConversion> = Default::default();
            let _r = self.0.CreateSamplerYcbcrConversionKHR(
                device.abi(), 
                create_info.abi(), 
                Default::default(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| _v)
        }
    }
    /// ```c
    /// void vkDestroySamplerYcbcrConversion(VkDevice device, VkSamplerYcbcrConversion ycbcrConversion, VkAllocationCallbacks const* pAllocator)
    /// ```
    pub unsafe fn destroy_sampler_ycbcr_conversion(
        &self,
        device: vk::Device,
        ycbcr_conversion: Option<vk::SamplerYcbcrConversion>,
    ) -> () {
        unsafe {
            self.0.DestroySamplerYcbcrConversionKHR(
                device.abi(), 
                ycbcr_conversion.abi(), 
                Default::default(), 
            );
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::khr::sampler_ycbcr_conversion {
    type Commands = Device;
}

/// Device object
pub trait KhrSamplerYcbcrConversionDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkCreateSamplerYcbcrConversion(VkDevice device, VkSamplerYcbcrConversionCreateInfo const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkSamplerYcbcrConversion* pYcbcrConversion)
    /// ```
    unsafe fn create_sampler_ycbcr_conversion(
        &self,
        create_info: &SamplerYcbcrConversionCreateInfo,
    ) -> crate::Result<vk::SamplerYcbcrConversion> {
        unsafe {
            self.commands().create_sampler_ycbcr_conversion(
                self.raw(),
                create_info,
            )
        }
    }
    /// ```c
    /// void vkDestroySamplerYcbcrConversion(VkDevice device, VkSamplerYcbcrConversion ycbcrConversion, VkAllocationCallbacks const* pAllocator)
    /// ```
    unsafe fn destroy_sampler_ycbcr_conversion(
        &self,
        ycbcr_conversion: Option<vk::SamplerYcbcrConversion>,
    ) -> () {
        unsafe {
            self.commands().destroy_sampler_ycbcr_conversion(
                self.raw(),
                ycbcr_conversion,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::khr::sampler_ycbcr_conversion {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::khr::sampler_ycbcr_conversion> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::khr::sampler_ycbcr_conversion {
        type Output = crate::hnd::Device<vk::extensions::khr::sampler_ycbcr_conversion>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::khr::sampler_ycbcr_conversion>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::khr::sampler_ycbcr_conversion> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::khr::sampler_ycbcr_conversion> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::khr::sampler_ycbcr_conversion> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::khr::sampler_ycbcr_conversion> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::KhrSamplerYcbcrConversionDevice for crate::hnd::Device<vk::extensions::khr::sampler_ycbcr_conversion> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::khr::sampler_ycbcr_conversion, vk::Device> for crate::hnd::Device<vk::extensions::khr::sampler_ycbcr_conversion> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

impl<O: crate::HndCtx<vk::extensions::khr::sampler_ycbcr_conversion, vk::Device>> crate::Owner<vk::SamplerYcbcrConversion, vk::extensions::khr::sampler_ycbcr_conversion> for O {
    fn drop(&mut self, raw: vk::SamplerYcbcrConversion) {
        unsafe {
            self.commands().destroy_sampler_ycbcr_conversion(self.raw(), Some(raw))
        }
    }
}
impl<O> crate::HndCtx<vk::extensions::khr::sampler_ycbcr_conversion, vk::Device> for ::alloc::sync::Arc<crate::Unique<vk::SamplerYcbcrConversion, O, vk::extensions::khr::sampler_ycbcr_conversion>>
    where O: crate::HndCtx<vk::extensions::khr::sampler_ycbcr_conversion, vk::Device> + Send + Sync + 'static,
{
    type Ctx = Self;
    fn ctx(&self) -> Self::Ctx { self.clone() }
    fn raw(&self) -> vk::Device { self.owner.raw() }
    fn commands(&self) -> &::alloc::sync::Arc<Device> { self.owner.commands() }
}

impl crate::HndScope<vk::SamplerYcbcrConversion> for vk::extensions::khr::sampler_ycbcr_conversion {
    type Impl = _hs_SamplerYcbcrConversion::SamplerYcbcrConversion;
}


mod _hs_SamplerYcbcrConversion {
    use super::*;

    #[repr(transparent)]
    #[derive(Debug)]
    pub struct SamplerYcbcrConversion(pub(crate) crate::handle::Hnd<vk::SamplerYcbcrConversion, ::alloc::sync::Arc<super::Device>>);

    impl Clone for SamplerYcbcrConversion {
        fn clone(&self) -> Self { Self(self.0.clone()) }
    }

    #[derive(Debug, Clone, Copy)]
    struct Inst;
    impl<Ctx, Dep> crate::handle::HndInst<vk::SamplerYcbcrConversion, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)> for Inst
        where Ctx: crate::HndCtx<vk::extensions::khr::sampler_ycbcr_conversion, vk::Device>,
    {
        fn drop(this: &mut crate::handle::HndData<vk::SamplerYcbcrConversion, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)>) {
            unsafe {
                this.scope.destroy_sampler_ycbcr_conversion(this.dep.0.raw(), Some(this.raw))
            }
        }
    }

    impl crate::hnd::SamplerYcbcrConversion<vk::extensions::khr::sampler_ycbcr_conversion>
    {
        pub unsafe fn new(ctx: &impl crate::HndCtx<vk::extensions::khr::sampler_ycbcr_conversion, vk::Device>, raw: vk::SamplerYcbcrConversion) -> Self {
            unsafe { Self::new_with(ctx, raw, || ()) }
        }
        pub unsafe fn new_with<Dep: Send + Sync + 'static>(ctx: &impl crate::HndCtx<vk::extensions::khr::sampler_ycbcr_conversion, vk::Device>, raw: vk::SamplerYcbcrConversion, dep: impl FnOnce() -> Dep) -> Self {
            Self(SamplerYcbcrConversion(crate::handle::Hnd::new(
                ctx.commands().clone(),
                raw,
                (ctx.ctx(), dep()),
                Inst,
            )))
        }
    }

    impl crate::hnd::SamplerYcbcrConversion<vk::extensions::khr::sampler_ycbcr_conversion> {
        pub fn raw(&self) -> vk::SamplerYcbcrConversion { self.0.0.raw }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0.scope }
    }

    impl ::core::fmt::Debug for crate::hnd::SamplerYcbcrConversion<vk::extensions::khr::sampler_ycbcr_conversion> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("SamplerYcbcrConversion({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::SamplerYcbcrConversion<vk::extensions::khr::sampler_ycbcr_conversion> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl<Ctx> crate::MakeHnd<Ctx, vk::extensions::khr::sampler_ycbcr_conversion> for vk::SamplerYcbcrConversion
        where Ctx: crate::HndCtx<vk::extensions::khr::sampler_ycbcr_conversion, vk::Device>,
    {
        type Output = crate::hnd::SamplerYcbcrConversion<vk::extensions::khr::sampler_ycbcr_conversion>;
        unsafe fn hnd_with<Dep: Send + Sync + 'static>(self, ctx: &Ctx, dep: impl FnOnce() -> Dep) -> Self::Output {
            unsafe { crate::hnd::SamplerYcbcrConversion::<vk::extensions::khr::sampler_ycbcr_conversion>::new_with(ctx, self, dep) }
        }
    }
}
