// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_EXT_extended_dynamic_state2` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::ext::extended_dynamic_state2::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::ext::extended_dynamic_state2::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// void vkCmdSetDepthBiasEnable(VkCommandBuffer commandBuffer, VkBool32 depthBiasEnable)
    /// ```
    pub unsafe fn cmd_set_depth_bias_enable(
        &self,
        command_buffer: vk::CommandBuffer,
        depth_bias_enable: bool,
    ) -> () {
        unsafe {
            self.0.CmdSetDepthBiasEnableEXT(
                command_buffer.abi(), 
                depth_bias_enable.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetLogicOpEXT(VkCommandBuffer commandBuffer, VkLogicOp logicOp)
    /// ```
    pub unsafe fn cmd_set_logic_op(
        &self,
        command_buffer: vk::CommandBuffer,
        logic_op: LogicOp,
    ) -> () {
        unsafe {
            self.0.CmdSetLogicOpEXT(
                command_buffer.abi(), 
                logic_op.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetPatchControlPointsEXT(VkCommandBuffer commandBuffer, uint32_t patchControlPoints)
    /// ```
    pub unsafe fn cmd_set_patch_control_points(
        &self,
        command_buffer: vk::CommandBuffer,
        patch_control_points: uint32_t,
    ) -> () {
        unsafe {
            self.0.CmdSetPatchControlPointsEXT(
                command_buffer.abi(), 
                patch_control_points.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetPrimitiveRestartEnable(VkCommandBuffer commandBuffer, VkBool32 primitiveRestartEnable)
    /// ```
    pub unsafe fn cmd_set_primitive_restart_enable(
        &self,
        command_buffer: vk::CommandBuffer,
        primitive_restart_enable: bool,
    ) -> () {
        unsafe {
            self.0.CmdSetPrimitiveRestartEnableEXT(
                command_buffer.abi(), 
                primitive_restart_enable.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetRasterizerDiscardEnable(VkCommandBuffer commandBuffer, VkBool32 rasterizerDiscardEnable)
    /// ```
    pub unsafe fn cmd_set_rasterizer_discard_enable(
        &self,
        command_buffer: vk::CommandBuffer,
        rasterizer_discard_enable: bool,
    ) -> () {
        unsafe {
            self.0.CmdSetRasterizerDiscardEnableEXT(
                command_buffer.abi(), 
                rasterizer_discard_enable.abi(), 
            );
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::ext::extended_dynamic_state2 {
    type Commands = Device;
}

/// Device object
pub trait ExtExtendedDynamicState2Device {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

}

impl crate::HndScope<vk::Device> for vk::extensions::ext::extended_dynamic_state2 {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::ext::extended_dynamic_state2> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::ext::extended_dynamic_state2 {
        type Output = crate::hnd::Device<vk::extensions::ext::extended_dynamic_state2>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::ext::extended_dynamic_state2>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::ext::extended_dynamic_state2> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::ext::extended_dynamic_state2> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::ext::extended_dynamic_state2> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::ext::extended_dynamic_state2> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::ExtExtendedDynamicState2Device for crate::hnd::Device<vk::extensions::ext::extended_dynamic_state2> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::ext::extended_dynamic_state2, vk::Device> for crate::hnd::Device<vk::extensions::ext::extended_dynamic_state2> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

/// CommandBuffer object
pub trait ExtExtendedDynamicState2CommandBuffer {
    fn raw(&self) -> vk::CommandBuffer;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkCmdSetDepthBiasEnable(VkCommandBuffer commandBuffer, VkBool32 depthBiasEnable)
    /// ```
    unsafe fn set_depth_bias_enable(
        &self,
        depth_bias_enable: bool,
    ) -> () {
        unsafe {
            self.commands().cmd_set_depth_bias_enable(
                self.raw(),
                depth_bias_enable,
            )
        }
    }
    /// ```c
    /// void vkCmdSetLogicOpEXT(VkCommandBuffer commandBuffer, VkLogicOp logicOp)
    /// ```
    unsafe fn set_logic_op(
        &self,
        logic_op: LogicOp,
    ) -> () {
        unsafe {
            self.commands().cmd_set_logic_op(
                self.raw(),
                logic_op,
            )
        }
    }
    /// ```c
    /// void vkCmdSetPatchControlPointsEXT(VkCommandBuffer commandBuffer, uint32_t patchControlPoints)
    /// ```
    unsafe fn set_patch_control_points(
        &self,
        patch_control_points: uint32_t,
    ) -> () {
        unsafe {
            self.commands().cmd_set_patch_control_points(
                self.raw(),
                patch_control_points,
            )
        }
    }
    /// ```c
    /// void vkCmdSetPrimitiveRestartEnable(VkCommandBuffer commandBuffer, VkBool32 primitiveRestartEnable)
    /// ```
    unsafe fn set_primitive_restart_enable(
        &self,
        primitive_restart_enable: bool,
    ) -> () {
        unsafe {
            self.commands().cmd_set_primitive_restart_enable(
                self.raw(),
                primitive_restart_enable,
            )
        }
    }
    /// ```c
    /// void vkCmdSetRasterizerDiscardEnable(VkCommandBuffer commandBuffer, VkBool32 rasterizerDiscardEnable)
    /// ```
    unsafe fn set_rasterizer_discard_enable(
        &self,
        rasterizer_discard_enable: bool,
    ) -> () {
        unsafe {
            self.commands().cmd_set_rasterizer_discard_enable(
                self.raw(),
                rasterizer_discard_enable,
            )
        }
    }
}
