// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_EXT_memory_decompression` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::ext::memory_decompression::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::ext::memory_decompression::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// void vkCmdDecompressMemoryEXT(VkCommandBuffer commandBuffer, VkDecompressMemoryInfoEXT const* pDecompressMemoryInfoEXT)
    /// ```
    pub unsafe fn cmd_decompress_memory(
        &self,
        command_buffer: vk::CommandBuffer,
        decompress_memory_info_ext: &DecompressMemoryInfoEXT,
    ) -> () {
        unsafe {
            self.0.CmdDecompressMemoryEXT(
                command_buffer.abi(), 
                decompress_memory_info_ext.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdDecompressMemoryIndirectCountEXT(VkCommandBuffer commandBuffer, VkMemoryDecompressionMethodFlagsEXT decompressionMethod, VkDeviceAddress indirectCommandsAddress, VkDeviceAddress indirectCommandsCountAddress, uint32_t maxDecompressionCount, uint32_t stride)
    /// ```
    pub unsafe fn cmd_decompress_memory_indirect_count(
        &self,
        command_buffer: vk::CommandBuffer,
        decompression_method: MemoryDecompressionMethodFlagsEXT,
        indirect_commands_address: DeviceAddress,
        indirect_commands_count_address: DeviceAddress,
        max_decompression_count: uint32_t,
        stride: uint32_t,
    ) -> () {
        unsafe {
            self.0.CmdDecompressMemoryIndirectCountEXT(
                command_buffer.abi(), 
                decompression_method.abi(), 
                indirect_commands_address.abi(), 
                indirect_commands_count_address.abi(), 
                max_decompression_count.abi(), 
                stride.abi(), 
            );
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::ext::memory_decompression {
    type Commands = Device;
}

/// Device object
pub trait ExtMemoryDecompressionDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

}

impl crate::HndScope<vk::Device> for vk::extensions::ext::memory_decompression {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::ext::memory_decompression> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::ext::memory_decompression {
        type Output = crate::hnd::Device<vk::extensions::ext::memory_decompression>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::ext::memory_decompression>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::ext::memory_decompression> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::ext::memory_decompression> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::ext::memory_decompression> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::ext::memory_decompression> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::ExtMemoryDecompressionDevice for crate::hnd::Device<vk::extensions::ext::memory_decompression> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::ext::memory_decompression, vk::Device> for crate::hnd::Device<vk::extensions::ext::memory_decompression> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

/// CommandBuffer object
pub trait ExtMemoryDecompressionCommandBuffer {
    fn raw(&self) -> vk::CommandBuffer;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkCmdDecompressMemoryEXT(VkCommandBuffer commandBuffer, VkDecompressMemoryInfoEXT const* pDecompressMemoryInfoEXT)
    /// ```
    unsafe fn decompress_memory(
        &self,
        decompress_memory_info_ext: &DecompressMemoryInfoEXT,
    ) -> () {
        unsafe {
            self.commands().cmd_decompress_memory(
                self.raw(),
                decompress_memory_info_ext,
            )
        }
    }
    /// ```c
    /// void vkCmdDecompressMemoryIndirectCountEXT(VkCommandBuffer commandBuffer, VkMemoryDecompressionMethodFlagsEXT decompressionMethod, VkDeviceAddress indirectCommandsAddress, VkDeviceAddress indirectCommandsCountAddress, uint32_t maxDecompressionCount, uint32_t stride)
    /// ```
    unsafe fn decompress_memory_indirect_count(
        &self,
        decompression_method: MemoryDecompressionMethodFlagsEXT,
        indirect_commands_address: DeviceAddress,
        indirect_commands_count_address: DeviceAddress,
        max_decompression_count: uint32_t,
        stride: uint32_t,
    ) -> () {
        unsafe {
            self.commands().cmd_decompress_memory_indirect_count(
                self.raw(),
                decompression_method,
                indirect_commands_address,
                indirect_commands_count_address,
                max_decompression_count,
                stride,
            )
        }
    }
}
