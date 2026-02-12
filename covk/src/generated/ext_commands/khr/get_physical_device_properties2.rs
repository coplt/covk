// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_KHR_get_physical_device_properties2` InstanceCommands
#[derive(Debug, Clone, Copy)]
pub struct Instance(pub sys::khr::get_physical_device_properties2::InstanceCommands);

impl Instance {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::khr::get_physical_device_properties2::InstanceCommands::load(get) })
    }
}

impl Instance {
    /// ```c
    /// void vkGetPhysicalDeviceFeatures2(VkPhysicalDevice physicalDevice, VkPhysicalDeviceFeatures2* pFeatures)
    /// ```
    pub unsafe fn get_physical_device_features2(
        &self,
        physical_device: vk::PhysicalDevice,
        features: &mut PhysicalDeviceFeatures2,
    ) -> () {
        unsafe {
            self.0.GetPhysicalDeviceFeatures2KHR(
                physical_device.abi(), 
                features.abi(), 
            );
        }
    }
    /// ```c
    /// void vkGetPhysicalDeviceFormatProperties2(VkPhysicalDevice physicalDevice, VkFormat format, VkFormatProperties2* pFormatProperties)
    /// ```
    pub unsafe fn get_physical_device_format_properties2(
        &self,
        physical_device: vk::PhysicalDevice,
        format: Format,
        format_properties: &mut FormatProperties2,
    ) -> () {
        unsafe {
            self.0.GetPhysicalDeviceFormatProperties2KHR(
                physical_device.abi(), 
                format.abi(), 
                format_properties.abi(), 
            );
        }
    }
    /// ```c
    /// VkResult vkGetPhysicalDeviceImageFormatProperties2(VkPhysicalDevice physicalDevice, VkPhysicalDeviceImageFormatInfo2 const* pImageFormatInfo, VkImageFormatProperties2* pImageFormatProperties)
    /// ```
    pub unsafe fn get_physical_device_image_format_properties2(
        &self,
        physical_device: vk::PhysicalDevice,
        image_format_info: &PhysicalDeviceImageFormatInfo2,
        image_format_properties: &mut ImageFormatProperties2,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.GetPhysicalDeviceImageFormatProperties2KHR(
                physical_device.abi(), 
                image_format_info.abi(), 
                image_format_properties.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// void vkGetPhysicalDeviceMemoryProperties2(VkPhysicalDevice physicalDevice, VkPhysicalDeviceMemoryProperties2* pMemoryProperties)
    /// ```
    pub unsafe fn get_physical_device_memory_properties2(
        &self,
        physical_device: vk::PhysicalDevice,
        memory_properties: &mut PhysicalDeviceMemoryProperties2,
    ) -> () {
        unsafe {
            self.0.GetPhysicalDeviceMemoryProperties2KHR(
                physical_device.abi(), 
                memory_properties.abi(), 
            );
        }
    }
    /// ```c
    /// void vkGetPhysicalDeviceProperties2(VkPhysicalDevice physicalDevice, VkPhysicalDeviceProperties2* pProperties)
    /// ```
    pub unsafe fn get_physical_device_properties2(
        &self,
        physical_device: vk::PhysicalDevice,
        properties: &mut PhysicalDeviceProperties2,
    ) -> () {
        unsafe {
            self.0.GetPhysicalDeviceProperties2KHR(
                physical_device.abi(), 
                properties.abi(), 
            );
        }
    }
    /// ```c
    /// void vkGetPhysicalDeviceQueueFamilyProperties2(VkPhysicalDevice physicalDevice, uint32_t* pQueueFamilyPropertyCount, VkQueueFamilyProperties2* pQueueFamilyProperties)
    /// ```
    pub unsafe fn get_physical_device_queue_family_properties2(
        &self,
        physical_device: vk::PhysicalDevice,
        queue_family_properties: Option<&mut ::alloc::vec::Vec<QueueFamilyProperties2>>,
    ) -> uint32_t {
        unsafe {
            let mut _c: uint32_t = Default::default();
            self.0.GetPhysicalDeviceQueueFamilyProperties2KHR(
                physical_device.abi(), 
                &mut _c, 
                Default::default(), 
            );
            if let Some(_b) = queue_family_properties {
                _b.reserve(_c as usize);
                self.0.GetPhysicalDeviceQueueFamilyProperties2KHR(
                    physical_device.abi(), 
                    &mut _c, 
                    (&mut **_b).abi(), 
                );
                _b.set_len(_b.len() + _c as usize);
            }
            _c
        }
    }
    /// ```c
    /// void vkGetPhysicalDeviceSparseImageFormatProperties2(VkPhysicalDevice physicalDevice, VkPhysicalDeviceSparseImageFormatInfo2 const* pFormatInfo, uint32_t* pPropertyCount, VkSparseImageFormatProperties2* pProperties)
    /// ```
    pub unsafe fn get_physical_device_sparse_image_format_properties2(
        &self,
        physical_device: vk::PhysicalDevice,
        format_info: &PhysicalDeviceSparseImageFormatInfo2,
        properties: Option<&mut ::alloc::vec::Vec<SparseImageFormatProperties2>>,
    ) -> uint32_t {
        unsafe {
            let mut _c: uint32_t = Default::default();
            self.0.GetPhysicalDeviceSparseImageFormatProperties2KHR(
                physical_device.abi(), 
                format_info.abi(), 
                &mut _c, 
                Default::default(), 
            );
            if let Some(_b) = properties {
                _b.reserve(_c as usize);
                self.0.GetPhysicalDeviceSparseImageFormatProperties2KHR(
                    physical_device.abi(), 
                    format_info.abi(), 
                    &mut _c, 
                    (&mut **_b).abi(), 
                );
                _b.set_len(_b.len() + _c as usize);
            }
            _c
        }
    }
}

impl crate::CommandScope<vk::Instance> for vk::extensions::khr::get_physical_device_properties2 {
    type Commands = Instance;
}

/// Instance object
pub trait KhrGetPhysicalDeviceProperties2Instance {
    fn raw(&self) -> vk::Instance;
    fn commands(&self) -> &Instance;

}

impl crate::HndScope<vk::Instance> for vk::extensions::khr::get_physical_device_properties2 {
    type Impl = _hs_Instance::Instance;
}

mod _hs_Instance {
    use super::*;
    #[derive(Debug)]
    pub struct Instance(pub(crate) ::alloc::sync::Arc<super::Instance>, pub(crate) crate::hnd::Instance<vk::core>);

    impl Clone for Instance {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Instance<vk::extensions::khr::get_physical_device_properties2> {
        pub unsafe fn new(base: &crate::hnd::Instance<vk::core>) -> Self {
            unsafe {
                Self(Instance(
                   ::alloc::sync::Arc::new(super::Instance::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Instance<vk::core>> for vk::extensions::khr::get_physical_device_properties2 {
        type Output = crate::hnd::Instance<vk::extensions::khr::get_physical_device_properties2>;
        unsafe fn make(target: &crate::hnd::Instance<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Instance::<vk::extensions::khr::get_physical_device_properties2>::new(target) }
        }
    }

    impl crate::hnd::Instance<vk::extensions::khr::get_physical_device_properties2> {
        pub fn raw(&self) -> vk::Instance { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Instance<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Instance<vk::extensions::khr::get_physical_device_properties2> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Instance({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Instance<vk::extensions::khr::get_physical_device_properties2> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Instance<vk::extensions::khr::get_physical_device_properties2> {
        type Target = super::Instance;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::KhrGetPhysicalDeviceProperties2Instance for crate::hnd::Instance<vk::extensions::khr::get_physical_device_properties2> {
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &super::Instance { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::khr::get_physical_device_properties2, vk::Instance> for crate::hnd::Instance<vk::extensions::khr::get_physical_device_properties2> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { self.commands() }
    }
}

/// PhysicalDevice object
pub trait KhrGetPhysicalDeviceProperties2PhysicalDevice {
    fn raw(&self) -> vk::PhysicalDevice;
    fn commands(&self) -> &Instance;

    /// ```c
    /// void vkGetPhysicalDeviceFeatures2(VkPhysicalDevice physicalDevice, VkPhysicalDeviceFeatures2* pFeatures)
    /// ```
    unsafe fn get_features2(
        &self,
        features: &mut PhysicalDeviceFeatures2,
    ) -> () {
        unsafe {
            self.commands().get_physical_device_features2(
                self.raw(),
                features,
            )
        }
    }
    /// ```c
    /// void vkGetPhysicalDeviceFormatProperties2(VkPhysicalDevice physicalDevice, VkFormat format, VkFormatProperties2* pFormatProperties)
    /// ```
    unsafe fn get_format_properties2(
        &self,
        format: Format,
        format_properties: &mut FormatProperties2,
    ) -> () {
        unsafe {
            self.commands().get_physical_device_format_properties2(
                self.raw(),
                format,
                format_properties,
            )
        }
    }
    /// ```c
    /// VkResult vkGetPhysicalDeviceImageFormatProperties2(VkPhysicalDevice physicalDevice, VkPhysicalDeviceImageFormatInfo2 const* pImageFormatInfo, VkImageFormatProperties2* pImageFormatProperties)
    /// ```
    unsafe fn get_image_format_properties2(
        &self,
        image_format_info: &PhysicalDeviceImageFormatInfo2,
        image_format_properties: &mut ImageFormatProperties2,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().get_physical_device_image_format_properties2(
                self.raw(),
                image_format_info,
                image_format_properties,
            )
        }
    }
    /// ```c
    /// void vkGetPhysicalDeviceMemoryProperties2(VkPhysicalDevice physicalDevice, VkPhysicalDeviceMemoryProperties2* pMemoryProperties)
    /// ```
    unsafe fn get_memory_properties2(
        &self,
        memory_properties: &mut PhysicalDeviceMemoryProperties2,
    ) -> () {
        unsafe {
            self.commands().get_physical_device_memory_properties2(
                self.raw(),
                memory_properties,
            )
        }
    }
    /// ```c
    /// void vkGetPhysicalDeviceProperties2(VkPhysicalDevice physicalDevice, VkPhysicalDeviceProperties2* pProperties)
    /// ```
    unsafe fn get_properties2(
        &self,
        properties: &mut PhysicalDeviceProperties2,
    ) -> () {
        unsafe {
            self.commands().get_physical_device_properties2(
                self.raw(),
                properties,
            )
        }
    }
    /// ```c
    /// void vkGetPhysicalDeviceQueueFamilyProperties2(VkPhysicalDevice physicalDevice, uint32_t* pQueueFamilyPropertyCount, VkQueueFamilyProperties2* pQueueFamilyProperties)
    /// ```
    unsafe fn get_queue_family_properties2(
        &self,
        queue_family_properties: Option<&mut ::alloc::vec::Vec<QueueFamilyProperties2>>,
    ) -> uint32_t {
        unsafe {
            self.commands().get_physical_device_queue_family_properties2(
                self.raw(),
                queue_family_properties,
            )
        }
    }
    /// ```c
    /// void vkGetPhysicalDeviceSparseImageFormatProperties2(VkPhysicalDevice physicalDevice, VkPhysicalDeviceSparseImageFormatInfo2 const* pFormatInfo, uint32_t* pPropertyCount, VkSparseImageFormatProperties2* pProperties)
    /// ```
    unsafe fn get_sparse_image_format_properties2(
        &self,
        format_info: &PhysicalDeviceSparseImageFormatInfo2,
        properties: Option<&mut ::alloc::vec::Vec<SparseImageFormatProperties2>>,
    ) -> uint32_t {
        unsafe {
            self.commands().get_physical_device_sparse_image_format_properties2(
                self.raw(),
                format_info,
                properties,
            )
        }
    }
}
