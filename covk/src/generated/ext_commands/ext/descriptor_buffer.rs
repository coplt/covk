// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_EXT_descriptor_buffer` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::ext::descriptor_buffer::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::ext::descriptor_buffer::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// void vkCmdBindDescriptorBufferEmbeddedSamplersEXT(VkCommandBuffer commandBuffer, VkPipelineBindPoint pipelineBindPoint, VkPipelineLayout layout, uint32_t set)
    /// ```
    pub unsafe fn cmd_bind_descriptor_buffer_embedded_samplers(
        &self,
        command_buffer: vk::CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        layout: vk::PipelineLayout,
        set: uint32_t,
    ) -> () {
        unsafe {
            self.0.CmdBindDescriptorBufferEmbeddedSamplersEXT(
                command_buffer.abi(), 
                pipeline_bind_point.abi(), 
                layout.abi(), 
                set.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdBindDescriptorBuffersEXT(VkCommandBuffer commandBuffer, uint32_t bufferCount, VkDescriptorBufferBindingInfoEXT const* pBindingInfos)
    /// ```
    pub unsafe fn cmd_bind_descriptor_buffers(
        &self,
        command_buffer: vk::CommandBuffer,
        binding_infos: &[DescriptorBufferBindingInfoEXT],
    ) -> () {
        unsafe {
            self.0.CmdBindDescriptorBuffersEXT(
                command_buffer.abi(), 
                binding_infos.len() as _, 
                binding_infos.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetDescriptorBufferOffsetsEXT(VkCommandBuffer commandBuffer, VkPipelineBindPoint pipelineBindPoint, VkPipelineLayout layout, uint32_t firstSet, uint32_t setCount, uint32_t const* pBufferIndices, VkDeviceSize const* pOffsets)
    /// ```
    pub unsafe fn cmd_set_descriptor_buffer_offsets(
        &self,
        command_buffer: vk::CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        layout: vk::PipelineLayout,
        first_set: uint32_t,
        buffer_indices: &[uint32_t],
        offsets: &[DeviceSize],
    ) -> () {
        unsafe {
            self.0.CmdSetDescriptorBufferOffsetsEXT(
                command_buffer.abi(), 
                pipeline_bind_point.abi(), 
                layout.abi(), 
                first_set.abi(), 
                buffer_indices.len() as _, 
                buffer_indices.abi(), 
                offsets.abi(), 
            );
        }
    }
    /// ```c
    /// VkResult vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT(VkDevice device, VkAccelerationStructureCaptureDescriptorDataInfoEXT const* pInfo, void* pData)
    /// ```
    pub unsafe fn get_acceleration_structure_opaque_capture_descriptor_data(
        &self,
        device: vk::Device,
        info: &AccelerationStructureCaptureDescriptorDataInfoEXT,
        data: *mut void,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.GetAccelerationStructureOpaqueCaptureDescriptorDataEXT(
                device.abi(), 
                info.abi(), 
                data.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkResult vkGetBufferOpaqueCaptureDescriptorDataEXT(VkDevice device, VkBufferCaptureDescriptorDataInfoEXT const* pInfo, void* pData)
    /// ```
    pub unsafe fn get_buffer_opaque_capture_descriptor_data(
        &self,
        device: vk::Device,
        info: &BufferCaptureDescriptorDataInfoEXT,
        data: *mut void,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.GetBufferOpaqueCaptureDescriptorDataEXT(
                device.abi(), 
                info.abi(), 
                data.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// void vkGetDescriptorEXT(VkDevice device, VkDescriptorGetInfoEXT const* pDescriptorInfo, size_t dataSize, void* pDescriptor)
    /// ```
    pub unsafe fn get_descriptor(
        &self,
        device: vk::Device,
        descriptor_info: &DescriptorGetInfoEXT,
        data_size: size_t,
        descriptor: *mut void,
    ) -> () {
        unsafe {
            self.0.GetDescriptorEXT(
                device.abi(), 
                descriptor_info.abi(), 
                data_size.abi(), 
                descriptor.abi(), 
            );
        }
    }
    /// ```c
    /// void vkGetDescriptorSetLayoutBindingOffsetEXT(VkDevice device, VkDescriptorSetLayout layout, uint32_t binding, VkDeviceSize* pOffset)
    /// ```
    pub unsafe fn get_descriptor_set_layout_binding_offset(
        &self,
        device: vk::Device,
        layout: vk::DescriptorSetLayout,
        binding: uint32_t,
    ) -> DeviceSize {
        unsafe {
            let mut _v: DeviceSize = Default::default();
            self.0.GetDescriptorSetLayoutBindingOffsetEXT(
                device.abi(), 
                layout.abi(), 
                binding.abi(), 
                (&mut _v).abi(), 
            );
            _v
        }
    }
    /// ```c
    /// void vkGetDescriptorSetLayoutSizeEXT(VkDevice device, VkDescriptorSetLayout layout, VkDeviceSize* pLayoutSizeInBytes)
    /// ```
    pub unsafe fn get_descriptor_set_layout_size(
        &self,
        device: vk::Device,
        layout: vk::DescriptorSetLayout,
    ) -> DeviceSize {
        unsafe {
            let mut _v: DeviceSize = Default::default();
            self.0.GetDescriptorSetLayoutSizeEXT(
                device.abi(), 
                layout.abi(), 
                (&mut _v).abi(), 
            );
            _v
        }
    }
    /// ```c
    /// VkResult vkGetImageOpaqueCaptureDescriptorDataEXT(VkDevice device, VkImageCaptureDescriptorDataInfoEXT const* pInfo, void* pData)
    /// ```
    pub unsafe fn get_image_opaque_capture_descriptor_data(
        &self,
        device: vk::Device,
        info: &ImageCaptureDescriptorDataInfoEXT,
        data: *mut void,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.GetImageOpaqueCaptureDescriptorDataEXT(
                device.abi(), 
                info.abi(), 
                data.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkResult vkGetImageViewOpaqueCaptureDescriptorDataEXT(VkDevice device, VkImageViewCaptureDescriptorDataInfoEXT const* pInfo, void* pData)
    /// ```
    pub unsafe fn get_image_view_opaque_capture_descriptor_data(
        &self,
        device: vk::Device,
        info: &ImageViewCaptureDescriptorDataInfoEXT,
        data: *mut void,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.GetImageViewOpaqueCaptureDescriptorDataEXT(
                device.abi(), 
                info.abi(), 
                data.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkResult vkGetSamplerOpaqueCaptureDescriptorDataEXT(VkDevice device, VkSamplerCaptureDescriptorDataInfoEXT const* pInfo, void* pData)
    /// ```
    pub unsafe fn get_sampler_opaque_capture_descriptor_data(
        &self,
        device: vk::Device,
        info: &SamplerCaptureDescriptorDataInfoEXT,
        data: *mut void,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.GetSamplerOpaqueCaptureDescriptorDataEXT(
                device.abi(), 
                info.abi(), 
                data.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::ext::descriptor_buffer {
    type Commands = Device;
}

/// Device object
pub trait ExtDescriptorBufferDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT(VkDevice device, VkAccelerationStructureCaptureDescriptorDataInfoEXT const* pInfo, void* pData)
    /// ```
    unsafe fn get_acceleration_structure_opaque_capture_descriptor_data(
        &self,
        info: &AccelerationStructureCaptureDescriptorDataInfoEXT,
        data: *mut void,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().get_acceleration_structure_opaque_capture_descriptor_data(
                self.raw(),
                info,
                data,
            )
        }
    }
    /// ```c
    /// VkResult vkGetBufferOpaqueCaptureDescriptorDataEXT(VkDevice device, VkBufferCaptureDescriptorDataInfoEXT const* pInfo, void* pData)
    /// ```
    unsafe fn get_buffer_opaque_capture_descriptor_data(
        &self,
        info: &BufferCaptureDescriptorDataInfoEXT,
        data: *mut void,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().get_buffer_opaque_capture_descriptor_data(
                self.raw(),
                info,
                data,
            )
        }
    }
    /// ```c
    /// void vkGetDescriptorEXT(VkDevice device, VkDescriptorGetInfoEXT const* pDescriptorInfo, size_t dataSize, void* pDescriptor)
    /// ```
    unsafe fn get_descriptor(
        &self,
        descriptor_info: &DescriptorGetInfoEXT,
        data_size: size_t,
        descriptor: *mut void,
    ) -> () {
        unsafe {
            self.commands().get_descriptor(
                self.raw(),
                descriptor_info,
                data_size,
                descriptor,
            )
        }
    }
    /// ```c
    /// void vkGetDescriptorSetLayoutBindingOffsetEXT(VkDevice device, VkDescriptorSetLayout layout, uint32_t binding, VkDeviceSize* pOffset)
    /// ```
    unsafe fn get_descriptor_set_layout_binding_offset(
        &self,
        layout: vk::DescriptorSetLayout,
        binding: uint32_t,
    ) -> DeviceSize {
        unsafe {
            self.commands().get_descriptor_set_layout_binding_offset(
                self.raw(),
                layout,
                binding,
            )
        }
    }
    /// ```c
    /// void vkGetDescriptorSetLayoutSizeEXT(VkDevice device, VkDescriptorSetLayout layout, VkDeviceSize* pLayoutSizeInBytes)
    /// ```
    unsafe fn get_descriptor_set_layout_size(
        &self,
        layout: vk::DescriptorSetLayout,
    ) -> DeviceSize {
        unsafe {
            self.commands().get_descriptor_set_layout_size(
                self.raw(),
                layout,
            )
        }
    }
    /// ```c
    /// VkResult vkGetImageOpaqueCaptureDescriptorDataEXT(VkDevice device, VkImageCaptureDescriptorDataInfoEXT const* pInfo, void* pData)
    /// ```
    unsafe fn get_image_opaque_capture_descriptor_data(
        &self,
        info: &ImageCaptureDescriptorDataInfoEXT,
        data: *mut void,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().get_image_opaque_capture_descriptor_data(
                self.raw(),
                info,
                data,
            )
        }
    }
    /// ```c
    /// VkResult vkGetImageViewOpaqueCaptureDescriptorDataEXT(VkDevice device, VkImageViewCaptureDescriptorDataInfoEXT const* pInfo, void* pData)
    /// ```
    unsafe fn get_image_view_opaque_capture_descriptor_data(
        &self,
        info: &ImageViewCaptureDescriptorDataInfoEXT,
        data: *mut void,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().get_image_view_opaque_capture_descriptor_data(
                self.raw(),
                info,
                data,
            )
        }
    }
    /// ```c
    /// VkResult vkGetSamplerOpaqueCaptureDescriptorDataEXT(VkDevice device, VkSamplerCaptureDescriptorDataInfoEXT const* pInfo, void* pData)
    /// ```
    unsafe fn get_sampler_opaque_capture_descriptor_data(
        &self,
        info: &SamplerCaptureDescriptorDataInfoEXT,
        data: *mut void,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().get_sampler_opaque_capture_descriptor_data(
                self.raw(),
                info,
                data,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::ext::descriptor_buffer {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::ext::descriptor_buffer> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::ext::descriptor_buffer {
        type Output = crate::hnd::Device<vk::extensions::ext::descriptor_buffer>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::ext::descriptor_buffer>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::ext::descriptor_buffer> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::ext::descriptor_buffer> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::ext::descriptor_buffer> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::ext::descriptor_buffer> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::ExtDescriptorBufferDevice for crate::hnd::Device<vk::extensions::ext::descriptor_buffer> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::ext::descriptor_buffer, vk::Device> for crate::hnd::Device<vk::extensions::ext::descriptor_buffer> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

/// CommandBuffer object
pub trait ExtDescriptorBufferCommandBuffer {
    fn raw(&self) -> vk::CommandBuffer;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkCmdBindDescriptorBufferEmbeddedSamplersEXT(VkCommandBuffer commandBuffer, VkPipelineBindPoint pipelineBindPoint, VkPipelineLayout layout, uint32_t set)
    /// ```
    unsafe fn bind_descriptor_buffer_embedded_samplers(
        &self,
        pipeline_bind_point: PipelineBindPoint,
        layout: vk::PipelineLayout,
        set: uint32_t,
    ) -> () {
        unsafe {
            self.commands().cmd_bind_descriptor_buffer_embedded_samplers(
                self.raw(),
                pipeline_bind_point,
                layout,
                set,
            )
        }
    }
    /// ```c
    /// void vkCmdBindDescriptorBuffersEXT(VkCommandBuffer commandBuffer, uint32_t bufferCount, VkDescriptorBufferBindingInfoEXT const* pBindingInfos)
    /// ```
    unsafe fn bind_descriptor_buffers(
        &self,
        binding_infos: &[DescriptorBufferBindingInfoEXT],
    ) -> () {
        unsafe {
            self.commands().cmd_bind_descriptor_buffers(
                self.raw(),
                binding_infos,
            )
        }
    }
    /// ```c
    /// void vkCmdSetDescriptorBufferOffsetsEXT(VkCommandBuffer commandBuffer, VkPipelineBindPoint pipelineBindPoint, VkPipelineLayout layout, uint32_t firstSet, uint32_t setCount, uint32_t const* pBufferIndices, VkDeviceSize const* pOffsets)
    /// ```
    unsafe fn set_descriptor_buffer_offsets(
        &self,
        pipeline_bind_point: PipelineBindPoint,
        layout: vk::PipelineLayout,
        first_set: uint32_t,
        buffer_indices: &[uint32_t],
        offsets: &[DeviceSize],
    ) -> () {
        unsafe {
            self.commands().cmd_set_descriptor_buffer_offsets(
                self.raw(),
                pipeline_bind_point,
                layout,
                first_set,
                buffer_indices,
                offsets,
            )
        }
    }
}
