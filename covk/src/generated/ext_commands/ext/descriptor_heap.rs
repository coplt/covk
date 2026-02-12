// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_EXT_descriptor_heap` InstanceCommands
#[derive(Debug, Clone, Copy)]
pub struct Instance(pub sys::ext::descriptor_heap::InstanceCommands);

impl Instance {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::ext::descriptor_heap::InstanceCommands::load(get) })
    }
}

impl Instance {
    /// ```c
    /// VkDeviceSize vkGetPhysicalDeviceDescriptorSizeEXT(VkPhysicalDevice physicalDevice, VkDescriptorType descriptorType)
    /// ```
    pub unsafe fn get_physical_device_descriptor_size(
        &self,
        physical_device: vk::PhysicalDevice,
        descriptor_type: DescriptorType,
    ) -> DeviceSize {
        unsafe {
            let _r = self.0.GetPhysicalDeviceDescriptorSizeEXT(
                physical_device.abi(), 
                descriptor_type.abi(), 
            );
            _r
        }
    }
}

impl crate::CommandScope<vk::Instance> for vk::extensions::ext::descriptor_heap {
    type Commands = Instance;
}

/// Instance object
pub trait ExtDescriptorHeapInstance {
    fn raw(&self) -> vk::Instance;
    fn commands(&self) -> &Instance;

}

impl crate::HndScope<vk::Instance> for vk::extensions::ext::descriptor_heap {
    type Impl = _hs_Instance::Instance;
}

mod _hs_Instance {
    use super::*;
    #[derive(Debug)]
    pub struct Instance(pub(crate) ::alloc::sync::Arc<super::Instance>, pub(crate) crate::hnd::Instance<vk::core>);

