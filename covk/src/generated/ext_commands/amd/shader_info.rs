// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_AMD_shader_info` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::amd::shader_info::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::amd::shader_info::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// VkResult vkGetShaderInfoAMD(VkDevice device, VkPipeline pipeline, VkShaderStageFlagBits shaderStage, VkShaderInfoTypeAMD infoType, size_t* pInfoSize, void* pInfo)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    pub unsafe fn get_shader_info(
        &self,
        device: vk::Device,
        pipeline: vk::Pipeline,
        shader_stage: ShaderStageFlags,
        info_type: ShaderInfoTypeAMD,
        info_size: *mut size_t,
        info: *mut void,
    ) -> crate::Result<Result> {
        unsafe {
            let _r = self.0.GetShaderInfoAMD(
                device.abi(), 
                pipeline.abi(), 
                shader_stage.abi(), 
                info_type.abi(), 
                info_size.abi(), 
                info.abi(), 
            ).vk();
            _r.result(|| Some(_r))
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::amd::shader_info {
    type Commands = Device;
}

/// Device object
pub trait AmdShaderInfoDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkGetShaderInfoAMD(VkDevice device, VkPipeline pipeline, VkShaderStageFlagBits shaderStage, VkShaderInfoTypeAMD infoType, size_t* pInfoSize, void* pInfo)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    unsafe fn get_shader_info(
        &self,
        pipeline: vk::Pipeline,
        shader_stage: ShaderStageFlags,
        info_type: ShaderInfoTypeAMD,
        info_size: *mut size_t,
        info: *mut void,
    ) -> crate::Result<Result> {
        unsafe {
            self.commands().get_shader_info(
                self.raw(),
                pipeline,
                shader_stage,
                info_type,
                info_size,
                info,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::amd::shader_info {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::amd::shader_info> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::amd::shader_info {
        type Output = crate::hnd::Device<vk::extensions::amd::shader_info>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::amd::shader_info>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::amd::shader_info> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::amd::shader_info> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::amd::shader_info> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::amd::shader_info> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::AmdShaderInfoDevice for crate::hnd::Device<vk::extensions::amd::shader_info> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::amd::shader_info, vk::Device> for crate::hnd::Device<vk::extensions::amd::shader_info> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}
