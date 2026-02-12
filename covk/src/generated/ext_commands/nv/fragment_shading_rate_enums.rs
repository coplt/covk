// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_NV_fragment_shading_rate_enums` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::nv::fragment_shading_rate_enums::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::nv::fragment_shading_rate_enums::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// void vkCmdSetFragmentShadingRateEnumNV(VkCommandBuffer commandBuffer, VkFragmentShadingRateNV shadingRate, const VkFragmentShadingRateCombinerOpKHR combinerOps[2])
    /// ```
    pub unsafe fn cmd_set_fragment_shading_rate_enum(
        &self,
        command_buffer: vk::CommandBuffer,
        shading_rate: FragmentShadingRateNV,
        combiner_ops: [FragmentShadingRateCombinerOpKHR; 2],
    ) -> () {
        unsafe {
            self.0.CmdSetFragmentShadingRateEnumNV(
                command_buffer.abi(), 
                shading_rate.abi(), 
                combiner_ops.as_ref().abi(), 
            );
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::nv::fragment_shading_rate_enums {
    type Commands = Device;
}

/// Device object
pub trait NvFragmentShadingRateEnumsDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

}

impl crate::HndScope<vk::Device> for vk::extensions::nv::fragment_shading_rate_enums {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::nv::fragment_shading_rate_enums> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::nv::fragment_shading_rate_enums {
        type Output = crate::hnd::Device<vk::extensions::nv::fragment_shading_rate_enums>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::nv::fragment_shading_rate_enums>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::nv::fragment_shading_rate_enums> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::nv::fragment_shading_rate_enums> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::nv::fragment_shading_rate_enums> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::nv::fragment_shading_rate_enums> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::NvFragmentShadingRateEnumsDevice for crate::hnd::Device<vk::extensions::nv::fragment_shading_rate_enums> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::nv::fragment_shading_rate_enums, vk::Device> for crate::hnd::Device<vk::extensions::nv::fragment_shading_rate_enums> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

/// CommandBuffer object
pub trait NvFragmentShadingRateEnumsCommandBuffer {
    fn raw(&self) -> vk::CommandBuffer;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkCmdSetFragmentShadingRateEnumNV(VkCommandBuffer commandBuffer, VkFragmentShadingRateNV shadingRate, const VkFragmentShadingRateCombinerOpKHR combinerOps[2])
    /// ```
    unsafe fn set_fragment_shading_rate_enum(
        &self,
        shading_rate: FragmentShadingRateNV,
        combiner_ops: [FragmentShadingRateCombinerOpKHR; 2],
    ) -> () {
        unsafe {
            self.commands().cmd_set_fragment_shading_rate_enum(
                self.raw(),
                shading_rate,
                combiner_ops,
            )
        }
    }
}