    impl Clone for Instance {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Instance<vk::extensions::ext::descriptor_heap> {
        pub unsafe fn new(base: &crate::hnd::Instance<vk::core>) -> Self {
            unsafe {
                Self(Instance(
                   ::alloc::sync::Arc::new(super::Instance::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Instance<vk::core>> for vk::extensions::ext::descriptor_heap {
        type Output = crate::hnd::Instance<vk::extensions::ext::descriptor_heap>;
        unsafe fn make(target: &crate::hnd::Instance<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Instance::<vk::extensions::ext::descriptor_heap>::new(target) }
        }
    }

    impl crate::hnd::Instance<vk::extensions::ext::descriptor_heap> {
        pub fn raw(&self) -> vk::Instance { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Instance<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Instance<vk::extensions::ext::descriptor_heap> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Instance({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Instance<vk::extensions::ext::descriptor_heap> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Instance<vk::extensions::ext::descriptor_heap> {
        type Target = super::Instance;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::ExtDescriptorHeapInstance for crate::hnd::Instance<vk::extensions::ext::descriptor_heap> {
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &super::Instance { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::ext::descriptor_heap, vk::Instance> for crate::hnd::Instance<vk::extensions::ext::descriptor_heap> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { self.commands() }
    }
}

/// PhysicalDevice object
pub trait ExtDescriptorHeapPhysicalDevice {
    fn raw(&self) -> vk::PhysicalDevice;
    fn commands(&self) -> &Instance;

    /// ```c
    /// VkDeviceSize vkGetPhysicalDeviceDescriptorSizeEXT(VkPhysicalDevice physicalDevice, VkDescriptorType descriptorType)
    /// ```
    unsafe fn get_descriptor_size(
        &self,
        descriptor_type: DescriptorType,
    ) -> DeviceSize {
        unsafe {
            self.commands().get_physical_device_descriptor_size(
                self.raw(),
                descriptor_type,
            )
        }
    }
}

/// `VK_EXT_descriptor_heap` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::ext::descriptor_heap::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::ext::descriptor_heap::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// void vkCmdBindResourceHeapEXT(VkCommandBuffer commandBuffer, VkBindHeapInfoEXT const* pBindInfo)
    /// ```
    pub unsafe fn cmd_bind_resource_heap(
        &self,
        command_buffer: vk::CommandBuffer,
        bind_info: &BindHeapInfoEXT,
    ) -> () {
        unsafe {
            self.0.CmdBindResourceHeapEXT(
                command_buffer.abi(), 
                bind_info.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdBindSamplerHeapEXT(VkCommandBuffer commandBuffer, VkBindHeapInfoEXT const* pBindInfo)
    /// ```
    pub unsafe fn cmd_bind_sampler_heap(
        &self,
        command_buffer: vk::CommandBuffer,
        bind_info: &BindHeapInfoEXT,
    ) -> () {
        unsafe {
            self.0.CmdBindSamplerHeapEXT(
                command_buffer.abi(), 
                bind_info.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdPushDataEXT(VkCommandBuffer commandBuffer, VkPushDataInfoEXT const* pPushDataInfo)
    /// ```
    pub unsafe fn cmd_push_data(
        &self,
        command_buffer: vk::CommandBuffer,
        push_data_info: &PushDataInfoEXT,
    ) -> () {
        unsafe {
            self.0.CmdPushDataEXT(
                command_buffer.abi(), 
                push_data_info.abi(), 
            );
        }
    }
    /// ```c
    /// VkResult vkGetImageOpaqueCaptureDataEXT(VkDevice device, uint32_t imageCount, VkImage const* pImages, VkHostAddressRangeEXT* pDatas)
    /// ```
    pub unsafe fn get_image_opaque_capture_data(
        &self,
        device: vk::Device,
        images: &[vk::Image],
        datas: &mut [HostAddressRangeEXT],
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.GetImageOpaqueCaptureDataEXT(
                device.abi(), 
                images.len() as _, 
                images.abi(), 
                datas.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkResult vkGetTensorOpaqueCaptureDataARM(VkDevice device, uint32_t tensorCount, VkTensorARM const* pTensors, VkHostAddressRangeEXT* pDatas)
    /// ```
    pub unsafe fn get_tensor_opaque_capture_data_arm(
        &self,
        device: vk::Device,
        tensors: &[vk::TensorARM],
        datas: &mut [HostAddressRangeEXT],
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.GetTensorOpaqueCaptureDataARM(
                device.abi(), 
                tensors.len() as _, 
                tensors.abi(), 
                datas.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkResult vkRegisterCustomBorderColorEXT(VkDevice device, VkSamplerCustomBorderColorCreateInfoEXT const* pBorderColor, VkBool32 requestIndex, uint32_t* pIndex)
    /// ```
    pub unsafe fn register_custom_border_color(
        &self,
        device: vk::Device,
        border_color: &SamplerCustomBorderColorCreateInfoEXT,
        request_index: bool,
        index: *mut uint32_t,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.RegisterCustomBorderColorEXT(
                device.abi(), 
                border_color.abi(), 
                request_index.abi(), 
                index.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// void vkUnregisterCustomBorderColorEXT(VkDevice device, uint32_t index)
    /// ```
    pub unsafe fn unregister_custom_border_color(
        &self,
        device: vk::Device,
        index: uint32_t,
    ) -> () {
        unsafe {
            self.0.UnregisterCustomBorderColorEXT(
                device.abi(), 
                index.abi(), 
            );
        }
    }
    /// ```c
    /// VkResult vkWriteResourceDescriptorsEXT(VkDevice device, uint32_t resourceCount, VkResourceDescriptorInfoEXT const* pResources, VkHostAddressRangeEXT const* pDescriptors)
    /// ```
    pub unsafe fn write_resource_descriptors(
        &self,
        device: vk::Device,
        resources: &[ResourceDescriptorInfoEXT],
        descriptors: &[HostAddressRangeEXT],
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.WriteResourceDescriptorsEXT(
                device.abi(), 
                resources.len() as _, 
                resources.abi(), 
                descriptors.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkResult vkWriteSamplerDescriptorsEXT(VkDevice device, uint32_t samplerCount, VkSamplerCreateInfo const* pSamplers, VkHostAddressRangeEXT const* pDescriptors)
    /// ```
    pub unsafe fn write_sampler_descriptors(
        &self,
        device: vk::Device,
        samplers: &[SamplerCreateInfo],
        descriptors: &[HostAddressRangeEXT],
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.WriteSamplerDescriptorsEXT(
                device.abi(), 
                samplers.len() as _, 
                samplers.abi(), 
                descriptors.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::ext::descriptor_heap {
    type Commands = Device;
}

/// Device object
pub trait ExtDescriptorHeapDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkGetImageOpaqueCaptureDataEXT(VkDevice device, uint32_t imageCount, VkImage const* pImages, VkHostAddressRangeEXT* pDatas)
    /// ```
    unsafe fn get_image_opaque_capture_data(
        &self,
        images: &[vk::Image],
        datas: &mut [HostAddressRangeEXT],
    ) -> crate::Result<()> {
        unsafe {
            self.commands().get_image_opaque_capture_data(
                self.raw(),
                images,
                datas,
            )
        }
    }
    /// ```c
    /// VkResult vkGetTensorOpaqueCaptureDataARM(VkDevice device, uint32_t tensorCount, VkTensorARM const* pTensors, VkHostAddressRangeEXT* pDatas)
    /// ```
    unsafe fn get_tensor_opaque_capture_data_arm(
        &self,
        tensors: &[vk::TensorARM],
        datas: &mut [HostAddressRangeEXT],
    ) -> crate::Result<()> {
        unsafe {
            self.commands().get_tensor_opaque_capture_data_arm(
                self.raw(),
                tensors,
                datas,
            )
        }
    }
    /// ```c
    /// VkResult vkRegisterCustomBorderColorEXT(VkDevice device, VkSamplerCustomBorderColorCreateInfoEXT const* pBorderColor, VkBool32 requestIndex, uint32_t* pIndex)
    /// ```
    unsafe fn register_custom_border_color(
        &self,
        border_color: &SamplerCustomBorderColorCreateInfoEXT,
        request_index: bool,
        index: *mut uint32_t,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().register_custom_border_color(
                self.raw(),
                border_color,
                request_index,
                index,
            )
        }
    }
    /// ```c
    /// void vkUnregisterCustomBorderColorEXT(VkDevice device, uint32_t index)
    /// ```
    unsafe fn unregister_custom_border_color(
        &self,
        index: uint32_t,
    ) -> () {
        unsafe {
            self.commands().unregister_custom_border_color(
                self.raw(),
                index,
            )
        }
    }
    /// ```c
    /// VkResult vkWriteResourceDescriptorsEXT(VkDevice device, uint32_t resourceCount, VkResourceDescriptorInfoEXT const* pResources, VkHostAddressRangeEXT const* pDescriptors)
    /// ```
    unsafe fn write_resource_descriptors(
        &self,
        resources: &[ResourceDescriptorInfoEXT],
        descriptors: &[HostAddressRangeEXT],
    ) -> crate::Result<()> {
        unsafe {
            self.commands().write_resource_descriptors(
                self.raw(),
                resources,
                descriptors,
            )
        }
    }
    /// ```c
    /// VkResult vkWriteSamplerDescriptorsEXT(VkDevice device, uint32_t samplerCount, VkSamplerCreateInfo const* pSamplers, VkHostAddressRangeEXT const* pDescriptors)
    /// ```
    unsafe fn write_sampler_descriptors(
        &self,
        samplers: &[SamplerCreateInfo],
        descriptors: &[HostAddressRangeEXT],
    ) -> crate::Result<()> {
        unsafe {
            self.commands().write_sampler_descriptors(
                self.raw(),
                samplers,
                descriptors,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::ext::descriptor_heap {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::ext::descriptor_heap> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::ext::descriptor_heap {
        type Output = crate::hnd::Device<vk::extensions::ext::descriptor_heap>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::ext::descriptor_heap>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::ext::descriptor_heap> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::ext::descriptor_heap> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::ext::descriptor_heap> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::ext::descriptor_heap> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::ExtDescriptorHeapDevice for crate::hnd::Device<vk::extensions::ext::descriptor_heap> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::ext::descriptor_heap, vk::Device> for crate::hnd::Device<vk::extensions::ext::descriptor_heap> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

/// CommandBuffer object
pub trait ExtDescriptorHeapCommandBuffer {
    fn raw(&self) -> vk::CommandBuffer;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkCmdBindResourceHeapEXT(VkCommandBuffer commandBuffer, VkBindHeapInfoEXT const* pBindInfo)
    /// ```
    unsafe fn bind_resource_heap(
        &self,
        bind_info: &BindHeapInfoEXT,
    ) -> () {
        unsafe {
            self.commands().cmd_bind_resource_heap(
                self.raw(),
                bind_info,
            )
        }
    }
    /// ```c
    /// void vkCmdBindSamplerHeapEXT(VkCommandBuffer commandBuffer, VkBindHeapInfoEXT const* pBindInfo)
    /// ```
    unsafe fn bind_sampler_heap(
        &self,
        bind_info: &BindHeapInfoEXT,
    ) -> () {
        unsafe {
            self.commands().cmd_bind_sampler_heap(
                self.raw(),
                bind_info,
            )
        }
    }
    /// ```c
    /// void vkCmdPushDataEXT(VkCommandBuffer commandBuffer, VkPushDataInfoEXT const* pPushDataInfo)
    /// ```
    unsafe fn push_data(
        &self,
        push_data_info: &PushDataInfoEXT,
    ) -> () {
        unsafe {
            self.commands().cmd_push_data(
                self.raw(),
                push_data_info,
            )
        }
    }
}
