// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_NV_optical_flow` InstanceCommands
#[derive(Debug, Clone, Copy)]
pub struct Instance(pub sys::nv::optical_flow::InstanceCommands);

impl Instance {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::nv::optical_flow::InstanceCommands::load(get) })
    }
}

impl Instance {
    /// ```c
    /// VkResult vkGetPhysicalDeviceOpticalFlowImageFormatsNV(VkPhysicalDevice physicalDevice, VkOpticalFlowImageFormatInfoNV const* pOpticalFlowImageFormatInfo, uint32_t* pFormatCount, VkOpticalFlowImageFormatPropertiesNV* pImageFormatProperties)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    pub unsafe fn get_physical_device_optical_flow_image_formats(
        &self,
        physical_device: vk::PhysicalDevice,
        optical_flow_image_format_info: &OpticalFlowImageFormatInfoNV,
        image_format_properties: Option<&mut ::alloc::vec::Vec<OpticalFlowImageFormatPropertiesNV>>,
    ) -> crate::Result<(uint32_t, Result)> {
        unsafe {
            let mut _c: uint32_t = Default::default();
            let mut _r = self.0.GetPhysicalDeviceOpticalFlowImageFormatsNV(
                physical_device.abi(), 
                optical_flow_image_format_info.abi(), 
                &mut _c, 
                Default::default(), 
            ).vk();
            if let Some(_b) = image_format_properties {
                _r.result(|| Some(()))?;
                _b.reserve(_c as usize);
                _r = self.0.GetPhysicalDeviceOpticalFlowImageFormatsNV(
                    physical_device.abi(), 
                    optical_flow_image_format_info.abi(), 
                    &mut _c, 
                    (&mut **_b).abi(), 
                ).vk();
                _r.result(|| Some(()))?;
                _b.set_len(_b.len() + _c as usize);
            }
            _r.result_multi(|| Some(_c))
        }
    }
}

impl crate::CommandScope<vk::Instance> for vk::extensions::nv::optical_flow {
    type Commands = Instance;
}

/// Instance object
pub trait NvOpticalFlowInstance {
    fn raw(&self) -> vk::Instance;
    fn commands(&self) -> &Instance;

}

impl crate::HndScope<vk::Instance> for vk::extensions::nv::optical_flow {
    type Impl = _hs_Instance::Instance;
}

mod _hs_Instance {
    use super::*;
    #[derive(Debug)]
    pub struct Instance(pub(crate) ::alloc::sync::Arc<super::Instance>, pub(crate) crate::hnd::Instance<vk::core>);

