// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_EXT_extended_dynamic_state` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::ext::extended_dynamic_state::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::ext::extended_dynamic_state::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// void vkCmdBindVertexBuffers2(VkCommandBuffer commandBuffer, uint32_t firstBinding, uint32_t bindingCount, VkBuffer const* pBuffers, VkDeviceSize const* pOffsets, VkDeviceSize const* pSizes, VkDeviceSize const* pStrides)
    /// ```
    pub unsafe fn cmd_bind_vertex_buffers2(
        &self,
        command_buffer: vk::CommandBuffer,
        first_binding: uint32_t,
        buffers: &[Option<vk::Buffer>],
        offsets: &[DeviceSize],
        sizes: Option<&[DeviceSize]>,
        strides: Option<&[DeviceSize]>,
    ) -> () {
        unsafe {
            self.0.CmdBindVertexBuffers2EXT(
                command_buffer.abi(), 
                first_binding.abi(), 
                buffers.len() as _, 
                buffers.abi(), 
                offsets.abi(), 
                sizes.abi(), 
                strides.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetCullMode(VkCommandBuffer commandBuffer, VkCullModeFlags cullMode)
    /// ```
    pub unsafe fn cmd_set_cull_mode(
        &self,
        command_buffer: vk::CommandBuffer,
        cull_mode: CullModeFlags,
    ) -> () {
        unsafe {
            self.0.CmdSetCullModeEXT(
                command_buffer.abi(), 
                cull_mode.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetDepthBoundsTestEnable(VkCommandBuffer commandBuffer, VkBool32 depthBoundsTestEnable)
    /// ```
    pub unsafe fn cmd_set_depth_bounds_test_enable(
        &self,
        command_buffer: vk::CommandBuffer,
        depth_bounds_test_enable: bool,
    ) -> () {
        unsafe {
            self.0.CmdSetDepthBoundsTestEnableEXT(
                command_buffer.abi(), 
                depth_bounds_test_enable.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetDepthCompareOp(VkCommandBuffer commandBuffer, VkCompareOp depthCompareOp)
    /// ```
    pub unsafe fn cmd_set_depth_compare_op(
        &self,
        command_buffer: vk::CommandBuffer,
        depth_compare_op: CompareOp,
    ) -> () {
        unsafe {
            self.0.CmdSetDepthCompareOpEXT(
                command_buffer.abi(), 
                depth_compare_op.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetDepthTestEnable(VkCommandBuffer commandBuffer, VkBool32 depthTestEnable)
    /// ```
    pub unsafe fn cmd_set_depth_test_enable(
        &self,
        command_buffer: vk::CommandBuffer,
        depth_test_enable: bool,
    ) -> () {
        unsafe {
            self.0.CmdSetDepthTestEnableEXT(
                command_buffer.abi(), 
                depth_test_enable.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetDepthWriteEnable(VkCommandBuffer commandBuffer, VkBool32 depthWriteEnable)
    /// ```
    pub unsafe fn cmd_set_depth_write_enable(
        &self,
        command_buffer: vk::CommandBuffer,
        depth_write_enable: bool,
    ) -> () {
        unsafe {
            self.0.CmdSetDepthWriteEnableEXT(
                command_buffer.abi(), 
                depth_write_enable.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetFrontFace(VkCommandBuffer commandBuffer, VkFrontFace frontFace)
    /// ```
    pub unsafe fn cmd_set_front_face(
        &self,
        command_buffer: vk::CommandBuffer,
        front_face: FrontFace,
    ) -> () {
        unsafe {
            self.0.CmdSetFrontFaceEXT(
                command_buffer.abi(), 
                front_face.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetPrimitiveTopology(VkCommandBuffer commandBuffer, VkPrimitiveTopology primitiveTopology)
    /// ```
    pub unsafe fn cmd_set_primitive_topology(
        &self,
        command_buffer: vk::CommandBuffer,
        primitive_topology: PrimitiveTopology,
    ) -> () {
        unsafe {
            self.0.CmdSetPrimitiveTopologyEXT(
                command_buffer.abi(), 
                primitive_topology.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetScissorWithCount(VkCommandBuffer commandBuffer, uint32_t scissorCount, VkRect2D const* pScissors)
    /// ```
    pub unsafe fn cmd_set_scissor_with_count(
        &self,
        command_buffer: vk::CommandBuffer,
        scissors: &[Rect2D],
    ) -> () {
        unsafe {
            self.0.CmdSetScissorWithCountEXT(
                command_buffer.abi(), 
                scissors.len() as _, 
                scissors.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetStencilOp(VkCommandBuffer commandBuffer, VkStencilFaceFlags faceMask, VkStencilOp failOp, VkStencilOp passOp, VkStencilOp depthFailOp, VkCompareOp compareOp)
    /// ```
    pub unsafe fn cmd_set_stencil_op(
        &self,
        command_buffer: vk::CommandBuffer,
        face_mask: StencilFaceFlags,
        fail_op: StencilOp,
        pass_op: StencilOp,
        depth_fail_op: StencilOp,
        compare_op: CompareOp,
    ) -> () {
        unsafe {
            self.0.CmdSetStencilOpEXT(
                command_buffer.abi(), 
                face_mask.abi(), 
                fail_op.abi(), 
                pass_op.abi(), 
                depth_fail_op.abi(), 
                compare_op.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetStencilTestEnable(VkCommandBuffer commandBuffer, VkBool32 stencilTestEnable)
    /// ```
    pub unsafe fn cmd_set_stencil_test_enable(
        &self,
        command_buffer: vk::CommandBuffer,
        stencil_test_enable: bool,
    ) -> () {
        unsafe {
            self.0.CmdSetStencilTestEnableEXT(
                command_buffer.abi(), 
                stencil_test_enable.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetViewportWithCount(VkCommandBuffer commandBuffer, uint32_t viewportCount, VkViewport const* pViewports)
    /// ```
    pub unsafe fn cmd_set_viewport_with_count(
        &self,
        command_buffer: vk::CommandBuffer,
        viewports: &[Viewport],
    ) -> () {
        unsafe {
            self.0.CmdSetViewportWithCountEXT(
                command_buffer.abi(), 
                viewports.len() as _, 
                viewports.abi(), 
            );
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::ext::extended_dynamic_state {
    type Commands = Device;
}

/// Device object
pub trait ExtExtendedDynamicStateDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

}

impl crate::HndScope<vk::Device> for vk::extensions::ext::extended_dynamic_state {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::ext::extended_dynamic_state> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::ext::extended_dynamic_state {
        type Output = crate::hnd::Device<vk::extensions::ext::extended_dynamic_state>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::ext::extended_dynamic_state>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::ext::extended_dynamic_state> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::ext::extended_dynamic_state> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::ext::extended_dynamic_state> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::ext::extended_dynamic_state> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::ExtExtendedDynamicStateDevice for crate::hnd::Device<vk::extensions::ext::extended_dynamic_state> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::ext::extended_dynamic_state, vk::Device> for crate::hnd::Device<vk::extensions::ext::extended_dynamic_state> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

/// CommandBuffer object
pub trait ExtExtendedDynamicStateCommandBuffer {
    fn raw(&self) -> vk::CommandBuffer;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkCmdBindVertexBuffers2(VkCommandBuffer commandBuffer, uint32_t firstBinding, uint32_t bindingCount, VkBuffer const* pBuffers, VkDeviceSize const* pOffsets, VkDeviceSize const* pSizes, VkDeviceSize const* pStrides)
    /// ```
    unsafe fn bind_vertex_buffers2(
        &self,
        first_binding: uint32_t,
        buffers: &[Option<vk::Buffer>],
        offsets: &[DeviceSize],
        sizes: Option<&[DeviceSize]>,
        strides: Option<&[DeviceSize]>,
    ) -> () {
        unsafe {
            self.commands().cmd_bind_vertex_buffers2(
                self.raw(),
                first_binding,
                buffers,
                offsets,
                sizes,
                strides,
            )
        }
    }
    /// ```c
    /// void vkCmdSetCullMode(VkCommandBuffer commandBuffer, VkCullModeFlags cullMode)
    /// ```
    unsafe fn set_cull_mode(
        &self,
        cull_mode: CullModeFlags,
    ) -> () {
        unsafe {
            self.commands().cmd_set_cull_mode(
                self.raw(),
                cull_mode,
            )
        }
    }
    /// ```c
    /// void vkCmdSetDepthBoundsTestEnable(VkCommandBuffer commandBuffer, VkBool32 depthBoundsTestEnable)
    /// ```
    unsafe fn set_depth_bounds_test_enable(
        &self,
        depth_bounds_test_enable: bool,
    ) -> () {
        unsafe {
            self.commands().cmd_set_depth_bounds_test_enable(
                self.raw(),
                depth_bounds_test_enable,
            )
        }
    }
    /// ```c
    /// void vkCmdSetDepthCompareOp(VkCommandBuffer commandBuffer, VkCompareOp depthCompareOp)
    /// ```
    unsafe fn set_depth_compare_op(
        &self,
        depth_compare_op: CompareOp,
    ) -> () {
        unsafe {
            self.commands().cmd_set_depth_compare_op(
                self.raw(),
                depth_compare_op,
            )
        }
    }
    /// ```c
    /// void vkCmdSetDepthTestEnable(VkCommandBuffer commandBuffer, VkBool32 depthTestEnable)
    /// ```
    unsafe fn set_depth_test_enable(
        &self,
        depth_test_enable: bool,
    ) -> () {
        unsafe {
            self.commands().cmd_set_depth_test_enable(
                self.raw(),
                depth_test_enable,
            )
        }
    }
    /// ```c
    /// void vkCmdSetDepthWriteEnable(VkCommandBuffer commandBuffer, VkBool32 depthWriteEnable)
    /// ```
    unsafe fn set_depth_write_enable(
        &self,
        depth_write_enable: bool,
    ) -> () {
        unsafe {
            self.commands().cmd_set_depth_write_enable(
                self.raw(),
                depth_write_enable,
            )
        }
    }
    /// ```c
    /// void vkCmdSetFrontFace(VkCommandBuffer commandBuffer, VkFrontFace frontFace)
    /// ```
    unsafe fn set_front_face(
        &self,
        front_face: FrontFace,
    ) -> () {
        unsafe {
            self.commands().cmd_set_front_face(
                self.raw(),
                front_face,
            )
        }
    }
    /// ```c
    /// void vkCmdSetPrimitiveTopology(VkCommandBuffer commandBuffer, VkPrimitiveTopology primitiveTopology)
    /// ```
    unsafe fn set_primitive_topology(
        &self,
        primitive_topology: PrimitiveTopology,
    ) -> () {
        unsafe {
            self.commands().cmd_set_primitive_topology(
                self.raw(),
                primitive_topology,
            )
        }
    }
    /// ```c
    /// void vkCmdSetScissorWithCount(VkCommandBuffer commandBuffer, uint32_t scissorCount, VkRect2D const* pScissors)
    /// ```
    unsafe fn set_scissor_with_count(
        &self,
        scissors: &[Rect2D],
    ) -> () {
        unsafe {
            self.commands().cmd_set_scissor_with_count(
                self.raw(),
                scissors,
            )
        }
    }
    /// ```c
    /// void vkCmdSetStencilOp(VkCommandBuffer commandBuffer, VkStencilFaceFlags faceMask, VkStencilOp failOp, VkStencilOp passOp, VkStencilOp depthFailOp, VkCompareOp compareOp)
    /// ```
    unsafe fn set_stencil_op(
        &self,
        face_mask: StencilFaceFlags,
        fail_op: StencilOp,
        pass_op: StencilOp,
        depth_fail_op: StencilOp,
        compare_op: CompareOp,
    ) -> () {
        unsafe {
            self.commands().cmd_set_stencil_op(
                self.raw(),
                face_mask,
                fail_op,
                pass_op,
                depth_fail_op,
                compare_op,
            )
        }
    }
    /// ```c
    /// void vkCmdSetStencilTestEnable(VkCommandBuffer commandBuffer, VkBool32 stencilTestEnable)
    /// ```
    unsafe fn set_stencil_test_enable(
        &self,
        stencil_test_enable: bool,
    ) -> () {
        unsafe {
            self.commands().cmd_set_stencil_test_enable(
                self.raw(),
                stencil_test_enable,
            )
        }
    }
    /// ```c
    /// void vkCmdSetViewportWithCount(VkCommandBuffer commandBuffer, uint32_t viewportCount, VkViewport const* pViewports)
    /// ```
    unsafe fn set_viewport_with_count(
        &self,
        viewports: &[Viewport],
    ) -> () {
        unsafe {
            self.commands().cmd_set_viewport_with_count(
                self.raw(),
                viewports,
            )
        }
    }
}
