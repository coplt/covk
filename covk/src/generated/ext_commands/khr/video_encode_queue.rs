// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_KHR_video_encode_queue` InstanceCommands
#[derive(Debug, Clone, Copy)]
pub struct Instance(pub sys::khr::video_encode_queue::InstanceCommands);

impl Instance {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::khr::video_encode_queue::InstanceCommands::load(get) })
    }
}

impl Instance {
    /// ```c
    /// VkResult vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR(VkPhysicalDevice physicalDevice, VkPhysicalDeviceVideoEncodeQualityLevelInfoKHR const* pQualityLevelInfo, VkVideoEncodeQualityLevelPropertiesKHR* pQualityLevelProperties)
    /// ```
    pub unsafe fn get_physical_device_video_encode_quality_level_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        quality_level_info: &PhysicalDeviceVideoEncodeQualityLevelInfoKHR,
        quality_level_properties: &mut VideoEncodeQualityLevelPropertiesKHR,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.GetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR(
                physical_device.abi(), 
                quality_level_info.abi(), 
                quality_level_properties.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
}

impl crate::CommandScope<vk::Instance> for vk::extensions::khr::video_encode_queue {
    type Commands = Instance;
}

/// Instance object
pub trait KhrVideoEncodeQueueInstance {
    fn raw(&self) -> vk::Instance;
    fn commands(&self) -> &Instance;

}

impl crate::HndScope<vk::Instance> for vk::extensions::khr::video_encode_queue {
    type Impl = _hs_Instance::Instance;
}

mod _hs_Instance {
    use super::*;
    #[derive(Debug)]
    pub struct Instance(pub(crate) ::alloc::sync::Arc<super::Instance>, pub(crate) crate::hnd::Instance<vk::core>);

    impl Clone for Instance {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Instance<vk::extensions::khr::video_encode_queue> {
        pub unsafe fn new(base: &crate::hnd::Instance<vk::core>) -> Self {
            unsafe {
                Self(Instance(
                   ::alloc::sync::Arc::new(super::Instance::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Instance<vk::core>> for vk::extensions::khr::video_encode_queue {
        type Output = crate::hnd::Instance<vk::extensions::khr::video_encode_queue>;
        unsafe fn make(target: &crate::hnd::Instance<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Instance::<vk::extensions::khr::video_encode_queue>::new(target) }
        }
    }

    impl crate::hnd::Instance<vk::extensions::khr::video_encode_queue> {
        pub fn raw(&self) -> vk::Instance { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Instance<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Instance<vk::extensions::khr::video_encode_queue> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Instance({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Instance<vk::extensions::khr::video_encode_queue> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Instance<vk::extensions::khr::video_encode_queue> {
        type Target = super::Instance;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::KhrVideoEncodeQueueInstance for crate::hnd::Instance<vk::extensions::khr::video_encode_queue> {
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &super::Instance { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::khr::video_encode_queue, vk::Instance> for crate::hnd::Instance<vk::extensions::khr::video_encode_queue> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { self.commands() }
    }
}

/// PhysicalDevice object
pub trait KhrVideoEncodeQueuePhysicalDevice {
    fn raw(&self) -> vk::PhysicalDevice;
    fn commands(&self) -> &Instance;

    /// ```c
    /// VkResult vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR(VkPhysicalDevice physicalDevice, VkPhysicalDeviceVideoEncodeQualityLevelInfoKHR const* pQualityLevelInfo, VkVideoEncodeQualityLevelPropertiesKHR* pQualityLevelProperties)
    /// ```
    unsafe fn get_video_encode_quality_level_properties(
        &self,
        quality_level_info: &PhysicalDeviceVideoEncodeQualityLevelInfoKHR,
        quality_level_properties: &mut VideoEncodeQualityLevelPropertiesKHR,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().get_physical_device_video_encode_quality_level_properties(
                self.raw(),
                quality_level_info,
                quality_level_properties,
            )
        }
    }
}

/// `VK_KHR_video_encode_queue` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::khr::video_encode_queue::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::khr::video_encode_queue::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// void vkCmdEncodeVideoKHR(VkCommandBuffer commandBuffer, VkVideoEncodeInfoKHR const* pEncodeInfo)
    /// ```
    pub unsafe fn cmd_encode_video(
        &self,
        command_buffer: vk::CommandBuffer,
        encode_info: &VideoEncodeInfoKHR,
    ) -> () {
        unsafe {
            self.0.CmdEncodeVideoKHR(
                command_buffer.abi(), 
                encode_info.abi(), 
            );
        }
    }
    /// ```c
    /// VkResult vkGetEncodedVideoSessionParametersKHR(VkDevice device, VkVideoEncodeSessionParametersGetInfoKHR const* pVideoSessionParametersInfo, VkVideoEncodeSessionParametersFeedbackInfoKHR* pFeedbackInfo, size_t* pDataSize, void* pData)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    pub unsafe fn get_encoded_video_session_parameters(
        &self,
        device: vk::Device,
        video_session_parameters_info: &VideoEncodeSessionParametersGetInfoKHR,
        feedback_info: Option<&mut VideoEncodeSessionParametersFeedbackInfoKHR>,
        data_size: *mut size_t,
        data: *mut void,
    ) -> crate::Result<Result> {
        unsafe {
            let _r = self.0.GetEncodedVideoSessionParametersKHR(
                device.abi(), 
                video_session_parameters_info.abi(), 
                feedback_info.abi(), 
                data_size.abi(), 
                data.abi(), 
            ).vk();
            _r.result(|| Some(_r))
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::khr::video_encode_queue {
    type Commands = Device;
}

/// Device object
pub trait KhrVideoEncodeQueueDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkGetEncodedVideoSessionParametersKHR(VkDevice device, VkVideoEncodeSessionParametersGetInfoKHR const* pVideoSessionParametersInfo, VkVideoEncodeSessionParametersFeedbackInfoKHR* pFeedbackInfo, size_t* pDataSize, void* pData)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    unsafe fn get_encoded_video_session_parameters(
        &self,
        video_session_parameters_info: &VideoEncodeSessionParametersGetInfoKHR,
        feedback_info: Option<&mut VideoEncodeSessionParametersFeedbackInfoKHR>,
        data_size: *mut size_t,
        data: *mut void,
    ) -> crate::Result<Result> {
        unsafe {
            self.commands().get_encoded_video_session_parameters(
                self.raw(),
                video_session_parameters_info,
                feedback_info,
                data_size,
                data,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::khr::video_encode_queue {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::khr::video_encode_queue> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::khr::video_encode_queue {
        type Output = crate::hnd::Device<vk::extensions::khr::video_encode_queue>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::khr::video_encode_queue>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::khr::video_encode_queue> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::khr::video_encode_queue> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::khr::video_encode_queue> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::khr::video_encode_queue> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::KhrVideoEncodeQueueDevice for crate::hnd::Device<vk::extensions::khr::video_encode_queue> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::khr::video_encode_queue, vk::Device> for crate::hnd::Device<vk::extensions::khr::video_encode_queue> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

/// CommandBuffer object
pub trait KhrVideoEncodeQueueCommandBuffer {
    fn raw(&self) -> vk::CommandBuffer;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkCmdEncodeVideoKHR(VkCommandBuffer commandBuffer, VkVideoEncodeInfoKHR const* pEncodeInfo)
    /// ```
    unsafe fn encode_video(
        &self,
        encode_info: &VideoEncodeInfoKHR,
    ) -> () {
        unsafe {
            self.commands().cmd_encode_video(
                self.raw(),
                encode_info,
            )
        }
    }
}