    impl Clone for Instance {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Instance<vk::extensions::nv::optical_flow> {
        pub unsafe fn new(base: &crate::hnd::Instance<vk::core>) -> Self {
            unsafe {
                Self(Instance(
                   ::alloc::sync::Arc::new(super::Instance::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Instance<vk::core>> for vk::extensions::nv::optical_flow {
        type Output = crate::hnd::Instance<vk::extensions::nv::optical_flow>;
        unsafe fn make(target: &crate::hnd::Instance<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Instance::<vk::extensions::nv::optical_flow>::new(target) }
        }
    }

    impl crate::hnd::Instance<vk::extensions::nv::optical_flow> {
        pub fn raw(&self) -> vk::Instance { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Instance<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Instance<vk::extensions::nv::optical_flow> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Instance({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Instance<vk::extensions::nv::optical_flow> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Instance<vk::extensions::nv::optical_flow> {
        type Target = super::Instance;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::NvOpticalFlowInstance for crate::hnd::Instance<vk::extensions::nv::optical_flow> {
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &super::Instance { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::nv::optical_flow, vk::Instance> for crate::hnd::Instance<vk::extensions::nv::optical_flow> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { self.commands() }
    }
}

/// PhysicalDevice object
pub trait NvOpticalFlowPhysicalDevice {
    fn raw(&self) -> vk::PhysicalDevice;
    fn commands(&self) -> &Instance;

    /// ```c
    /// VkResult vkGetPhysicalDeviceOpticalFlowImageFormatsNV(VkPhysicalDevice physicalDevice, VkOpticalFlowImageFormatInfoNV const* pOpticalFlowImageFormatInfo, uint32_t* pFormatCount, VkOpticalFlowImageFormatPropertiesNV* pImageFormatProperties)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    unsafe fn get_optical_flow_image_formats(
        &self,
        optical_flow_image_format_info: &OpticalFlowImageFormatInfoNV,
        image_format_properties: Option<&mut ::alloc::vec::Vec<OpticalFlowImageFormatPropertiesNV>>,
    ) -> crate::Result<(uint32_t, Result)> {
        unsafe {
            self.commands().get_physical_device_optical_flow_image_formats(
                self.raw(),
                optical_flow_image_format_info,
                image_format_properties,
            )
        }
    }
}

/// `VK_NV_optical_flow` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::nv::optical_flow::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::nv::optical_flow::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// VkResult vkBindOpticalFlowSessionImageNV(VkDevice device, VkOpticalFlowSessionNV session, VkOpticalFlowSessionBindingPointNV bindingPoint, VkImageView view, VkImageLayout layout)
    /// ```
    pub unsafe fn bind_optical_flow_session_image(
        &self,
        device: vk::Device,
        session: vk::OpticalFlowSessionNV,
        binding_point: OpticalFlowSessionBindingPointNV,
        view: Option<vk::ImageView>,
        layout: ImageLayout,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.BindOpticalFlowSessionImageNV(
                device.abi(), 
                session.abi(), 
                binding_point.abi(), 
                view.abi(), 
                layout.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// void vkCmdOpticalFlowExecuteNV(VkCommandBuffer commandBuffer, VkOpticalFlowSessionNV session, VkOpticalFlowExecuteInfoNV const* pExecuteInfo)
    /// ```
    pub unsafe fn cmd_optical_flow_execute(
        &self,
        command_buffer: vk::CommandBuffer,
        session: vk::OpticalFlowSessionNV,
        execute_info: &OpticalFlowExecuteInfoNV,
    ) -> () {
        unsafe {
            self.0.CmdOpticalFlowExecuteNV(
                command_buffer.abi(), 
                session.abi(), 
                execute_info.abi(), 
            );
        }
    }
    /// ```c
    /// VkResult vkCreateOpticalFlowSessionNV(VkDevice device, VkOpticalFlowSessionCreateInfoNV const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkOpticalFlowSessionNV* pSession)
    /// ```
    pub unsafe fn create_optical_flow_session(
        &self,
        device: vk::Device,
        create_info: &OpticalFlowSessionCreateInfoNV,
    ) -> crate::Result<vk::OpticalFlowSessionNV> {
        unsafe {
            let mut _v: Option<vk::OpticalFlowSessionNV> = Default::default();
            let _r = self.0.CreateOpticalFlowSessionNV(
                device.abi(), 
                create_info.abi(), 
                Default::default(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| _v)
        }
    }
    /// ```c
    /// void vkDestroyOpticalFlowSessionNV(VkDevice device, VkOpticalFlowSessionNV session, VkAllocationCallbacks const* pAllocator)
    /// ```
    pub unsafe fn destroy_optical_flow_session(
        &self,
        device: vk::Device,
        session: vk::OpticalFlowSessionNV,
    ) -> () {
        unsafe {
            self.0.DestroyOpticalFlowSessionNV(
                device.abi(), 
                session.abi(), 
                Default::default(), 
            );
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::nv::optical_flow {
    type Commands = Device;
}

/// Device object
pub trait NvOpticalFlowDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkBindOpticalFlowSessionImageNV(VkDevice device, VkOpticalFlowSessionNV session, VkOpticalFlowSessionBindingPointNV bindingPoint, VkImageView view, VkImageLayout layout)
    /// ```
    unsafe fn bind_optical_flow_session_image(
        &self,
        session: vk::OpticalFlowSessionNV,
        binding_point: OpticalFlowSessionBindingPointNV,
        view: Option<vk::ImageView>,
        layout: ImageLayout,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().bind_optical_flow_session_image(
                self.raw(),
                session,
                binding_point,
                view,
                layout,
            )
        }
    }
    /// ```c
    /// VkResult vkCreateOpticalFlowSessionNV(VkDevice device, VkOpticalFlowSessionCreateInfoNV const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkOpticalFlowSessionNV* pSession)
    /// ```
    unsafe fn create_optical_flow_session(
        &self,
        create_info: &OpticalFlowSessionCreateInfoNV,
    ) -> crate::Result<vk::OpticalFlowSessionNV> {
        unsafe {
            self.commands().create_optical_flow_session(
                self.raw(),
                create_info,
            )
        }
    }
    /// ```c
    /// void vkDestroyOpticalFlowSessionNV(VkDevice device, VkOpticalFlowSessionNV session, VkAllocationCallbacks const* pAllocator)
    /// ```
    unsafe fn destroy_optical_flow_session(
        &self,
        session: vk::OpticalFlowSessionNV,
    ) -> () {
        unsafe {
            self.commands().destroy_optical_flow_session(
                self.raw(),
                session,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::nv::optical_flow {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::nv::optical_flow> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::nv::optical_flow {
        type Output = crate::hnd::Device<vk::extensions::nv::optical_flow>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::nv::optical_flow>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::nv::optical_flow> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::nv::optical_flow> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::nv::optical_flow> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::nv::optical_flow> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::NvOpticalFlowDevice for crate::hnd::Device<vk::extensions::nv::optical_flow> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::nv::optical_flow, vk::Device> for crate::hnd::Device<vk::extensions::nv::optical_flow> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

/// CommandBuffer object
pub trait NvOpticalFlowCommandBuffer {
    fn raw(&self) -> vk::CommandBuffer;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkCmdOpticalFlowExecuteNV(VkCommandBuffer commandBuffer, VkOpticalFlowSessionNV session, VkOpticalFlowExecuteInfoNV const* pExecuteInfo)
    /// ```
    unsafe fn optical_flow_execute(
        &self,
        session: vk::OpticalFlowSessionNV,
        execute_info: &OpticalFlowExecuteInfoNV,
    ) -> () {
        unsafe {
            self.commands().cmd_optical_flow_execute(
                self.raw(),
                session,
                execute_info,
            )
        }
    }
}
