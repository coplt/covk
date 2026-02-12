// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_QCOM_tile_shading` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::qcom::tile_shading::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::qcom::tile_shading::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// void vkCmdBeginPerTileExecutionQCOM(VkCommandBuffer commandBuffer, VkPerTileBeginInfoQCOM const* pPerTileBeginInfo)
    /// ```
    pub unsafe fn cmd_begin_per_tile_execution(
        &self,
        command_buffer: vk::CommandBuffer,
        per_tile_begin_info: &PerTileBeginInfoQCOM,
    ) -> () {
        unsafe {
            self.0.CmdBeginPerTileExecutionQCOM(
                command_buffer.abi(), 
                per_tile_begin_info.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdDispatchTileQCOM(VkCommandBuffer commandBuffer, VkDispatchTileInfoQCOM const* pDispatchTileInfo)
    /// ```
    pub unsafe fn cmd_dispatch_tile(
        &self,
        command_buffer: vk::CommandBuffer,
        dispatch_tile_info: &DispatchTileInfoQCOM,
    ) -> () {
        unsafe {
            self.0.CmdDispatchTileQCOM(
                command_buffer.abi(), 
                dispatch_tile_info.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdEndPerTileExecutionQCOM(VkCommandBuffer commandBuffer, VkPerTileEndInfoQCOM const* pPerTileEndInfo)
    /// ```
    pub unsafe fn cmd_end_per_tile_execution(
        &self,
        command_buffer: vk::CommandBuffer,
        per_tile_end_info: &PerTileEndInfoQCOM,
    ) -> () {
        unsafe {
            self.0.CmdEndPerTileExecutionQCOM(
                command_buffer.abi(), 
                per_tile_end_info.abi(), 
            );
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::qcom::tile_shading {
    type Commands = Device;
}

/// Device object
pub trait QcomTileShadingDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

}

impl crate::HndScope<vk::Device> for vk::extensions::qcom::tile_shading {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::qcom::tile_shading> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::qcom::tile_shading {
        type Output = crate::hnd::Device<vk::extensions::qcom::tile_shading>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::qcom::tile_shading>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::qcom::tile_shading> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::qcom::tile_shading> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::qcom::tile_shading> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::qcom::tile_shading> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::QcomTileShadingDevice for crate::hnd::Device<vk::extensions::qcom::tile_shading> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::qcom::tile_shading, vk::Device> for crate::hnd::Device<vk::extensions::qcom::tile_shading> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

/// CommandBuffer object
pub trait QcomTileShadingCommandBuffer {
    fn raw(&self) -> vk::CommandBuffer;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkCmdBeginPerTileExecutionQCOM(VkCommandBuffer commandBuffer, VkPerTileBeginInfoQCOM const* pPerTileBeginInfo)
    /// ```
    unsafe fn begin_per_tile_execution(
        &self,
        per_tile_begin_info: &PerTileBeginInfoQCOM,
    ) -> () {
        unsafe {
            self.commands().cmd_begin_per_tile_execution(
                self.raw(),
                per_tile_begin_info,
            )
        }
    }
    /// ```c
    /// void vkCmdDispatchTileQCOM(VkCommandBuffer commandBuffer, VkDispatchTileInfoQCOM const* pDispatchTileInfo)
    /// ```
    unsafe fn dispatch_tile(
        &self,
        dispatch_tile_info: &DispatchTileInfoQCOM,
    ) -> () {
        unsafe {
            self.commands().cmd_dispatch_tile(
                self.raw(),
                dispatch_tile_info,
            )
        }
    }
    /// ```c
    /// void vkCmdEndPerTileExecutionQCOM(VkCommandBuffer commandBuffer, VkPerTileEndInfoQCOM const* pPerTileEndInfo)
    /// ```
    unsafe fn end_per_tile_execution(
        &self,
        per_tile_end_info: &PerTileEndInfoQCOM,
    ) -> () {
        unsafe {
            self.commands().cmd_end_per_tile_execution(
                self.raw(),
                per_tile_end_info,
            )
        }
    }
}
