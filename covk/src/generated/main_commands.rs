// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
#![allow(ambiguous_glob_reexports)]

use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};
pub use super::ext_commands::*;

/// Instance commands
#[derive(Debug, Clone, Copy)]
pub struct Instance {
     pub _1_0: sys::vtbl::InstanceCommands_1_0, 
     pub _1_1: sys::vtbl::InstanceCommands_1_1, 
     pub _1_2: sys::vtbl::InstanceCommands_1_2, 
     pub _1_3: sys::vtbl::InstanceCommands_1_3, 
     pub _1_4: sys::vtbl::InstanceCommands_1_4, 
}

impl Instance {
    pub fn load(mut get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self {
            _1_0: unsafe { sys::vtbl::InstanceCommands_1_0::load(&mut get) },
            _1_1: unsafe { sys::vtbl::InstanceCommands_1_1::load(&mut get) },
            _1_2: unsafe { sys::vtbl::InstanceCommands_1_2::load(&mut get) },
            _1_3: unsafe { sys::vtbl::InstanceCommands_1_3::load(&mut get) },
            _1_4: unsafe { sys::vtbl::InstanceCommands_1_4::load(&mut get) },
        }
    }
}

/// `Vulkan 1.0` InstanceCommands
impl Instance {
    /// ```c
    /// VkResult vkCreateDevice(VkPhysicalDevice physicalDevice, VkDeviceCreateInfo const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkDevice* pDevice)
    /// ```
    pub unsafe fn create_device(
        &self,
        physical_device: vk::PhysicalDevice,
        create_info: &DeviceCreateInfo,
    ) -> crate::Result<vk::Device> {
        unsafe {
            let mut _v: Option<vk::Device> = Default::default();
            let _r = self._1_0.CreateDevice(
                physical_device.abi(), 
                create_info.abi(), 
                Default::default(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| _v)
        }
    }
    /// ```c
    /// void vkDestroyInstance(VkInstance instance, VkAllocationCallbacks const* pAllocator)
    /// ```
    pub unsafe fn destroy_instance(
        &self,
        instance: Option<vk::Instance>,
    ) -> () {
        unsafe {
            self._1_0.DestroyInstance(
                instance.abi(), 
                Default::default(), 
            );
        }
    }
    /// ```c
    /// VkResult vkEnumerateDeviceExtensionProperties(VkPhysicalDevice physicalDevice, char const* pLayerName, uint32_t* pPropertyCount, VkExtensionProperties* pProperties)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    pub unsafe fn enumerate_device_extension_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        layer_name: Option<&::core::ffi::CStr>,
        properties: Option<&mut ::alloc::vec::Vec<ExtensionProperties>>,
    ) -> crate::Result<(uint32_t, Result)> {
        unsafe {
            let mut _c: uint32_t = Default::default();
            let mut _r = self._1_0.EnumerateDeviceExtensionProperties(
                physical_device.abi(), 
                layer_name.abi(), 
                &mut _c, 
                Default::default(), 
            ).vk();
            if let Some(_b) = properties {
                _r.result(|| Some(()))?;
                _b.reserve(_c as usize);
                _r = self._1_0.EnumerateDeviceExtensionProperties(
                    physical_device.abi(), 
                    layer_name.abi(), 
                    &mut _c, 
                    (&mut **_b).abi(), 
                ).vk();
                _r.result(|| Some(()))?;
                _b.set_len(_b.len() + _c as usize);
            }
            _r.result_multi(|| Some(_c))
        }
    }
    /// ```c
    /// VkResult vkEnumerateDeviceLayerProperties(VkPhysicalDevice physicalDevice, uint32_t* pPropertyCount, VkLayerProperties* pProperties)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    pub unsafe fn enumerate_device_layer_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        properties: Option<&mut ::alloc::vec::Vec<LayerProperties>>,
    ) -> crate::Result<(uint32_t, Result)> {
        unsafe {
            let mut _c: uint32_t = Default::default();
            let mut _r = self._1_0.EnumerateDeviceLayerProperties(
                physical_device.abi(), 
                &mut _c, 
                Default::default(), 
            ).vk();
            if let Some(_b) = properties {
                _r.result(|| Some(()))?;
                _b.reserve(_c as usize);
                _r = self._1_0.EnumerateDeviceLayerProperties(
                    physical_device.abi(), 
                    &mut _c, 
                    (&mut **_b).abi(), 
                ).vk();
                _r.result(|| Some(()))?;
                _b.set_len(_b.len() + _c as usize);
            }
            _r.result_multi(|| Some(_c))
        }
    }
    /// ```c
    /// VkResult vkEnumeratePhysicalDevices(VkInstance instance, uint32_t* pPhysicalDeviceCount, VkPhysicalDevice* pPhysicalDevices)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    pub unsafe fn enumerate_physical_devices(
        &self,
        instance: vk::Instance,
        physical_devices: Option<&mut ::alloc::vec::Vec<Option<vk::PhysicalDevice>>>,
    ) -> crate::Result<(uint32_t, Result)> {
        unsafe {
            let mut _c: uint32_t = Default::default();
            let mut _r = self._1_0.EnumeratePhysicalDevices(
                instance.abi(), 
                &mut _c, 
                Default::default(), 
            ).vk();
            if let Some(_b) = physical_devices {
                _r.result(|| Some(()))?;
                _b.reserve(_c as usize);
                _r = self._1_0.EnumeratePhysicalDevices(
                    instance.abi(), 
                    &mut _c, 
                    (&mut **_b).abi(), 
                ).vk();
                _r.result(|| Some(()))?;
                _b.set_len(_b.len() + _c as usize);
            }
            _r.result_multi(|| Some(_c))
        }
    }
    /// ```c
    /// PFN_vkVoidFunction vkGetDeviceProcAddr(VkDevice device, char const* pName)
    /// ```
    pub unsafe fn get_device_proc_addr(
        &self,
        device: vk::Device,
        name: &::core::ffi::CStr,
    ) -> Option<crate::ProcAddr> {
        unsafe {
            let _r = self._1_0.GetDeviceProcAddr(
                device.abi(), 
                name.abi(), 
            );
            _r
        }
    }
    /// ```c
    /// PFN_vkVoidFunction vkGetInstanceProcAddr(VkInstance instance, char const* pName)
    /// ```
    pub unsafe fn get_instance_proc_addr(
        &self,
        instance: Option<vk::Instance>,
        name: &::core::ffi::CStr,
    ) -> Option<crate::ProcAddr> {
        unsafe {
            let _r = self._1_0.GetInstanceProcAddr(
                instance.abi(), 
                name.abi(), 
            );
            _r
        }
    }
    /// ```c
    /// void vkGetPhysicalDeviceFeatures(VkPhysicalDevice physicalDevice, VkPhysicalDeviceFeatures* pFeatures)
    /// ```
    pub unsafe fn get_physical_device_features(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> PhysicalDeviceFeatures {
        unsafe {
            let mut _v: PhysicalDeviceFeatures = Default::default();
            self._1_0.GetPhysicalDeviceFeatures(
                physical_device.abi(), 
                (&mut _v).abi(), 
            );
            _v
        }
    }
    /// ```c
    /// void vkGetPhysicalDeviceFormatProperties(VkPhysicalDevice physicalDevice, VkFormat format, VkFormatProperties* pFormatProperties)
    /// ```
    pub unsafe fn get_physical_device_format_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        format: Format,
    ) -> FormatProperties {
        unsafe {
            let mut _v: FormatProperties = Default::default();
            self._1_0.GetPhysicalDeviceFormatProperties(
                physical_device.abi(), 
                format.abi(), 
                (&mut _v).abi(), 
            );
            _v
        }
    }
    /// ```c
    /// VkResult vkGetPhysicalDeviceImageFormatProperties(VkPhysicalDevice physicalDevice, VkFormat format, VkImageType type, VkImageTiling tiling, VkImageUsageFlags usage, VkImageCreateFlags flags, VkImageFormatProperties* pImageFormatProperties)
    /// ```
    pub unsafe fn get_physical_device_image_format_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        format: Format,
        typ: ImageType,
        tiling: ImageTiling,
        usage: ImageUsageFlags,
        flags: ImageCreateFlags,
    ) -> crate::Result<ImageFormatProperties> {
        unsafe {
            let mut _v: ImageFormatProperties = Default::default();
            let _r = self._1_0.GetPhysicalDeviceImageFormatProperties(
                physical_device.abi(), 
                format.abi(), 
                typ.abi(), 
                tiling.abi(), 
                usage.abi(), 
                flags.abi(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| Some(_v))
        }
    }
    /// ```c
    /// void vkGetPhysicalDeviceMemoryProperties(VkPhysicalDevice physicalDevice, VkPhysicalDeviceMemoryProperties* pMemoryProperties)
    /// ```
    pub unsafe fn get_physical_device_memory_properties(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> PhysicalDeviceMemoryProperties {
        unsafe {
            let mut _v: PhysicalDeviceMemoryProperties = Default::default();
            self._1_0.GetPhysicalDeviceMemoryProperties(
                physical_device.abi(), 
                (&mut _v).abi(), 
            );
            _v
        }
    }
    /// ```c
    /// void vkGetPhysicalDeviceProperties(VkPhysicalDevice physicalDevice, VkPhysicalDeviceProperties* pProperties)
    /// ```
    pub unsafe fn get_physical_device_properties(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> PhysicalDeviceProperties {
        unsafe {
            let mut _v: PhysicalDeviceProperties = Default::default();
            self._1_0.GetPhysicalDeviceProperties(
                physical_device.abi(), 
                (&mut _v).abi(), 
            );
            _v
        }
    }
    /// ```c
    /// void vkGetPhysicalDeviceQueueFamilyProperties(VkPhysicalDevice physicalDevice, uint32_t* pQueueFamilyPropertyCount, VkQueueFamilyProperties* pQueueFamilyProperties)
    /// ```
    pub unsafe fn get_physical_device_queue_family_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        queue_family_properties: Option<&mut ::alloc::vec::Vec<QueueFamilyProperties>>,
    ) -> uint32_t {
        unsafe {
            let mut _c: uint32_t = Default::default();
            self._1_0.GetPhysicalDeviceQueueFamilyProperties(
                physical_device.abi(), 
                &mut _c, 
                Default::default(), 
            );
            if let Some(_b) = queue_family_properties {
                _b.reserve(_c as usize);
                self._1_0.GetPhysicalDeviceQueueFamilyProperties(
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
    /// void vkGetPhysicalDeviceSparseImageFormatProperties(VkPhysicalDevice physicalDevice, VkFormat format, VkImageType type, VkSampleCountFlagBits samples, VkImageUsageFlags usage, VkImageTiling tiling, uint32_t* pPropertyCount, VkSparseImageFormatProperties* pProperties)
    /// ```
    pub unsafe fn get_physical_device_sparse_image_format_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        format: Format,
        typ: ImageType,
        samples: SampleCountFlags,
        usage: ImageUsageFlags,
        tiling: ImageTiling,
        properties: Option<&mut ::alloc::vec::Vec<SparseImageFormatProperties>>,
    ) -> uint32_t {
        unsafe {
            let mut _c: uint32_t = Default::default();
            self._1_0.GetPhysicalDeviceSparseImageFormatProperties(
                physical_device.abi(), 
                format.abi(), 
                typ.abi(), 
                samples.abi(), 
                usage.abi(), 
                tiling.abi(), 
                &mut _c, 
                Default::default(), 
            );
            if let Some(_b) = properties {
                _b.reserve(_c as usize);
                self._1_0.GetPhysicalDeviceSparseImageFormatProperties(
                    physical_device.abi(), 
                    format.abi(), 
                    typ.abi(), 
                    samples.abi(), 
                    usage.abi(), 
                    tiling.abi(), 
                    &mut _c, 
                    (&mut **_b).abi(), 
                );
                _b.set_len(_b.len() + _c as usize);
            }
            _c
        }
    }
}

/// `Vulkan 1.1` InstanceCommands
impl Instance {
    /// ```c
    /// VkResult vkEnumeratePhysicalDeviceGroups(VkInstance instance, uint32_t* pPhysicalDeviceGroupCount, VkPhysicalDeviceGroupProperties* pPhysicalDeviceGroupProperties)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    pub unsafe fn enumerate_physical_device_groups(
        &self,
        instance: vk::Instance,
        physical_device_group_properties: Option<&mut ::alloc::vec::Vec<PhysicalDeviceGroupProperties>>,
    ) -> crate::Result<(uint32_t, Result)> {
        unsafe {
            let mut _c: uint32_t = Default::default();
            let mut _r = self._1_1.EnumeratePhysicalDeviceGroups(
                instance.abi(), 
                &mut _c, 
                Default::default(), 
            ).vk();
            if let Some(_b) = physical_device_group_properties {
                _r.result(|| Some(()))?;
                _b.reserve(_c as usize);
                _r = self._1_1.EnumeratePhysicalDeviceGroups(
                    instance.abi(), 
                    &mut _c, 
                    (&mut **_b).abi(), 
                ).vk();
                _r.result(|| Some(()))?;
                _b.set_len(_b.len() + _c as usize);
            }
            _r.result_multi(|| Some(_c))
        }
    }
    /// ```c
    /// void vkGetPhysicalDeviceExternalBufferProperties(VkPhysicalDevice physicalDevice, VkPhysicalDeviceExternalBufferInfo const* pExternalBufferInfo, VkExternalBufferProperties* pExternalBufferProperties)
    /// ```
    pub unsafe fn get_physical_device_external_buffer_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        external_buffer_info: &PhysicalDeviceExternalBufferInfo,
        external_buffer_properties: &mut ExternalBufferProperties,
    ) -> () {
        unsafe {
            self._1_1.GetPhysicalDeviceExternalBufferProperties(
                physical_device.abi(), 
                external_buffer_info.abi(), 
                external_buffer_properties.abi(), 
            );
        }
    }
    /// ```c
    /// void vkGetPhysicalDeviceExternalFenceProperties(VkPhysicalDevice physicalDevice, VkPhysicalDeviceExternalFenceInfo const* pExternalFenceInfo, VkExternalFenceProperties* pExternalFenceProperties)
    /// ```
    pub unsafe fn get_physical_device_external_fence_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        external_fence_info: &PhysicalDeviceExternalFenceInfo,
        external_fence_properties: &mut ExternalFenceProperties,
    ) -> () {
        unsafe {
            self._1_1.GetPhysicalDeviceExternalFenceProperties(
                physical_device.abi(), 
                external_fence_info.abi(), 
                external_fence_properties.abi(), 
            );
        }
    }
    /// ```c
    /// void vkGetPhysicalDeviceExternalSemaphoreProperties(VkPhysicalDevice physicalDevice, VkPhysicalDeviceExternalSemaphoreInfo const* pExternalSemaphoreInfo, VkExternalSemaphoreProperties* pExternalSemaphoreProperties)
    /// ```
    pub unsafe fn get_physical_device_external_semaphore_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        external_semaphore_info: &PhysicalDeviceExternalSemaphoreInfo,
        external_semaphore_properties: &mut ExternalSemaphoreProperties,
    ) -> () {
        unsafe {
            self._1_1.GetPhysicalDeviceExternalSemaphoreProperties(
                physical_device.abi(), 
                external_semaphore_info.abi(), 
                external_semaphore_properties.abi(), 
            );
        }
    }
    /// ```c
    /// void vkGetPhysicalDeviceFeatures2(VkPhysicalDevice physicalDevice, VkPhysicalDeviceFeatures2* pFeatures)
    /// ```
    pub unsafe fn get_physical_device_features2(
        &self,
        physical_device: vk::PhysicalDevice,
        features: &mut PhysicalDeviceFeatures2,
    ) -> () {
        unsafe {
            self._1_1.GetPhysicalDeviceFeatures2(
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
            self._1_1.GetPhysicalDeviceFormatProperties2(
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
            let _r = self._1_1.GetPhysicalDeviceImageFormatProperties2(
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
            self._1_1.GetPhysicalDeviceMemoryProperties2(
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
            self._1_1.GetPhysicalDeviceProperties2(
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
            self._1_1.GetPhysicalDeviceQueueFamilyProperties2(
                physical_device.abi(), 
                &mut _c, 
                Default::default(), 
            );
            if let Some(_b) = queue_family_properties {
                _b.reserve(_c as usize);
                self._1_1.GetPhysicalDeviceQueueFamilyProperties2(
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
            self._1_1.GetPhysicalDeviceSparseImageFormatProperties2(
                physical_device.abi(), 
                format_info.abi(), 
                &mut _c, 
                Default::default(), 
            );
            if let Some(_b) = properties {
                _b.reserve(_c as usize);
                self._1_1.GetPhysicalDeviceSparseImageFormatProperties2(
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

/// `Vulkan 1.2` InstanceCommands
impl Instance {
}

/// `Vulkan 1.3` InstanceCommands
impl Instance {
    /// ```c
    /// VkResult vkGetPhysicalDeviceToolProperties(VkPhysicalDevice physicalDevice, uint32_t* pToolCount, VkPhysicalDeviceToolProperties* pToolProperties)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    pub unsafe fn get_physical_device_tool_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        tool_properties: Option<&mut ::alloc::vec::Vec<PhysicalDeviceToolProperties>>,
    ) -> crate::Result<(uint32_t, Result)> {
        unsafe {
            let mut _c: uint32_t = Default::default();
            let mut _r = self._1_3.GetPhysicalDeviceToolProperties(
                physical_device.abi(), 
                &mut _c, 
                Default::default(), 
            ).vk();
            if let Some(_b) = tool_properties {
                _r.result(|| Some(()))?;
                _b.reserve(_c as usize);
                _r = self._1_3.GetPhysicalDeviceToolProperties(
                    physical_device.abi(), 
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

/// `Vulkan 1.4` InstanceCommands
impl Instance {
}

/// Instance object
pub trait CoreInstance {
    fn raw(&self) -> vk::Instance;
    fn commands(&self) -> &Instance;

    /// ```c
    /// VkResult vkEnumeratePhysicalDevices(VkInstance instance, uint32_t* pPhysicalDeviceCount, VkPhysicalDevice* pPhysicalDevices)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    unsafe fn enumerate_physical_devices(
        &self,
        physical_devices: Option<&mut ::alloc::vec::Vec<Option<vk::PhysicalDevice>>>,
    ) -> crate::Result<(uint32_t, Result)> {
        unsafe {
            self.commands().enumerate_physical_devices(
                self.raw(),
                physical_devices,
            )
        }
    }
    /// ```c
    /// VkResult vkEnumeratePhysicalDeviceGroups(VkInstance instance, uint32_t* pPhysicalDeviceGroupCount, VkPhysicalDeviceGroupProperties* pPhysicalDeviceGroupProperties)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    unsafe fn enumerate_physical_device_groups(
        &self,
        physical_device_group_properties: Option<&mut ::alloc::vec::Vec<PhysicalDeviceGroupProperties>>,
    ) -> crate::Result<(uint32_t, Result)> {
        unsafe {
            self.commands().enumerate_physical_device_groups(
                self.raw(),
                physical_device_group_properties,
            )
        }
    }
}

impl crate::Owner<vk::Instance, vk::core> for Instance {
    fn drop(&mut self, raw: vk::Instance) {
        unsafe {
            self.destroy_instance(Some(raw))
        }
    }
}

impl crate::Owner<vk::Instance, vk::core> for ::alloc::sync::Arc<Instance> {
    fn drop(&mut self, raw: vk::Instance) {
        unsafe {
            self.destroy_instance(Some(raw))
        }
    }
}

impl CoreInstance for crate::Unique<vk::Instance, Instance, vk::core> {
    fn raw(&self) -> vk::Instance { self.raw }
    fn commands(&self) -> &Instance { &self.owner }
}

impl CoreInstance for crate::Unique<vk::Instance, ::alloc::sync::Arc<Instance>, vk::core> {
    fn raw(&self) -> vk::Instance { self.raw }
    fn commands(&self) -> &Instance { &self.owner }
}

impl crate::HndCtx<vk::core, vk::Instance> for ::alloc::sync::Arc<crate::Unique<vk::Instance, ::alloc::sync::Arc<Instance>, vk::core>> {
    type Ctx = Self;
    fn ctx(&self) -> Self::Ctx { self.clone() }
    fn raw(&self) -> vk::Instance { self.raw }
    fn commands(&self) -> &::alloc::sync::Arc<Instance> { &self.owner }
}

impl crate::HndScope<vk::Instance> for vk::core {
    type Impl = _hs_Instance::Instance;
}

mod _hs_Instance {
    use super::*;
    #[repr(transparent)]
    #[derive(Debug)]
    pub struct Instance(pub(crate) crate::handle::Hnd<vk::Instance, ::alloc::sync::Arc<super::Instance>>);

    impl Clone for Instance {
        fn clone(&self) -> Self { Self(self.0.clone()) }
    }

    #[derive(Debug, Clone, Copy)]
    struct Inst;
    impl<Dep> crate::handle::HndInst<vk::Instance, ::alloc::sync::Arc<super::Instance>, Dep> for Inst {
        fn drop(this: &mut crate::handle::HndData<vk::Instance, ::alloc::sync::Arc<super::Instance>, Dep>) {
            unsafe {
                this.scope.destroy_instance(Some(this.raw))
            }
        }
    }

    impl crate::hnd::Instance<vk::core> {
        pub unsafe fn new(ctx: &crate::Vulkan, raw: vk::Instance) -> Self {
            unsafe { Self::new_with(ctx, raw, || ()) }
        }
        pub unsafe fn new_with<Dep: Send + Sync + 'static>(ctx: &crate::Vulkan, raw: vk::Instance, dep: impl FnOnce() -> Dep) -> Self {
            Self(Instance(crate::handle::Hnd::new(
                ::alloc::sync::Arc::new(super::Instance::load(|name| unsafe { ctx.get_instance_proc_addr(Some(raw), name) })),
                raw,
                (ctx.clone(), dep()),
                Inst,
            )))
        }
    }

    impl crate::MakeHnd<crate::Vulkan, vk::core> for vk::Instance {
        type Output = crate::hnd::Instance<vk::core>;
        unsafe fn hnd_with<Dep: Send + Sync + 'static>(self, ctx: &crate::Vulkan, dep: impl FnOnce() -> Dep) -> Self::Output {
            unsafe { crate::hnd::Instance::new_with(ctx, self, dep) }
        }
    }

    impl crate::hnd::Instance<vk::core> {
        pub fn raw(&self) -> vk::Instance { self.0.0.raw }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { &self.0.0.scope }
        pub fn get_proc_addr(&self, name: &::core::ffi::CStr) -> Option<crate::ProcAddr> {
            unsafe { self.commands().get_instance_proc_addr(Some(self.raw()), name) }
        }
    }

    impl ::core::fmt::Debug for crate::hnd::Instance<vk::core> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Instance({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Instance<vk::core> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Instance<vk::core> {
        type Target = super::Instance;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::CoreInstance for crate::hnd::Instance<vk::core> {
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &super::Instance { self.commands() }
    }

    impl crate::HndCtx<vk::core, vk::Instance> for crate::hnd::Instance<vk::core> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { self.commands() }
    }
}

/// PhysicalDevice object
pub trait CorePhysicalDevice {
    fn raw(&self) -> vk::PhysicalDevice;
    fn commands(&self) -> &Instance;

    /// ```c
    /// VkResult vkCreateDevice(VkPhysicalDevice physicalDevice, VkDeviceCreateInfo const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkDevice* pDevice)
    /// ```
    unsafe fn create_device(
        &self,
        create_info: &DeviceCreateInfo,
    ) -> crate::Result<vk::Device> {
        unsafe {
            self.commands().create_device(
                self.raw(),
                create_info,
            )
        }
    }
    /// ```c
    /// VkResult vkEnumerateDeviceExtensionProperties(VkPhysicalDevice physicalDevice, char const* pLayerName, uint32_t* pPropertyCount, VkExtensionProperties* pProperties)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    unsafe fn enumerate_device_extension_properties(
        &self,
        layer_name: Option<&::core::ffi::CStr>,
        properties: Option<&mut ::alloc::vec::Vec<ExtensionProperties>>,
    ) -> crate::Result<(uint32_t, Result)> {
        unsafe {
            self.commands().enumerate_device_extension_properties(
                self.raw(),
                layer_name,
                properties,
            )
        }
    }
    /// ```c
    /// VkResult vkEnumerateDeviceLayerProperties(VkPhysicalDevice physicalDevice, uint32_t* pPropertyCount, VkLayerProperties* pProperties)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    unsafe fn enumerate_device_layer_properties(
        &self,
        properties: Option<&mut ::alloc::vec::Vec<LayerProperties>>,
    ) -> crate::Result<(uint32_t, Result)> {
        unsafe {
            self.commands().enumerate_device_layer_properties(
                self.raw(),
                properties,
            )
        }
    }
    /// ```c
    /// void vkGetPhysicalDeviceExternalBufferProperties(VkPhysicalDevice physicalDevice, VkPhysicalDeviceExternalBufferInfo const* pExternalBufferInfo, VkExternalBufferProperties* pExternalBufferProperties)
    /// ```
    unsafe fn get_external_buffer_properties(
        &self,
        external_buffer_info: &PhysicalDeviceExternalBufferInfo,
        external_buffer_properties: &mut ExternalBufferProperties,
    ) -> () {
        unsafe {
            self.commands().get_physical_device_external_buffer_properties(
                self.raw(),
                external_buffer_info,
                external_buffer_properties,
            )
        }
    }
    /// ```c
    /// void vkGetPhysicalDeviceExternalFenceProperties(VkPhysicalDevice physicalDevice, VkPhysicalDeviceExternalFenceInfo const* pExternalFenceInfo, VkExternalFenceProperties* pExternalFenceProperties)
    /// ```
    unsafe fn get_external_fence_properties(
        &self,
        external_fence_info: &PhysicalDeviceExternalFenceInfo,
        external_fence_properties: &mut ExternalFenceProperties,
    ) -> () {
        unsafe {
            self.commands().get_physical_device_external_fence_properties(
                self.raw(),
                external_fence_info,
                external_fence_properties,
            )
        }
    }
    /// ```c
    /// void vkGetPhysicalDeviceExternalSemaphoreProperties(VkPhysicalDevice physicalDevice, VkPhysicalDeviceExternalSemaphoreInfo const* pExternalSemaphoreInfo, VkExternalSemaphoreProperties* pExternalSemaphoreProperties)
    /// ```
    unsafe fn get_external_semaphore_properties(
        &self,
        external_semaphore_info: &PhysicalDeviceExternalSemaphoreInfo,
        external_semaphore_properties: &mut ExternalSemaphoreProperties,
    ) -> () {
        unsafe {
            self.commands().get_physical_device_external_semaphore_properties(
                self.raw(),
                external_semaphore_info,
                external_semaphore_properties,
            )
        }
    }
    /// ```c
    /// void vkGetPhysicalDeviceFeatures(VkPhysicalDevice physicalDevice, VkPhysicalDeviceFeatures* pFeatures)
    /// ```
    unsafe fn get_features(
        &self,
    ) -> PhysicalDeviceFeatures {
        unsafe {
            self.commands().get_physical_device_features(
                self.raw(),
            )
        }
    }
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
    /// void vkGetPhysicalDeviceFormatProperties(VkPhysicalDevice physicalDevice, VkFormat format, VkFormatProperties* pFormatProperties)
    /// ```
    unsafe fn get_format_properties(
        &self,
        format: Format,
    ) -> FormatProperties {
        unsafe {
            self.commands().get_physical_device_format_properties(
                self.raw(),
                format,
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
    /// VkResult vkGetPhysicalDeviceImageFormatProperties(VkPhysicalDevice physicalDevice, VkFormat format, VkImageType type, VkImageTiling tiling, VkImageUsageFlags usage, VkImageCreateFlags flags, VkImageFormatProperties* pImageFormatProperties)
    /// ```
    unsafe fn get_image_format_properties(
        &self,
        format: Format,
        typ: ImageType,
        tiling: ImageTiling,
        usage: ImageUsageFlags,
        flags: ImageCreateFlags,
    ) -> crate::Result<ImageFormatProperties> {
        unsafe {
            self.commands().get_physical_device_image_format_properties(
                self.raw(),
                format,
                typ,
                tiling,
                usage,
                flags,
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
    /// void vkGetPhysicalDeviceMemoryProperties(VkPhysicalDevice physicalDevice, VkPhysicalDeviceMemoryProperties* pMemoryProperties)
    /// ```
    unsafe fn get_memory_properties(
        &self,
    ) -> PhysicalDeviceMemoryProperties {
        unsafe {
            self.commands().get_physical_device_memory_properties(
                self.raw(),
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
    /// void vkGetPhysicalDeviceProperties(VkPhysicalDevice physicalDevice, VkPhysicalDeviceProperties* pProperties)
    /// ```
    unsafe fn get_properties(
        &self,
    ) -> PhysicalDeviceProperties {
        unsafe {
            self.commands().get_physical_device_properties(
                self.raw(),
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
    /// void vkGetPhysicalDeviceQueueFamilyProperties(VkPhysicalDevice physicalDevice, uint32_t* pQueueFamilyPropertyCount, VkQueueFamilyProperties* pQueueFamilyProperties)
    /// ```
    unsafe fn get_queue_family_properties(
        &self,
        queue_family_properties: Option<&mut ::alloc::vec::Vec<QueueFamilyProperties>>,
    ) -> uint32_t {
        unsafe {
            self.commands().get_physical_device_queue_family_properties(
                self.raw(),
                queue_family_properties,
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
    /// void vkGetPhysicalDeviceSparseImageFormatProperties(VkPhysicalDevice physicalDevice, VkFormat format, VkImageType type, VkSampleCountFlagBits samples, VkImageUsageFlags usage, VkImageTiling tiling, uint32_t* pPropertyCount, VkSparseImageFormatProperties* pProperties)
    /// ```
    unsafe fn get_sparse_image_format_properties(
        &self,
        format: Format,
        typ: ImageType,
        samples: SampleCountFlags,
        usage: ImageUsageFlags,
        tiling: ImageTiling,
        properties: Option<&mut ::alloc::vec::Vec<SparseImageFormatProperties>>,
    ) -> uint32_t {
        unsafe {
            self.commands().get_physical_device_sparse_image_format_properties(
                self.raw(),
                format,
                typ,
                samples,
                usage,
                tiling,
                properties,
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
    /// ```c
    /// VkResult vkGetPhysicalDeviceToolProperties(VkPhysicalDevice physicalDevice, uint32_t* pToolCount, VkPhysicalDeviceToolProperties* pToolProperties)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    unsafe fn get_tool_properties(
        &self,
        tool_properties: Option<&mut ::alloc::vec::Vec<PhysicalDeviceToolProperties>>,
    ) -> crate::Result<(uint32_t, Result)> {
        unsafe {
            self.commands().get_physical_device_tool_properties(
                self.raw(),
                tool_properties,
            )
        }
    }
}

/// Device commands
#[derive(Debug, Clone, Copy)]
pub struct Device {
     pub _1_0: sys::vtbl::DeviceCommands_1_0, 
     pub _1_1: sys::vtbl::DeviceCommands_1_1, 
     pub _1_2: sys::vtbl::DeviceCommands_1_2, 
     pub _1_3: sys::vtbl::DeviceCommands_1_3, 
     pub _1_4: sys::vtbl::DeviceCommands_1_4, 
}

impl Device {
    pub fn load(mut get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self {
            _1_0: unsafe { sys::vtbl::DeviceCommands_1_0::load(&mut get) },
            _1_1: unsafe { sys::vtbl::DeviceCommands_1_1::load(&mut get) },
            _1_2: unsafe { sys::vtbl::DeviceCommands_1_2::load(&mut get) },
            _1_3: unsafe { sys::vtbl::DeviceCommands_1_3::load(&mut get) },
            _1_4: unsafe { sys::vtbl::DeviceCommands_1_4::load(&mut get) },
        }
    }
}

/// `Vulkan 1.0` DeviceCommands
impl Device {
    /// ```c
    /// VkResult vkAllocateCommandBuffers(VkDevice device, VkCommandBufferAllocateInfo const* pAllocateInfo, VkCommandBuffer* pCommandBuffers)
    /// ```
    pub unsafe fn allocate_command_buffers(
        &self,
        device: vk::Device,
        allocate_info: &CommandBufferAllocateInfo,
        command_buffers: &mut [Option<vk::CommandBuffer>],
    ) -> crate::Result<()> {
        unsafe {
            let _r = self._1_0.AllocateCommandBuffers(
                device.abi(), 
                allocate_info.abi(), 
                command_buffers.as_mut_ptr().cast(), 
            ).vk();
            Ok(())
        }
    }
    /// ```c
    /// VkResult vkAllocateDescriptorSets(VkDevice device, VkDescriptorSetAllocateInfo const* pAllocateInfo, VkDescriptorSet* pDescriptorSets)
    /// ```
    pub unsafe fn allocate_descriptor_sets(
        &self,
        device: vk::Device,
        allocate_info: &DescriptorSetAllocateInfo,
        descriptor_sets: &mut [Option<vk::DescriptorSet>],
    ) -> crate::Result<()> {
        unsafe {
            let _r = self._1_0.AllocateDescriptorSets(
                device.abi(), 
                allocate_info.abi(), 
                descriptor_sets.as_mut_ptr().cast(), 
            ).vk();
            Ok(())
        }
    }
    /// ```c
    /// VkResult vkAllocateMemory(VkDevice device, VkMemoryAllocateInfo const* pAllocateInfo, VkAllocationCallbacks const* pAllocator, VkDeviceMemory* pMemory)
    /// ```
    pub unsafe fn allocate_memory(
        &self,
        device: vk::Device,
        allocate_info: &MemoryAllocateInfo,
    ) -> crate::Result<vk::DeviceMemory> {
        unsafe {
            let mut _v: Option<vk::DeviceMemory> = Default::default();
            let _r = self._1_0.AllocateMemory(
                device.abi(), 
                allocate_info.abi(), 
                Default::default(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| _v)
        }
    }
    /// ```c
    /// VkResult vkBeginCommandBuffer(VkCommandBuffer commandBuffer, VkCommandBufferBeginInfo const* pBeginInfo)
    /// ```
    pub unsafe fn begin_command_buffer(
        &self,
        command_buffer: vk::CommandBuffer,
        begin_info: &CommandBufferBeginInfo,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self._1_0.BeginCommandBuffer(
                command_buffer.abi(), 
                begin_info.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkResult vkBindBufferMemory(VkDevice device, VkBuffer buffer, VkDeviceMemory memory, VkDeviceSize memoryOffset)
    /// ```
    pub unsafe fn bind_buffer_memory(
        &self,
        device: vk::Device,
        buffer: vk::Buffer,
        memory: vk::DeviceMemory,
        memory_offset: DeviceSize,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self._1_0.BindBufferMemory(
                device.abi(), 
                buffer.abi(), 
                memory.abi(), 
                memory_offset.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkResult vkBindImageMemory(VkDevice device, VkImage image, VkDeviceMemory memory, VkDeviceSize memoryOffset)
    /// ```
    pub unsafe fn bind_image_memory(
        &self,
        device: vk::Device,
        image: vk::Image,
        memory: vk::DeviceMemory,
        memory_offset: DeviceSize,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self._1_0.BindImageMemory(
                device.abi(), 
                image.abi(), 
                memory.abi(), 
                memory_offset.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// void vkCmdBeginQuery(VkCommandBuffer commandBuffer, VkQueryPool queryPool, uint32_t query, VkQueryControlFlags flags)
    /// ```
    pub unsafe fn cmd_begin_query(
        &self,
        command_buffer: vk::CommandBuffer,
        query_pool: vk::QueryPool,
        query: uint32_t,
        flags: QueryControlFlags,
    ) -> () {
        unsafe {
            self._1_0.CmdBeginQuery(
                command_buffer.abi(), 
                query_pool.abi(), 
                query.abi(), 
                flags.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdBeginRenderPass(VkCommandBuffer commandBuffer, VkRenderPassBeginInfo const* pRenderPassBegin, VkSubpassContents contents)
    /// ```
    pub unsafe fn cmd_begin_render_pass(
        &self,
        command_buffer: vk::CommandBuffer,
        render_pass_begin: &RenderPassBeginInfo,
        contents: SubpassContents,
    ) -> () {
        unsafe {
            self._1_0.CmdBeginRenderPass(
                command_buffer.abi(), 
                render_pass_begin.abi(), 
                contents.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdBindDescriptorSets(VkCommandBuffer commandBuffer, VkPipelineBindPoint pipelineBindPoint, VkPipelineLayout layout, uint32_t firstSet, uint32_t descriptorSetCount, VkDescriptorSet const* pDescriptorSets, uint32_t dynamicOffsetCount, uint32_t const* pDynamicOffsets)
    /// ```
    pub unsafe fn cmd_bind_descriptor_sets(
        &self,
        command_buffer: vk::CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        layout: vk::PipelineLayout,
        first_set: uint32_t,
        descriptor_sets: &[Option<vk::DescriptorSet>],
        dynamic_offsets: &[uint32_t],
    ) -> () {
        unsafe {
            self._1_0.CmdBindDescriptorSets(
                command_buffer.abi(), 
                pipeline_bind_point.abi(), 
                layout.abi(), 
                first_set.abi(), 
                descriptor_sets.len() as _, 
                descriptor_sets.abi(), 
                dynamic_offsets.len() as _, 
                dynamic_offsets.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdBindIndexBuffer(VkCommandBuffer commandBuffer, VkBuffer buffer, VkDeviceSize offset, VkIndexType indexType)
    /// ```
    pub unsafe fn cmd_bind_index_buffer(
        &self,
        command_buffer: vk::CommandBuffer,
        buffer: Option<vk::Buffer>,
        offset: DeviceSize,
        index_type: IndexType,
    ) -> () {
        unsafe {
            self._1_0.CmdBindIndexBuffer(
                command_buffer.abi(), 
                buffer.abi(), 
                offset.abi(), 
                index_type.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdBindPipeline(VkCommandBuffer commandBuffer, VkPipelineBindPoint pipelineBindPoint, VkPipeline pipeline)
    /// ```
    pub unsafe fn cmd_bind_pipeline(
        &self,
        command_buffer: vk::CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        pipeline: vk::Pipeline,
    ) -> () {
        unsafe {
            self._1_0.CmdBindPipeline(
                command_buffer.abi(), 
                pipeline_bind_point.abi(), 
                pipeline.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdBindVertexBuffers(VkCommandBuffer commandBuffer, uint32_t firstBinding, uint32_t bindingCount, VkBuffer const* pBuffers, VkDeviceSize const* pOffsets)
    /// ```
    pub unsafe fn cmd_bind_vertex_buffers(
        &self,
        command_buffer: vk::CommandBuffer,
        first_binding: uint32_t,
        buffers: &[Option<vk::Buffer>],
        offsets: &[DeviceSize],
    ) -> () {
        unsafe {
            self._1_0.CmdBindVertexBuffers(
                command_buffer.abi(), 
                first_binding.abi(), 
                buffers.len() as _, 
                buffers.abi(), 
                offsets.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdBlitImage(VkCommandBuffer commandBuffer, VkImage srcImage, VkImageLayout srcImageLayout, VkImage dstImage, VkImageLayout dstImageLayout, uint32_t regionCount, VkImageBlit const* pRegions, VkFilter filter)
    /// ```
    pub unsafe fn cmd_blit_image(
        &self,
        command_buffer: vk::CommandBuffer,
        src_image: vk::Image,
        src_image_layout: ImageLayout,
        dst_image: vk::Image,
        dst_image_layout: ImageLayout,
        regions: &[ImageBlit],
        filter: Filter,
    ) -> () {
        unsafe {
            self._1_0.CmdBlitImage(
                command_buffer.abi(), 
                src_image.abi(), 
                src_image_layout.abi(), 
                dst_image.abi(), 
                dst_image_layout.abi(), 
                regions.len() as _, 
                regions.abi(), 
                filter.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdClearAttachments(VkCommandBuffer commandBuffer, uint32_t attachmentCount, VkClearAttachment const* pAttachments, uint32_t rectCount, VkClearRect const* pRects)
    /// ```
    pub unsafe fn cmd_clear_attachments(
        &self,
        command_buffer: vk::CommandBuffer,
        attachments: &[ClearAttachment],
        rects: &[ClearRect],
    ) -> () {
        unsafe {
            self._1_0.CmdClearAttachments(
                command_buffer.abi(), 
                attachments.len() as _, 
                attachments.abi(), 
                rects.len() as _, 
                rects.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdClearColorImage(VkCommandBuffer commandBuffer, VkImage image, VkImageLayout imageLayout, VkClearColorValue const* pColor, uint32_t rangeCount, VkImageSubresourceRange const* pRanges)
    /// ```
    pub unsafe fn cmd_clear_color_image(
        &self,
        command_buffer: vk::CommandBuffer,
        image: vk::Image,
        image_layout: ImageLayout,
        color: &ClearColorValue,
        ranges: &[ImageSubresourceRange],
    ) -> () {
        unsafe {
            self._1_0.CmdClearColorImage(
                command_buffer.abi(), 
                image.abi(), 
                image_layout.abi(), 
                color.abi(), 
                ranges.len() as _, 
                ranges.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdClearDepthStencilImage(VkCommandBuffer commandBuffer, VkImage image, VkImageLayout imageLayout, VkClearDepthStencilValue const* pDepthStencil, uint32_t rangeCount, VkImageSubresourceRange const* pRanges)
    /// ```
    pub unsafe fn cmd_clear_depth_stencil_image(
        &self,
        command_buffer: vk::CommandBuffer,
        image: vk::Image,
        image_layout: ImageLayout,
        depth_stencil: &ClearDepthStencilValue,
        ranges: &[ImageSubresourceRange],
    ) -> () {
        unsafe {
            self._1_0.CmdClearDepthStencilImage(
                command_buffer.abi(), 
                image.abi(), 
                image_layout.abi(), 
                depth_stencil.abi(), 
                ranges.len() as _, 
                ranges.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdCopyBuffer(VkCommandBuffer commandBuffer, VkBuffer srcBuffer, VkBuffer dstBuffer, uint32_t regionCount, VkBufferCopy const* pRegions)
    /// ```
    pub unsafe fn cmd_copy_buffer(
        &self,
        command_buffer: vk::CommandBuffer,
        src_buffer: vk::Buffer,
        dst_buffer: vk::Buffer,
        regions: &[BufferCopy],
    ) -> () {
        unsafe {
            self._1_0.CmdCopyBuffer(
                command_buffer.abi(), 
                src_buffer.abi(), 
                dst_buffer.abi(), 
                regions.len() as _, 
                regions.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdCopyBufferToImage(VkCommandBuffer commandBuffer, VkBuffer srcBuffer, VkImage dstImage, VkImageLayout dstImageLayout, uint32_t regionCount, VkBufferImageCopy const* pRegions)
    /// ```
    pub unsafe fn cmd_copy_buffer_to_image(
        &self,
        command_buffer: vk::CommandBuffer,
        src_buffer: vk::Buffer,
        dst_image: vk::Image,
        dst_image_layout: ImageLayout,
        regions: &[BufferImageCopy],
    ) -> () {
        unsafe {
            self._1_0.CmdCopyBufferToImage(
                command_buffer.abi(), 
                src_buffer.abi(), 
                dst_image.abi(), 
                dst_image_layout.abi(), 
                regions.len() as _, 
                regions.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdCopyImage(VkCommandBuffer commandBuffer, VkImage srcImage, VkImageLayout srcImageLayout, VkImage dstImage, VkImageLayout dstImageLayout, uint32_t regionCount, VkImageCopy const* pRegions)
    /// ```
    pub unsafe fn cmd_copy_image(
        &self,
        command_buffer: vk::CommandBuffer,
        src_image: vk::Image,
        src_image_layout: ImageLayout,
        dst_image: vk::Image,
        dst_image_layout: ImageLayout,
        regions: &[ImageCopy],
    ) -> () {
        unsafe {
            self._1_0.CmdCopyImage(
                command_buffer.abi(), 
                src_image.abi(), 
                src_image_layout.abi(), 
                dst_image.abi(), 
                dst_image_layout.abi(), 
                regions.len() as _, 
                regions.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdCopyImageToBuffer(VkCommandBuffer commandBuffer, VkImage srcImage, VkImageLayout srcImageLayout, VkBuffer dstBuffer, uint32_t regionCount, VkBufferImageCopy const* pRegions)
    /// ```
    pub unsafe fn cmd_copy_image_to_buffer(
        &self,
        command_buffer: vk::CommandBuffer,
        src_image: vk::Image,
        src_image_layout: ImageLayout,
        dst_buffer: vk::Buffer,
        regions: &[BufferImageCopy],
    ) -> () {
        unsafe {
            self._1_0.CmdCopyImageToBuffer(
                command_buffer.abi(), 
                src_image.abi(), 
                src_image_layout.abi(), 
                dst_buffer.abi(), 
                regions.len() as _, 
                regions.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdCopyQueryPoolResults(VkCommandBuffer commandBuffer, VkQueryPool queryPool, uint32_t firstQuery, uint32_t queryCount, VkBuffer dstBuffer, VkDeviceSize dstOffset, VkDeviceSize stride, VkQueryResultFlags flags)
    /// ```
    pub unsafe fn cmd_copy_query_pool_results(
        &self,
        command_buffer: vk::CommandBuffer,
        query_pool: vk::QueryPool,
        first_query: uint32_t,
        query_count: uint32_t,
        dst_buffer: vk::Buffer,
        dst_offset: DeviceSize,
        stride: DeviceSize,
        flags: QueryResultFlags,
    ) -> () {
        unsafe {
            self._1_0.CmdCopyQueryPoolResults(
                command_buffer.abi(), 
                query_pool.abi(), 
                first_query.abi(), 
                query_count.abi(), 
                dst_buffer.abi(), 
                dst_offset.abi(), 
                stride.abi(), 
                flags.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdDispatch(VkCommandBuffer commandBuffer, uint32_t groupCountX, uint32_t groupCountY, uint32_t groupCountZ)
    /// ```
    pub unsafe fn cmd_dispatch(
        &self,
        command_buffer: vk::CommandBuffer,
        group_count_x: uint32_t,
        group_count_y: uint32_t,
        group_count_z: uint32_t,
    ) -> () {
        unsafe {
            self._1_0.CmdDispatch(
                command_buffer.abi(), 
                group_count_x.abi(), 
                group_count_y.abi(), 
                group_count_z.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdDispatchIndirect(VkCommandBuffer commandBuffer, VkBuffer buffer, VkDeviceSize offset)
    /// ```
    pub unsafe fn cmd_dispatch_indirect(
        &self,
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: DeviceSize,
    ) -> () {
        unsafe {
            self._1_0.CmdDispatchIndirect(
                command_buffer.abi(), 
                buffer.abi(), 
                offset.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdDraw(VkCommandBuffer commandBuffer, uint32_t vertexCount, uint32_t instanceCount, uint32_t firstVertex, uint32_t firstInstance)
    /// ```
    pub unsafe fn cmd_draw(
        &self,
        command_buffer: vk::CommandBuffer,
        vertex_count: uint32_t,
        instance_count: uint32_t,
        first_vertex: uint32_t,
        first_instance: uint32_t,
    ) -> () {
        unsafe {
            self._1_0.CmdDraw(
                command_buffer.abi(), 
                vertex_count.abi(), 
                instance_count.abi(), 
                first_vertex.abi(), 
                first_instance.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdDrawIndexed(VkCommandBuffer commandBuffer, uint32_t indexCount, uint32_t instanceCount, uint32_t firstIndex, int32_t vertexOffset, uint32_t firstInstance)
    /// ```
    pub unsafe fn cmd_draw_indexed(
        &self,
        command_buffer: vk::CommandBuffer,
        index_count: uint32_t,
        instance_count: uint32_t,
        first_index: uint32_t,
        vertex_offset: int32_t,
        first_instance: uint32_t,
    ) -> () {
        unsafe {
            self._1_0.CmdDrawIndexed(
                command_buffer.abi(), 
                index_count.abi(), 
                instance_count.abi(), 
                first_index.abi(), 
                vertex_offset.abi(), 
                first_instance.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdDrawIndexedIndirect(VkCommandBuffer commandBuffer, VkBuffer buffer, VkDeviceSize offset, uint32_t drawCount, uint32_t stride)
    /// ```
    pub unsafe fn cmd_draw_indexed_indirect(
        &self,
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: DeviceSize,
        draw_count: uint32_t,
        stride: uint32_t,
    ) -> () {
        unsafe {
            self._1_0.CmdDrawIndexedIndirect(
                command_buffer.abi(), 
                buffer.abi(), 
                offset.abi(), 
                draw_count.abi(), 
                stride.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdDrawIndirect(VkCommandBuffer commandBuffer, VkBuffer buffer, VkDeviceSize offset, uint32_t drawCount, uint32_t stride)
    /// ```
    pub unsafe fn cmd_draw_indirect(
        &self,
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: DeviceSize,
        draw_count: uint32_t,
        stride: uint32_t,
    ) -> () {
        unsafe {
            self._1_0.CmdDrawIndirect(
                command_buffer.abi(), 
                buffer.abi(), 
                offset.abi(), 
                draw_count.abi(), 
                stride.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdEndQuery(VkCommandBuffer commandBuffer, VkQueryPool queryPool, uint32_t query)
    /// ```
    pub unsafe fn cmd_end_query(
        &self,
        command_buffer: vk::CommandBuffer,
        query_pool: vk::QueryPool,
        query: uint32_t,
    ) -> () {
        unsafe {
            self._1_0.CmdEndQuery(
                command_buffer.abi(), 
                query_pool.abi(), 
                query.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdEndRenderPass(VkCommandBuffer commandBuffer)
    /// ```
    pub unsafe fn cmd_end_render_pass(
        &self,
        command_buffer: vk::CommandBuffer,
    ) -> () {
        unsafe {
            self._1_0.CmdEndRenderPass(
                command_buffer.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdExecuteCommands(VkCommandBuffer commandBuffer, uint32_t commandBufferCount, VkCommandBuffer const* pCommandBuffers)
    /// ```
    pub unsafe fn cmd_execute_commands(
        &self,
        command_buffer: vk::CommandBuffer,
        command_buffers: &[vk::CommandBuffer],
    ) -> () {
        unsafe {
            self._1_0.CmdExecuteCommands(
                command_buffer.abi(), 
                command_buffers.len() as _, 
                command_buffers.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdFillBuffer(VkCommandBuffer commandBuffer, VkBuffer dstBuffer, VkDeviceSize dstOffset, VkDeviceSize size, uint32_t data)
    /// ```
    pub unsafe fn cmd_fill_buffer(
        &self,
        command_buffer: vk::CommandBuffer,
        dst_buffer: vk::Buffer,
        dst_offset: DeviceSize,
        size: DeviceSize,
        data: uint32_t,
    ) -> () {
        unsafe {
            self._1_0.CmdFillBuffer(
                command_buffer.abi(), 
                dst_buffer.abi(), 
                dst_offset.abi(), 
                size.abi(), 
                data.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdNextSubpass(VkCommandBuffer commandBuffer, VkSubpassContents contents)
    /// ```
    pub unsafe fn cmd_next_subpass(
        &self,
        command_buffer: vk::CommandBuffer,
        contents: SubpassContents,
    ) -> () {
        unsafe {
            self._1_0.CmdNextSubpass(
                command_buffer.abi(), 
                contents.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdPipelineBarrier(VkCommandBuffer commandBuffer, VkPipelineStageFlags srcStageMask, VkPipelineStageFlags dstStageMask, VkDependencyFlags dependencyFlags, uint32_t memoryBarrierCount, VkMemoryBarrier const* pMemoryBarriers, uint32_t bufferMemoryBarrierCount, VkBufferMemoryBarrier const* pBufferMemoryBarriers, uint32_t imageMemoryBarrierCount, VkImageMemoryBarrier const* pImageMemoryBarriers)
    /// ```
    pub unsafe fn cmd_pipeline_barrier(
        &self,
        command_buffer: vk::CommandBuffer,
        src_stage_mask: PipelineStageFlags,
        dst_stage_mask: PipelineStageFlags,
        dependency_flags: DependencyFlags,
        memory_barriers: &[MemoryBarrier],
        buffer_memory_barriers: &[BufferMemoryBarrier],
        image_memory_barriers: &[ImageMemoryBarrier],
    ) -> () {
        unsafe {
            self._1_0.CmdPipelineBarrier(
                command_buffer.abi(), 
                src_stage_mask.abi(), 
                dst_stage_mask.abi(), 
                dependency_flags.abi(), 
                memory_barriers.len() as _, 
                memory_barriers.abi(), 
                buffer_memory_barriers.len() as _, 
                buffer_memory_barriers.abi(), 
                image_memory_barriers.len() as _, 
                image_memory_barriers.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdPushConstants(VkCommandBuffer commandBuffer, VkPipelineLayout layout, VkShaderStageFlags stageFlags, uint32_t offset, uint32_t size, void const* pValues)
    /// ```
    pub unsafe fn cmd_push_constants(
        &self,
        command_buffer: vk::CommandBuffer,
        layout: vk::PipelineLayout,
        stage_flags: ShaderStageFlags,
        offset: uint32_t,
        size: uint32_t,
        values: *const void,
    ) -> () {
        unsafe {
            self._1_0.CmdPushConstants(
                command_buffer.abi(), 
                layout.abi(), 
                stage_flags.abi(), 
                offset.abi(), 
                size.abi(), 
                values.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdResetEvent(VkCommandBuffer commandBuffer, VkEvent event, VkPipelineStageFlags stageMask)
    /// ```
    pub unsafe fn cmd_reset_event(
        &self,
        command_buffer: vk::CommandBuffer,
        event: vk::Event,
        stage_mask: PipelineStageFlags,
    ) -> () {
        unsafe {
            self._1_0.CmdResetEvent(
                command_buffer.abi(), 
                event.abi(), 
                stage_mask.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdResetQueryPool(VkCommandBuffer commandBuffer, VkQueryPool queryPool, uint32_t firstQuery, uint32_t queryCount)
    /// ```
    pub unsafe fn cmd_reset_query_pool(
        &self,
        command_buffer: vk::CommandBuffer,
        query_pool: vk::QueryPool,
        first_query: uint32_t,
        query_count: uint32_t,
    ) -> () {
        unsafe {
            self._1_0.CmdResetQueryPool(
                command_buffer.abi(), 
                query_pool.abi(), 
                first_query.abi(), 
                query_count.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdResolveImage(VkCommandBuffer commandBuffer, VkImage srcImage, VkImageLayout srcImageLayout, VkImage dstImage, VkImageLayout dstImageLayout, uint32_t regionCount, VkImageResolve const* pRegions)
    /// ```
    pub unsafe fn cmd_resolve_image(
        &self,
        command_buffer: vk::CommandBuffer,
        src_image: vk::Image,
        src_image_layout: ImageLayout,
        dst_image: vk::Image,
        dst_image_layout: ImageLayout,
        regions: &[ImageResolve],
    ) -> () {
        unsafe {
            self._1_0.CmdResolveImage(
                command_buffer.abi(), 
                src_image.abi(), 
                src_image_layout.abi(), 
                dst_image.abi(), 
                dst_image_layout.abi(), 
                regions.len() as _, 
                regions.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetBlendConstants(VkCommandBuffer commandBuffer, const float blendConstants[4])
    /// ```
    pub unsafe fn cmd_set_blend_constants(
        &self,
        command_buffer: vk::CommandBuffer,
        blend_constants: [float; 4],
    ) -> () {
        unsafe {
            self._1_0.CmdSetBlendConstants(
                command_buffer.abi(), 
                blend_constants.as_ref().abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetDepthBias(VkCommandBuffer commandBuffer, float depthBiasConstantFactor, float depthBiasClamp, float depthBiasSlopeFactor)
    /// ```
    pub unsafe fn cmd_set_depth_bias(
        &self,
        command_buffer: vk::CommandBuffer,
        depth_bias_constant_factor: float,
        depth_bias_clamp: float,
        depth_bias_slope_factor: float,
    ) -> () {
        unsafe {
            self._1_0.CmdSetDepthBias(
                command_buffer.abi(), 
                depth_bias_constant_factor.abi(), 
                depth_bias_clamp.abi(), 
                depth_bias_slope_factor.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetDepthBounds(VkCommandBuffer commandBuffer, float minDepthBounds, float maxDepthBounds)
    /// ```
    pub unsafe fn cmd_set_depth_bounds(
        &self,
        command_buffer: vk::CommandBuffer,
        min_depth_bounds: float,
        max_depth_bounds: float,
    ) -> () {
        unsafe {
            self._1_0.CmdSetDepthBounds(
                command_buffer.abi(), 
                min_depth_bounds.abi(), 
                max_depth_bounds.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetEvent(VkCommandBuffer commandBuffer, VkEvent event, VkPipelineStageFlags stageMask)
    /// ```
    pub unsafe fn cmd_set_event(
        &self,
        command_buffer: vk::CommandBuffer,
        event: vk::Event,
        stage_mask: PipelineStageFlags,
    ) -> () {
        unsafe {
            self._1_0.CmdSetEvent(
                command_buffer.abi(), 
                event.abi(), 
                stage_mask.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetLineWidth(VkCommandBuffer commandBuffer, float lineWidth)
    /// ```
    pub unsafe fn cmd_set_line_width(
        &self,
        command_buffer: vk::CommandBuffer,
        line_width: float,
    ) -> () {
        unsafe {
            self._1_0.CmdSetLineWidth(
                command_buffer.abi(), 
                line_width.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetScissor(VkCommandBuffer commandBuffer, uint32_t firstScissor, uint32_t scissorCount, VkRect2D const* pScissors)
    /// ```
    pub unsafe fn cmd_set_scissor(
        &self,
        command_buffer: vk::CommandBuffer,
        first_scissor: uint32_t,
        scissors: &[Rect2D],
    ) -> () {
        unsafe {
            self._1_0.CmdSetScissor(
                command_buffer.abi(), 
                first_scissor.abi(), 
                scissors.len() as _, 
                scissors.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetStencilCompareMask(VkCommandBuffer commandBuffer, VkStencilFaceFlags faceMask, uint32_t compareMask)
    /// ```
    pub unsafe fn cmd_set_stencil_compare_mask(
        &self,
        command_buffer: vk::CommandBuffer,
        face_mask: StencilFaceFlags,
        compare_mask: uint32_t,
    ) -> () {
        unsafe {
            self._1_0.CmdSetStencilCompareMask(
                command_buffer.abi(), 
                face_mask.abi(), 
                compare_mask.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetStencilReference(VkCommandBuffer commandBuffer, VkStencilFaceFlags faceMask, uint32_t reference)
    /// ```
    pub unsafe fn cmd_set_stencil_reference(
        &self,
        command_buffer: vk::CommandBuffer,
        face_mask: StencilFaceFlags,
        reference: uint32_t,
    ) -> () {
        unsafe {
            self._1_0.CmdSetStencilReference(
                command_buffer.abi(), 
                face_mask.abi(), 
                reference.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetStencilWriteMask(VkCommandBuffer commandBuffer, VkStencilFaceFlags faceMask, uint32_t writeMask)
    /// ```
    pub unsafe fn cmd_set_stencil_write_mask(
        &self,
        command_buffer: vk::CommandBuffer,
        face_mask: StencilFaceFlags,
        write_mask: uint32_t,
    ) -> () {
        unsafe {
            self._1_0.CmdSetStencilWriteMask(
                command_buffer.abi(), 
                face_mask.abi(), 
                write_mask.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetViewport(VkCommandBuffer commandBuffer, uint32_t firstViewport, uint32_t viewportCount, VkViewport const* pViewports)
    /// ```
    pub unsafe fn cmd_set_viewport(
        &self,
        command_buffer: vk::CommandBuffer,
        first_viewport: uint32_t,
        viewports: &[Viewport],
    ) -> () {
        unsafe {
            self._1_0.CmdSetViewport(
                command_buffer.abi(), 
                first_viewport.abi(), 
                viewports.len() as _, 
                viewports.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdUpdateBuffer(VkCommandBuffer commandBuffer, VkBuffer dstBuffer, VkDeviceSize dstOffset, VkDeviceSize dataSize, void const* pData)
    /// ```
    pub unsafe fn cmd_update_buffer(
        &self,
        command_buffer: vk::CommandBuffer,
        dst_buffer: vk::Buffer,
        dst_offset: DeviceSize,
        data_size: DeviceSize,
        data: *const void,
    ) -> () {
        unsafe {
            self._1_0.CmdUpdateBuffer(
                command_buffer.abi(), 
                dst_buffer.abi(), 
                dst_offset.abi(), 
                data_size.abi(), 
                data.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdWaitEvents(VkCommandBuffer commandBuffer, uint32_t eventCount, VkEvent const* pEvents, VkPipelineStageFlags srcStageMask, VkPipelineStageFlags dstStageMask, uint32_t memoryBarrierCount, VkMemoryBarrier const* pMemoryBarriers, uint32_t bufferMemoryBarrierCount, VkBufferMemoryBarrier const* pBufferMemoryBarriers, uint32_t imageMemoryBarrierCount, VkImageMemoryBarrier const* pImageMemoryBarriers)
    /// ```
    pub unsafe fn cmd_wait_events(
        &self,
        command_buffer: vk::CommandBuffer,
        events: &[vk::Event],
        src_stage_mask: PipelineStageFlags,
        dst_stage_mask: PipelineStageFlags,
        memory_barriers: &[MemoryBarrier],
        buffer_memory_barriers: &[BufferMemoryBarrier],
        image_memory_barriers: &[ImageMemoryBarrier],
    ) -> () {
        unsafe {
            self._1_0.CmdWaitEvents(
                command_buffer.abi(), 
                events.len() as _, 
                events.abi(), 
                src_stage_mask.abi(), 
                dst_stage_mask.abi(), 
                memory_barriers.len() as _, 
                memory_barriers.abi(), 
                buffer_memory_barriers.len() as _, 
                buffer_memory_barriers.abi(), 
                image_memory_barriers.len() as _, 
                image_memory_barriers.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdWriteTimestamp(VkCommandBuffer commandBuffer, VkPipelineStageFlagBits pipelineStage, VkQueryPool queryPool, uint32_t query)
    /// ```
    pub unsafe fn cmd_write_timestamp(
        &self,
        command_buffer: vk::CommandBuffer,
        pipeline_stage: PipelineStageFlags,
        query_pool: vk::QueryPool,
        query: uint32_t,
    ) -> () {
        unsafe {
            self._1_0.CmdWriteTimestamp(
                command_buffer.abi(), 
                pipeline_stage.abi(), 
                query_pool.abi(), 
                query.abi(), 
            );
        }
    }
    /// ```c
    /// VkResult vkCreateBuffer(VkDevice device, VkBufferCreateInfo const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkBuffer* pBuffer)
    /// ```
    pub unsafe fn create_buffer(
        &self,
        device: vk::Device,
        create_info: &BufferCreateInfo,
    ) -> crate::Result<vk::Buffer> {
        unsafe {
            let mut _v: Option<vk::Buffer> = Default::default();
            let _r = self._1_0.CreateBuffer(
                device.abi(), 
                create_info.abi(), 
                Default::default(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| _v)
        }
    }
    /// ```c
    /// VkResult vkCreateBufferView(VkDevice device, VkBufferViewCreateInfo const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkBufferView* pView)
    /// ```
    pub unsafe fn create_buffer_view(
        &self,
        device: vk::Device,
        create_info: &BufferViewCreateInfo,
    ) -> crate::Result<vk::BufferView> {
        unsafe {
            let mut _v: Option<vk::BufferView> = Default::default();
            let _r = self._1_0.CreateBufferView(
                device.abi(), 
                create_info.abi(), 
                Default::default(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| _v)
        }
    }
    /// ```c
    /// VkResult vkCreateCommandPool(VkDevice device, VkCommandPoolCreateInfo const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkCommandPool* pCommandPool)
    /// ```
    pub unsafe fn create_command_pool(
        &self,
        device: vk::Device,
        create_info: &CommandPoolCreateInfo,
    ) -> crate::Result<vk::CommandPool> {
        unsafe {
            let mut _v: Option<vk::CommandPool> = Default::default();
            let _r = self._1_0.CreateCommandPool(
                device.abi(), 
                create_info.abi(), 
                Default::default(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| _v)
        }
    }
    /// ```c
    /// VkResult vkCreateComputePipelines(VkDevice device, VkPipelineCache pipelineCache, uint32_t createInfoCount, VkComputePipelineCreateInfo const* pCreateInfos, VkAllocationCallbacks const* pAllocator, VkPipeline* pPipelines)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::PipelineCompileRequiredExt]
    pub unsafe fn create_compute_pipelines(
        &self,
        device: vk::Device,
        pipeline_cache: Option<vk::PipelineCache>,
        create_infos: &[ComputePipelineCreateInfo],
        pipelines: &mut [Option<vk::Pipeline>],
    ) -> crate::Result<Result> {
        unsafe {
            let _r = self._1_0.CreateComputePipelines(
                device.abi(), 
                pipeline_cache.abi(), 
                create_infos.len() as _, 
                create_infos.abi(), 
                Default::default(), 
                pipelines.abi(), 
            ).vk();
            _r.result(|| Some(_r))
        }
    }
    /// ```c
    /// VkResult vkCreateDescriptorPool(VkDevice device, VkDescriptorPoolCreateInfo const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkDescriptorPool* pDescriptorPool)
    /// ```
    pub unsafe fn create_descriptor_pool(
        &self,
        device: vk::Device,
        create_info: &DescriptorPoolCreateInfo,
    ) -> crate::Result<vk::DescriptorPool> {
        unsafe {
            let mut _v: Option<vk::DescriptorPool> = Default::default();
            let _r = self._1_0.CreateDescriptorPool(
                device.abi(), 
                create_info.abi(), 
                Default::default(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| _v)
        }
    }
    /// ```c
    /// VkResult vkCreateDescriptorSetLayout(VkDevice device, VkDescriptorSetLayoutCreateInfo const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkDescriptorSetLayout* pSetLayout)
    /// ```
    pub unsafe fn create_descriptor_set_layout(
        &self,
        device: vk::Device,
        create_info: &DescriptorSetLayoutCreateInfo,
    ) -> crate::Result<vk::DescriptorSetLayout> {
        unsafe {
            let mut _v: Option<vk::DescriptorSetLayout> = Default::default();
            let _r = self._1_0.CreateDescriptorSetLayout(
                device.abi(), 
                create_info.abi(), 
                Default::default(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| _v)
        }
    }
    /// ```c
    /// VkResult vkCreateEvent(VkDevice device, VkEventCreateInfo const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkEvent* pEvent)
    /// ```
    pub unsafe fn create_event(
        &self,
        device: vk::Device,
        create_info: &EventCreateInfo,
    ) -> crate::Result<vk::Event> {
        unsafe {
            let mut _v: Option<vk::Event> = Default::default();
            let _r = self._1_0.CreateEvent(
                device.abi(), 
                create_info.abi(), 
                Default::default(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| _v)
        }
    }
    /// ```c
    /// VkResult vkCreateFence(VkDevice device, VkFenceCreateInfo const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkFence* pFence)
    /// ```
    pub unsafe fn create_fence(
        &self,
        device: vk::Device,
        create_info: &FenceCreateInfo,
    ) -> crate::Result<vk::Fence> {
        unsafe {
            let mut _v: Option<vk::Fence> = Default::default();
            let _r = self._1_0.CreateFence(
                device.abi(), 
                create_info.abi(), 
                Default::default(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| _v)
        }
    }
    /// ```c
    /// VkResult vkCreateFramebuffer(VkDevice device, VkFramebufferCreateInfo const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkFramebuffer* pFramebuffer)
    /// ```
    pub unsafe fn create_framebuffer(
        &self,
        device: vk::Device,
        create_info: &FramebufferCreateInfo,
    ) -> crate::Result<vk::Framebuffer> {
        unsafe {
            let mut _v: Option<vk::Framebuffer> = Default::default();
            let _r = self._1_0.CreateFramebuffer(
                device.abi(), 
                create_info.abi(), 
                Default::default(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| _v)
        }
    }
    /// ```c
    /// VkResult vkCreateGraphicsPipelines(VkDevice device, VkPipelineCache pipelineCache, uint32_t createInfoCount, VkGraphicsPipelineCreateInfo const* pCreateInfos, VkAllocationCallbacks const* pAllocator, VkPipeline* pPipelines)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::PipelineCompileRequiredExt]
    pub unsafe fn create_graphics_pipelines(
        &self,
        device: vk::Device,
        pipeline_cache: Option<vk::PipelineCache>,
        create_infos: &[GraphicsPipelineCreateInfo],
        pipelines: &mut [Option<vk::Pipeline>],
    ) -> crate::Result<Result> {
        unsafe {
            let _r = self._1_0.CreateGraphicsPipelines(
                device.abi(), 
                pipeline_cache.abi(), 
                create_infos.len() as _, 
                create_infos.abi(), 
                Default::default(), 
                pipelines.abi(), 
            ).vk();
            _r.result(|| Some(_r))
        }
    }
    /// ```c
    /// VkResult vkCreateImage(VkDevice device, VkImageCreateInfo const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkImage* pImage)
    /// ```
    pub unsafe fn create_image(
        &self,
        device: vk::Device,
        create_info: &ImageCreateInfo,
    ) -> crate::Result<vk::Image> {
        unsafe {
            let mut _v: Option<vk::Image> = Default::default();
            let _r = self._1_0.CreateImage(
                device.abi(), 
                create_info.abi(), 
                Default::default(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| _v)
        }
    }
    /// ```c
    /// VkResult vkCreateImageView(VkDevice device, VkImageViewCreateInfo const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkImageView* pView)
    /// ```
    pub unsafe fn create_image_view(
        &self,
        device: vk::Device,
        create_info: &ImageViewCreateInfo,
    ) -> crate::Result<vk::ImageView> {
        unsafe {
            let mut _v: Option<vk::ImageView> = Default::default();
            let _r = self._1_0.CreateImageView(
                device.abi(), 
                create_info.abi(), 
                Default::default(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| _v)
        }
    }
    /// ```c
    /// VkResult vkCreatePipelineCache(VkDevice device, VkPipelineCacheCreateInfo const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkPipelineCache* pPipelineCache)
    /// ```
    pub unsafe fn create_pipeline_cache(
        &self,
        device: vk::Device,
        create_info: &PipelineCacheCreateInfo,
    ) -> crate::Result<vk::PipelineCache> {
        unsafe {
            let mut _v: Option<vk::PipelineCache> = Default::default();
            let _r = self._1_0.CreatePipelineCache(
                device.abi(), 
                create_info.abi(), 
                Default::default(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| _v)
        }
    }
    /// ```c
    /// VkResult vkCreatePipelineLayout(VkDevice device, VkPipelineLayoutCreateInfo const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkPipelineLayout* pPipelineLayout)
    /// ```
    pub unsafe fn create_pipeline_layout(
        &self,
        device: vk::Device,
        create_info: &PipelineLayoutCreateInfo,
    ) -> crate::Result<vk::PipelineLayout> {
        unsafe {
            let mut _v: Option<vk::PipelineLayout> = Default::default();
            let _r = self._1_0.CreatePipelineLayout(
                device.abi(), 
                create_info.abi(), 
                Default::default(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| _v)
        }
    }
    /// ```c
    /// VkResult vkCreateQueryPool(VkDevice device, VkQueryPoolCreateInfo const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkQueryPool* pQueryPool)
    /// ```
    pub unsafe fn create_query_pool(
        &self,
        device: vk::Device,
        create_info: &QueryPoolCreateInfo,
    ) -> crate::Result<vk::QueryPool> {
        unsafe {
            let mut _v: Option<vk::QueryPool> = Default::default();
            let _r = self._1_0.CreateQueryPool(
                device.abi(), 
                create_info.abi(), 
                Default::default(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| _v)
        }
    }
    /// ```c
    /// VkResult vkCreateRenderPass(VkDevice device, VkRenderPassCreateInfo const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkRenderPass* pRenderPass)
    /// ```
    pub unsafe fn create_render_pass(
        &self,
        device: vk::Device,
        create_info: &RenderPassCreateInfo,
    ) -> crate::Result<vk::RenderPass> {
        unsafe {
            let mut _v: Option<vk::RenderPass> = Default::default();
            let _r = self._1_0.CreateRenderPass(
                device.abi(), 
                create_info.abi(), 
                Default::default(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| _v)
        }
    }
    /// ```c
    /// VkResult vkCreateSampler(VkDevice device, VkSamplerCreateInfo const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkSampler* pSampler)
    /// ```
    pub unsafe fn create_sampler(
        &self,
        device: vk::Device,
        create_info: &SamplerCreateInfo,
    ) -> crate::Result<vk::Sampler> {
        unsafe {
            let mut _v: Option<vk::Sampler> = Default::default();
            let _r = self._1_0.CreateSampler(
                device.abi(), 
                create_info.abi(), 
                Default::default(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| _v)
        }
    }
    /// ```c
    /// VkResult vkCreateSemaphore(VkDevice device, VkSemaphoreCreateInfo const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkSemaphore* pSemaphore)
    /// ```
    pub unsafe fn create_semaphore(
        &self,
        device: vk::Device,
        create_info: &SemaphoreCreateInfo,
    ) -> crate::Result<vk::Semaphore> {
        unsafe {
            let mut _v: Option<vk::Semaphore> = Default::default();
            let _r = self._1_0.CreateSemaphore(
                device.abi(), 
                create_info.abi(), 
                Default::default(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| _v)
        }
    }
    /// ```c
    /// VkResult vkCreateShaderModule(VkDevice device, VkShaderModuleCreateInfo const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkShaderModule* pShaderModule)
    /// ```
    pub unsafe fn create_shader_module(
        &self,
        device: vk::Device,
        create_info: &ShaderModuleCreateInfo,
    ) -> crate::Result<vk::ShaderModule> {
        unsafe {
            let mut _v: Option<vk::ShaderModule> = Default::default();
            let _r = self._1_0.CreateShaderModule(
                device.abi(), 
                create_info.abi(), 
                Default::default(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| _v)
        }
    }
    /// ```c
    /// void vkDestroyBuffer(VkDevice device, VkBuffer buffer, VkAllocationCallbacks const* pAllocator)
    /// ```
    pub unsafe fn destroy_buffer(
        &self,
        device: vk::Device,
        buffer: Option<vk::Buffer>,
    ) -> () {
        unsafe {
            self._1_0.DestroyBuffer(
                device.abi(), 
                buffer.abi(), 
                Default::default(), 
            );
        }
    }
    /// ```c
    /// void vkDestroyBufferView(VkDevice device, VkBufferView bufferView, VkAllocationCallbacks const* pAllocator)
    /// ```
    pub unsafe fn destroy_buffer_view(
        &self,
        device: vk::Device,
        buffer_view: Option<vk::BufferView>,
    ) -> () {
        unsafe {
            self._1_0.DestroyBufferView(
                device.abi(), 
                buffer_view.abi(), 
                Default::default(), 
            );
        }
    }
    /// ```c
    /// void vkDestroyCommandPool(VkDevice device, VkCommandPool commandPool, VkAllocationCallbacks const* pAllocator)
    /// ```
    pub unsafe fn destroy_command_pool(
        &self,
        device: vk::Device,
        command_pool: Option<vk::CommandPool>,
    ) -> () {
        unsafe {
            self._1_0.DestroyCommandPool(
                device.abi(), 
                command_pool.abi(), 
                Default::default(), 
            );
        }
    }
    /// ```c
    /// void vkDestroyDescriptorPool(VkDevice device, VkDescriptorPool descriptorPool, VkAllocationCallbacks const* pAllocator)
    /// ```
    pub unsafe fn destroy_descriptor_pool(
        &self,
        device: vk::Device,
        descriptor_pool: Option<vk::DescriptorPool>,
    ) -> () {
        unsafe {
            self._1_0.DestroyDescriptorPool(
                device.abi(), 
                descriptor_pool.abi(), 
                Default::default(), 
            );
        }
    }
    /// ```c
    /// void vkDestroyDescriptorSetLayout(VkDevice device, VkDescriptorSetLayout descriptorSetLayout, VkAllocationCallbacks const* pAllocator)
    /// ```
    pub unsafe fn destroy_descriptor_set_layout(
        &self,
        device: vk::Device,
        descriptor_set_layout: Option<vk::DescriptorSetLayout>,
    ) -> () {
        unsafe {
            self._1_0.DestroyDescriptorSetLayout(
                device.abi(), 
                descriptor_set_layout.abi(), 
                Default::default(), 
            );
        }
    }
    /// ```c
    /// void vkDestroyDevice(VkDevice device, VkAllocationCallbacks const* pAllocator)
    /// ```
    pub unsafe fn destroy_device(
        &self,
        device: Option<vk::Device>,
    ) -> () {
        unsafe {
            self._1_0.DestroyDevice(
                device.abi(), 
                Default::default(), 
            );
        }
    }
    /// ```c
    /// void vkDestroyEvent(VkDevice device, VkEvent event, VkAllocationCallbacks const* pAllocator)
    /// ```
    pub unsafe fn destroy_event(
        &self,
        device: vk::Device,
        event: Option<vk::Event>,
    ) -> () {
        unsafe {
            self._1_0.DestroyEvent(
                device.abi(), 
                event.abi(), 
                Default::default(), 
            );
        }
    }
    /// ```c
    /// void vkDestroyFence(VkDevice device, VkFence fence, VkAllocationCallbacks const* pAllocator)
    /// ```
    pub unsafe fn destroy_fence(
        &self,
        device: vk::Device,
        fence: Option<vk::Fence>,
    ) -> () {
        unsafe {
            self._1_0.DestroyFence(
                device.abi(), 
                fence.abi(), 
                Default::default(), 
            );
        }
    }
    /// ```c
    /// void vkDestroyFramebuffer(VkDevice device, VkFramebuffer framebuffer, VkAllocationCallbacks const* pAllocator)
    /// ```
    pub unsafe fn destroy_framebuffer(
        &self,
        device: vk::Device,
        framebuffer: Option<vk::Framebuffer>,
    ) -> () {
        unsafe {
            self._1_0.DestroyFramebuffer(
                device.abi(), 
                framebuffer.abi(), 
                Default::default(), 
            );
        }
    }
    /// ```c
    /// void vkDestroyImage(VkDevice device, VkImage image, VkAllocationCallbacks const* pAllocator)
    /// ```
    pub unsafe fn destroy_image(
        &self,
        device: vk::Device,
        image: Option<vk::Image>,
    ) -> () {
        unsafe {
            self._1_0.DestroyImage(
                device.abi(), 
                image.abi(), 
                Default::default(), 
            );
        }
    }
    /// ```c
    /// void vkDestroyImageView(VkDevice device, VkImageView imageView, VkAllocationCallbacks const* pAllocator)
    /// ```
    pub unsafe fn destroy_image_view(
        &self,
        device: vk::Device,
        image_view: Option<vk::ImageView>,
    ) -> () {
        unsafe {
            self._1_0.DestroyImageView(
                device.abi(), 
                image_view.abi(), 
                Default::default(), 
            );
        }
    }
    /// ```c
    /// void vkDestroyPipeline(VkDevice device, VkPipeline pipeline, VkAllocationCallbacks const* pAllocator)
    /// ```
    pub unsafe fn destroy_pipeline(
        &self,
        device: vk::Device,
        pipeline: Option<vk::Pipeline>,
    ) -> () {
        unsafe {
            self._1_0.DestroyPipeline(
                device.abi(), 
                pipeline.abi(), 
                Default::default(), 
            );
        }
    }
    /// ```c
    /// void vkDestroyPipelineCache(VkDevice device, VkPipelineCache pipelineCache, VkAllocationCallbacks const* pAllocator)
    /// ```
    pub unsafe fn destroy_pipeline_cache(
        &self,
        device: vk::Device,
        pipeline_cache: Option<vk::PipelineCache>,
    ) -> () {
        unsafe {
            self._1_0.DestroyPipelineCache(
                device.abi(), 
                pipeline_cache.abi(), 
                Default::default(), 
            );
        }
    }
    /// ```c
    /// void vkDestroyPipelineLayout(VkDevice device, VkPipelineLayout pipelineLayout, VkAllocationCallbacks const* pAllocator)
    /// ```
    pub unsafe fn destroy_pipeline_layout(
        &self,
        device: vk::Device,
        pipeline_layout: Option<vk::PipelineLayout>,
    ) -> () {
        unsafe {
            self._1_0.DestroyPipelineLayout(
                device.abi(), 
                pipeline_layout.abi(), 
                Default::default(), 
            );
        }
    }
    /// ```c
    /// void vkDestroyQueryPool(VkDevice device, VkQueryPool queryPool, VkAllocationCallbacks const* pAllocator)
    /// ```
    pub unsafe fn destroy_query_pool(
        &self,
        device: vk::Device,
        query_pool: Option<vk::QueryPool>,
    ) -> () {
        unsafe {
            self._1_0.DestroyQueryPool(
                device.abi(), 
                query_pool.abi(), 
                Default::default(), 
            );
        }
    }
    /// ```c
    /// void vkDestroyRenderPass(VkDevice device, VkRenderPass renderPass, VkAllocationCallbacks const* pAllocator)
    /// ```
    pub unsafe fn destroy_render_pass(
        &self,
        device: vk::Device,
        render_pass: Option<vk::RenderPass>,
    ) -> () {
        unsafe {
            self._1_0.DestroyRenderPass(
                device.abi(), 
                render_pass.abi(), 
                Default::default(), 
            );
        }
    }
    /// ```c
    /// void vkDestroySampler(VkDevice device, VkSampler sampler, VkAllocationCallbacks const* pAllocator)
    /// ```
    pub unsafe fn destroy_sampler(
        &self,
        device: vk::Device,
        sampler: Option<vk::Sampler>,
    ) -> () {
        unsafe {
            self._1_0.DestroySampler(
                device.abi(), 
                sampler.abi(), 
                Default::default(), 
            );
        }
    }
    /// ```c
    /// void vkDestroySemaphore(VkDevice device, VkSemaphore semaphore, VkAllocationCallbacks const* pAllocator)
    /// ```
    pub unsafe fn destroy_semaphore(
        &self,
        device: vk::Device,
        semaphore: Option<vk::Semaphore>,
    ) -> () {
        unsafe {
            self._1_0.DestroySemaphore(
                device.abi(), 
                semaphore.abi(), 
                Default::default(), 
            );
        }
    }
    /// ```c
    /// void vkDestroyShaderModule(VkDevice device, VkShaderModule shaderModule, VkAllocationCallbacks const* pAllocator)
    /// ```
    pub unsafe fn destroy_shader_module(
        &self,
        device: vk::Device,
        shader_module: Option<vk::ShaderModule>,
    ) -> () {
        unsafe {
            self._1_0.DestroyShaderModule(
                device.abi(), 
                shader_module.abi(), 
                Default::default(), 
            );
        }
    }
    /// ```c
    /// VkResult vkDeviceWaitIdle(VkDevice device)
    /// ```
    pub unsafe fn device_wait_idle(
        &self,
        device: vk::Device,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self._1_0.DeviceWaitIdle(
                device.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkResult vkEndCommandBuffer(VkCommandBuffer commandBuffer)
    /// ```
    pub unsafe fn end_command_buffer(
        &self,
        command_buffer: vk::CommandBuffer,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self._1_0.EndCommandBuffer(
                command_buffer.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkResult vkFlushMappedMemoryRanges(VkDevice device, uint32_t memoryRangeCount, VkMappedMemoryRange const* pMemoryRanges)
    /// ```
    pub unsafe fn flush_mapped_memory_ranges(
        &self,
        device: vk::Device,
        memory_ranges: &[MappedMemoryRange],
    ) -> crate::Result<()> {
        unsafe {
            let _r = self._1_0.FlushMappedMemoryRanges(
                device.abi(), 
                memory_ranges.len() as _, 
                memory_ranges.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// void vkFreeCommandBuffers(VkDevice device, VkCommandPool commandPool, uint32_t commandBufferCount, VkCommandBuffer const* pCommandBuffers)
    /// ```
    pub unsafe fn free_command_buffers(
        &self,
        device: vk::Device,
        command_pool: vk::CommandPool,
        command_buffers: &[vk::CommandBuffer],
    ) -> () {
        unsafe {
            self._1_0.FreeCommandBuffers(
                device.abi(), 
                command_pool.abi(), 
                command_buffers.len() as _, 
                command_buffers.abi(), 
            );
        }
    }
    /// ```c
    /// VkResult vkFreeDescriptorSets(VkDevice device, VkDescriptorPool descriptorPool, uint32_t descriptorSetCount, VkDescriptorSet const* pDescriptorSets)
    /// ```
    pub unsafe fn free_descriptor_sets(
        &self,
        device: vk::Device,
        descriptor_pool: vk::DescriptorPool,
        descriptor_sets: &[vk::DescriptorSet],
    ) -> crate::Result<()> {
        unsafe {
            let _r = self._1_0.FreeDescriptorSets(
                device.abi(), 
                descriptor_pool.abi(), 
                descriptor_sets.len() as _, 
                descriptor_sets.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// void vkFreeMemory(VkDevice device, VkDeviceMemory memory, VkAllocationCallbacks const* pAllocator)
    /// ```
    pub unsafe fn free_memory(
        &self,
        device: vk::Device,
        memory: Option<vk::DeviceMemory>,
    ) -> () {
        unsafe {
            self._1_0.FreeMemory(
                device.abi(), 
                memory.abi(), 
                Default::default(), 
            );
        }
    }
    /// ```c
    /// void vkGetBufferMemoryRequirements(VkDevice device, VkBuffer buffer, VkMemoryRequirements* pMemoryRequirements)
    /// ```
    pub unsafe fn get_buffer_memory_requirements(
        &self,
        device: vk::Device,
        buffer: vk::Buffer,
    ) -> MemoryRequirements {
        unsafe {
            let mut _v: MemoryRequirements = Default::default();
            self._1_0.GetBufferMemoryRequirements(
                device.abi(), 
                buffer.abi(), 
                (&mut _v).abi(), 
            );
            _v
        }
    }
    /// ```c
    /// void vkGetDeviceMemoryCommitment(VkDevice device, VkDeviceMemory memory, VkDeviceSize* pCommittedMemoryInBytes)
    /// ```
    pub unsafe fn get_device_memory_commitment(
        &self,
        device: vk::Device,
        memory: vk::DeviceMemory,
    ) -> DeviceSize {
        unsafe {
            let mut _v: DeviceSize = Default::default();
            self._1_0.GetDeviceMemoryCommitment(
                device.abi(), 
                memory.abi(), 
                (&mut _v).abi(), 
            );
            _v
        }
    }
    /// ```c
    /// void vkGetDeviceQueue(VkDevice device, uint32_t queueFamilyIndex, uint32_t queueIndex, VkQueue* pQueue)
    /// ```
    pub unsafe fn get_device_queue(
        &self,
        device: vk::Device,
        queue_family_index: uint32_t,
        queue_index: uint32_t,
    ) -> vk::Queue {
        unsafe {
            let mut _v: Option<vk::Queue> = Default::default();
            self._1_0.GetDeviceQueue(
                device.abi(), 
                queue_family_index.abi(), 
                queue_index.abi(), 
                (&mut _v).abi(), 
            );
            unsafe { _v.unwrap_unchecked() }
        }
    }
    /// ```c
    /// VkResult vkGetEventStatus(VkDevice device, VkEvent event)
    /// ```
    ///
    /// SuccessCodes: [Result::EventSet], [Result::EventReset]
    pub unsafe fn get_event_status(
        &self,
        device: vk::Device,
        event: vk::Event,
    ) -> crate::Result<Result> {
        unsafe {
            let _r = self._1_0.GetEventStatus(
                device.abi(), 
                event.abi(), 
            ).vk();
            _r.result(|| Some(_r))
        }
    }
    /// ```c
    /// VkResult vkGetFenceStatus(VkDevice device, VkFence fence)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::NotReady]
    pub unsafe fn get_fence_status(
        &self,
        device: vk::Device,
        fence: vk::Fence,
    ) -> crate::Result<Result> {
        unsafe {
            let _r = self._1_0.GetFenceStatus(
                device.abi(), 
                fence.abi(), 
            ).vk();
            _r.result(|| Some(_r))
        }
    }
    /// ```c
    /// void vkGetImageMemoryRequirements(VkDevice device, VkImage image, VkMemoryRequirements* pMemoryRequirements)
    /// ```
    pub unsafe fn get_image_memory_requirements(
        &self,
        device: vk::Device,
        image: vk::Image,
    ) -> MemoryRequirements {
        unsafe {
            let mut _v: MemoryRequirements = Default::default();
            self._1_0.GetImageMemoryRequirements(
                device.abi(), 
                image.abi(), 
                (&mut _v).abi(), 
            );
            _v
        }
    }
    /// ```c
    /// void vkGetImageSparseMemoryRequirements(VkDevice device, VkImage image, uint32_t* pSparseMemoryRequirementCount, VkSparseImageMemoryRequirements* pSparseMemoryRequirements)
    /// ```
    pub unsafe fn get_image_sparse_memory_requirements(
        &self,
        device: vk::Device,
        image: vk::Image,
        sparse_memory_requirements: Option<&mut ::alloc::vec::Vec<SparseImageMemoryRequirements>>,
    ) -> uint32_t {
        unsafe {
            let mut _c: uint32_t = Default::default();
            self._1_0.GetImageSparseMemoryRequirements(
                device.abi(), 
                image.abi(), 
                &mut _c, 
                Default::default(), 
            );
            if let Some(_b) = sparse_memory_requirements {
                _b.reserve(_c as usize);
                self._1_0.GetImageSparseMemoryRequirements(
                    device.abi(), 
                    image.abi(), 
                    &mut _c, 
                    (&mut **_b).abi(), 
                );
                _b.set_len(_b.len() + _c as usize);
            }
            _c
        }
    }
    /// ```c
    /// void vkGetImageSubresourceLayout(VkDevice device, VkImage image, VkImageSubresource const* pSubresource, VkSubresourceLayout* pLayout)
    /// ```
    pub unsafe fn get_image_subresource_layout(
        &self,
        device: vk::Device,
        image: vk::Image,
        subresource: &ImageSubresource,
    ) -> SubresourceLayout {
        unsafe {
            let mut _v: SubresourceLayout = Default::default();
            self._1_0.GetImageSubresourceLayout(
                device.abi(), 
                image.abi(), 
                subresource.abi(), 
                (&mut _v).abi(), 
            );
            _v
        }
    }
    /// ```c
    /// VkResult vkGetPipelineCacheData(VkDevice device, VkPipelineCache pipelineCache, size_t* pDataSize, void* pData)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    pub unsafe fn get_pipeline_cache_data(
        &self,
        device: vk::Device,
        pipeline_cache: vk::PipelineCache,
        data_size: *mut size_t,
        data: *mut void,
    ) -> crate::Result<Result> {
        unsafe {
            let _r = self._1_0.GetPipelineCacheData(
                device.abi(), 
                pipeline_cache.abi(), 
                data_size.abi(), 
                data.abi(), 
            ).vk();
            _r.result(|| Some(_r))
        }
    }
    /// ```c
    /// VkResult vkGetQueryPoolResults(VkDevice device, VkQueryPool queryPool, uint32_t firstQuery, uint32_t queryCount, size_t dataSize, void* pData, VkDeviceSize stride, VkQueryResultFlags flags)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::NotReady]
    pub unsafe fn get_query_pool_results(
        &self,
        device: vk::Device,
        query_pool: vk::QueryPool,
        first_query: uint32_t,
        query_count: uint32_t,
        data_size: size_t,
        data: *mut void,
        stride: DeviceSize,
        flags: QueryResultFlags,
    ) -> crate::Result<Result> {
        unsafe {
            let _r = self._1_0.GetQueryPoolResults(
                device.abi(), 
                query_pool.abi(), 
                first_query.abi(), 
                query_count.abi(), 
                data_size.abi(), 
                data.abi(), 
                stride.abi(), 
                flags.abi(), 
            ).vk();
            _r.result(|| Some(_r))
        }
    }
    /// ```c
    /// void vkGetRenderAreaGranularity(VkDevice device, VkRenderPass renderPass, VkExtent2D* pGranularity)
    /// ```
    pub unsafe fn get_render_area_granularity(
        &self,
        device: vk::Device,
        render_pass: vk::RenderPass,
    ) -> Extent2D {
        unsafe {
            let mut _v: Extent2D = Default::default();
            self._1_0.GetRenderAreaGranularity(
                device.abi(), 
                render_pass.abi(), 
                (&mut _v).abi(), 
            );
            _v
        }
    }
    /// ```c
    /// VkResult vkInvalidateMappedMemoryRanges(VkDevice device, uint32_t memoryRangeCount, VkMappedMemoryRange const* pMemoryRanges)
    /// ```
    pub unsafe fn invalidate_mapped_memory_ranges(
        &self,
        device: vk::Device,
        memory_ranges: &[MappedMemoryRange],
    ) -> crate::Result<()> {
        unsafe {
            let _r = self._1_0.InvalidateMappedMemoryRanges(
                device.abi(), 
                memory_ranges.len() as _, 
                memory_ranges.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkResult vkMapMemory(VkDevice device, VkDeviceMemory memory, VkDeviceSize offset, VkDeviceSize size, VkMemoryMapFlags flags, void** ppData)
    /// ```
    pub unsafe fn map_memory(
        &self,
        device: vk::Device,
        memory: vk::DeviceMemory,
        offset: DeviceSize,
        size: DeviceSize,
        flags: MemoryMapFlags,
        p_data: *mut *mut void,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self._1_0.MapMemory(
                device.abi(), 
                memory.abi(), 
                offset.abi(), 
                size.abi(), 
                flags.abi(), 
                p_data.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkResult vkMergePipelineCaches(VkDevice device, VkPipelineCache dstCache, uint32_t srcCacheCount, VkPipelineCache const* pSrcCaches)
    /// ```
    pub unsafe fn merge_pipeline_caches(
        &self,
        device: vk::Device,
        dst_cache: vk::PipelineCache,
        src_caches: &[vk::PipelineCache],
    ) -> crate::Result<()> {
        unsafe {
            let _r = self._1_0.MergePipelineCaches(
                device.abi(), 
                dst_cache.abi(), 
                src_caches.len() as _, 
                src_caches.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkResult vkQueueBindSparse(VkQueue queue, uint32_t bindInfoCount, VkBindSparseInfo const* pBindInfo, VkFence fence)
    /// ```
    pub unsafe fn queue_bind_sparse(
        &self,
        queue: vk::Queue,
        bind_info: &[BindSparseInfo],
        fence: Option<vk::Fence>,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self._1_0.QueueBindSparse(
                queue.abi(), 
                bind_info.len() as _, 
                bind_info.abi(), 
                fence.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkResult vkQueueSubmit(VkQueue queue, uint32_t submitCount, VkSubmitInfo const* pSubmits, VkFence fence)
    /// ```
    pub unsafe fn queue_submit(
        &self,
        queue: vk::Queue,
        submits: &[SubmitInfo],
        fence: Option<vk::Fence>,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self._1_0.QueueSubmit(
                queue.abi(), 
                submits.len() as _, 
                submits.abi(), 
                fence.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkResult vkQueueWaitIdle(VkQueue queue)
    /// ```
    pub unsafe fn queue_wait_idle(
        &self,
        queue: vk::Queue,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self._1_0.QueueWaitIdle(
                queue.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkResult vkResetCommandBuffer(VkCommandBuffer commandBuffer, VkCommandBufferResetFlags flags)
    /// ```
    pub unsafe fn reset_command_buffer(
        &self,
        command_buffer: vk::CommandBuffer,
        flags: CommandBufferResetFlags,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self._1_0.ResetCommandBuffer(
                command_buffer.abi(), 
                flags.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkResult vkResetCommandPool(VkDevice device, VkCommandPool commandPool, VkCommandPoolResetFlags flags)
    /// ```
    pub unsafe fn reset_command_pool(
        &self,
        device: vk::Device,
        command_pool: vk::CommandPool,
        flags: CommandPoolResetFlags,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self._1_0.ResetCommandPool(
                device.abi(), 
                command_pool.abi(), 
                flags.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkResult vkResetDescriptorPool(VkDevice device, VkDescriptorPool descriptorPool, VkDescriptorPoolResetFlags flags)
    /// ```
    pub unsafe fn reset_descriptor_pool(
        &self,
        device: vk::Device,
        descriptor_pool: vk::DescriptorPool,
        flags: DescriptorPoolResetFlags,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self._1_0.ResetDescriptorPool(
                device.abi(), 
                descriptor_pool.abi(), 
                flags.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkResult vkResetEvent(VkDevice device, VkEvent event)
    /// ```
    pub unsafe fn reset_event(
        &self,
        device: vk::Device,
        event: vk::Event,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self._1_0.ResetEvent(
                device.abi(), 
                event.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkResult vkResetFences(VkDevice device, uint32_t fenceCount, VkFence const* pFences)
    /// ```
    pub unsafe fn reset_fences(
        &self,
        device: vk::Device,
        fences: &[vk::Fence],
    ) -> crate::Result<()> {
        unsafe {
            let _r = self._1_0.ResetFences(
                device.abi(), 
                fences.len() as _, 
                fences.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkResult vkSetEvent(VkDevice device, VkEvent event)
    /// ```
    pub unsafe fn set_event(
        &self,
        device: vk::Device,
        event: vk::Event,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self._1_0.SetEvent(
                device.abi(), 
                event.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// void vkUnmapMemory(VkDevice device, VkDeviceMemory memory)
    /// ```
    pub unsafe fn unmap_memory(
        &self,
        device: vk::Device,
        memory: vk::DeviceMemory,
    ) -> () {
        unsafe {
            self._1_0.UnmapMemory(
                device.abi(), 
                memory.abi(), 
            );
        }
    }
    /// ```c
    /// void vkUpdateDescriptorSets(VkDevice device, uint32_t descriptorWriteCount, VkWriteDescriptorSet const* pDescriptorWrites, uint32_t descriptorCopyCount, VkCopyDescriptorSet const* pDescriptorCopies)
    /// ```
    pub unsafe fn update_descriptor_sets(
        &self,
        device: vk::Device,
        descriptor_writes: &[WriteDescriptorSet],
        descriptor_copies: &[CopyDescriptorSet],
    ) -> () {
        unsafe {
            self._1_0.UpdateDescriptorSets(
                device.abi(), 
                descriptor_writes.len() as _, 
                descriptor_writes.abi(), 
                descriptor_copies.len() as _, 
                descriptor_copies.abi(), 
            );
        }
    }
    /// ```c
    /// VkResult vkWaitForFences(VkDevice device, uint32_t fenceCount, VkFence const* pFences, VkBool32 waitAll, uint64_t timeout)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Timeout]
    pub unsafe fn wait_for_fences(
        &self,
        device: vk::Device,
        fences: &[vk::Fence],
        wait_all: bool,
        timeout: uint64_t,
    ) -> crate::Result<Result> {
        unsafe {
            let _r = self._1_0.WaitForFences(
                device.abi(), 
                fences.len() as _, 
                fences.abi(), 
                wait_all.abi(), 
                timeout.abi(), 
            ).vk();
            _r.result(|| Some(_r))
        }
    }
}

/// `Vulkan 1.1` DeviceCommands
impl Device {
    /// ```c
    /// VkResult vkBindBufferMemory2(VkDevice device, uint32_t bindInfoCount, VkBindBufferMemoryInfo const* pBindInfos)
    /// ```
    pub unsafe fn bind_buffer_memory2(
        &self,
        device: vk::Device,
        bind_infos: &[BindBufferMemoryInfo],
    ) -> crate::Result<()> {
        unsafe {
            let _r = self._1_1.BindBufferMemory2(
                device.abi(), 
                bind_infos.len() as _, 
                bind_infos.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkResult vkBindImageMemory2(VkDevice device, uint32_t bindInfoCount, VkBindImageMemoryInfo const* pBindInfos)
    /// ```
    pub unsafe fn bind_image_memory2(
        &self,
        device: vk::Device,
        bind_infos: &[BindImageMemoryInfo],
    ) -> crate::Result<()> {
        unsafe {
            let _r = self._1_1.BindImageMemory2(
                device.abi(), 
                bind_infos.len() as _, 
                bind_infos.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// void vkCmdDispatchBase(VkCommandBuffer commandBuffer, uint32_t baseGroupX, uint32_t baseGroupY, uint32_t baseGroupZ, uint32_t groupCountX, uint32_t groupCountY, uint32_t groupCountZ)
    /// ```
    pub unsafe fn cmd_dispatch_base(
        &self,
        command_buffer: vk::CommandBuffer,
        base_group_x: uint32_t,
        base_group_y: uint32_t,
        base_group_z: uint32_t,
        group_count_x: uint32_t,
        group_count_y: uint32_t,
        group_count_z: uint32_t,
    ) -> () {
        unsafe {
            self._1_1.CmdDispatchBase(
                command_buffer.abi(), 
                base_group_x.abi(), 
                base_group_y.abi(), 
                base_group_z.abi(), 
                group_count_x.abi(), 
                group_count_y.abi(), 
                group_count_z.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetDeviceMask(VkCommandBuffer commandBuffer, uint32_t deviceMask)
    /// ```
    pub unsafe fn cmd_set_device_mask(
        &self,
        command_buffer: vk::CommandBuffer,
        device_mask: uint32_t,
    ) -> () {
        unsafe {
            self._1_1.CmdSetDeviceMask(
                command_buffer.abi(), 
                device_mask.abi(), 
            );
        }
    }
    /// ```c
    /// VkResult vkCreateDescriptorUpdateTemplate(VkDevice device, VkDescriptorUpdateTemplateCreateInfo const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkDescriptorUpdateTemplate* pDescriptorUpdateTemplate)
    /// ```
    pub unsafe fn create_descriptor_update_template(
        &self,
        device: vk::Device,
        create_info: &DescriptorUpdateTemplateCreateInfo,
    ) -> crate::Result<vk::DescriptorUpdateTemplate> {
        unsafe {
            let mut _v: Option<vk::DescriptorUpdateTemplate> = Default::default();
            let _r = self._1_1.CreateDescriptorUpdateTemplate(
                device.abi(), 
                create_info.abi(), 
                Default::default(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| _v)
        }
    }
    /// ```c
    /// VkResult vkCreateSamplerYcbcrConversion(VkDevice device, VkSamplerYcbcrConversionCreateInfo const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkSamplerYcbcrConversion* pYcbcrConversion)
    /// ```
    pub unsafe fn create_sampler_ycbcr_conversion(
        &self,
        device: vk::Device,
        create_info: &SamplerYcbcrConversionCreateInfo,
    ) -> crate::Result<vk::SamplerYcbcrConversion> {
        unsafe {
            let mut _v: Option<vk::SamplerYcbcrConversion> = Default::default();
            let _r = self._1_1.CreateSamplerYcbcrConversion(
                device.abi(), 
                create_info.abi(), 
                Default::default(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| _v)
        }
    }
    /// ```c
    /// void vkDestroyDescriptorUpdateTemplate(VkDevice device, VkDescriptorUpdateTemplate descriptorUpdateTemplate, VkAllocationCallbacks const* pAllocator)
    /// ```
    pub unsafe fn destroy_descriptor_update_template(
        &self,
        device: vk::Device,
        descriptor_update_template: Option<vk::DescriptorUpdateTemplate>,
    ) -> () {
        unsafe {
            self._1_1.DestroyDescriptorUpdateTemplate(
                device.abi(), 
                descriptor_update_template.abi(), 
                Default::default(), 
            );
        }
    }
    /// ```c
    /// void vkDestroySamplerYcbcrConversion(VkDevice device, VkSamplerYcbcrConversion ycbcrConversion, VkAllocationCallbacks const* pAllocator)
    /// ```
    pub unsafe fn destroy_sampler_ycbcr_conversion(
        &self,
        device: vk::Device,
        ycbcr_conversion: Option<vk::SamplerYcbcrConversion>,
    ) -> () {
        unsafe {
            self._1_1.DestroySamplerYcbcrConversion(
                device.abi(), 
                ycbcr_conversion.abi(), 
                Default::default(), 
            );
        }
    }
    /// ```c
    /// void vkGetBufferMemoryRequirements2(VkDevice device, VkBufferMemoryRequirementsInfo2 const* pInfo, VkMemoryRequirements2* pMemoryRequirements)
    /// ```
    pub unsafe fn get_buffer_memory_requirements2(
        &self,
        device: vk::Device,
        info: &BufferMemoryRequirementsInfo2,
        memory_requirements: &mut MemoryRequirements2,
    ) -> () {
        unsafe {
            self._1_1.GetBufferMemoryRequirements2(
                device.abi(), 
                info.abi(), 
                memory_requirements.abi(), 
            );
        }
    }
    /// ```c
    /// void vkGetDescriptorSetLayoutSupport(VkDevice device, VkDescriptorSetLayoutCreateInfo const* pCreateInfo, VkDescriptorSetLayoutSupport* pSupport)
    /// ```
    pub unsafe fn get_descriptor_set_layout_support(
        &self,
        device: vk::Device,
        create_info: &DescriptorSetLayoutCreateInfo,
        support: &mut DescriptorSetLayoutSupport,
    ) -> () {
        unsafe {
            self._1_1.GetDescriptorSetLayoutSupport(
                device.abi(), 
                create_info.abi(), 
                support.abi(), 
            );
        }
    }
    /// ```c
    /// void vkGetDeviceGroupPeerMemoryFeatures(VkDevice device, uint32_t heapIndex, uint32_t localDeviceIndex, uint32_t remoteDeviceIndex, VkPeerMemoryFeatureFlags* pPeerMemoryFeatures)
    /// ```
    pub unsafe fn get_device_group_peer_memory_features(
        &self,
        device: vk::Device,
        heap_index: uint32_t,
        local_device_index: uint32_t,
        remote_device_index: uint32_t,
    ) -> PeerMemoryFeatureFlags {
        unsafe {
            let mut _v: PeerMemoryFeatureFlags = Default::default();
            self._1_1.GetDeviceGroupPeerMemoryFeatures(
                device.abi(), 
                heap_index.abi(), 
                local_device_index.abi(), 
                remote_device_index.abi(), 
                (&mut _v).abi(), 
            );
            _v
        }
    }
    /// ```c
    /// void vkGetDeviceQueue2(VkDevice device, VkDeviceQueueInfo2 const* pQueueInfo, VkQueue* pQueue)
    /// ```
    pub unsafe fn get_device_queue2(
        &self,
        device: vk::Device,
        queue_info: &DeviceQueueInfo2,
    ) -> vk::Queue {
        unsafe {
            let mut _v: Option<vk::Queue> = Default::default();
            self._1_1.GetDeviceQueue2(
                device.abi(), 
                queue_info.abi(), 
                (&mut _v).abi(), 
            );
            unsafe { _v.unwrap_unchecked() }
        }
    }
    /// ```c
    /// void vkGetImageMemoryRequirements2(VkDevice device, VkImageMemoryRequirementsInfo2 const* pInfo, VkMemoryRequirements2* pMemoryRequirements)
    /// ```
    pub unsafe fn get_image_memory_requirements2(
        &self,
        device: vk::Device,
        info: &ImageMemoryRequirementsInfo2,
        memory_requirements: &mut MemoryRequirements2,
    ) -> () {
        unsafe {
            self._1_1.GetImageMemoryRequirements2(
                device.abi(), 
                info.abi(), 
                memory_requirements.abi(), 
            );
        }
    }
    /// ```c
    /// void vkGetImageSparseMemoryRequirements2(VkDevice device, VkImageSparseMemoryRequirementsInfo2 const* pInfo, uint32_t* pSparseMemoryRequirementCount, VkSparseImageMemoryRequirements2* pSparseMemoryRequirements)
    /// ```
    pub unsafe fn get_image_sparse_memory_requirements2(
        &self,
        device: vk::Device,
        info: &ImageSparseMemoryRequirementsInfo2,
        sparse_memory_requirements: Option<&mut ::alloc::vec::Vec<SparseImageMemoryRequirements2>>,
    ) -> uint32_t {
        unsafe {
            let mut _c: uint32_t = Default::default();
            self._1_1.GetImageSparseMemoryRequirements2(
                device.abi(), 
                info.abi(), 
                &mut _c, 
                Default::default(), 
            );
            if let Some(_b) = sparse_memory_requirements {
                _b.reserve(_c as usize);
                self._1_1.GetImageSparseMemoryRequirements2(
                    device.abi(), 
                    info.abi(), 
                    &mut _c, 
                    (&mut **_b).abi(), 
                );
                _b.set_len(_b.len() + _c as usize);
            }
            _c
        }
    }
    /// ```c
    /// void vkTrimCommandPool(VkDevice device, VkCommandPool commandPool, VkCommandPoolTrimFlags flags)
    /// ```
    pub unsafe fn trim_command_pool(
        &self,
        device: vk::Device,
        command_pool: vk::CommandPool,
        flags: CommandPoolTrimFlags,
    ) -> () {
        unsafe {
            self._1_1.TrimCommandPool(
                device.abi(), 
                command_pool.abi(), 
                flags.abi(), 
            );
        }
    }
    /// ```c
    /// void vkUpdateDescriptorSetWithTemplate(VkDevice device, VkDescriptorSet descriptorSet, VkDescriptorUpdateTemplate descriptorUpdateTemplate, void const* pData)
    /// ```
    pub unsafe fn update_descriptor_set_with_template(
        &self,
        device: vk::Device,
        descriptor_set: vk::DescriptorSet,
        descriptor_update_template: vk::DescriptorUpdateTemplate,
        data: *const void,
    ) -> () {
        unsafe {
            self._1_1.UpdateDescriptorSetWithTemplate(
                device.abi(), 
                descriptor_set.abi(), 
                descriptor_update_template.abi(), 
                data.abi(), 
            );
        }
    }
}

/// `Vulkan 1.2` DeviceCommands
impl Device {
    /// ```c
    /// void vkCmdBeginRenderPass2(VkCommandBuffer commandBuffer, VkRenderPassBeginInfo const* pRenderPassBegin, VkSubpassBeginInfo const* pSubpassBeginInfo)
    /// ```
    pub unsafe fn cmd_begin_render_pass2(
        &self,
        command_buffer: vk::CommandBuffer,
        render_pass_begin: &RenderPassBeginInfo,
        subpass_begin_info: &SubpassBeginInfo,
    ) -> () {
        unsafe {
            self._1_2.CmdBeginRenderPass2(
                command_buffer.abi(), 
                render_pass_begin.abi(), 
                subpass_begin_info.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdDrawIndexedIndirectCount(VkCommandBuffer commandBuffer, VkBuffer buffer, VkDeviceSize offset, VkBuffer countBuffer, VkDeviceSize countBufferOffset, uint32_t maxDrawCount, uint32_t stride)
    /// ```
    pub unsafe fn cmd_draw_indexed_indirect_count(
        &self,
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: DeviceSize,
        count_buffer: vk::Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: uint32_t,
        stride: uint32_t,
    ) -> () {
        unsafe {
            self._1_2.CmdDrawIndexedIndirectCount(
                command_buffer.abi(), 
                buffer.abi(), 
                offset.abi(), 
                count_buffer.abi(), 
                count_buffer_offset.abi(), 
                max_draw_count.abi(), 
                stride.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdDrawIndirectCount(VkCommandBuffer commandBuffer, VkBuffer buffer, VkDeviceSize offset, VkBuffer countBuffer, VkDeviceSize countBufferOffset, uint32_t maxDrawCount, uint32_t stride)
    /// ```
    pub unsafe fn cmd_draw_indirect_count(
        &self,
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: DeviceSize,
        count_buffer: vk::Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: uint32_t,
        stride: uint32_t,
    ) -> () {
        unsafe {
            self._1_2.CmdDrawIndirectCount(
                command_buffer.abi(), 
                buffer.abi(), 
                offset.abi(), 
                count_buffer.abi(), 
                count_buffer_offset.abi(), 
                max_draw_count.abi(), 
                stride.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdEndRenderPass2(VkCommandBuffer commandBuffer, VkSubpassEndInfo const* pSubpassEndInfo)
    /// ```
    pub unsafe fn cmd_end_render_pass2(
        &self,
        command_buffer: vk::CommandBuffer,
        subpass_end_info: &SubpassEndInfo,
    ) -> () {
        unsafe {
            self._1_2.CmdEndRenderPass2(
                command_buffer.abi(), 
                subpass_end_info.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdNextSubpass2(VkCommandBuffer commandBuffer, VkSubpassBeginInfo const* pSubpassBeginInfo, VkSubpassEndInfo const* pSubpassEndInfo)
    /// ```
    pub unsafe fn cmd_next_subpass2(
        &self,
        command_buffer: vk::CommandBuffer,
        subpass_begin_info: &SubpassBeginInfo,
        subpass_end_info: &SubpassEndInfo,
    ) -> () {
        unsafe {
            self._1_2.CmdNextSubpass2(
                command_buffer.abi(), 
                subpass_begin_info.abi(), 
                subpass_end_info.abi(), 
            );
        }
    }
    /// ```c
    /// VkResult vkCreateRenderPass2(VkDevice device, VkRenderPassCreateInfo2 const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkRenderPass* pRenderPass)
    /// ```
    pub unsafe fn create_render_pass2(
        &self,
        device: vk::Device,
        create_info: &RenderPassCreateInfo2,
    ) -> crate::Result<vk::RenderPass> {
        unsafe {
            let mut _v: Option<vk::RenderPass> = Default::default();
            let _r = self._1_2.CreateRenderPass2(
                device.abi(), 
                create_info.abi(), 
                Default::default(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| _v)
        }
    }
    /// ```c
    /// VkDeviceAddress vkGetBufferDeviceAddress(VkDevice device, VkBufferDeviceAddressInfo const* pInfo)
    /// ```
    pub unsafe fn get_buffer_device_address(
        &self,
        device: vk::Device,
        info: &BufferDeviceAddressInfo,
    ) -> DeviceAddress {
        unsafe {
            let _r = self._1_2.GetBufferDeviceAddress(
                device.abi(), 
                info.abi(), 
            );
            _r
        }
    }
    /// ```c
    /// uint64_t vkGetBufferOpaqueCaptureAddress(VkDevice device, VkBufferDeviceAddressInfo const* pInfo)
    /// ```
    pub unsafe fn get_buffer_opaque_capture_address(
        &self,
        device: vk::Device,
        info: &BufferDeviceAddressInfo,
    ) -> uint64_t {
        unsafe {
            let _r = self._1_2.GetBufferOpaqueCaptureAddress(
                device.abi(), 
                info.abi(), 
            );
            _r
        }
    }
    /// ```c
    /// uint64_t vkGetDeviceMemoryOpaqueCaptureAddress(VkDevice device, VkDeviceMemoryOpaqueCaptureAddressInfo const* pInfo)
    /// ```
    pub unsafe fn get_device_memory_opaque_capture_address(
        &self,
        device: vk::Device,
        info: &DeviceMemoryOpaqueCaptureAddressInfo,
    ) -> uint64_t {
        unsafe {
            let _r = self._1_2.GetDeviceMemoryOpaqueCaptureAddress(
                device.abi(), 
                info.abi(), 
            );
            _r
        }
    }
    /// ```c
    /// VkResult vkGetSemaphoreCounterValue(VkDevice device, VkSemaphore semaphore, uint64_t* pValue)
    /// ```
    pub unsafe fn get_semaphore_counter_value(
        &self,
        device: vk::Device,
        semaphore: vk::Semaphore,
    ) -> crate::Result<uint64_t> {
        unsafe {
            let mut _v: uint64_t = Default::default();
            let _r = self._1_2.GetSemaphoreCounterValue(
                device.abi(), 
                semaphore.abi(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| Some(_v))
        }
    }
    /// ```c
    /// void vkResetQueryPool(VkDevice device, VkQueryPool queryPool, uint32_t firstQuery, uint32_t queryCount)
    /// ```
    pub unsafe fn reset_query_pool(
        &self,
        device: vk::Device,
        query_pool: vk::QueryPool,
        first_query: uint32_t,
        query_count: uint32_t,
    ) -> () {
        unsafe {
            self._1_2.ResetQueryPool(
                device.abi(), 
                query_pool.abi(), 
                first_query.abi(), 
                query_count.abi(), 
            );
        }
    }
    /// ```c
    /// VkResult vkSignalSemaphore(VkDevice device, VkSemaphoreSignalInfo const* pSignalInfo)
    /// ```
    pub unsafe fn signal_semaphore(
        &self,
        device: vk::Device,
        signal_info: &SemaphoreSignalInfo,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self._1_2.SignalSemaphore(
                device.abi(), 
                signal_info.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkResult vkWaitSemaphores(VkDevice device, VkSemaphoreWaitInfo const* pWaitInfo, uint64_t timeout)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Timeout]
    pub unsafe fn wait_semaphores(
        &self,
        device: vk::Device,
        wait_info: &SemaphoreWaitInfo,
        timeout: uint64_t,
    ) -> crate::Result<Result> {
        unsafe {
            let _r = self._1_2.WaitSemaphores(
                device.abi(), 
                wait_info.abi(), 
                timeout.abi(), 
            ).vk();
            _r.result(|| Some(_r))
        }
    }
}

/// `Vulkan 1.3` DeviceCommands
impl Device {
    /// ```c
    /// void vkCmdBeginRendering(VkCommandBuffer commandBuffer, VkRenderingInfo const* pRenderingInfo)
    /// ```
    pub unsafe fn cmd_begin_rendering(
        &self,
        command_buffer: vk::CommandBuffer,
        rendering_info: &RenderingInfo,
    ) -> () {
        unsafe {
            self._1_3.CmdBeginRendering(
                command_buffer.abi(), 
                rendering_info.abi(), 
            );
        }
    }
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
            self._1_3.CmdBindVertexBuffers2(
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
    /// void vkCmdBlitImage2(VkCommandBuffer commandBuffer, VkBlitImageInfo2 const* pBlitImageInfo)
    /// ```
    pub unsafe fn cmd_blit_image2(
        &self,
        command_buffer: vk::CommandBuffer,
        blit_image_info: &BlitImageInfo2,
    ) -> () {
        unsafe {
            self._1_3.CmdBlitImage2(
                command_buffer.abi(), 
                blit_image_info.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdCopyBuffer2(VkCommandBuffer commandBuffer, VkCopyBufferInfo2 const* pCopyBufferInfo)
    /// ```
    pub unsafe fn cmd_copy_buffer2(
        &self,
        command_buffer: vk::CommandBuffer,
        copy_buffer_info: &CopyBufferInfo2,
    ) -> () {
        unsafe {
            self._1_3.CmdCopyBuffer2(
                command_buffer.abi(), 
                copy_buffer_info.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdCopyBufferToImage2(VkCommandBuffer commandBuffer, VkCopyBufferToImageInfo2 const* pCopyBufferToImageInfo)
    /// ```
    pub unsafe fn cmd_copy_buffer_to_image2(
        &self,
        command_buffer: vk::CommandBuffer,
        copy_buffer_to_image_info: &CopyBufferToImageInfo2,
    ) -> () {
        unsafe {
            self._1_3.CmdCopyBufferToImage2(
                command_buffer.abi(), 
                copy_buffer_to_image_info.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdCopyImage2(VkCommandBuffer commandBuffer, VkCopyImageInfo2 const* pCopyImageInfo)
    /// ```
    pub unsafe fn cmd_copy_image2(
        &self,
        command_buffer: vk::CommandBuffer,
        copy_image_info: &CopyImageInfo2,
    ) -> () {
        unsafe {
            self._1_3.CmdCopyImage2(
                command_buffer.abi(), 
                copy_image_info.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdCopyImageToBuffer2(VkCommandBuffer commandBuffer, VkCopyImageToBufferInfo2 const* pCopyImageToBufferInfo)
    /// ```
    pub unsafe fn cmd_copy_image_to_buffer2(
        &self,
        command_buffer: vk::CommandBuffer,
        copy_image_to_buffer_info: &CopyImageToBufferInfo2,
    ) -> () {
        unsafe {
            self._1_3.CmdCopyImageToBuffer2(
                command_buffer.abi(), 
                copy_image_to_buffer_info.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdEndRendering(VkCommandBuffer commandBuffer)
    /// ```
    pub unsafe fn cmd_end_rendering(
        &self,
        command_buffer: vk::CommandBuffer,
    ) -> () {
        unsafe {
            self._1_3.CmdEndRendering(
                command_buffer.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdPipelineBarrier2(VkCommandBuffer commandBuffer, VkDependencyInfo const* pDependencyInfo)
    /// ```
    pub unsafe fn cmd_pipeline_barrier2(
        &self,
        command_buffer: vk::CommandBuffer,
        dependency_info: &DependencyInfo,
    ) -> () {
        unsafe {
            self._1_3.CmdPipelineBarrier2(
                command_buffer.abi(), 
                dependency_info.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdResetEvent2(VkCommandBuffer commandBuffer, VkEvent event, VkPipelineStageFlags2 stageMask)
    /// ```
    pub unsafe fn cmd_reset_event2(
        &self,
        command_buffer: vk::CommandBuffer,
        event: vk::Event,
        stage_mask: PipelineStageFlags2,
    ) -> () {
        unsafe {
            self._1_3.CmdResetEvent2(
                command_buffer.abi(), 
                event.abi(), 
                stage_mask.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdResolveImage2(VkCommandBuffer commandBuffer, VkResolveImageInfo2 const* pResolveImageInfo)
    /// ```
    pub unsafe fn cmd_resolve_image2(
        &self,
        command_buffer: vk::CommandBuffer,
        resolve_image_info: &ResolveImageInfo2,
    ) -> () {
        unsafe {
            self._1_3.CmdResolveImage2(
                command_buffer.abi(), 
                resolve_image_info.abi(), 
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
            self._1_3.CmdSetCullMode(
                command_buffer.abi(), 
                cull_mode.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetDepthBiasEnable(VkCommandBuffer commandBuffer, VkBool32 depthBiasEnable)
    /// ```
    pub unsafe fn cmd_set_depth_bias_enable(
        &self,
        command_buffer: vk::CommandBuffer,
        depth_bias_enable: bool,
    ) -> () {
        unsafe {
            self._1_3.CmdSetDepthBiasEnable(
                command_buffer.abi(), 
                depth_bias_enable.abi(), 
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
            self._1_3.CmdSetDepthBoundsTestEnable(
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
            self._1_3.CmdSetDepthCompareOp(
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
            self._1_3.CmdSetDepthTestEnable(
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
            self._1_3.CmdSetDepthWriteEnable(
                command_buffer.abi(), 
                depth_write_enable.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetEvent2(VkCommandBuffer commandBuffer, VkEvent event, VkDependencyInfo const* pDependencyInfo)
    /// ```
    pub unsafe fn cmd_set_event2(
        &self,
        command_buffer: vk::CommandBuffer,
        event: vk::Event,
        dependency_info: &DependencyInfo,
    ) -> () {
        unsafe {
            self._1_3.CmdSetEvent2(
                command_buffer.abi(), 
                event.abi(), 
                dependency_info.abi(), 
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
            self._1_3.CmdSetFrontFace(
                command_buffer.abi(), 
                front_face.abi(), 
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
            self._1_3.CmdSetPrimitiveRestartEnable(
                command_buffer.abi(), 
                primitive_restart_enable.abi(), 
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
            self._1_3.CmdSetPrimitiveTopology(
                command_buffer.abi(), 
                primitive_topology.abi(), 
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
            self._1_3.CmdSetRasterizerDiscardEnable(
                command_buffer.abi(), 
                rasterizer_discard_enable.abi(), 
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
            self._1_3.CmdSetScissorWithCount(
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
            self._1_3.CmdSetStencilOp(
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
            self._1_3.CmdSetStencilTestEnable(
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
            self._1_3.CmdSetViewportWithCount(
                command_buffer.abi(), 
                viewports.len() as _, 
                viewports.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdWaitEvents2(VkCommandBuffer commandBuffer, uint32_t eventCount, VkEvent const* pEvents, VkDependencyInfo const* pDependencyInfos)
    /// ```
    pub unsafe fn cmd_wait_events2(
        &self,
        command_buffer: vk::CommandBuffer,
        events: &[vk::Event],
        dependency_infos: &[DependencyInfo],
    ) -> () {
        unsafe {
            self._1_3.CmdWaitEvents2(
                command_buffer.abi(), 
                events.len() as _, 
                events.abi(), 
                dependency_infos.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdWriteTimestamp2(VkCommandBuffer commandBuffer, VkPipelineStageFlags2 stage, VkQueryPool queryPool, uint32_t query)
    /// ```
    pub unsafe fn cmd_write_timestamp2(
        &self,
        command_buffer: vk::CommandBuffer,
        stage: PipelineStageFlags2,
        query_pool: vk::QueryPool,
        query: uint32_t,
    ) -> () {
        unsafe {
            self._1_3.CmdWriteTimestamp2(
                command_buffer.abi(), 
                stage.abi(), 
                query_pool.abi(), 
                query.abi(), 
            );
        }
    }
    /// ```c
    /// VkResult vkCreatePrivateDataSlot(VkDevice device, VkPrivateDataSlotCreateInfo const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkPrivateDataSlot* pPrivateDataSlot)
    /// ```
    pub unsafe fn create_private_data_slot(
        &self,
        device: vk::Device,
        create_info: &PrivateDataSlotCreateInfo,
    ) -> crate::Result<vk::PrivateDataSlot> {
        unsafe {
            let mut _v: Option<vk::PrivateDataSlot> = Default::default();
            let _r = self._1_3.CreatePrivateDataSlot(
                device.abi(), 
                create_info.abi(), 
                Default::default(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| _v)
        }
    }
    /// ```c
    /// void vkDestroyPrivateDataSlot(VkDevice device, VkPrivateDataSlot privateDataSlot, VkAllocationCallbacks const* pAllocator)
    /// ```
    pub unsafe fn destroy_private_data_slot(
        &self,
        device: vk::Device,
        private_data_slot: Option<vk::PrivateDataSlot>,
    ) -> () {
        unsafe {
            self._1_3.DestroyPrivateDataSlot(
                device.abi(), 
                private_data_slot.abi(), 
                Default::default(), 
            );
        }
    }
    /// ```c
    /// void vkGetDeviceBufferMemoryRequirements(VkDevice device, VkDeviceBufferMemoryRequirements const* pInfo, VkMemoryRequirements2* pMemoryRequirements)
    /// ```
    pub unsafe fn get_device_buffer_memory_requirements(
        &self,
        device: vk::Device,
        info: &DeviceBufferMemoryRequirements,
        memory_requirements: &mut MemoryRequirements2,
    ) -> () {
        unsafe {
            self._1_3.GetDeviceBufferMemoryRequirements(
                device.abi(), 
                info.abi(), 
                memory_requirements.abi(), 
            );
        }
    }
    /// ```c
    /// void vkGetDeviceImageMemoryRequirements(VkDevice device, VkDeviceImageMemoryRequirements const* pInfo, VkMemoryRequirements2* pMemoryRequirements)
    /// ```
    pub unsafe fn get_device_image_memory_requirements(
        &self,
        device: vk::Device,
        info: &DeviceImageMemoryRequirements,
        memory_requirements: &mut MemoryRequirements2,
    ) -> () {
        unsafe {
            self._1_3.GetDeviceImageMemoryRequirements(
                device.abi(), 
                info.abi(), 
                memory_requirements.abi(), 
            );
        }
    }
    /// ```c
    /// void vkGetDeviceImageSparseMemoryRequirements(VkDevice device, VkDeviceImageMemoryRequirements const* pInfo, uint32_t* pSparseMemoryRequirementCount, VkSparseImageMemoryRequirements2* pSparseMemoryRequirements)
    /// ```
    pub unsafe fn get_device_image_sparse_memory_requirements(
        &self,
        device: vk::Device,
        info: &DeviceImageMemoryRequirements,
        sparse_memory_requirements: Option<&mut ::alloc::vec::Vec<SparseImageMemoryRequirements2>>,
    ) -> uint32_t {
        unsafe {
            let mut _c: uint32_t = Default::default();
            self._1_3.GetDeviceImageSparseMemoryRequirements(
                device.abi(), 
                info.abi(), 
                &mut _c, 
                Default::default(), 
            );
            if let Some(_b) = sparse_memory_requirements {
                _b.reserve(_c as usize);
                self._1_3.GetDeviceImageSparseMemoryRequirements(
                    device.abi(), 
                    info.abi(), 
                    &mut _c, 
                    (&mut **_b).abi(), 
                );
                _b.set_len(_b.len() + _c as usize);
            }
            _c
        }
    }
    /// ```c
    /// void vkGetPrivateData(VkDevice device, VkObjectType objectType, uint64_t objectHandle, VkPrivateDataSlot privateDataSlot, uint64_t* pData)
    /// ```
    pub unsafe fn get_private_data(
        &self,
        device: vk::Device,
        object_type: ObjectType,
        object_handle: uint64_t,
        private_data_slot: vk::PrivateDataSlot,
    ) -> uint64_t {
        unsafe {
            let mut _v: uint64_t = Default::default();
            self._1_3.GetPrivateData(
                device.abi(), 
                object_type.abi(), 
                object_handle.abi(), 
                private_data_slot.abi(), 
                (&mut _v).abi(), 
            );
            _v
        }
    }
    /// ```c
    /// VkResult vkQueueSubmit2(VkQueue queue, uint32_t submitCount, VkSubmitInfo2 const* pSubmits, VkFence fence)
    /// ```
    pub unsafe fn queue_submit2(
        &self,
        queue: vk::Queue,
        submits: &[SubmitInfo2],
        fence: Option<vk::Fence>,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self._1_3.QueueSubmit2(
                queue.abi(), 
                submits.len() as _, 
                submits.abi(), 
                fence.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkResult vkSetPrivateData(VkDevice device, VkObjectType objectType, uint64_t objectHandle, VkPrivateDataSlot privateDataSlot, uint64_t data)
    /// ```
    pub unsafe fn set_private_data(
        &self,
        device: vk::Device,
        object_type: ObjectType,
        object_handle: uint64_t,
        private_data_slot: vk::PrivateDataSlot,
        data: uint64_t,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self._1_3.SetPrivateData(
                device.abi(), 
                object_type.abi(), 
                object_handle.abi(), 
                private_data_slot.abi(), 
                data.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
}

/// `Vulkan 1.4` DeviceCommands
impl Device {
    /// ```c
    /// void vkCmdBindDescriptorSets2(VkCommandBuffer commandBuffer, VkBindDescriptorSetsInfo const* pBindDescriptorSetsInfo)
    /// ```
    pub unsafe fn cmd_bind_descriptor_sets2(
        &self,
        command_buffer: vk::CommandBuffer,
        bind_descriptor_sets_info: &BindDescriptorSetsInfo,
    ) -> () {
        unsafe {
            self._1_4.CmdBindDescriptorSets2(
                command_buffer.abi(), 
                bind_descriptor_sets_info.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdBindIndexBuffer2(VkCommandBuffer commandBuffer, VkBuffer buffer, VkDeviceSize offset, VkDeviceSize size, VkIndexType indexType)
    /// ```
    pub unsafe fn cmd_bind_index_buffer2(
        &self,
        command_buffer: vk::CommandBuffer,
        buffer: Option<vk::Buffer>,
        offset: DeviceSize,
        size: DeviceSize,
        index_type: IndexType,
    ) -> () {
        unsafe {
            self._1_4.CmdBindIndexBuffer2(
                command_buffer.abi(), 
                buffer.abi(), 
                offset.abi(), 
                size.abi(), 
                index_type.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdPushConstants2(VkCommandBuffer commandBuffer, VkPushConstantsInfo const* pPushConstantsInfo)
    /// ```
    pub unsafe fn cmd_push_constants2(
        &self,
        command_buffer: vk::CommandBuffer,
        push_constants_info: &PushConstantsInfo,
    ) -> () {
        unsafe {
            self._1_4.CmdPushConstants2(
                command_buffer.abi(), 
                push_constants_info.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdPushDescriptorSet(VkCommandBuffer commandBuffer, VkPipelineBindPoint pipelineBindPoint, VkPipelineLayout layout, uint32_t set, uint32_t descriptorWriteCount, VkWriteDescriptorSet const* pDescriptorWrites)
    /// ```
    pub unsafe fn cmd_push_descriptor_set(
        &self,
        command_buffer: vk::CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        layout: vk::PipelineLayout,
        set: uint32_t,
        descriptor_writes: &[WriteDescriptorSet],
    ) -> () {
        unsafe {
            self._1_4.CmdPushDescriptorSet(
                command_buffer.abi(), 
                pipeline_bind_point.abi(), 
                layout.abi(), 
                set.abi(), 
                descriptor_writes.len() as _, 
                descriptor_writes.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdPushDescriptorSet2(VkCommandBuffer commandBuffer, VkPushDescriptorSetInfo const* pPushDescriptorSetInfo)
    /// ```
    pub unsafe fn cmd_push_descriptor_set2(
        &self,
        command_buffer: vk::CommandBuffer,
        push_descriptor_set_info: &PushDescriptorSetInfo,
    ) -> () {
        unsafe {
            self._1_4.CmdPushDescriptorSet2(
                command_buffer.abi(), 
                push_descriptor_set_info.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdPushDescriptorSetWithTemplate(VkCommandBuffer commandBuffer, VkDescriptorUpdateTemplate descriptorUpdateTemplate, VkPipelineLayout layout, uint32_t set, void const* pData)
    /// ```
    pub unsafe fn cmd_push_descriptor_set_with_template(
        &self,
        command_buffer: vk::CommandBuffer,
        descriptor_update_template: vk::DescriptorUpdateTemplate,
        layout: vk::PipelineLayout,
        set: uint32_t,
        data: *const void,
    ) -> () {
        unsafe {
            self._1_4.CmdPushDescriptorSetWithTemplate(
                command_buffer.abi(), 
                descriptor_update_template.abi(), 
                layout.abi(), 
                set.abi(), 
                data.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdPushDescriptorSetWithTemplate2(VkCommandBuffer commandBuffer, VkPushDescriptorSetWithTemplateInfo const* pPushDescriptorSetWithTemplateInfo)
    /// ```
    pub unsafe fn cmd_push_descriptor_set_with_template2(
        &self,
        command_buffer: vk::CommandBuffer,
        push_descriptor_set_with_template_info: &PushDescriptorSetWithTemplateInfo,
    ) -> () {
        unsafe {
            self._1_4.CmdPushDescriptorSetWithTemplate2(
                command_buffer.abi(), 
                push_descriptor_set_with_template_info.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetLineStipple(VkCommandBuffer commandBuffer, uint32_t lineStippleFactor, uint16_t lineStipplePattern)
    /// ```
    pub unsafe fn cmd_set_line_stipple(
        &self,
        command_buffer: vk::CommandBuffer,
        line_stipple_factor: uint32_t,
        line_stipple_pattern: uint16_t,
    ) -> () {
        unsafe {
            self._1_4.CmdSetLineStipple(
                command_buffer.abi(), 
                line_stipple_factor.abi(), 
                line_stipple_pattern.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetRenderingAttachmentLocations(VkCommandBuffer commandBuffer, VkRenderingAttachmentLocationInfo const* pLocationInfo)
    /// ```
    pub unsafe fn cmd_set_rendering_attachment_locations(
        &self,
        command_buffer: vk::CommandBuffer,
        location_info: &RenderingAttachmentLocationInfo,
    ) -> () {
        unsafe {
            self._1_4.CmdSetRenderingAttachmentLocations(
                command_buffer.abi(), 
                location_info.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetRenderingInputAttachmentIndices(VkCommandBuffer commandBuffer, VkRenderingInputAttachmentIndexInfo const* pInputAttachmentIndexInfo)
    /// ```
    pub unsafe fn cmd_set_rendering_input_attachment_indices(
        &self,
        command_buffer: vk::CommandBuffer,
        input_attachment_index_info: &RenderingInputAttachmentIndexInfo,
    ) -> () {
        unsafe {
            self._1_4.CmdSetRenderingInputAttachmentIndices(
                command_buffer.abi(), 
                input_attachment_index_info.abi(), 
            );
        }
    }
    /// ```c
    /// VkResult vkCopyImageToImage(VkDevice device, VkCopyImageToImageInfo const* pCopyImageToImageInfo)
    /// ```
    pub unsafe fn copy_image_to_image(
        &self,
        device: vk::Device,
        copy_image_to_image_info: &CopyImageToImageInfo,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self._1_4.CopyImageToImage(
                device.abi(), 
                copy_image_to_image_info.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkResult vkCopyImageToMemory(VkDevice device, VkCopyImageToMemoryInfo const* pCopyImageToMemoryInfo)
    /// ```
    pub unsafe fn copy_image_to_memory(
        &self,
        device: vk::Device,
        copy_image_to_memory_info: &CopyImageToMemoryInfo,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self._1_4.CopyImageToMemory(
                device.abi(), 
                copy_image_to_memory_info.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkResult vkCopyMemoryToImage(VkDevice device, VkCopyMemoryToImageInfo const* pCopyMemoryToImageInfo)
    /// ```
    pub unsafe fn copy_memory_to_image(
        &self,
        device: vk::Device,
        copy_memory_to_image_info: &CopyMemoryToImageInfo,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self._1_4.CopyMemoryToImage(
                device.abi(), 
                copy_memory_to_image_info.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// void vkGetDeviceImageSubresourceLayout(VkDevice device, VkDeviceImageSubresourceInfo const* pInfo, VkSubresourceLayout2* pLayout)
    /// ```
    pub unsafe fn get_device_image_subresource_layout(
        &self,
        device: vk::Device,
        info: &DeviceImageSubresourceInfo,
        layout: &mut SubresourceLayout2,
    ) -> () {
        unsafe {
            self._1_4.GetDeviceImageSubresourceLayout(
                device.abi(), 
                info.abi(), 
                layout.abi(), 
            );
        }
    }
    /// ```c
    /// void vkGetImageSubresourceLayout2(VkDevice device, VkImage image, VkImageSubresource2 const* pSubresource, VkSubresourceLayout2* pLayout)
    /// ```
    pub unsafe fn get_image_subresource_layout2(
        &self,
        device: vk::Device,
        image: vk::Image,
        subresource: &ImageSubresource2,
        layout: &mut SubresourceLayout2,
    ) -> () {
        unsafe {
            self._1_4.GetImageSubresourceLayout2(
                device.abi(), 
                image.abi(), 
                subresource.abi(), 
                layout.abi(), 
            );
        }
    }
    /// ```c
    /// void vkGetRenderingAreaGranularity(VkDevice device, VkRenderingAreaInfo const* pRenderingAreaInfo, VkExtent2D* pGranularity)
    /// ```
    pub unsafe fn get_rendering_area_granularity(
        &self,
        device: vk::Device,
        rendering_area_info: &RenderingAreaInfo,
    ) -> Extent2D {
        unsafe {
            let mut _v: Extent2D = Default::default();
            self._1_4.GetRenderingAreaGranularity(
                device.abi(), 
                rendering_area_info.abi(), 
                (&mut _v).abi(), 
            );
            _v
        }
    }
    /// ```c
    /// VkResult vkMapMemory2(VkDevice device, VkMemoryMapInfo const* pMemoryMapInfo, void** ppData)
    /// ```
    pub unsafe fn map_memory2(
        &self,
        device: vk::Device,
        memory_map_info: &MemoryMapInfo,
        p_data: *mut *mut void,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self._1_4.MapMemory2(
                device.abi(), 
                memory_map_info.abi(), 
                p_data.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkResult vkTransitionImageLayout(VkDevice device, uint32_t transitionCount, VkHostImageLayoutTransitionInfo const* pTransitions)
    /// ```
    pub unsafe fn transition_image_layout(
        &self,
        device: vk::Device,
        transitions: &[HostImageLayoutTransitionInfo],
    ) -> crate::Result<()> {
        unsafe {
            let _r = self._1_4.TransitionImageLayout(
                device.abi(), 
                transitions.len() as _, 
                transitions.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkResult vkUnmapMemory2(VkDevice device, VkMemoryUnmapInfo const* pMemoryUnmapInfo)
    /// ```
    pub unsafe fn unmap_memory2(
        &self,
        device: vk::Device,
        memory_unmap_info: &MemoryUnmapInfo,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self._1_4.UnmapMemory2(
                device.abi(), 
                memory_unmap_info.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
}

/// Device object
pub trait CoreDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkAllocateCommandBuffers(VkDevice device, VkCommandBufferAllocateInfo const* pAllocateInfo, VkCommandBuffer* pCommandBuffers)
    /// ```
    unsafe fn allocate_command_buffers(
        &self,
        allocate_info: &CommandBufferAllocateInfo,
        command_buffers: &mut [Option<vk::CommandBuffer>],
    ) -> crate::Result<()> {
        unsafe {
            self.commands().allocate_command_buffers(
                self.raw(),
                allocate_info,
                command_buffers,
            )
        }
    }
    /// ```c
    /// VkResult vkAllocateDescriptorSets(VkDevice device, VkDescriptorSetAllocateInfo const* pAllocateInfo, VkDescriptorSet* pDescriptorSets)
    /// ```
    unsafe fn allocate_descriptor_sets(
        &self,
        allocate_info: &DescriptorSetAllocateInfo,
        descriptor_sets: &mut [Option<vk::DescriptorSet>],
    ) -> crate::Result<()> {
        unsafe {
            self.commands().allocate_descriptor_sets(
                self.raw(),
                allocate_info,
                descriptor_sets,
            )
        }
    }
    /// ```c
    /// VkResult vkAllocateMemory(VkDevice device, VkMemoryAllocateInfo const* pAllocateInfo, VkAllocationCallbacks const* pAllocator, VkDeviceMemory* pMemory)
    /// ```
    unsafe fn allocate_memory(
        &self,
        allocate_info: &MemoryAllocateInfo,
    ) -> crate::Result<vk::DeviceMemory> {
        unsafe {
            self.commands().allocate_memory(
                self.raw(),
                allocate_info,
            )
        }
    }
    /// ```c
    /// VkResult vkBindBufferMemory(VkDevice device, VkBuffer buffer, VkDeviceMemory memory, VkDeviceSize memoryOffset)
    /// ```
    unsafe fn bind_buffer_memory(
        &self,
        buffer: vk::Buffer,
        memory: vk::DeviceMemory,
        memory_offset: DeviceSize,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().bind_buffer_memory(
                self.raw(),
                buffer,
                memory,
                memory_offset,
            )
        }
    }
    /// ```c
    /// VkResult vkBindImageMemory(VkDevice device, VkImage image, VkDeviceMemory memory, VkDeviceSize memoryOffset)
    /// ```
    unsafe fn bind_image_memory(
        &self,
        image: vk::Image,
        memory: vk::DeviceMemory,
        memory_offset: DeviceSize,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().bind_image_memory(
                self.raw(),
                image,
                memory,
                memory_offset,
            )
        }
    }
    /// ```c
    /// VkResult vkCreateBuffer(VkDevice device, VkBufferCreateInfo const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkBuffer* pBuffer)
    /// ```
    unsafe fn create_buffer(
        &self,
        create_info: &BufferCreateInfo,
    ) -> crate::Result<vk::Buffer> {
        unsafe {
            self.commands().create_buffer(
                self.raw(),
                create_info,
            )
        }
    }
    /// ```c
    /// VkResult vkCreateBufferView(VkDevice device, VkBufferViewCreateInfo const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkBufferView* pView)
    /// ```
    unsafe fn create_buffer_view(
        &self,
        create_info: &BufferViewCreateInfo,
    ) -> crate::Result<vk::BufferView> {
        unsafe {
            self.commands().create_buffer_view(
                self.raw(),
                create_info,
            )
        }
    }
    /// ```c
    /// VkResult vkCreateCommandPool(VkDevice device, VkCommandPoolCreateInfo const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkCommandPool* pCommandPool)
    /// ```
    unsafe fn create_command_pool(
        &self,
        create_info: &CommandPoolCreateInfo,
    ) -> crate::Result<vk::CommandPool> {
        unsafe {
            self.commands().create_command_pool(
                self.raw(),
                create_info,
            )
        }
    }
    /// ```c
    /// VkResult vkCreateComputePipelines(VkDevice device, VkPipelineCache pipelineCache, uint32_t createInfoCount, VkComputePipelineCreateInfo const* pCreateInfos, VkAllocationCallbacks const* pAllocator, VkPipeline* pPipelines)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::PipelineCompileRequiredExt]
    unsafe fn create_compute_pipelines(
        &self,
        pipeline_cache: Option<vk::PipelineCache>,
        create_infos: &[ComputePipelineCreateInfo],
        pipelines: &mut [Option<vk::Pipeline>],
    ) -> crate::Result<Result> {
        unsafe {
            self.commands().create_compute_pipelines(
                self.raw(),
                pipeline_cache,
                create_infos,
                pipelines,
            )
        }
    }
    /// ```c
    /// VkResult vkCreateDescriptorPool(VkDevice device, VkDescriptorPoolCreateInfo const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkDescriptorPool* pDescriptorPool)
    /// ```
    unsafe fn create_descriptor_pool(
        &self,
        create_info: &DescriptorPoolCreateInfo,
    ) -> crate::Result<vk::DescriptorPool> {
        unsafe {
            self.commands().create_descriptor_pool(
                self.raw(),
                create_info,
            )
        }
    }
    /// ```c
    /// VkResult vkCreateDescriptorSetLayout(VkDevice device, VkDescriptorSetLayoutCreateInfo const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkDescriptorSetLayout* pSetLayout)
    /// ```
    unsafe fn create_descriptor_set_layout(
        &self,
        create_info: &DescriptorSetLayoutCreateInfo,
    ) -> crate::Result<vk::DescriptorSetLayout> {
        unsafe {
            self.commands().create_descriptor_set_layout(
                self.raw(),
                create_info,
            )
        }
    }
    /// ```c
    /// VkResult vkCreateEvent(VkDevice device, VkEventCreateInfo const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkEvent* pEvent)
    /// ```
    unsafe fn create_event(
        &self,
        create_info: &EventCreateInfo,
    ) -> crate::Result<vk::Event> {
        unsafe {
            self.commands().create_event(
                self.raw(),
                create_info,
            )
        }
    }
    /// ```c
    /// VkResult vkCreateFence(VkDevice device, VkFenceCreateInfo const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkFence* pFence)
    /// ```
    unsafe fn create_fence(
        &self,
        create_info: &FenceCreateInfo,
    ) -> crate::Result<vk::Fence> {
        unsafe {
            self.commands().create_fence(
                self.raw(),
                create_info,
            )
        }
    }
    /// ```c
    /// VkResult vkCreateFramebuffer(VkDevice device, VkFramebufferCreateInfo const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkFramebuffer* pFramebuffer)
    /// ```
    unsafe fn create_framebuffer(
        &self,
        create_info: &FramebufferCreateInfo,
    ) -> crate::Result<vk::Framebuffer> {
        unsafe {
            self.commands().create_framebuffer(
                self.raw(),
                create_info,
            )
        }
    }
    /// ```c
    /// VkResult vkCreateGraphicsPipelines(VkDevice device, VkPipelineCache pipelineCache, uint32_t createInfoCount, VkGraphicsPipelineCreateInfo const* pCreateInfos, VkAllocationCallbacks const* pAllocator, VkPipeline* pPipelines)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::PipelineCompileRequiredExt]
    unsafe fn create_graphics_pipelines(
        &self,
        pipeline_cache: Option<vk::PipelineCache>,
        create_infos: &[GraphicsPipelineCreateInfo],
        pipelines: &mut [Option<vk::Pipeline>],
    ) -> crate::Result<Result> {
        unsafe {
            self.commands().create_graphics_pipelines(
                self.raw(),
                pipeline_cache,
                create_infos,
                pipelines,
            )
        }
    }
    /// ```c
    /// VkResult vkCreateImage(VkDevice device, VkImageCreateInfo const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkImage* pImage)
    /// ```
    unsafe fn create_image(
        &self,
        create_info: &ImageCreateInfo,
    ) -> crate::Result<vk::Image> {
        unsafe {
            self.commands().create_image(
                self.raw(),
                create_info,
            )
        }
    }
    /// ```c
    /// VkResult vkCreateImageView(VkDevice device, VkImageViewCreateInfo const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkImageView* pView)
    /// ```
    unsafe fn create_image_view(
        &self,
        create_info: &ImageViewCreateInfo,
    ) -> crate::Result<vk::ImageView> {
        unsafe {
            self.commands().create_image_view(
                self.raw(),
                create_info,
            )
        }
    }
    /// ```c
    /// VkResult vkCreatePipelineCache(VkDevice device, VkPipelineCacheCreateInfo const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkPipelineCache* pPipelineCache)
    /// ```
    unsafe fn create_pipeline_cache(
        &self,
        create_info: &PipelineCacheCreateInfo,
    ) -> crate::Result<vk::PipelineCache> {
        unsafe {
            self.commands().create_pipeline_cache(
                self.raw(),
                create_info,
            )
        }
    }
    /// ```c
    /// VkResult vkCreatePipelineLayout(VkDevice device, VkPipelineLayoutCreateInfo const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkPipelineLayout* pPipelineLayout)
    /// ```
    unsafe fn create_pipeline_layout(
        &self,
        create_info: &PipelineLayoutCreateInfo,
    ) -> crate::Result<vk::PipelineLayout> {
        unsafe {
            self.commands().create_pipeline_layout(
                self.raw(),
                create_info,
            )
        }
    }
    /// ```c
    /// VkResult vkCreateQueryPool(VkDevice device, VkQueryPoolCreateInfo const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkQueryPool* pQueryPool)
    /// ```
    unsafe fn create_query_pool(
        &self,
        create_info: &QueryPoolCreateInfo,
    ) -> crate::Result<vk::QueryPool> {
        unsafe {
            self.commands().create_query_pool(
                self.raw(),
                create_info,
            )
        }
    }
    /// ```c
    /// VkResult vkCreateRenderPass(VkDevice device, VkRenderPassCreateInfo const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkRenderPass* pRenderPass)
    /// ```
    unsafe fn create_render_pass(
        &self,
        create_info: &RenderPassCreateInfo,
    ) -> crate::Result<vk::RenderPass> {
        unsafe {
            self.commands().create_render_pass(
                self.raw(),
                create_info,
            )
        }
    }
    /// ```c
    /// VkResult vkCreateSampler(VkDevice device, VkSamplerCreateInfo const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkSampler* pSampler)
    /// ```
    unsafe fn create_sampler(
        &self,
        create_info: &SamplerCreateInfo,
    ) -> crate::Result<vk::Sampler> {
        unsafe {
            self.commands().create_sampler(
                self.raw(),
                create_info,
            )
        }
    }
    /// ```c
    /// VkResult vkCreateSemaphore(VkDevice device, VkSemaphoreCreateInfo const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkSemaphore* pSemaphore)
    /// ```
    unsafe fn create_semaphore(
        &self,
        create_info: &SemaphoreCreateInfo,
    ) -> crate::Result<vk::Semaphore> {
        unsafe {
            self.commands().create_semaphore(
                self.raw(),
                create_info,
            )
        }
    }
    /// ```c
    /// VkResult vkCreateShaderModule(VkDevice device, VkShaderModuleCreateInfo const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkShaderModule* pShaderModule)
    /// ```
    unsafe fn create_shader_module(
        &self,
        create_info: &ShaderModuleCreateInfo,
    ) -> crate::Result<vk::ShaderModule> {
        unsafe {
            self.commands().create_shader_module(
                self.raw(),
                create_info,
            )
        }
    }
    /// ```c
    /// void vkDestroyBuffer(VkDevice device, VkBuffer buffer, VkAllocationCallbacks const* pAllocator)
    /// ```
    unsafe fn destroy_buffer(
        &self,
        buffer: Option<vk::Buffer>,
    ) -> () {
        unsafe {
            self.commands().destroy_buffer(
                self.raw(),
                buffer,
            )
        }
    }
    /// ```c
    /// void vkDestroyBufferView(VkDevice device, VkBufferView bufferView, VkAllocationCallbacks const* pAllocator)
    /// ```
    unsafe fn destroy_buffer_view(
        &self,
        buffer_view: Option<vk::BufferView>,
    ) -> () {
        unsafe {
            self.commands().destroy_buffer_view(
                self.raw(),
                buffer_view,
            )
        }
    }
    /// ```c
    /// void vkDestroyCommandPool(VkDevice device, VkCommandPool commandPool, VkAllocationCallbacks const* pAllocator)
    /// ```
    unsafe fn destroy_command_pool(
        &self,
        command_pool: Option<vk::CommandPool>,
    ) -> () {
        unsafe {
            self.commands().destroy_command_pool(
                self.raw(),
                command_pool,
            )
        }
    }
    /// ```c
    /// void vkDestroyDescriptorPool(VkDevice device, VkDescriptorPool descriptorPool, VkAllocationCallbacks const* pAllocator)
    /// ```
    unsafe fn destroy_descriptor_pool(
        &self,
        descriptor_pool: Option<vk::DescriptorPool>,
    ) -> () {
        unsafe {
            self.commands().destroy_descriptor_pool(
                self.raw(),
                descriptor_pool,
            )
        }
    }
    /// ```c
    /// void vkDestroyDescriptorSetLayout(VkDevice device, VkDescriptorSetLayout descriptorSetLayout, VkAllocationCallbacks const* pAllocator)
    /// ```
    unsafe fn destroy_descriptor_set_layout(
        &self,
        descriptor_set_layout: Option<vk::DescriptorSetLayout>,
    ) -> () {
        unsafe {
            self.commands().destroy_descriptor_set_layout(
                self.raw(),
                descriptor_set_layout,
            )
        }
    }
    /// ```c
    /// void vkDestroyEvent(VkDevice device, VkEvent event, VkAllocationCallbacks const* pAllocator)
    /// ```
    unsafe fn destroy_event(
        &self,
        event: Option<vk::Event>,
    ) -> () {
        unsafe {
            self.commands().destroy_event(
                self.raw(),
                event,
            )
        }
    }
    /// ```c
    /// void vkDestroyFence(VkDevice device, VkFence fence, VkAllocationCallbacks const* pAllocator)
    /// ```
    unsafe fn destroy_fence(
        &self,
        fence: Option<vk::Fence>,
    ) -> () {
        unsafe {
            self.commands().destroy_fence(
                self.raw(),
                fence,
            )
        }
    }
    /// ```c
    /// void vkDestroyFramebuffer(VkDevice device, VkFramebuffer framebuffer, VkAllocationCallbacks const* pAllocator)
    /// ```
    unsafe fn destroy_framebuffer(
        &self,
        framebuffer: Option<vk::Framebuffer>,
    ) -> () {
        unsafe {
            self.commands().destroy_framebuffer(
                self.raw(),
                framebuffer,
            )
        }
    }
    /// ```c
    /// void vkDestroyImage(VkDevice device, VkImage image, VkAllocationCallbacks const* pAllocator)
    /// ```
    unsafe fn destroy_image(
        &self,
        image: Option<vk::Image>,
    ) -> () {
        unsafe {
            self.commands().destroy_image(
                self.raw(),
                image,
            )
        }
    }
    /// ```c
    /// void vkDestroyImageView(VkDevice device, VkImageView imageView, VkAllocationCallbacks const* pAllocator)
    /// ```
    unsafe fn destroy_image_view(
        &self,
        image_view: Option<vk::ImageView>,
    ) -> () {
        unsafe {
            self.commands().destroy_image_view(
                self.raw(),
                image_view,
            )
        }
    }
    /// ```c
    /// void vkDestroyPipeline(VkDevice device, VkPipeline pipeline, VkAllocationCallbacks const* pAllocator)
    /// ```
    unsafe fn destroy_pipeline(
        &self,
        pipeline: Option<vk::Pipeline>,
    ) -> () {
        unsafe {
            self.commands().destroy_pipeline(
                self.raw(),
                pipeline,
            )
        }
    }
    /// ```c
    /// void vkDestroyPipelineCache(VkDevice device, VkPipelineCache pipelineCache, VkAllocationCallbacks const* pAllocator)
    /// ```
    unsafe fn destroy_pipeline_cache(
        &self,
        pipeline_cache: Option<vk::PipelineCache>,
    ) -> () {
        unsafe {
            self.commands().destroy_pipeline_cache(
                self.raw(),
                pipeline_cache,
            )
        }
    }
    /// ```c
    /// void vkDestroyPipelineLayout(VkDevice device, VkPipelineLayout pipelineLayout, VkAllocationCallbacks const* pAllocator)
    /// ```
    unsafe fn destroy_pipeline_layout(
        &self,
        pipeline_layout: Option<vk::PipelineLayout>,
    ) -> () {
        unsafe {
            self.commands().destroy_pipeline_layout(
                self.raw(),
                pipeline_layout,
            )
        }
    }
    /// ```c
    /// void vkDestroyQueryPool(VkDevice device, VkQueryPool queryPool, VkAllocationCallbacks const* pAllocator)
    /// ```
    unsafe fn destroy_query_pool(
        &self,
        query_pool: Option<vk::QueryPool>,
    ) -> () {
        unsafe {
            self.commands().destroy_query_pool(
                self.raw(),
                query_pool,
            )
        }
    }
    /// ```c
    /// void vkDestroyRenderPass(VkDevice device, VkRenderPass renderPass, VkAllocationCallbacks const* pAllocator)
    /// ```
    unsafe fn destroy_render_pass(
        &self,
        render_pass: Option<vk::RenderPass>,
    ) -> () {
        unsafe {
            self.commands().destroy_render_pass(
                self.raw(),
                render_pass,
            )
        }
    }
    /// ```c
    /// void vkDestroySampler(VkDevice device, VkSampler sampler, VkAllocationCallbacks const* pAllocator)
    /// ```
    unsafe fn destroy_sampler(
        &self,
        sampler: Option<vk::Sampler>,
    ) -> () {
        unsafe {
            self.commands().destroy_sampler(
                self.raw(),
                sampler,
            )
        }
    }
    /// ```c
    /// void vkDestroySemaphore(VkDevice device, VkSemaphore semaphore, VkAllocationCallbacks const* pAllocator)
    /// ```
    unsafe fn destroy_semaphore(
        &self,
        semaphore: Option<vk::Semaphore>,
    ) -> () {
        unsafe {
            self.commands().destroy_semaphore(
                self.raw(),
                semaphore,
            )
        }
    }
    /// ```c
    /// void vkDestroyShaderModule(VkDevice device, VkShaderModule shaderModule, VkAllocationCallbacks const* pAllocator)
    /// ```
    unsafe fn destroy_shader_module(
        &self,
        shader_module: Option<vk::ShaderModule>,
    ) -> () {
        unsafe {
            self.commands().destroy_shader_module(
                self.raw(),
                shader_module,
            )
        }
    }
    /// ```c
    /// VkResult vkDeviceWaitIdle(VkDevice device)
    /// ```
    unsafe fn device_wait_idle(
        &self,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().device_wait_idle(
                self.raw(),
            )
        }
    }
    /// ```c
    /// VkResult vkFlushMappedMemoryRanges(VkDevice device, uint32_t memoryRangeCount, VkMappedMemoryRange const* pMemoryRanges)
    /// ```
    unsafe fn flush_mapped_memory_ranges(
        &self,
        memory_ranges: &[MappedMemoryRange],
    ) -> crate::Result<()> {
        unsafe {
            self.commands().flush_mapped_memory_ranges(
                self.raw(),
                memory_ranges,
            )
        }
    }
    /// ```c
    /// void vkFreeCommandBuffers(VkDevice device, VkCommandPool commandPool, uint32_t commandBufferCount, VkCommandBuffer const* pCommandBuffers)
    /// ```
    unsafe fn free_command_buffers(
        &self,
        command_pool: vk::CommandPool,
        command_buffers: &[vk::CommandBuffer],
    ) -> () {
        unsafe {
            self.commands().free_command_buffers(
                self.raw(),
                command_pool,
                command_buffers,
            )
        }
    }
    /// ```c
    /// VkResult vkFreeDescriptorSets(VkDevice device, VkDescriptorPool descriptorPool, uint32_t descriptorSetCount, VkDescriptorSet const* pDescriptorSets)
    /// ```
    unsafe fn free_descriptor_sets(
        &self,
        descriptor_pool: vk::DescriptorPool,
        descriptor_sets: &[vk::DescriptorSet],
    ) -> crate::Result<()> {
        unsafe {
            self.commands().free_descriptor_sets(
                self.raw(),
                descriptor_pool,
                descriptor_sets,
            )
        }
    }
    /// ```c
    /// void vkFreeMemory(VkDevice device, VkDeviceMemory memory, VkAllocationCallbacks const* pAllocator)
    /// ```
    unsafe fn free_memory(
        &self,
        memory: Option<vk::DeviceMemory>,
    ) -> () {
        unsafe {
            self.commands().free_memory(
                self.raw(),
                memory,
            )
        }
    }
    /// ```c
    /// void vkGetBufferMemoryRequirements(VkDevice device, VkBuffer buffer, VkMemoryRequirements* pMemoryRequirements)
    /// ```
    unsafe fn get_buffer_memory_requirements(
        &self,
        buffer: vk::Buffer,
    ) -> MemoryRequirements {
        unsafe {
            self.commands().get_buffer_memory_requirements(
                self.raw(),
                buffer,
            )
        }
    }
    /// ```c
    /// void vkGetDeviceMemoryCommitment(VkDevice device, VkDeviceMemory memory, VkDeviceSize* pCommittedMemoryInBytes)
    /// ```
    unsafe fn get_device_memory_commitment(
        &self,
        memory: vk::DeviceMemory,
    ) -> DeviceSize {
        unsafe {
            self.commands().get_device_memory_commitment(
                self.raw(),
                memory,
            )
        }
    }
    /// ```c
    /// void vkGetDeviceQueue(VkDevice device, uint32_t queueFamilyIndex, uint32_t queueIndex, VkQueue* pQueue)
    /// ```
    unsafe fn get_queue(
        &self,
        queue_family_index: uint32_t,
        queue_index: uint32_t,
    ) -> vk::Queue {
        unsafe {
            self.commands().get_device_queue(
                self.raw(),
                queue_family_index,
                queue_index,
            )
        }
    }
    /// ```c
    /// VkResult vkGetEventStatus(VkDevice device, VkEvent event)
    /// ```
    ///
    /// SuccessCodes: [Result::EventSet], [Result::EventReset]
    unsafe fn get_event_status(
        &self,
        event: vk::Event,
    ) -> crate::Result<Result> {
        unsafe {
            self.commands().get_event_status(
                self.raw(),
                event,
            )
        }
    }
    /// ```c
    /// VkResult vkGetFenceStatus(VkDevice device, VkFence fence)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::NotReady]
    unsafe fn get_fence_status(
        &self,
        fence: vk::Fence,
    ) -> crate::Result<Result> {
        unsafe {
            self.commands().get_fence_status(
                self.raw(),
                fence,
            )
        }
    }
    /// ```c
    /// void vkGetImageMemoryRequirements(VkDevice device, VkImage image, VkMemoryRequirements* pMemoryRequirements)
    /// ```
    unsafe fn get_image_memory_requirements(
        &self,
        image: vk::Image,
    ) -> MemoryRequirements {
        unsafe {
            self.commands().get_image_memory_requirements(
                self.raw(),
                image,
            )
        }
    }
    /// ```c
    /// void vkGetImageSparseMemoryRequirements(VkDevice device, VkImage image, uint32_t* pSparseMemoryRequirementCount, VkSparseImageMemoryRequirements* pSparseMemoryRequirements)
    /// ```
    unsafe fn get_image_sparse_memory_requirements(
        &self,
        image: vk::Image,
        sparse_memory_requirements: Option<&mut ::alloc::vec::Vec<SparseImageMemoryRequirements>>,
    ) -> uint32_t {
        unsafe {
            self.commands().get_image_sparse_memory_requirements(
                self.raw(),
                image,
                sparse_memory_requirements,
            )
        }
    }
    /// ```c
    /// void vkGetImageSubresourceLayout(VkDevice device, VkImage image, VkImageSubresource const* pSubresource, VkSubresourceLayout* pLayout)
    /// ```
    unsafe fn get_image_subresource_layout(
        &self,
        image: vk::Image,
        subresource: &ImageSubresource,
    ) -> SubresourceLayout {
        unsafe {
            self.commands().get_image_subresource_layout(
                self.raw(),
                image,
                subresource,
            )
        }
    }
    /// ```c
    /// VkResult vkGetPipelineCacheData(VkDevice device, VkPipelineCache pipelineCache, size_t* pDataSize, void* pData)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    unsafe fn get_pipeline_cache_data(
        &self,
        pipeline_cache: vk::PipelineCache,
        data_size: *mut size_t,
        data: *mut void,
    ) -> crate::Result<Result> {
        unsafe {
            self.commands().get_pipeline_cache_data(
                self.raw(),
                pipeline_cache,
                data_size,
                data,
            )
        }
    }
    /// ```c
    /// VkResult vkGetQueryPoolResults(VkDevice device, VkQueryPool queryPool, uint32_t firstQuery, uint32_t queryCount, size_t dataSize, void* pData, VkDeviceSize stride, VkQueryResultFlags flags)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::NotReady]
    unsafe fn get_query_pool_results(
        &self,
        query_pool: vk::QueryPool,
        first_query: uint32_t,
        query_count: uint32_t,
        data_size: size_t,
        data: *mut void,
        stride: DeviceSize,
        flags: QueryResultFlags,
    ) -> crate::Result<Result> {
        unsafe {
            self.commands().get_query_pool_results(
                self.raw(),
                query_pool,
                first_query,
                query_count,
                data_size,
                data,
                stride,
                flags,
            )
        }
    }
    /// ```c
    /// void vkGetRenderAreaGranularity(VkDevice device, VkRenderPass renderPass, VkExtent2D* pGranularity)
    /// ```
    unsafe fn get_render_area_granularity(
        &self,
        render_pass: vk::RenderPass,
    ) -> Extent2D {
        unsafe {
            self.commands().get_render_area_granularity(
                self.raw(),
                render_pass,
            )
        }
    }
    /// ```c
    /// VkResult vkInvalidateMappedMemoryRanges(VkDevice device, uint32_t memoryRangeCount, VkMappedMemoryRange const* pMemoryRanges)
    /// ```
    unsafe fn invalidate_mapped_memory_ranges(
        &self,
        memory_ranges: &[MappedMemoryRange],
    ) -> crate::Result<()> {
        unsafe {
            self.commands().invalidate_mapped_memory_ranges(
                self.raw(),
                memory_ranges,
            )
        }
    }
    /// ```c
    /// VkResult vkMapMemory(VkDevice device, VkDeviceMemory memory, VkDeviceSize offset, VkDeviceSize size, VkMemoryMapFlags flags, void** ppData)
    /// ```
    unsafe fn map_memory(
        &self,
        memory: vk::DeviceMemory,
        offset: DeviceSize,
        size: DeviceSize,
        flags: MemoryMapFlags,
        p_data: *mut *mut void,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().map_memory(
                self.raw(),
                memory,
                offset,
                size,
                flags,
                p_data,
            )
        }
    }
    /// ```c
    /// VkResult vkMergePipelineCaches(VkDevice device, VkPipelineCache dstCache, uint32_t srcCacheCount, VkPipelineCache const* pSrcCaches)
    /// ```
    unsafe fn merge_pipeline_caches(
        &self,
        dst_cache: vk::PipelineCache,
        src_caches: &[vk::PipelineCache],
    ) -> crate::Result<()> {
        unsafe {
            self.commands().merge_pipeline_caches(
                self.raw(),
                dst_cache,
                src_caches,
            )
        }
    }
    /// ```c
    /// VkResult vkResetCommandPool(VkDevice device, VkCommandPool commandPool, VkCommandPoolResetFlags flags)
    /// ```
    unsafe fn reset_command_pool(
        &self,
        command_pool: vk::CommandPool,
        flags: CommandPoolResetFlags,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().reset_command_pool(
                self.raw(),
                command_pool,
                flags,
            )
        }
    }
    /// ```c
    /// VkResult vkResetDescriptorPool(VkDevice device, VkDescriptorPool descriptorPool, VkDescriptorPoolResetFlags flags)
    /// ```
    unsafe fn reset_descriptor_pool(
        &self,
        descriptor_pool: vk::DescriptorPool,
        flags: DescriptorPoolResetFlags,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().reset_descriptor_pool(
                self.raw(),
                descriptor_pool,
                flags,
            )
        }
    }
    /// ```c
    /// VkResult vkResetEvent(VkDevice device, VkEvent event)
    /// ```
    unsafe fn reset_event(
        &self,
        event: vk::Event,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().reset_event(
                self.raw(),
                event,
            )
        }
    }
    /// ```c
    /// VkResult vkResetFences(VkDevice device, uint32_t fenceCount, VkFence const* pFences)
    /// ```
    unsafe fn reset_fences(
        &self,
        fences: &[vk::Fence],
    ) -> crate::Result<()> {
        unsafe {
            self.commands().reset_fences(
                self.raw(),
                fences,
            )
        }
    }
    /// ```c
    /// VkResult vkSetEvent(VkDevice device, VkEvent event)
    /// ```
    unsafe fn set_event(
        &self,
        event: vk::Event,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().set_event(
                self.raw(),
                event,
            )
        }
    }
    /// ```c
    /// void vkUnmapMemory(VkDevice device, VkDeviceMemory memory)
    /// ```
    unsafe fn unmap_memory(
        &self,
        memory: vk::DeviceMemory,
    ) -> () {
        unsafe {
            self.commands().unmap_memory(
                self.raw(),
                memory,
            )
        }
    }
    /// ```c
    /// void vkUpdateDescriptorSets(VkDevice device, uint32_t descriptorWriteCount, VkWriteDescriptorSet const* pDescriptorWrites, uint32_t descriptorCopyCount, VkCopyDescriptorSet const* pDescriptorCopies)
    /// ```
    unsafe fn update_descriptor_sets(
        &self,
        descriptor_writes: &[WriteDescriptorSet],
        descriptor_copies: &[CopyDescriptorSet],
    ) -> () {
        unsafe {
            self.commands().update_descriptor_sets(
                self.raw(),
                descriptor_writes,
                descriptor_copies,
            )
        }
    }
    /// ```c
    /// VkResult vkWaitForFences(VkDevice device, uint32_t fenceCount, VkFence const* pFences, VkBool32 waitAll, uint64_t timeout)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Timeout]
    unsafe fn wait_for_fences(
        &self,
        fences: &[vk::Fence],
        wait_all: bool,
        timeout: uint64_t,
    ) -> crate::Result<Result> {
        unsafe {
            self.commands().wait_for_fences(
                self.raw(),
                fences,
                wait_all,
                timeout,
            )
        }
    }
    /// ```c
    /// VkResult vkBindBufferMemory2(VkDevice device, uint32_t bindInfoCount, VkBindBufferMemoryInfo const* pBindInfos)
    /// ```
    unsafe fn bind_buffer_memory2(
        &self,
        bind_infos: &[BindBufferMemoryInfo],
    ) -> crate::Result<()> {
        unsafe {
            self.commands().bind_buffer_memory2(
                self.raw(),
                bind_infos,
            )
        }
    }
    /// ```c
    /// VkResult vkBindImageMemory2(VkDevice device, uint32_t bindInfoCount, VkBindImageMemoryInfo const* pBindInfos)
    /// ```
    unsafe fn bind_image_memory2(
        &self,
        bind_infos: &[BindImageMemoryInfo],
    ) -> crate::Result<()> {
        unsafe {
            self.commands().bind_image_memory2(
                self.raw(),
                bind_infos,
            )
        }
    }
    /// ```c
    /// VkResult vkCreateDescriptorUpdateTemplate(VkDevice device, VkDescriptorUpdateTemplateCreateInfo const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkDescriptorUpdateTemplate* pDescriptorUpdateTemplate)
    /// ```
    unsafe fn create_descriptor_update_template(
        &self,
        create_info: &DescriptorUpdateTemplateCreateInfo,
    ) -> crate::Result<vk::DescriptorUpdateTemplate> {
        unsafe {
            self.commands().create_descriptor_update_template(
                self.raw(),
                create_info,
            )
        }
    }
    /// ```c
    /// VkResult vkCreateSamplerYcbcrConversion(VkDevice device, VkSamplerYcbcrConversionCreateInfo const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkSamplerYcbcrConversion* pYcbcrConversion)
    /// ```
    unsafe fn create_sampler_ycbcr_conversion(
        &self,
        create_info: &SamplerYcbcrConversionCreateInfo,
    ) -> crate::Result<vk::SamplerYcbcrConversion> {
        unsafe {
            self.commands().create_sampler_ycbcr_conversion(
                self.raw(),
                create_info,
            )
        }
    }
    /// ```c
    /// void vkDestroyDescriptorUpdateTemplate(VkDevice device, VkDescriptorUpdateTemplate descriptorUpdateTemplate, VkAllocationCallbacks const* pAllocator)
    /// ```
    unsafe fn destroy_descriptor_update_template(
        &self,
        descriptor_update_template: Option<vk::DescriptorUpdateTemplate>,
    ) -> () {
        unsafe {
            self.commands().destroy_descriptor_update_template(
                self.raw(),
                descriptor_update_template,
            )
        }
    }
    /// ```c
    /// void vkDestroySamplerYcbcrConversion(VkDevice device, VkSamplerYcbcrConversion ycbcrConversion, VkAllocationCallbacks const* pAllocator)
    /// ```
    unsafe fn destroy_sampler_ycbcr_conversion(
        &self,
        ycbcr_conversion: Option<vk::SamplerYcbcrConversion>,
    ) -> () {
        unsafe {
            self.commands().destroy_sampler_ycbcr_conversion(
                self.raw(),
                ycbcr_conversion,
            )
        }
    }
    /// ```c
    /// void vkGetBufferMemoryRequirements2(VkDevice device, VkBufferMemoryRequirementsInfo2 const* pInfo, VkMemoryRequirements2* pMemoryRequirements)
    /// ```
    unsafe fn get_buffer_memory_requirements2(
        &self,
        info: &BufferMemoryRequirementsInfo2,
        memory_requirements: &mut MemoryRequirements2,
    ) -> () {
        unsafe {
            self.commands().get_buffer_memory_requirements2(
                self.raw(),
                info,
                memory_requirements,
            )
        }
    }
    /// ```c
    /// void vkGetDescriptorSetLayoutSupport(VkDevice device, VkDescriptorSetLayoutCreateInfo const* pCreateInfo, VkDescriptorSetLayoutSupport* pSupport)
    /// ```
    unsafe fn get_descriptor_set_layout_support(
        &self,
        create_info: &DescriptorSetLayoutCreateInfo,
        support: &mut DescriptorSetLayoutSupport,
    ) -> () {
        unsafe {
            self.commands().get_descriptor_set_layout_support(
                self.raw(),
                create_info,
                support,
            )
        }
    }
    /// ```c
    /// void vkGetDeviceGroupPeerMemoryFeatures(VkDevice device, uint32_t heapIndex, uint32_t localDeviceIndex, uint32_t remoteDeviceIndex, VkPeerMemoryFeatureFlags* pPeerMemoryFeatures)
    /// ```
    unsafe fn get_device_group_peer_memory_features(
        &self,
        heap_index: uint32_t,
        local_device_index: uint32_t,
        remote_device_index: uint32_t,
    ) -> PeerMemoryFeatureFlags {
        unsafe {
            self.commands().get_device_group_peer_memory_features(
                self.raw(),
                heap_index,
                local_device_index,
                remote_device_index,
            )
        }
    }
    /// ```c
    /// void vkGetDeviceQueue2(VkDevice device, VkDeviceQueueInfo2 const* pQueueInfo, VkQueue* pQueue)
    /// ```
    unsafe fn get_queue2(
        &self,
        queue_info: &DeviceQueueInfo2,
    ) -> vk::Queue {
        unsafe {
            self.commands().get_device_queue2(
                self.raw(),
                queue_info,
            )
        }
    }
    /// ```c
    /// void vkGetImageMemoryRequirements2(VkDevice device, VkImageMemoryRequirementsInfo2 const* pInfo, VkMemoryRequirements2* pMemoryRequirements)
    /// ```
    unsafe fn get_image_memory_requirements2(
        &self,
        info: &ImageMemoryRequirementsInfo2,
        memory_requirements: &mut MemoryRequirements2,
    ) -> () {
        unsafe {
            self.commands().get_image_memory_requirements2(
                self.raw(),
                info,
                memory_requirements,
            )
        }
    }
    /// ```c
    /// void vkGetImageSparseMemoryRequirements2(VkDevice device, VkImageSparseMemoryRequirementsInfo2 const* pInfo, uint32_t* pSparseMemoryRequirementCount, VkSparseImageMemoryRequirements2* pSparseMemoryRequirements)
    /// ```
    unsafe fn get_image_sparse_memory_requirements2(
        &self,
        info: &ImageSparseMemoryRequirementsInfo2,
        sparse_memory_requirements: Option<&mut ::alloc::vec::Vec<SparseImageMemoryRequirements2>>,
    ) -> uint32_t {
        unsafe {
            self.commands().get_image_sparse_memory_requirements2(
                self.raw(),
                info,
                sparse_memory_requirements,
            )
        }
    }
    /// ```c
    /// void vkTrimCommandPool(VkDevice device, VkCommandPool commandPool, VkCommandPoolTrimFlags flags)
    /// ```
    unsafe fn trim_command_pool(
        &self,
        command_pool: vk::CommandPool,
        flags: CommandPoolTrimFlags,
    ) -> () {
        unsafe {
            self.commands().trim_command_pool(
                self.raw(),
                command_pool,
                flags,
            )
        }
    }
    /// ```c
    /// void vkUpdateDescriptorSetWithTemplate(VkDevice device, VkDescriptorSet descriptorSet, VkDescriptorUpdateTemplate descriptorUpdateTemplate, void const* pData)
    /// ```
    unsafe fn update_descriptor_set_with_template(
        &self,
        descriptor_set: vk::DescriptorSet,
        descriptor_update_template: vk::DescriptorUpdateTemplate,
        data: *const void,
    ) -> () {
        unsafe {
            self.commands().update_descriptor_set_with_template(
                self.raw(),
                descriptor_set,
                descriptor_update_template,
                data,
            )
        }
    }
    /// ```c
    /// VkResult vkCreateRenderPass2(VkDevice device, VkRenderPassCreateInfo2 const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkRenderPass* pRenderPass)
    /// ```
    unsafe fn create_render_pass2(
        &self,
        create_info: &RenderPassCreateInfo2,
    ) -> crate::Result<vk::RenderPass> {
        unsafe {
            self.commands().create_render_pass2(
                self.raw(),
                create_info,
            )
        }
    }
    /// ```c
    /// VkDeviceAddress vkGetBufferDeviceAddress(VkDevice device, VkBufferDeviceAddressInfo const* pInfo)
    /// ```
    unsafe fn get_buffer_device_address(
        &self,
        info: &BufferDeviceAddressInfo,
    ) -> DeviceAddress {
        unsafe {
            self.commands().get_buffer_device_address(
                self.raw(),
                info,
            )
        }
    }
    /// ```c
    /// uint64_t vkGetBufferOpaqueCaptureAddress(VkDevice device, VkBufferDeviceAddressInfo const* pInfo)
    /// ```
    unsafe fn get_buffer_opaque_capture_address(
        &self,
        info: &BufferDeviceAddressInfo,
    ) -> uint64_t {
        unsafe {
            self.commands().get_buffer_opaque_capture_address(
                self.raw(),
                info,
            )
        }
    }
    /// ```c
    /// uint64_t vkGetDeviceMemoryOpaqueCaptureAddress(VkDevice device, VkDeviceMemoryOpaqueCaptureAddressInfo const* pInfo)
    /// ```
    unsafe fn get_device_memory_opaque_capture_address(
        &self,
        info: &DeviceMemoryOpaqueCaptureAddressInfo,
    ) -> uint64_t {
        unsafe {
            self.commands().get_device_memory_opaque_capture_address(
                self.raw(),
                info,
            )
        }
    }
    /// ```c
    /// VkResult vkGetSemaphoreCounterValue(VkDevice device, VkSemaphore semaphore, uint64_t* pValue)
    /// ```
    unsafe fn get_semaphore_counter_value(
        &self,
        semaphore: vk::Semaphore,
    ) -> crate::Result<uint64_t> {
        unsafe {
            self.commands().get_semaphore_counter_value(
                self.raw(),
                semaphore,
            )
        }
    }
    /// ```c
    /// void vkResetQueryPool(VkDevice device, VkQueryPool queryPool, uint32_t firstQuery, uint32_t queryCount)
    /// ```
    unsafe fn reset_query_pool(
        &self,
        query_pool: vk::QueryPool,
        first_query: uint32_t,
        query_count: uint32_t,
    ) -> () {
        unsafe {
            self.commands().reset_query_pool(
                self.raw(),
                query_pool,
                first_query,
                query_count,
            )
        }
    }
    /// ```c
    /// VkResult vkSignalSemaphore(VkDevice device, VkSemaphoreSignalInfo const* pSignalInfo)
    /// ```
    unsafe fn signal_semaphore(
        &self,
        signal_info: &SemaphoreSignalInfo,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().signal_semaphore(
                self.raw(),
                signal_info,
            )
        }
    }
    /// ```c
    /// VkResult vkWaitSemaphores(VkDevice device, VkSemaphoreWaitInfo const* pWaitInfo, uint64_t timeout)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Timeout]
    unsafe fn wait_semaphores(
        &self,
        wait_info: &SemaphoreWaitInfo,
        timeout: uint64_t,
    ) -> crate::Result<Result> {
        unsafe {
            self.commands().wait_semaphores(
                self.raw(),
                wait_info,
                timeout,
            )
        }
    }
    /// ```c
    /// VkResult vkCreatePrivateDataSlot(VkDevice device, VkPrivateDataSlotCreateInfo const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkPrivateDataSlot* pPrivateDataSlot)
    /// ```
    unsafe fn create_private_data_slot(
        &self,
        create_info: &PrivateDataSlotCreateInfo,
    ) -> crate::Result<vk::PrivateDataSlot> {
        unsafe {
            self.commands().create_private_data_slot(
                self.raw(),
                create_info,
            )
        }
    }
    /// ```c
    /// void vkDestroyPrivateDataSlot(VkDevice device, VkPrivateDataSlot privateDataSlot, VkAllocationCallbacks const* pAllocator)
    /// ```
    unsafe fn destroy_private_data_slot(
        &self,
        private_data_slot: Option<vk::PrivateDataSlot>,
    ) -> () {
        unsafe {
            self.commands().destroy_private_data_slot(
                self.raw(),
                private_data_slot,
            )
        }
    }
    /// ```c
    /// void vkGetDeviceBufferMemoryRequirements(VkDevice device, VkDeviceBufferMemoryRequirements const* pInfo, VkMemoryRequirements2* pMemoryRequirements)
    /// ```
    unsafe fn get_device_buffer_memory_requirements(
        &self,
        info: &DeviceBufferMemoryRequirements,
        memory_requirements: &mut MemoryRequirements2,
    ) -> () {
        unsafe {
            self.commands().get_device_buffer_memory_requirements(
                self.raw(),
                info,
                memory_requirements,
            )
        }
    }
    /// ```c
    /// void vkGetDeviceImageMemoryRequirements(VkDevice device, VkDeviceImageMemoryRequirements const* pInfo, VkMemoryRequirements2* pMemoryRequirements)
    /// ```
    unsafe fn get_device_image_memory_requirements(
        &self,
        info: &DeviceImageMemoryRequirements,
        memory_requirements: &mut MemoryRequirements2,
    ) -> () {
        unsafe {
            self.commands().get_device_image_memory_requirements(
                self.raw(),
                info,
                memory_requirements,
            )
        }
    }
    /// ```c
    /// void vkGetDeviceImageSparseMemoryRequirements(VkDevice device, VkDeviceImageMemoryRequirements const* pInfo, uint32_t* pSparseMemoryRequirementCount, VkSparseImageMemoryRequirements2* pSparseMemoryRequirements)
    /// ```
    unsafe fn get_device_image_sparse_memory_requirements(
        &self,
        info: &DeviceImageMemoryRequirements,
        sparse_memory_requirements: Option<&mut ::alloc::vec::Vec<SparseImageMemoryRequirements2>>,
    ) -> uint32_t {
        unsafe {
            self.commands().get_device_image_sparse_memory_requirements(
                self.raw(),
                info,
                sparse_memory_requirements,
            )
        }
    }
    /// ```c
    /// void vkGetPrivateData(VkDevice device, VkObjectType objectType, uint64_t objectHandle, VkPrivateDataSlot privateDataSlot, uint64_t* pData)
    /// ```
    unsafe fn get_private_data(
        &self,
        object_type: ObjectType,
        object_handle: uint64_t,
        private_data_slot: vk::PrivateDataSlot,
    ) -> uint64_t {
        unsafe {
            self.commands().get_private_data(
                self.raw(),
                object_type,
                object_handle,
                private_data_slot,
            )
        }
    }
    /// ```c
    /// VkResult vkSetPrivateData(VkDevice device, VkObjectType objectType, uint64_t objectHandle, VkPrivateDataSlot privateDataSlot, uint64_t data)
    /// ```
    unsafe fn set_private_data(
        &self,
        object_type: ObjectType,
        object_handle: uint64_t,
        private_data_slot: vk::PrivateDataSlot,
        data: uint64_t,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().set_private_data(
                self.raw(),
                object_type,
                object_handle,
                private_data_slot,
                data,
            )
        }
    }
    /// ```c
    /// VkResult vkCopyImageToImage(VkDevice device, VkCopyImageToImageInfo const* pCopyImageToImageInfo)
    /// ```
    unsafe fn copy_image_to_image(
        &self,
        copy_image_to_image_info: &CopyImageToImageInfo,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().copy_image_to_image(
                self.raw(),
                copy_image_to_image_info,
            )
        }
    }
    /// ```c
    /// VkResult vkCopyImageToMemory(VkDevice device, VkCopyImageToMemoryInfo const* pCopyImageToMemoryInfo)
    /// ```
    unsafe fn copy_image_to_memory(
        &self,
        copy_image_to_memory_info: &CopyImageToMemoryInfo,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().copy_image_to_memory(
                self.raw(),
                copy_image_to_memory_info,
            )
        }
    }
    /// ```c
    /// VkResult vkCopyMemoryToImage(VkDevice device, VkCopyMemoryToImageInfo const* pCopyMemoryToImageInfo)
    /// ```
    unsafe fn copy_memory_to_image(
        &self,
        copy_memory_to_image_info: &CopyMemoryToImageInfo,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().copy_memory_to_image(
                self.raw(),
                copy_memory_to_image_info,
            )
        }
    }
    /// ```c
    /// void vkGetDeviceImageSubresourceLayout(VkDevice device, VkDeviceImageSubresourceInfo const* pInfo, VkSubresourceLayout2* pLayout)
    /// ```
    unsafe fn get_device_image_subresource_layout(
        &self,
        info: &DeviceImageSubresourceInfo,
        layout: &mut SubresourceLayout2,
    ) -> () {
        unsafe {
            self.commands().get_device_image_subresource_layout(
                self.raw(),
                info,
                layout,
            )
        }
    }
    /// ```c
    /// void vkGetImageSubresourceLayout2(VkDevice device, VkImage image, VkImageSubresource2 const* pSubresource, VkSubresourceLayout2* pLayout)
    /// ```
    unsafe fn get_image_subresource_layout2(
        &self,
        image: vk::Image,
        subresource: &ImageSubresource2,
        layout: &mut SubresourceLayout2,
    ) -> () {
        unsafe {
            self.commands().get_image_subresource_layout2(
                self.raw(),
                image,
                subresource,
                layout,
            )
        }
    }
    /// ```c
    /// void vkGetRenderingAreaGranularity(VkDevice device, VkRenderingAreaInfo const* pRenderingAreaInfo, VkExtent2D* pGranularity)
    /// ```
    unsafe fn get_rendering_area_granularity(
        &self,
        rendering_area_info: &RenderingAreaInfo,
    ) -> Extent2D {
        unsafe {
            self.commands().get_rendering_area_granularity(
                self.raw(),
                rendering_area_info,
            )
        }
    }
    /// ```c
    /// VkResult vkMapMemory2(VkDevice device, VkMemoryMapInfo const* pMemoryMapInfo, void** ppData)
    /// ```
    unsafe fn map_memory2(
        &self,
        memory_map_info: &MemoryMapInfo,
        p_data: *mut *mut void,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().map_memory2(
                self.raw(),
                memory_map_info,
                p_data,
            )
        }
    }
    /// ```c
    /// VkResult vkTransitionImageLayout(VkDevice device, uint32_t transitionCount, VkHostImageLayoutTransitionInfo const* pTransitions)
    /// ```
    unsafe fn transition_image_layout(
        &self,
        transitions: &[HostImageLayoutTransitionInfo],
    ) -> crate::Result<()> {
        unsafe {
            self.commands().transition_image_layout(
                self.raw(),
                transitions,
            )
        }
    }
    /// ```c
    /// VkResult vkUnmapMemory2(VkDevice device, VkMemoryUnmapInfo const* pMemoryUnmapInfo)
    /// ```
    unsafe fn unmap_memory2(
        &self,
        memory_unmap_info: &MemoryUnmapInfo,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().unmap_memory2(
                self.raw(),
                memory_unmap_info,
            )
        }
    }
}

impl crate::Owner<vk::Device, vk::core> for Device {
    fn drop(&mut self, raw: vk::Device) {
        unsafe {
            self.destroy_device(Some(raw))
        }
    }
}

impl crate::Owner<vk::Device, vk::core> for ::alloc::sync::Arc<Device> {
    fn drop(&mut self, raw: vk::Device) {
        unsafe {
            self.destroy_device(Some(raw))
        }
    }
}

impl CoreDevice for crate::Unique<vk::Device, Device, vk::core> {
    fn raw(&self) -> vk::Device { self.raw }
    fn commands(&self) -> &Device { &self.owner }
}

impl CoreDevice for crate::Unique<vk::Device, ::alloc::sync::Arc<Device>, vk::core> {
    fn raw(&self) -> vk::Device { self.raw }
    fn commands(&self) -> &Device { &self.owner }
}

impl crate::HndCtx<vk::core, vk::Device> for ::alloc::sync::Arc<crate::Unique<vk::Device, ::alloc::sync::Arc<Device>, vk::core>> {
    type Ctx = Self;
    fn ctx(&self) -> Self::Ctx { self.clone() }
    fn raw(&self) -> vk::Device { self.raw }
    fn commands(&self) -> &::alloc::sync::Arc<Device> { &self.owner }
}

impl crate::HndScope<vk::Device> for vk::core {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[repr(transparent)]
    #[derive(Debug)]
    pub struct Device(pub(crate) crate::handle::Hnd<vk::Device, (::alloc::sync::Arc<super::Instance>, ::alloc::sync::Arc<super::Device>)>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone()) }
    }

    #[derive(Debug, Clone, Copy)]
    struct Inst;
    impl<Dep> crate::handle::HndInst<vk::Device, (::alloc::sync::Arc<super::Instance>, ::alloc::sync::Arc<super::Device>), Dep> for Inst {
        fn drop(this: &mut crate::handle::HndData<vk::Device, (::alloc::sync::Arc<super::Instance>, ::alloc::sync::Arc<super::Device>), Dep>) {
            unsafe {
                this.scope.1.destroy_device(Some(this.raw))
            }
        }
    }

    impl crate::hnd::Device<vk::core> {
        pub unsafe fn new(ctx: &(impl crate::HndCtx<vk::core, vk::Instance>), raw: vk::Device) -> Self {
            unsafe { Self::new_with(ctx, raw, || ()) }
        }
        pub unsafe fn new_with<Dep: Send + Sync + 'static>(ctx: &(impl crate::HndCtx<vk::core, vk::Instance>), raw: vk::Device, dep: impl FnOnce() -> Dep) -> Self {
            Self(Device(crate::handle::Hnd::new(
                (ctx.commands().clone(), ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { ctx.commands().get_device_proc_addr(raw, name) }))),
                raw,
                (ctx.ctx(), dep()),
                Inst,
            )))
        }
    }

    impl<Ctx: crate::HndCtx<vk::core, vk::Instance>> crate::MakeHnd<Ctx, vk::core> for vk::Device  {
        type Output = crate::hnd::Device<vk::core>;
        unsafe fn hnd_with<Dep: Send + Sync + 'static>(self, ctx: &Ctx, dep: impl FnOnce() -> Dep) -> Self::Output {
            unsafe { crate::hnd::Device::new_with(ctx, self, dep) }
        }
    }

    impl crate::hnd::Device<vk::core> {
        pub fn raw(&self) -> vk::Device { self.0.0.raw }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0.scope.1 }
        fn inst_commands(&self) -> &::alloc::sync::Arc<super::Instance> { &self.0.0.scope.0 }
        pub fn get_proc_addr(&self, name: &::core::ffi::CStr) -> Option<crate::ProcAddr> {
            unsafe { self.inst_commands().get_device_proc_addr(self.raw(), name) }
        }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::core> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::core> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::core> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::CoreDevice for crate::hnd::Device<vk::core> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::core, vk::Device> for crate::hnd::Device<vk::core> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

/// CommandBuffer object
pub trait CoreCommandBuffer {
    fn raw(&self) -> vk::CommandBuffer;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkBeginCommandBuffer(VkCommandBuffer commandBuffer, VkCommandBufferBeginInfo const* pBeginInfo)
    /// ```
    unsafe fn begin(
        &self,
        begin_info: &CommandBufferBeginInfo,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().begin_command_buffer(
                self.raw(),
                begin_info,
            )
        }
    }
    /// ```c
    /// void vkCmdBeginQuery(VkCommandBuffer commandBuffer, VkQueryPool queryPool, uint32_t query, VkQueryControlFlags flags)
    /// ```
    unsafe fn begin_query(
        &self,
        query_pool: vk::QueryPool,
        query: uint32_t,
        flags: QueryControlFlags,
    ) -> () {
        unsafe {
            self.commands().cmd_begin_query(
                self.raw(),
                query_pool,
                query,
                flags,
            )
        }
    }
    /// ```c
    /// void vkCmdBeginRenderPass(VkCommandBuffer commandBuffer, VkRenderPassBeginInfo const* pRenderPassBegin, VkSubpassContents contents)
    /// ```
    unsafe fn begin_render_pass(
        &self,
        render_pass_begin: &RenderPassBeginInfo,
        contents: SubpassContents,
    ) -> () {
        unsafe {
            self.commands().cmd_begin_render_pass(
                self.raw(),
                render_pass_begin,
                contents,
            )
        }
    }
    /// ```c
    /// void vkCmdBeginRenderPass2(VkCommandBuffer commandBuffer, VkRenderPassBeginInfo const* pRenderPassBegin, VkSubpassBeginInfo const* pSubpassBeginInfo)
    /// ```
    unsafe fn begin_render_pass2(
        &self,
        render_pass_begin: &RenderPassBeginInfo,
        subpass_begin_info: &SubpassBeginInfo,
    ) -> () {
        unsafe {
            self.commands().cmd_begin_render_pass2(
                self.raw(),
                render_pass_begin,
                subpass_begin_info,
            )
        }
    }
    /// ```c
    /// void vkCmdBeginRendering(VkCommandBuffer commandBuffer, VkRenderingInfo const* pRenderingInfo)
    /// ```
    unsafe fn begin_rendering(
        &self,
        rendering_info: &RenderingInfo,
    ) -> () {
        unsafe {
            self.commands().cmd_begin_rendering(
                self.raw(),
                rendering_info,
            )
        }
    }
    /// ```c
    /// void vkCmdBindDescriptorSets(VkCommandBuffer commandBuffer, VkPipelineBindPoint pipelineBindPoint, VkPipelineLayout layout, uint32_t firstSet, uint32_t descriptorSetCount, VkDescriptorSet const* pDescriptorSets, uint32_t dynamicOffsetCount, uint32_t const* pDynamicOffsets)
    /// ```
    unsafe fn bind_descriptor_sets(
        &self,
        pipeline_bind_point: PipelineBindPoint,
        layout: vk::PipelineLayout,
        first_set: uint32_t,
        descriptor_sets: &[Option<vk::DescriptorSet>],
        dynamic_offsets: &[uint32_t],
    ) -> () {
        unsafe {
            self.commands().cmd_bind_descriptor_sets(
                self.raw(),
                pipeline_bind_point,
                layout,
                first_set,
                descriptor_sets,
                dynamic_offsets,
            )
        }
    }
    /// ```c
    /// void vkCmdBindDescriptorSets2(VkCommandBuffer commandBuffer, VkBindDescriptorSetsInfo const* pBindDescriptorSetsInfo)
    /// ```
    unsafe fn bind_descriptor_sets2(
        &self,
        bind_descriptor_sets_info: &BindDescriptorSetsInfo,
    ) -> () {
        unsafe {
            self.commands().cmd_bind_descriptor_sets2(
                self.raw(),
                bind_descriptor_sets_info,
            )
        }
    }
    /// ```c
    /// void vkCmdBindIndexBuffer(VkCommandBuffer commandBuffer, VkBuffer buffer, VkDeviceSize offset, VkIndexType indexType)
    /// ```
    unsafe fn bind_index_buffer(
        &self,
        buffer: Option<vk::Buffer>,
        offset: DeviceSize,
        index_type: IndexType,
    ) -> () {
        unsafe {
            self.commands().cmd_bind_index_buffer(
                self.raw(),
                buffer,
                offset,
                index_type,
            )
        }
    }
    /// ```c
    /// void vkCmdBindIndexBuffer2(VkCommandBuffer commandBuffer, VkBuffer buffer, VkDeviceSize offset, VkDeviceSize size, VkIndexType indexType)
    /// ```
    unsafe fn bind_index_buffer2(
        &self,
        buffer: Option<vk::Buffer>,
        offset: DeviceSize,
        size: DeviceSize,
        index_type: IndexType,
    ) -> () {
        unsafe {
            self.commands().cmd_bind_index_buffer2(
                self.raw(),
                buffer,
                offset,
                size,
                index_type,
            )
        }
    }
    /// ```c
    /// void vkCmdBindPipeline(VkCommandBuffer commandBuffer, VkPipelineBindPoint pipelineBindPoint, VkPipeline pipeline)
    /// ```
    unsafe fn bind_pipeline(
        &self,
        pipeline_bind_point: PipelineBindPoint,
        pipeline: vk::Pipeline,
    ) -> () {
        unsafe {
            self.commands().cmd_bind_pipeline(
                self.raw(),
                pipeline_bind_point,
                pipeline,
            )
        }
    }
    /// ```c
    /// void vkCmdBindVertexBuffers(VkCommandBuffer commandBuffer, uint32_t firstBinding, uint32_t bindingCount, VkBuffer const* pBuffers, VkDeviceSize const* pOffsets)
    /// ```
    unsafe fn bind_vertex_buffers(
        &self,
        first_binding: uint32_t,
        buffers: &[Option<vk::Buffer>],
        offsets: &[DeviceSize],
    ) -> () {
        unsafe {
            self.commands().cmd_bind_vertex_buffers(
                self.raw(),
                first_binding,
                buffers,
                offsets,
            )
        }
    }
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
    /// void vkCmdBlitImage(VkCommandBuffer commandBuffer, VkImage srcImage, VkImageLayout srcImageLayout, VkImage dstImage, VkImageLayout dstImageLayout, uint32_t regionCount, VkImageBlit const* pRegions, VkFilter filter)
    /// ```
    unsafe fn blit_image(
        &self,
        src_image: vk::Image,
        src_image_layout: ImageLayout,
        dst_image: vk::Image,
        dst_image_layout: ImageLayout,
        regions: &[ImageBlit],
        filter: Filter,
    ) -> () {
        unsafe {
            self.commands().cmd_blit_image(
                self.raw(),
                src_image,
                src_image_layout,
                dst_image,
                dst_image_layout,
                regions,
                filter,
            )
        }
    }
    /// ```c
    /// void vkCmdBlitImage2(VkCommandBuffer commandBuffer, VkBlitImageInfo2 const* pBlitImageInfo)
    /// ```
    unsafe fn blit_image2(
        &self,
        blit_image_info: &BlitImageInfo2,
    ) -> () {
        unsafe {
            self.commands().cmd_blit_image2(
                self.raw(),
                blit_image_info,
            )
        }
    }
    /// ```c
    /// void vkCmdClearAttachments(VkCommandBuffer commandBuffer, uint32_t attachmentCount, VkClearAttachment const* pAttachments, uint32_t rectCount, VkClearRect const* pRects)
    /// ```
    unsafe fn clear_attachments(
        &self,
        attachments: &[ClearAttachment],
        rects: &[ClearRect],
    ) -> () {
        unsafe {
            self.commands().cmd_clear_attachments(
                self.raw(),
                attachments,
                rects,
            )
        }
    }
    /// ```c
    /// void vkCmdClearColorImage(VkCommandBuffer commandBuffer, VkImage image, VkImageLayout imageLayout, VkClearColorValue const* pColor, uint32_t rangeCount, VkImageSubresourceRange const* pRanges)
    /// ```
    unsafe fn clear_color_image(
        &self,
        image: vk::Image,
        image_layout: ImageLayout,
        color: &ClearColorValue,
        ranges: &[ImageSubresourceRange],
    ) -> () {
        unsafe {
            self.commands().cmd_clear_color_image(
                self.raw(),
                image,
                image_layout,
                color,
                ranges,
            )
        }
    }
    /// ```c
    /// void vkCmdClearDepthStencilImage(VkCommandBuffer commandBuffer, VkImage image, VkImageLayout imageLayout, VkClearDepthStencilValue const* pDepthStencil, uint32_t rangeCount, VkImageSubresourceRange const* pRanges)
    /// ```
    unsafe fn clear_depth_stencil_image(
        &self,
        image: vk::Image,
        image_layout: ImageLayout,
        depth_stencil: &ClearDepthStencilValue,
        ranges: &[ImageSubresourceRange],
    ) -> () {
        unsafe {
            self.commands().cmd_clear_depth_stencil_image(
                self.raw(),
                image,
                image_layout,
                depth_stencil,
                ranges,
            )
        }
    }
    /// ```c
    /// void vkCmdCopyBuffer(VkCommandBuffer commandBuffer, VkBuffer srcBuffer, VkBuffer dstBuffer, uint32_t regionCount, VkBufferCopy const* pRegions)
    /// ```
    unsafe fn copy_buffer(
        &self,
        src_buffer: vk::Buffer,
        dst_buffer: vk::Buffer,
        regions: &[BufferCopy],
    ) -> () {
        unsafe {
            self.commands().cmd_copy_buffer(
                self.raw(),
                src_buffer,
                dst_buffer,
                regions,
            )
        }
    }
    /// ```c
    /// void vkCmdCopyBuffer2(VkCommandBuffer commandBuffer, VkCopyBufferInfo2 const* pCopyBufferInfo)
    /// ```
    unsafe fn copy_buffer2(
        &self,
        copy_buffer_info: &CopyBufferInfo2,
    ) -> () {
        unsafe {
            self.commands().cmd_copy_buffer2(
                self.raw(),
                copy_buffer_info,
            )
        }
    }
    /// ```c
    /// void vkCmdCopyBufferToImage(VkCommandBuffer commandBuffer, VkBuffer srcBuffer, VkImage dstImage, VkImageLayout dstImageLayout, uint32_t regionCount, VkBufferImageCopy const* pRegions)
    /// ```
    unsafe fn copy_buffer_to_image(
        &self,
        src_buffer: vk::Buffer,
        dst_image: vk::Image,
        dst_image_layout: ImageLayout,
        regions: &[BufferImageCopy],
    ) -> () {
        unsafe {
            self.commands().cmd_copy_buffer_to_image(
                self.raw(),
                src_buffer,
                dst_image,
                dst_image_layout,
                regions,
            )
        }
    }
    /// ```c
    /// void vkCmdCopyBufferToImage2(VkCommandBuffer commandBuffer, VkCopyBufferToImageInfo2 const* pCopyBufferToImageInfo)
    /// ```
    unsafe fn copy_buffer_to_image2(
        &self,
        copy_buffer_to_image_info: &CopyBufferToImageInfo2,
    ) -> () {
        unsafe {
            self.commands().cmd_copy_buffer_to_image2(
                self.raw(),
                copy_buffer_to_image_info,
            )
        }
    }
    /// ```c
    /// void vkCmdCopyImage(VkCommandBuffer commandBuffer, VkImage srcImage, VkImageLayout srcImageLayout, VkImage dstImage, VkImageLayout dstImageLayout, uint32_t regionCount, VkImageCopy const* pRegions)
    /// ```
    unsafe fn copy_image(
        &self,
        src_image: vk::Image,
        src_image_layout: ImageLayout,
        dst_image: vk::Image,
        dst_image_layout: ImageLayout,
        regions: &[ImageCopy],
    ) -> () {
        unsafe {
            self.commands().cmd_copy_image(
                self.raw(),
                src_image,
                src_image_layout,
                dst_image,
                dst_image_layout,
                regions,
            )
        }
    }
    /// ```c
    /// void vkCmdCopyImage2(VkCommandBuffer commandBuffer, VkCopyImageInfo2 const* pCopyImageInfo)
    /// ```
    unsafe fn copy_image2(
        &self,
        copy_image_info: &CopyImageInfo2,
    ) -> () {
        unsafe {
            self.commands().cmd_copy_image2(
                self.raw(),
                copy_image_info,
            )
        }
    }
    /// ```c
    /// void vkCmdCopyImageToBuffer(VkCommandBuffer commandBuffer, VkImage srcImage, VkImageLayout srcImageLayout, VkBuffer dstBuffer, uint32_t regionCount, VkBufferImageCopy const* pRegions)
    /// ```
    unsafe fn copy_image_to_buffer(
        &self,
        src_image: vk::Image,
        src_image_layout: ImageLayout,
        dst_buffer: vk::Buffer,
        regions: &[BufferImageCopy],
    ) -> () {
        unsafe {
            self.commands().cmd_copy_image_to_buffer(
                self.raw(),
                src_image,
                src_image_layout,
                dst_buffer,
                regions,
            )
        }
    }
    /// ```c
    /// void vkCmdCopyImageToBuffer2(VkCommandBuffer commandBuffer, VkCopyImageToBufferInfo2 const* pCopyImageToBufferInfo)
    /// ```
    unsafe fn copy_image_to_buffer2(
        &self,
        copy_image_to_buffer_info: &CopyImageToBufferInfo2,
    ) -> () {
        unsafe {
            self.commands().cmd_copy_image_to_buffer2(
                self.raw(),
                copy_image_to_buffer_info,
            )
        }
    }
    /// ```c
    /// void vkCmdCopyQueryPoolResults(VkCommandBuffer commandBuffer, VkQueryPool queryPool, uint32_t firstQuery, uint32_t queryCount, VkBuffer dstBuffer, VkDeviceSize dstOffset, VkDeviceSize stride, VkQueryResultFlags flags)
    /// ```
    unsafe fn copy_query_pool_results(
        &self,
        query_pool: vk::QueryPool,
        first_query: uint32_t,
        query_count: uint32_t,
        dst_buffer: vk::Buffer,
        dst_offset: DeviceSize,
        stride: DeviceSize,
        flags: QueryResultFlags,
    ) -> () {
        unsafe {
            self.commands().cmd_copy_query_pool_results(
                self.raw(),
                query_pool,
                first_query,
                query_count,
                dst_buffer,
                dst_offset,
                stride,
                flags,
            )
        }
    }
    /// ```c
    /// void vkCmdDispatch(VkCommandBuffer commandBuffer, uint32_t groupCountX, uint32_t groupCountY, uint32_t groupCountZ)
    /// ```
    unsafe fn dispatch(
        &self,
        group_count_x: uint32_t,
        group_count_y: uint32_t,
        group_count_z: uint32_t,
    ) -> () {
        unsafe {
            self.commands().cmd_dispatch(
                self.raw(),
                group_count_x,
                group_count_y,
                group_count_z,
            )
        }
    }
    /// ```c
    /// void vkCmdDispatchBase(VkCommandBuffer commandBuffer, uint32_t baseGroupX, uint32_t baseGroupY, uint32_t baseGroupZ, uint32_t groupCountX, uint32_t groupCountY, uint32_t groupCountZ)
    /// ```
    unsafe fn dispatch_base(
        &self,
        base_group_x: uint32_t,
        base_group_y: uint32_t,
        base_group_z: uint32_t,
        group_count_x: uint32_t,
        group_count_y: uint32_t,
        group_count_z: uint32_t,
    ) -> () {
        unsafe {
            self.commands().cmd_dispatch_base(
                self.raw(),
                base_group_x,
                base_group_y,
                base_group_z,
                group_count_x,
                group_count_y,
                group_count_z,
            )
        }
    }
    /// ```c
    /// void vkCmdDispatchIndirect(VkCommandBuffer commandBuffer, VkBuffer buffer, VkDeviceSize offset)
    /// ```
    unsafe fn dispatch_indirect(
        &self,
        buffer: vk::Buffer,
        offset: DeviceSize,
    ) -> () {
        unsafe {
            self.commands().cmd_dispatch_indirect(
                self.raw(),
                buffer,
                offset,
            )
        }
    }
    /// ```c
    /// void vkCmdDraw(VkCommandBuffer commandBuffer, uint32_t vertexCount, uint32_t instanceCount, uint32_t firstVertex, uint32_t firstInstance)
    /// ```
    unsafe fn draw(
        &self,
        vertex_count: uint32_t,
        instance_count: uint32_t,
        first_vertex: uint32_t,
        first_instance: uint32_t,
    ) -> () {
        unsafe {
            self.commands().cmd_draw(
                self.raw(),
                vertex_count,
                instance_count,
                first_vertex,
                first_instance,
            )
        }
    }
    /// ```c
    /// void vkCmdDrawIndexed(VkCommandBuffer commandBuffer, uint32_t indexCount, uint32_t instanceCount, uint32_t firstIndex, int32_t vertexOffset, uint32_t firstInstance)
    /// ```
    unsafe fn draw_indexed(
        &self,
        index_count: uint32_t,
        instance_count: uint32_t,
        first_index: uint32_t,
        vertex_offset: int32_t,
        first_instance: uint32_t,
    ) -> () {
        unsafe {
            self.commands().cmd_draw_indexed(
                self.raw(),
                index_count,
                instance_count,
                first_index,
                vertex_offset,
                first_instance,
            )
        }
    }
    /// ```c
    /// void vkCmdDrawIndexedIndirect(VkCommandBuffer commandBuffer, VkBuffer buffer, VkDeviceSize offset, uint32_t drawCount, uint32_t stride)
    /// ```
    unsafe fn draw_indexed_indirect(
        &self,
        buffer: vk::Buffer,
        offset: DeviceSize,
        draw_count: uint32_t,
        stride: uint32_t,
    ) -> () {
        unsafe {
            self.commands().cmd_draw_indexed_indirect(
                self.raw(),
                buffer,
                offset,
                draw_count,
                stride,
            )
        }
    }
    /// ```c
    /// void vkCmdDrawIndexedIndirectCount(VkCommandBuffer commandBuffer, VkBuffer buffer, VkDeviceSize offset, VkBuffer countBuffer, VkDeviceSize countBufferOffset, uint32_t maxDrawCount, uint32_t stride)
    /// ```
    unsafe fn draw_indexed_indirect_count(
        &self,
        buffer: vk::Buffer,
        offset: DeviceSize,
        count_buffer: vk::Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: uint32_t,
        stride: uint32_t,
    ) -> () {
        unsafe {
            self.commands().cmd_draw_indexed_indirect_count(
                self.raw(),
                buffer,
                offset,
                count_buffer,
                count_buffer_offset,
                max_draw_count,
                stride,
            )
        }
    }
    /// ```c
    /// void vkCmdDrawIndirect(VkCommandBuffer commandBuffer, VkBuffer buffer, VkDeviceSize offset, uint32_t drawCount, uint32_t stride)
    /// ```
    unsafe fn draw_indirect(
        &self,
        buffer: vk::Buffer,
        offset: DeviceSize,
        draw_count: uint32_t,
        stride: uint32_t,
    ) -> () {
        unsafe {
            self.commands().cmd_draw_indirect(
                self.raw(),
                buffer,
                offset,
                draw_count,
                stride,
            )
        }
    }
    /// ```c
    /// void vkCmdDrawIndirectCount(VkCommandBuffer commandBuffer, VkBuffer buffer, VkDeviceSize offset, VkBuffer countBuffer, VkDeviceSize countBufferOffset, uint32_t maxDrawCount, uint32_t stride)
    /// ```
    unsafe fn draw_indirect_count(
        &self,
        buffer: vk::Buffer,
        offset: DeviceSize,
        count_buffer: vk::Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: uint32_t,
        stride: uint32_t,
    ) -> () {
        unsafe {
            self.commands().cmd_draw_indirect_count(
                self.raw(),
                buffer,
                offset,
                count_buffer,
                count_buffer_offset,
                max_draw_count,
                stride,
            )
        }
    }
    /// ```c
    /// void vkCmdEndQuery(VkCommandBuffer commandBuffer, VkQueryPool queryPool, uint32_t query)
    /// ```
    unsafe fn end_query(
        &self,
        query_pool: vk::QueryPool,
        query: uint32_t,
    ) -> () {
        unsafe {
            self.commands().cmd_end_query(
                self.raw(),
                query_pool,
                query,
            )
        }
    }
    /// ```c
    /// void vkCmdEndRenderPass(VkCommandBuffer commandBuffer)
    /// ```
    unsafe fn end_render_pass(
        &self,
    ) -> () {
        unsafe {
            self.commands().cmd_end_render_pass(
                self.raw(),
            )
        }
    }
    /// ```c
    /// void vkCmdEndRenderPass2(VkCommandBuffer commandBuffer, VkSubpassEndInfo const* pSubpassEndInfo)
    /// ```
    unsafe fn end_render_pass2(
        &self,
        subpass_end_info: &SubpassEndInfo,
    ) -> () {
        unsafe {
            self.commands().cmd_end_render_pass2(
                self.raw(),
                subpass_end_info,
            )
        }
    }
    /// ```c
    /// void vkCmdEndRendering(VkCommandBuffer commandBuffer)
    /// ```
    unsafe fn end_rendering(
        &self,
    ) -> () {
        unsafe {
            self.commands().cmd_end_rendering(
                self.raw(),
            )
        }
    }
    /// ```c
    /// void vkCmdExecuteCommands(VkCommandBuffer commandBuffer, uint32_t commandBufferCount, VkCommandBuffer const* pCommandBuffers)
    /// ```
    unsafe fn execute_commands(
        &self,
        command_buffers: &[vk::CommandBuffer],
    ) -> () {
        unsafe {
            self.commands().cmd_execute_commands(
                self.raw(),
                command_buffers,
            )
        }
    }
    /// ```c
    /// void vkCmdFillBuffer(VkCommandBuffer commandBuffer, VkBuffer dstBuffer, VkDeviceSize dstOffset, VkDeviceSize size, uint32_t data)
    /// ```
    unsafe fn fill_buffer(
        &self,
        dst_buffer: vk::Buffer,
        dst_offset: DeviceSize,
        size: DeviceSize,
        data: uint32_t,
    ) -> () {
        unsafe {
            self.commands().cmd_fill_buffer(
                self.raw(),
                dst_buffer,
                dst_offset,
                size,
                data,
            )
        }
    }
    /// ```c
    /// void vkCmdNextSubpass(VkCommandBuffer commandBuffer, VkSubpassContents contents)
    /// ```
    unsafe fn next_subpass(
        &self,
        contents: SubpassContents,
    ) -> () {
        unsafe {
            self.commands().cmd_next_subpass(
                self.raw(),
                contents,
            )
        }
    }
    /// ```c
    /// void vkCmdNextSubpass2(VkCommandBuffer commandBuffer, VkSubpassBeginInfo const* pSubpassBeginInfo, VkSubpassEndInfo const* pSubpassEndInfo)
    /// ```
    unsafe fn next_subpass2(
        &self,
        subpass_begin_info: &SubpassBeginInfo,
        subpass_end_info: &SubpassEndInfo,
    ) -> () {
        unsafe {
            self.commands().cmd_next_subpass2(
                self.raw(),
                subpass_begin_info,
                subpass_end_info,
            )
        }
    }
    /// ```c
    /// void vkCmdPipelineBarrier(VkCommandBuffer commandBuffer, VkPipelineStageFlags srcStageMask, VkPipelineStageFlags dstStageMask, VkDependencyFlags dependencyFlags, uint32_t memoryBarrierCount, VkMemoryBarrier const* pMemoryBarriers, uint32_t bufferMemoryBarrierCount, VkBufferMemoryBarrier const* pBufferMemoryBarriers, uint32_t imageMemoryBarrierCount, VkImageMemoryBarrier const* pImageMemoryBarriers)
    /// ```
    unsafe fn pipeline_barrier(
        &self,
        src_stage_mask: PipelineStageFlags,
        dst_stage_mask: PipelineStageFlags,
        dependency_flags: DependencyFlags,
        memory_barriers: &[MemoryBarrier],
        buffer_memory_barriers: &[BufferMemoryBarrier],
        image_memory_barriers: &[ImageMemoryBarrier],
    ) -> () {
        unsafe {
            self.commands().cmd_pipeline_barrier(
                self.raw(),
                src_stage_mask,
                dst_stage_mask,
                dependency_flags,
                memory_barriers,
                buffer_memory_barriers,
                image_memory_barriers,
            )
        }
    }
    /// ```c
    /// void vkCmdPipelineBarrier2(VkCommandBuffer commandBuffer, VkDependencyInfo const* pDependencyInfo)
    /// ```
    unsafe fn pipeline_barrier2(
        &self,
        dependency_info: &DependencyInfo,
    ) -> () {
        unsafe {
            self.commands().cmd_pipeline_barrier2(
                self.raw(),
                dependency_info,
            )
        }
    }
    /// ```c
    /// void vkCmdPushConstants(VkCommandBuffer commandBuffer, VkPipelineLayout layout, VkShaderStageFlags stageFlags, uint32_t offset, uint32_t size, void const* pValues)
    /// ```
    unsafe fn push_constants(
        &self,
        layout: vk::PipelineLayout,
        stage_flags: ShaderStageFlags,
        offset: uint32_t,
        size: uint32_t,
        values: *const void,
    ) -> () {
        unsafe {
            self.commands().cmd_push_constants(
                self.raw(),
                layout,
                stage_flags,
                offset,
                size,
                values,
            )
        }
    }
    /// ```c
    /// void vkCmdPushConstants2(VkCommandBuffer commandBuffer, VkPushConstantsInfo const* pPushConstantsInfo)
    /// ```
    unsafe fn push_constants2(
        &self,
        push_constants_info: &PushConstantsInfo,
    ) -> () {
        unsafe {
            self.commands().cmd_push_constants2(
                self.raw(),
                push_constants_info,
            )
        }
    }
    /// ```c
    /// void vkCmdPushDescriptorSet(VkCommandBuffer commandBuffer, VkPipelineBindPoint pipelineBindPoint, VkPipelineLayout layout, uint32_t set, uint32_t descriptorWriteCount, VkWriteDescriptorSet const* pDescriptorWrites)
    /// ```
    unsafe fn push_descriptor_set(
        &self,
        pipeline_bind_point: PipelineBindPoint,
        layout: vk::PipelineLayout,
        set: uint32_t,
        descriptor_writes: &[WriteDescriptorSet],
    ) -> () {
        unsafe {
            self.commands().cmd_push_descriptor_set(
                self.raw(),
                pipeline_bind_point,
                layout,
                set,
                descriptor_writes,
            )
        }
    }
    /// ```c
    /// void vkCmdPushDescriptorSet2(VkCommandBuffer commandBuffer, VkPushDescriptorSetInfo const* pPushDescriptorSetInfo)
    /// ```
    unsafe fn push_descriptor_set2(
        &self,
        push_descriptor_set_info: &PushDescriptorSetInfo,
    ) -> () {
        unsafe {
            self.commands().cmd_push_descriptor_set2(
                self.raw(),
                push_descriptor_set_info,
            )
        }
    }
    /// ```c
    /// void vkCmdPushDescriptorSetWithTemplate(VkCommandBuffer commandBuffer, VkDescriptorUpdateTemplate descriptorUpdateTemplate, VkPipelineLayout layout, uint32_t set, void const* pData)
    /// ```
    unsafe fn push_descriptor_set_with_template(
        &self,
        descriptor_update_template: vk::DescriptorUpdateTemplate,
        layout: vk::PipelineLayout,
        set: uint32_t,
        data: *const void,
    ) -> () {
        unsafe {
            self.commands().cmd_push_descriptor_set_with_template(
                self.raw(),
                descriptor_update_template,
                layout,
                set,
                data,
            )
        }
    }
    /// ```c
    /// void vkCmdPushDescriptorSetWithTemplate2(VkCommandBuffer commandBuffer, VkPushDescriptorSetWithTemplateInfo const* pPushDescriptorSetWithTemplateInfo)
    /// ```
    unsafe fn push_descriptor_set_with_template2(
        &self,
        push_descriptor_set_with_template_info: &PushDescriptorSetWithTemplateInfo,
    ) -> () {
        unsafe {
            self.commands().cmd_push_descriptor_set_with_template2(
                self.raw(),
                push_descriptor_set_with_template_info,
            )
        }
    }
    /// ```c
    /// void vkCmdResetEvent(VkCommandBuffer commandBuffer, VkEvent event, VkPipelineStageFlags stageMask)
    /// ```
    unsafe fn reset_event(
        &self,
        event: vk::Event,
        stage_mask: PipelineStageFlags,
    ) -> () {
        unsafe {
            self.commands().cmd_reset_event(
                self.raw(),
                event,
                stage_mask,
            )
        }
    }
    /// ```c
    /// void vkCmdResetEvent2(VkCommandBuffer commandBuffer, VkEvent event, VkPipelineStageFlags2 stageMask)
    /// ```
    unsafe fn reset_event2(
        &self,
        event: vk::Event,
        stage_mask: PipelineStageFlags2,
    ) -> () {
        unsafe {
            self.commands().cmd_reset_event2(
                self.raw(),
                event,
                stage_mask,
            )
        }
    }
    /// ```c
    /// void vkCmdResetQueryPool(VkCommandBuffer commandBuffer, VkQueryPool queryPool, uint32_t firstQuery, uint32_t queryCount)
    /// ```
    unsafe fn reset_query_pool(
        &self,
        query_pool: vk::QueryPool,
        first_query: uint32_t,
        query_count: uint32_t,
    ) -> () {
        unsafe {
            self.commands().cmd_reset_query_pool(
                self.raw(),
                query_pool,
                first_query,
                query_count,
            )
        }
    }
    /// ```c
    /// void vkCmdResolveImage(VkCommandBuffer commandBuffer, VkImage srcImage, VkImageLayout srcImageLayout, VkImage dstImage, VkImageLayout dstImageLayout, uint32_t regionCount, VkImageResolve const* pRegions)
    /// ```
    unsafe fn resolve_image(
        &self,
        src_image: vk::Image,
        src_image_layout: ImageLayout,
        dst_image: vk::Image,
        dst_image_layout: ImageLayout,
        regions: &[ImageResolve],
    ) -> () {
        unsafe {
            self.commands().cmd_resolve_image(
                self.raw(),
                src_image,
                src_image_layout,
                dst_image,
                dst_image_layout,
                regions,
            )
        }
    }
    /// ```c
    /// void vkCmdResolveImage2(VkCommandBuffer commandBuffer, VkResolveImageInfo2 const* pResolveImageInfo)
    /// ```
    unsafe fn resolve_image2(
        &self,
        resolve_image_info: &ResolveImageInfo2,
    ) -> () {
        unsafe {
            self.commands().cmd_resolve_image2(
                self.raw(),
                resolve_image_info,
            )
        }
    }
    /// ```c
    /// void vkCmdSetBlendConstants(VkCommandBuffer commandBuffer, const float blendConstants[4])
    /// ```
    unsafe fn set_blend_constants(
        &self,
        blend_constants: [float; 4],
    ) -> () {
        unsafe {
            self.commands().cmd_set_blend_constants(
                self.raw(),
                blend_constants,
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
    /// void vkCmdSetDepthBias(VkCommandBuffer commandBuffer, float depthBiasConstantFactor, float depthBiasClamp, float depthBiasSlopeFactor)
    /// ```
    unsafe fn set_depth_bias(
        &self,
        depth_bias_constant_factor: float,
        depth_bias_clamp: float,
        depth_bias_slope_factor: float,
    ) -> () {
        unsafe {
            self.commands().cmd_set_depth_bias(
                self.raw(),
                depth_bias_constant_factor,
                depth_bias_clamp,
                depth_bias_slope_factor,
            )
        }
    }
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
    /// void vkCmdSetDepthBounds(VkCommandBuffer commandBuffer, float minDepthBounds, float maxDepthBounds)
    /// ```
    unsafe fn set_depth_bounds(
        &self,
        min_depth_bounds: float,
        max_depth_bounds: float,
    ) -> () {
        unsafe {
            self.commands().cmd_set_depth_bounds(
                self.raw(),
                min_depth_bounds,
                max_depth_bounds,
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
    /// void vkCmdSetDeviceMask(VkCommandBuffer commandBuffer, uint32_t deviceMask)
    /// ```
    unsafe fn set_device_mask(
        &self,
        device_mask: uint32_t,
    ) -> () {
        unsafe {
            self.commands().cmd_set_device_mask(
                self.raw(),
                device_mask,
            )
        }
    }
    /// ```c
    /// void vkCmdSetEvent(VkCommandBuffer commandBuffer, VkEvent event, VkPipelineStageFlags stageMask)
    /// ```
    unsafe fn set_event(
        &self,
        event: vk::Event,
        stage_mask: PipelineStageFlags,
    ) -> () {
        unsafe {
            self.commands().cmd_set_event(
                self.raw(),
                event,
                stage_mask,
            )
        }
    }
    /// ```c
    /// void vkCmdSetEvent2(VkCommandBuffer commandBuffer, VkEvent event, VkDependencyInfo const* pDependencyInfo)
    /// ```
    unsafe fn set_event2(
        &self,
        event: vk::Event,
        dependency_info: &DependencyInfo,
    ) -> () {
        unsafe {
            self.commands().cmd_set_event2(
                self.raw(),
                event,
                dependency_info,
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
    /// void vkCmdSetLineStipple(VkCommandBuffer commandBuffer, uint32_t lineStippleFactor, uint16_t lineStipplePattern)
    /// ```
    unsafe fn set_line_stipple(
        &self,
        line_stipple_factor: uint32_t,
        line_stipple_pattern: uint16_t,
    ) -> () {
        unsafe {
            self.commands().cmd_set_line_stipple(
                self.raw(),
                line_stipple_factor,
                line_stipple_pattern,
            )
        }
    }
    /// ```c
    /// void vkCmdSetLineWidth(VkCommandBuffer commandBuffer, float lineWidth)
    /// ```
    unsafe fn set_line_width(
        &self,
        line_width: float,
    ) -> () {
        unsafe {
            self.commands().cmd_set_line_width(
                self.raw(),
                line_width,
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
    /// ```c
    /// void vkCmdSetRenderingAttachmentLocations(VkCommandBuffer commandBuffer, VkRenderingAttachmentLocationInfo const* pLocationInfo)
    /// ```
    unsafe fn set_rendering_attachment_locations(
        &self,
        location_info: &RenderingAttachmentLocationInfo,
    ) -> () {
        unsafe {
            self.commands().cmd_set_rendering_attachment_locations(
                self.raw(),
                location_info,
            )
        }
    }
    /// ```c
    /// void vkCmdSetRenderingInputAttachmentIndices(VkCommandBuffer commandBuffer, VkRenderingInputAttachmentIndexInfo const* pInputAttachmentIndexInfo)
    /// ```
    unsafe fn set_rendering_input_attachment_indices(
        &self,
        input_attachment_index_info: &RenderingInputAttachmentIndexInfo,
    ) -> () {
        unsafe {
            self.commands().cmd_set_rendering_input_attachment_indices(
                self.raw(),
                input_attachment_index_info,
            )
        }
    }
    /// ```c
    /// void vkCmdSetScissor(VkCommandBuffer commandBuffer, uint32_t firstScissor, uint32_t scissorCount, VkRect2D const* pScissors)
    /// ```
    unsafe fn set_scissor(
        &self,
        first_scissor: uint32_t,
        scissors: &[Rect2D],
    ) -> () {
        unsafe {
            self.commands().cmd_set_scissor(
                self.raw(),
                first_scissor,
                scissors,
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
    /// void vkCmdSetStencilCompareMask(VkCommandBuffer commandBuffer, VkStencilFaceFlags faceMask, uint32_t compareMask)
    /// ```
    unsafe fn set_stencil_compare_mask(
        &self,
        face_mask: StencilFaceFlags,
        compare_mask: uint32_t,
    ) -> () {
        unsafe {
            self.commands().cmd_set_stencil_compare_mask(
                self.raw(),
                face_mask,
                compare_mask,
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
    /// void vkCmdSetStencilReference(VkCommandBuffer commandBuffer, VkStencilFaceFlags faceMask, uint32_t reference)
    /// ```
    unsafe fn set_stencil_reference(
        &self,
        face_mask: StencilFaceFlags,
        reference: uint32_t,
    ) -> () {
        unsafe {
            self.commands().cmd_set_stencil_reference(
                self.raw(),
                face_mask,
                reference,
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
    /// void vkCmdSetStencilWriteMask(VkCommandBuffer commandBuffer, VkStencilFaceFlags faceMask, uint32_t writeMask)
    /// ```
    unsafe fn set_stencil_write_mask(
        &self,
        face_mask: StencilFaceFlags,
        write_mask: uint32_t,
    ) -> () {
        unsafe {
            self.commands().cmd_set_stencil_write_mask(
                self.raw(),
                face_mask,
                write_mask,
            )
        }
    }
    /// ```c
    /// void vkCmdSetViewport(VkCommandBuffer commandBuffer, uint32_t firstViewport, uint32_t viewportCount, VkViewport const* pViewports)
    /// ```
    unsafe fn set_viewport(
        &self,
        first_viewport: uint32_t,
        viewports: &[Viewport],
    ) -> () {
        unsafe {
            self.commands().cmd_set_viewport(
                self.raw(),
                first_viewport,
                viewports,
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
    /// ```c
    /// void vkCmdUpdateBuffer(VkCommandBuffer commandBuffer, VkBuffer dstBuffer, VkDeviceSize dstOffset, VkDeviceSize dataSize, void const* pData)
    /// ```
    unsafe fn update_buffer(
        &self,
        dst_buffer: vk::Buffer,
        dst_offset: DeviceSize,
        data_size: DeviceSize,
        data: *const void,
    ) -> () {
        unsafe {
            self.commands().cmd_update_buffer(
                self.raw(),
                dst_buffer,
                dst_offset,
                data_size,
                data,
            )
        }
    }
    /// ```c
    /// void vkCmdWaitEvents(VkCommandBuffer commandBuffer, uint32_t eventCount, VkEvent const* pEvents, VkPipelineStageFlags srcStageMask, VkPipelineStageFlags dstStageMask, uint32_t memoryBarrierCount, VkMemoryBarrier const* pMemoryBarriers, uint32_t bufferMemoryBarrierCount, VkBufferMemoryBarrier const* pBufferMemoryBarriers, uint32_t imageMemoryBarrierCount, VkImageMemoryBarrier const* pImageMemoryBarriers)
    /// ```
    unsafe fn wait_events(
        &self,
        events: &[vk::Event],
        src_stage_mask: PipelineStageFlags,
        dst_stage_mask: PipelineStageFlags,
        memory_barriers: &[MemoryBarrier],
        buffer_memory_barriers: &[BufferMemoryBarrier],
        image_memory_barriers: &[ImageMemoryBarrier],
    ) -> () {
        unsafe {
            self.commands().cmd_wait_events(
                self.raw(),
                events,
                src_stage_mask,
                dst_stage_mask,
                memory_barriers,
                buffer_memory_barriers,
                image_memory_barriers,
            )
        }
    }
    /// ```c
    /// void vkCmdWaitEvents2(VkCommandBuffer commandBuffer, uint32_t eventCount, VkEvent const* pEvents, VkDependencyInfo const* pDependencyInfos)
    /// ```
    unsafe fn wait_events2(
        &self,
        events: &[vk::Event],
        dependency_infos: &[DependencyInfo],
    ) -> () {
        unsafe {
            self.commands().cmd_wait_events2(
                self.raw(),
                events,
                dependency_infos,
            )
        }
    }
    /// ```c
    /// void vkCmdWriteTimestamp(VkCommandBuffer commandBuffer, VkPipelineStageFlagBits pipelineStage, VkQueryPool queryPool, uint32_t query)
    /// ```
    unsafe fn write_timestamp(
        &self,
        pipeline_stage: PipelineStageFlags,
        query_pool: vk::QueryPool,
        query: uint32_t,
    ) -> () {
        unsafe {
            self.commands().cmd_write_timestamp(
                self.raw(),
                pipeline_stage,
                query_pool,
                query,
            )
        }
    }
    /// ```c
    /// void vkCmdWriteTimestamp2(VkCommandBuffer commandBuffer, VkPipelineStageFlags2 stage, VkQueryPool queryPool, uint32_t query)
    /// ```
    unsafe fn write_timestamp2(
        &self,
        stage: PipelineStageFlags2,
        query_pool: vk::QueryPool,
        query: uint32_t,
    ) -> () {
        unsafe {
            self.commands().cmd_write_timestamp2(
                self.raw(),
                stage,
                query_pool,
                query,
            )
        }
    }
    /// ```c
    /// VkResult vkEndCommandBuffer(VkCommandBuffer commandBuffer)
    /// ```
    unsafe fn end(
        &self,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().end_command_buffer(
                self.raw(),
            )
        }
    }
    /// ```c
    /// VkResult vkResetCommandBuffer(VkCommandBuffer commandBuffer, VkCommandBufferResetFlags flags)
    /// ```
    unsafe fn reset(
        &self,
        flags: CommandBufferResetFlags,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().reset_command_buffer(
                self.raw(),
                flags,
            )
        }
    }
}

/// Queue object
pub trait CoreQueue {
    fn raw(&self) -> vk::Queue;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkQueueBindSparse(VkQueue queue, uint32_t bindInfoCount, VkBindSparseInfo const* pBindInfo, VkFence fence)
    /// ```
    unsafe fn bind_sparse(
        &self,
        bind_info: &[BindSparseInfo],
        fence: Option<vk::Fence>,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().queue_bind_sparse(
                self.raw(),
                bind_info,
                fence,
            )
        }
    }
    /// ```c
    /// VkResult vkQueueSubmit(VkQueue queue, uint32_t submitCount, VkSubmitInfo const* pSubmits, VkFence fence)
    /// ```
    unsafe fn submit(
        &self,
        submits: &[SubmitInfo],
        fence: Option<vk::Fence>,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().queue_submit(
                self.raw(),
                submits,
                fence,
            )
        }
    }
    /// ```c
    /// VkResult vkQueueSubmit2(VkQueue queue, uint32_t submitCount, VkSubmitInfo2 const* pSubmits, VkFence fence)
    /// ```
    unsafe fn submit2(
        &self,
        submits: &[SubmitInfo2],
        fence: Option<vk::Fence>,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().queue_submit2(
                self.raw(),
                submits,
                fence,
            )
        }
    }
    /// ```c
    /// VkResult vkQueueWaitIdle(VkQueue queue)
    /// ```
    unsafe fn wait_idle(
        &self,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().queue_wait_idle(
                self.raw(),
            )
        }
    }
}

impl<O: crate::HndCtx<vk::core, vk::Device>> crate::Owner<vk::Buffer, vk::core> for O {
    fn drop(&mut self, raw: vk::Buffer) {
        unsafe {
            self.commands().destroy_buffer(self.raw(), Some(raw))
        }
    }
}
impl<O> crate::HndCtx<vk::core, vk::Device> for ::alloc::sync::Arc<crate::Unique<vk::Buffer, O, vk::core>>
    where O: crate::HndCtx<vk::core, vk::Device> + Send + Sync + 'static,
{
    type Ctx = Self;
    fn ctx(&self) -> Self::Ctx { self.clone() }
    fn raw(&self) -> vk::Device { self.owner.raw() }
    fn commands(&self) -> &::alloc::sync::Arc<Device> { self.owner.commands() }
}

impl crate::HndScope<vk::Buffer> for vk::core {
    type Impl = _hs_Buffer::Buffer;
}

mod _hs_Buffer {
    use super::*;

    #[repr(transparent)]
    #[derive(Debug)]
    pub struct Buffer(pub(crate) crate::handle::Hnd<vk::Buffer, ::alloc::sync::Arc<super::Device>>);

    impl Clone for Buffer {
        fn clone(&self) -> Self { Self(self.0.clone()) }
    }

    #[derive(Debug, Clone, Copy)]
    struct Inst;
    impl<Ctx, Dep> crate::handle::HndInst<vk::Buffer, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)> for Inst
        where Ctx: crate::HndCtx<vk::core, vk::Device>,
    {
        fn drop(this: &mut crate::handle::HndData<vk::Buffer, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)>) {
            unsafe {
                this.scope.destroy_buffer(this.dep.0.raw(), Some(this.raw))
            }
        }
    }

    impl crate::hnd::Buffer<vk::core>
    {
        pub unsafe fn new(ctx: &impl crate::HndCtx<vk::core, vk::Device>, raw: vk::Buffer) -> Self {
            unsafe { Self::new_with(ctx, raw, || ()) }
        }
        pub unsafe fn new_with<Dep: Send + Sync + 'static>(ctx: &impl crate::HndCtx<vk::core, vk::Device>, raw: vk::Buffer, dep: impl FnOnce() -> Dep) -> Self {
            Self(Buffer(crate::handle::Hnd::new(
                ctx.commands().clone(),
                raw,
                (ctx.ctx(), dep()),
                Inst,
            )))
        }
    }

    impl crate::hnd::Buffer<vk::core> {
        pub fn raw(&self) -> vk::Buffer { self.0.0.raw }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0.scope }
    }

    impl ::core::fmt::Debug for crate::hnd::Buffer<vk::core> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Buffer({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Buffer<vk::core> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl<Ctx> crate::MakeHnd<Ctx, vk::core> for vk::Buffer
        where Ctx: crate::HndCtx<vk::core, vk::Device>,
    {
        type Output = crate::hnd::Buffer<vk::core>;
        unsafe fn hnd_with<Dep: Send + Sync + 'static>(self, ctx: &Ctx, dep: impl FnOnce() -> Dep) -> Self::Output {
            unsafe { crate::hnd::Buffer::<vk::core>::new_with(ctx, self, dep) }
        }
    }
}

impl<O: crate::HndCtx<vk::core, vk::Device>> crate::Owner<vk::BufferView, vk::core> for O {
    fn drop(&mut self, raw: vk::BufferView) {
        unsafe {
            self.commands().destroy_buffer_view(self.raw(), Some(raw))
        }
    }
}
impl<O> crate::HndCtx<vk::core, vk::Device> for ::alloc::sync::Arc<crate::Unique<vk::BufferView, O, vk::core>>
    where O: crate::HndCtx<vk::core, vk::Device> + Send + Sync + 'static,
{
    type Ctx = Self;
    fn ctx(&self) -> Self::Ctx { self.clone() }
    fn raw(&self) -> vk::Device { self.owner.raw() }
    fn commands(&self) -> &::alloc::sync::Arc<Device> { self.owner.commands() }
}

impl crate::HndScope<vk::BufferView> for vk::core {
    type Impl = _hs_BufferView::BufferView;
}

mod _hs_BufferView {
    use super::*;

    #[repr(transparent)]
    #[derive(Debug)]
    pub struct BufferView(pub(crate) crate::handle::Hnd<vk::BufferView, ::alloc::sync::Arc<super::Device>>);

    impl Clone for BufferView {
        fn clone(&self) -> Self { Self(self.0.clone()) }
    }

    #[derive(Debug, Clone, Copy)]
    struct Inst;
    impl<Ctx, Dep> crate::handle::HndInst<vk::BufferView, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)> for Inst
        where Ctx: crate::HndCtx<vk::core, vk::Device>,
    {
        fn drop(this: &mut crate::handle::HndData<vk::BufferView, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)>) {
            unsafe {
                this.scope.destroy_buffer_view(this.dep.0.raw(), Some(this.raw))
            }
        }
    }

    impl crate::hnd::BufferView<vk::core>
    {
        pub unsafe fn new(ctx: &impl crate::HndCtx<vk::core, vk::Device>, raw: vk::BufferView) -> Self {
            unsafe { Self::new_with(ctx, raw, || ()) }
        }
        pub unsafe fn new_with<Dep: Send + Sync + 'static>(ctx: &impl crate::HndCtx<vk::core, vk::Device>, raw: vk::BufferView, dep: impl FnOnce() -> Dep) -> Self {
            Self(BufferView(crate::handle::Hnd::new(
                ctx.commands().clone(),
                raw,
                (ctx.ctx(), dep()),
                Inst,
            )))
        }
    }

    impl crate::hnd::BufferView<vk::core> {
        pub fn raw(&self) -> vk::BufferView { self.0.0.raw }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0.scope }
    }

    impl ::core::fmt::Debug for crate::hnd::BufferView<vk::core> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("BufferView({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::BufferView<vk::core> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl<Ctx> crate::MakeHnd<Ctx, vk::core> for vk::BufferView
        where Ctx: crate::HndCtx<vk::core, vk::Device>,
    {
        type Output = crate::hnd::BufferView<vk::core>;
        unsafe fn hnd_with<Dep: Send + Sync + 'static>(self, ctx: &Ctx, dep: impl FnOnce() -> Dep) -> Self::Output {
            unsafe { crate::hnd::BufferView::<vk::core>::new_with(ctx, self, dep) }
        }
    }
}

impl<O: crate::HndCtx<vk::core, vk::Device>> crate::Owner<vk::CommandPool, vk::core> for O {
    fn drop(&mut self, raw: vk::CommandPool) {
        unsafe {
            self.commands().destroy_command_pool(self.raw(), Some(raw))
        }
    }
}
impl<O> crate::HndCtx<vk::core, vk::Device> for ::alloc::sync::Arc<crate::Unique<vk::CommandPool, O, vk::core>>
    where O: crate::HndCtx<vk::core, vk::Device> + Send + Sync + 'static,
{
    type Ctx = Self;
    fn ctx(&self) -> Self::Ctx { self.clone() }
    fn raw(&self) -> vk::Device { self.owner.raw() }
    fn commands(&self) -> &::alloc::sync::Arc<Device> { self.owner.commands() }
}

impl crate::HndScope<vk::CommandPool> for vk::core {
    type Impl = _hs_CommandPool::CommandPool;
}

mod _hs_CommandPool {
    use super::*;

    #[repr(transparent)]
    #[derive(Debug)]
    pub struct CommandPool(pub(crate) crate::handle::Hnd<vk::CommandPool, ::alloc::sync::Arc<super::Device>>);

    impl Clone for CommandPool {
        fn clone(&self) -> Self { Self(self.0.clone()) }
    }

    #[derive(Debug, Clone, Copy)]
    struct Inst;
    impl<Ctx, Dep> crate::handle::HndInst<vk::CommandPool, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)> for Inst
        where Ctx: crate::HndCtx<vk::core, vk::Device>,
    {
        fn drop(this: &mut crate::handle::HndData<vk::CommandPool, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)>) {
            unsafe {
                this.scope.destroy_command_pool(this.dep.0.raw(), Some(this.raw))
            }
        }
    }

    impl crate::hnd::CommandPool<vk::core>
    {
        pub unsafe fn new(ctx: &impl crate::HndCtx<vk::core, vk::Device>, raw: vk::CommandPool) -> Self {
            unsafe { Self::new_with(ctx, raw, || ()) }
        }
        pub unsafe fn new_with<Dep: Send + Sync + 'static>(ctx: &impl crate::HndCtx<vk::core, vk::Device>, raw: vk::CommandPool, dep: impl FnOnce() -> Dep) -> Self {
            Self(CommandPool(crate::handle::Hnd::new(
                ctx.commands().clone(),
                raw,
                (ctx.ctx(), dep()),
                Inst,
            )))
        }
    }

    impl crate::hnd::CommandPool<vk::core> {
        pub fn raw(&self) -> vk::CommandPool { self.0.0.raw }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0.scope }
    }

    impl ::core::fmt::Debug for crate::hnd::CommandPool<vk::core> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("CommandPool({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::CommandPool<vk::core> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl<Ctx> crate::MakeHnd<Ctx, vk::core> for vk::CommandPool
        where Ctx: crate::HndCtx<vk::core, vk::Device>,
    {
        type Output = crate::hnd::CommandPool<vk::core>;
        unsafe fn hnd_with<Dep: Send + Sync + 'static>(self, ctx: &Ctx, dep: impl FnOnce() -> Dep) -> Self::Output {
            unsafe { crate::hnd::CommandPool::<vk::core>::new_with(ctx, self, dep) }
        }
    }
}

impl<O: crate::HndCtx<vk::core, vk::Device>> crate::Owner<vk::DescriptorPool, vk::core> for O {
    fn drop(&mut self, raw: vk::DescriptorPool) {
        unsafe {
            self.commands().destroy_descriptor_pool(self.raw(), Some(raw))
        }
    }
}
impl<O> crate::HndCtx<vk::core, vk::Device> for ::alloc::sync::Arc<crate::Unique<vk::DescriptorPool, O, vk::core>>
    where O: crate::HndCtx<vk::core, vk::Device> + Send + Sync + 'static,
{
    type Ctx = Self;
    fn ctx(&self) -> Self::Ctx { self.clone() }
    fn raw(&self) -> vk::Device { self.owner.raw() }
    fn commands(&self) -> &::alloc::sync::Arc<Device> { self.owner.commands() }
}

impl crate::HndScope<vk::DescriptorPool> for vk::core {
    type Impl = _hs_DescriptorPool::DescriptorPool;
}

mod _hs_DescriptorPool {
    use super::*;

    #[repr(transparent)]
    #[derive(Debug)]
    pub struct DescriptorPool(pub(crate) crate::handle::Hnd<vk::DescriptorPool, ::alloc::sync::Arc<super::Device>>);

    impl Clone for DescriptorPool {
        fn clone(&self) -> Self { Self(self.0.clone()) }
    }

    #[derive(Debug, Clone, Copy)]
    struct Inst;
    impl<Ctx, Dep> crate::handle::HndInst<vk::DescriptorPool, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)> for Inst
        where Ctx: crate::HndCtx<vk::core, vk::Device>,
    {
        fn drop(this: &mut crate::handle::HndData<vk::DescriptorPool, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)>) {
            unsafe {
                this.scope.destroy_descriptor_pool(this.dep.0.raw(), Some(this.raw))
            }
        }
    }

    impl crate::hnd::DescriptorPool<vk::core>
    {
        pub unsafe fn new(ctx: &impl crate::HndCtx<vk::core, vk::Device>, raw: vk::DescriptorPool) -> Self {
            unsafe { Self::new_with(ctx, raw, || ()) }
        }
        pub unsafe fn new_with<Dep: Send + Sync + 'static>(ctx: &impl crate::HndCtx<vk::core, vk::Device>, raw: vk::DescriptorPool, dep: impl FnOnce() -> Dep) -> Self {
            Self(DescriptorPool(crate::handle::Hnd::new(
                ctx.commands().clone(),
                raw,
                (ctx.ctx(), dep()),
                Inst,
            )))
        }
    }

    impl crate::hnd::DescriptorPool<vk::core> {
        pub fn raw(&self) -> vk::DescriptorPool { self.0.0.raw }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0.scope }
    }

    impl ::core::fmt::Debug for crate::hnd::DescriptorPool<vk::core> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("DescriptorPool({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::DescriptorPool<vk::core> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl<Ctx> crate::MakeHnd<Ctx, vk::core> for vk::DescriptorPool
        where Ctx: crate::HndCtx<vk::core, vk::Device>,
    {
        type Output = crate::hnd::DescriptorPool<vk::core>;
        unsafe fn hnd_with<Dep: Send + Sync + 'static>(self, ctx: &Ctx, dep: impl FnOnce() -> Dep) -> Self::Output {
            unsafe { crate::hnd::DescriptorPool::<vk::core>::new_with(ctx, self, dep) }
        }
    }
}

impl<O: crate::HndCtx<vk::core, vk::Device>> crate::Owner<vk::DescriptorSetLayout, vk::core> for O {
    fn drop(&mut self, raw: vk::DescriptorSetLayout) {
        unsafe {
            self.commands().destroy_descriptor_set_layout(self.raw(), Some(raw))
        }
    }
}
impl<O> crate::HndCtx<vk::core, vk::Device> for ::alloc::sync::Arc<crate::Unique<vk::DescriptorSetLayout, O, vk::core>>
    where O: crate::HndCtx<vk::core, vk::Device> + Send + Sync + 'static,
{
    type Ctx = Self;
    fn ctx(&self) -> Self::Ctx { self.clone() }
    fn raw(&self) -> vk::Device { self.owner.raw() }
    fn commands(&self) -> &::alloc::sync::Arc<Device> { self.owner.commands() }
}

impl crate::HndScope<vk::DescriptorSetLayout> for vk::core {
    type Impl = _hs_DescriptorSetLayout::DescriptorSetLayout;
}

mod _hs_DescriptorSetLayout {
    use super::*;

    #[repr(transparent)]
    #[derive(Debug)]
    pub struct DescriptorSetLayout(pub(crate) crate::handle::Hnd<vk::DescriptorSetLayout, ::alloc::sync::Arc<super::Device>>);

    impl Clone for DescriptorSetLayout {
        fn clone(&self) -> Self { Self(self.0.clone()) }
    }

    #[derive(Debug, Clone, Copy)]
    struct Inst;
    impl<Ctx, Dep> crate::handle::HndInst<vk::DescriptorSetLayout, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)> for Inst
        where Ctx: crate::HndCtx<vk::core, vk::Device>,
    {
        fn drop(this: &mut crate::handle::HndData<vk::DescriptorSetLayout, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)>) {
            unsafe {
                this.scope.destroy_descriptor_set_layout(this.dep.0.raw(), Some(this.raw))
            }
        }
    }

    impl crate::hnd::DescriptorSetLayout<vk::core>
    {
        pub unsafe fn new(ctx: &impl crate::HndCtx<vk::core, vk::Device>, raw: vk::DescriptorSetLayout) -> Self {
            unsafe { Self::new_with(ctx, raw, || ()) }
        }
        pub unsafe fn new_with<Dep: Send + Sync + 'static>(ctx: &impl crate::HndCtx<vk::core, vk::Device>, raw: vk::DescriptorSetLayout, dep: impl FnOnce() -> Dep) -> Self {
            Self(DescriptorSetLayout(crate::handle::Hnd::new(
                ctx.commands().clone(),
                raw,
                (ctx.ctx(), dep()),
                Inst,
            )))
        }
    }

    impl crate::hnd::DescriptorSetLayout<vk::core> {
        pub fn raw(&self) -> vk::DescriptorSetLayout { self.0.0.raw }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0.scope }
    }

    impl ::core::fmt::Debug for crate::hnd::DescriptorSetLayout<vk::core> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("DescriptorSetLayout({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::DescriptorSetLayout<vk::core> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl<Ctx> crate::MakeHnd<Ctx, vk::core> for vk::DescriptorSetLayout
        where Ctx: crate::HndCtx<vk::core, vk::Device>,
    {
        type Output = crate::hnd::DescriptorSetLayout<vk::core>;
        unsafe fn hnd_with<Dep: Send + Sync + 'static>(self, ctx: &Ctx, dep: impl FnOnce() -> Dep) -> Self::Output {
            unsafe { crate::hnd::DescriptorSetLayout::<vk::core>::new_with(ctx, self, dep) }
        }
    }
}

impl<O: crate::HndCtx<vk::core, vk::Device>> crate::Owner<vk::DescriptorUpdateTemplate, vk::core> for O {
    fn drop(&mut self, raw: vk::DescriptorUpdateTemplate) {
        unsafe {
            self.commands().destroy_descriptor_update_template(self.raw(), Some(raw))
        }
    }
}
impl<O> crate::HndCtx<vk::core, vk::Device> for ::alloc::sync::Arc<crate::Unique<vk::DescriptorUpdateTemplate, O, vk::core>>
    where O: crate::HndCtx<vk::core, vk::Device> + Send + Sync + 'static,
{
    type Ctx = Self;
    fn ctx(&self) -> Self::Ctx { self.clone() }
    fn raw(&self) -> vk::Device { self.owner.raw() }
    fn commands(&self) -> &::alloc::sync::Arc<Device> { self.owner.commands() }
}

impl crate::HndScope<vk::DescriptorUpdateTemplate> for vk::core {
    type Impl = _hs_DescriptorUpdateTemplate::DescriptorUpdateTemplate;
}

mod _hs_DescriptorUpdateTemplate {
    use super::*;

    #[repr(transparent)]
    #[derive(Debug)]
    pub struct DescriptorUpdateTemplate(pub(crate) crate::handle::Hnd<vk::DescriptorUpdateTemplate, ::alloc::sync::Arc<super::Device>>);

    impl Clone for DescriptorUpdateTemplate {
        fn clone(&self) -> Self { Self(self.0.clone()) }
    }

    #[derive(Debug, Clone, Copy)]
    struct Inst;
    impl<Ctx, Dep> crate::handle::HndInst<vk::DescriptorUpdateTemplate, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)> for Inst
        where Ctx: crate::HndCtx<vk::core, vk::Device>,
    {
        fn drop(this: &mut crate::handle::HndData<vk::DescriptorUpdateTemplate, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)>) {
            unsafe {
                this.scope.destroy_descriptor_update_template(this.dep.0.raw(), Some(this.raw))
            }
        }
    }

    impl crate::hnd::DescriptorUpdateTemplate<vk::core>
    {
        pub unsafe fn new(ctx: &impl crate::HndCtx<vk::core, vk::Device>, raw: vk::DescriptorUpdateTemplate) -> Self {
            unsafe { Self::new_with(ctx, raw, || ()) }
        }
        pub unsafe fn new_with<Dep: Send + Sync + 'static>(ctx: &impl crate::HndCtx<vk::core, vk::Device>, raw: vk::DescriptorUpdateTemplate, dep: impl FnOnce() -> Dep) -> Self {
            Self(DescriptorUpdateTemplate(crate::handle::Hnd::new(
                ctx.commands().clone(),
                raw,
                (ctx.ctx(), dep()),
                Inst,
            )))
        }
    }

    impl crate::hnd::DescriptorUpdateTemplate<vk::core> {
        pub fn raw(&self) -> vk::DescriptorUpdateTemplate { self.0.0.raw }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0.scope }
    }

    impl ::core::fmt::Debug for crate::hnd::DescriptorUpdateTemplate<vk::core> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("DescriptorUpdateTemplate({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::DescriptorUpdateTemplate<vk::core> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl<Ctx> crate::MakeHnd<Ctx, vk::core> for vk::DescriptorUpdateTemplate
        where Ctx: crate::HndCtx<vk::core, vk::Device>,
    {
        type Output = crate::hnd::DescriptorUpdateTemplate<vk::core>;
        unsafe fn hnd_with<Dep: Send + Sync + 'static>(self, ctx: &Ctx, dep: impl FnOnce() -> Dep) -> Self::Output {
            unsafe { crate::hnd::DescriptorUpdateTemplate::<vk::core>::new_with(ctx, self, dep) }
        }
    }
}

impl<O: crate::HndCtx<vk::core, vk::Device>> crate::Owner<vk::Event, vk::core> for O {
    fn drop(&mut self, raw: vk::Event) {
        unsafe {
            self.commands().destroy_event(self.raw(), Some(raw))
        }
    }
}
impl<O> crate::HndCtx<vk::core, vk::Device> for ::alloc::sync::Arc<crate::Unique<vk::Event, O, vk::core>>
    where O: crate::HndCtx<vk::core, vk::Device> + Send + Sync + 'static,
{
    type Ctx = Self;
    fn ctx(&self) -> Self::Ctx { self.clone() }
    fn raw(&self) -> vk::Device { self.owner.raw() }
    fn commands(&self) -> &::alloc::sync::Arc<Device> { self.owner.commands() }
}

impl crate::HndScope<vk::Event> for vk::core {
    type Impl = _hs_Event::Event;
}

mod _hs_Event {
    use super::*;

    #[repr(transparent)]
    #[derive(Debug)]
    pub struct Event(pub(crate) crate::handle::Hnd<vk::Event, ::alloc::sync::Arc<super::Device>>);

    impl Clone for Event {
        fn clone(&self) -> Self { Self(self.0.clone()) }
    }

    #[derive(Debug, Clone, Copy)]
    struct Inst;
    impl<Ctx, Dep> crate::handle::HndInst<vk::Event, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)> for Inst
        where Ctx: crate::HndCtx<vk::core, vk::Device>,
    {
        fn drop(this: &mut crate::handle::HndData<vk::Event, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)>) {
            unsafe {
                this.scope.destroy_event(this.dep.0.raw(), Some(this.raw))
            }
        }
    }

    impl crate::hnd::Event<vk::core>
    {
        pub unsafe fn new(ctx: &impl crate::HndCtx<vk::core, vk::Device>, raw: vk::Event) -> Self {
            unsafe { Self::new_with(ctx, raw, || ()) }
        }
        pub unsafe fn new_with<Dep: Send + Sync + 'static>(ctx: &impl crate::HndCtx<vk::core, vk::Device>, raw: vk::Event, dep: impl FnOnce() -> Dep) -> Self {
            Self(Event(crate::handle::Hnd::new(
                ctx.commands().clone(),
                raw,
                (ctx.ctx(), dep()),
                Inst,
            )))
        }
    }

    impl crate::hnd::Event<vk::core> {
        pub fn raw(&self) -> vk::Event { self.0.0.raw }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0.scope }
    }

    impl ::core::fmt::Debug for crate::hnd::Event<vk::core> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Event({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Event<vk::core> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl<Ctx> crate::MakeHnd<Ctx, vk::core> for vk::Event
        where Ctx: crate::HndCtx<vk::core, vk::Device>,
    {
        type Output = crate::hnd::Event<vk::core>;
        unsafe fn hnd_with<Dep: Send + Sync + 'static>(self, ctx: &Ctx, dep: impl FnOnce() -> Dep) -> Self::Output {
            unsafe { crate::hnd::Event::<vk::core>::new_with(ctx, self, dep) }
        }
    }
}

impl<O: crate::HndCtx<vk::core, vk::Device>> crate::Owner<vk::Fence, vk::core> for O {
    fn drop(&mut self, raw: vk::Fence) {
        unsafe {
            self.commands().destroy_fence(self.raw(), Some(raw))
        }
    }
}
impl<O> crate::HndCtx<vk::core, vk::Device> for ::alloc::sync::Arc<crate::Unique<vk::Fence, O, vk::core>>
    where O: crate::HndCtx<vk::core, vk::Device> + Send + Sync + 'static,
{
    type Ctx = Self;
    fn ctx(&self) -> Self::Ctx { self.clone() }
    fn raw(&self) -> vk::Device { self.owner.raw() }
    fn commands(&self) -> &::alloc::sync::Arc<Device> { self.owner.commands() }
}

impl crate::HndScope<vk::Fence> for vk::core {
    type Impl = _hs_Fence::Fence;
}

mod _hs_Fence {
    use super::*;

    #[repr(transparent)]
    #[derive(Debug)]
    pub struct Fence(pub(crate) crate::handle::Hnd<vk::Fence, ::alloc::sync::Arc<super::Device>>);

    impl Clone for Fence {
        fn clone(&self) -> Self { Self(self.0.clone()) }
    }

    #[derive(Debug, Clone, Copy)]
    struct Inst;
    impl<Ctx, Dep> crate::handle::HndInst<vk::Fence, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)> for Inst
        where Ctx: crate::HndCtx<vk::core, vk::Device>,
    {
        fn drop(this: &mut crate::handle::HndData<vk::Fence, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)>) {
            unsafe {
                this.scope.destroy_fence(this.dep.0.raw(), Some(this.raw))
            }
        }
    }

    impl crate::hnd::Fence<vk::core>
    {
        pub unsafe fn new(ctx: &impl crate::HndCtx<vk::core, vk::Device>, raw: vk::Fence) -> Self {
            unsafe { Self::new_with(ctx, raw, || ()) }
        }
        pub unsafe fn new_with<Dep: Send + Sync + 'static>(ctx: &impl crate::HndCtx<vk::core, vk::Device>, raw: vk::Fence, dep: impl FnOnce() -> Dep) -> Self {
            Self(Fence(crate::handle::Hnd::new(
                ctx.commands().clone(),
                raw,
                (ctx.ctx(), dep()),
                Inst,
            )))
        }
    }

    impl crate::hnd::Fence<vk::core> {
        pub fn raw(&self) -> vk::Fence { self.0.0.raw }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0.scope }
    }

    impl ::core::fmt::Debug for crate::hnd::Fence<vk::core> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Fence({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Fence<vk::core> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl<Ctx> crate::MakeHnd<Ctx, vk::core> for vk::Fence
        where Ctx: crate::HndCtx<vk::core, vk::Device>,
    {
        type Output = crate::hnd::Fence<vk::core>;
        unsafe fn hnd_with<Dep: Send + Sync + 'static>(self, ctx: &Ctx, dep: impl FnOnce() -> Dep) -> Self::Output {
            unsafe { crate::hnd::Fence::<vk::core>::new_with(ctx, self, dep) }
        }
    }
}

impl<O: crate::HndCtx<vk::core, vk::Device>> crate::Owner<vk::Framebuffer, vk::core> for O {
    fn drop(&mut self, raw: vk::Framebuffer) {
        unsafe {
            self.commands().destroy_framebuffer(self.raw(), Some(raw))
        }
    }
}
impl<O> crate::HndCtx<vk::core, vk::Device> for ::alloc::sync::Arc<crate::Unique<vk::Framebuffer, O, vk::core>>
    where O: crate::HndCtx<vk::core, vk::Device> + Send + Sync + 'static,
{
    type Ctx = Self;
    fn ctx(&self) -> Self::Ctx { self.clone() }
    fn raw(&self) -> vk::Device { self.owner.raw() }
    fn commands(&self) -> &::alloc::sync::Arc<Device> { self.owner.commands() }
}

impl crate::HndScope<vk::Framebuffer> for vk::core {
    type Impl = _hs_Framebuffer::Framebuffer;
}

mod _hs_Framebuffer {
    use super::*;

    #[repr(transparent)]
    #[derive(Debug)]
    pub struct Framebuffer(pub(crate) crate::handle::Hnd<vk::Framebuffer, ::alloc::sync::Arc<super::Device>>);

    impl Clone for Framebuffer {
        fn clone(&self) -> Self { Self(self.0.clone()) }
    }

    #[derive(Debug, Clone, Copy)]
    struct Inst;
    impl<Ctx, Dep> crate::handle::HndInst<vk::Framebuffer, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)> for Inst
        where Ctx: crate::HndCtx<vk::core, vk::Device>,
    {
        fn drop(this: &mut crate::handle::HndData<vk::Framebuffer, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)>) {
            unsafe {
                this.scope.destroy_framebuffer(this.dep.0.raw(), Some(this.raw))
            }
        }
    }

    impl crate::hnd::Framebuffer<vk::core>
    {
        pub unsafe fn new(ctx: &impl crate::HndCtx<vk::core, vk::Device>, raw: vk::Framebuffer) -> Self {
            unsafe { Self::new_with(ctx, raw, || ()) }
        }
        pub unsafe fn new_with<Dep: Send + Sync + 'static>(ctx: &impl crate::HndCtx<vk::core, vk::Device>, raw: vk::Framebuffer, dep: impl FnOnce() -> Dep) -> Self {
            Self(Framebuffer(crate::handle::Hnd::new(
                ctx.commands().clone(),
                raw,
                (ctx.ctx(), dep()),
                Inst,
            )))
        }
    }

    impl crate::hnd::Framebuffer<vk::core> {
        pub fn raw(&self) -> vk::Framebuffer { self.0.0.raw }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0.scope }
    }

    impl ::core::fmt::Debug for crate::hnd::Framebuffer<vk::core> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Framebuffer({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Framebuffer<vk::core> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl<Ctx> crate::MakeHnd<Ctx, vk::core> for vk::Framebuffer
        where Ctx: crate::HndCtx<vk::core, vk::Device>,
    {
        type Output = crate::hnd::Framebuffer<vk::core>;
        unsafe fn hnd_with<Dep: Send + Sync + 'static>(self, ctx: &Ctx, dep: impl FnOnce() -> Dep) -> Self::Output {
            unsafe { crate::hnd::Framebuffer::<vk::core>::new_with(ctx, self, dep) }
        }
    }
}

impl<O: crate::HndCtx<vk::core, vk::Device>> crate::Owner<vk::Image, vk::core> for O {
    fn drop(&mut self, raw: vk::Image) {
        unsafe {
            self.commands().destroy_image(self.raw(), Some(raw))
        }
    }
}
impl<O> crate::HndCtx<vk::core, vk::Device> for ::alloc::sync::Arc<crate::Unique<vk::Image, O, vk::core>>
    where O: crate::HndCtx<vk::core, vk::Device> + Send + Sync + 'static,
{
    type Ctx = Self;
    fn ctx(&self) -> Self::Ctx { self.clone() }
    fn raw(&self) -> vk::Device { self.owner.raw() }
    fn commands(&self) -> &::alloc::sync::Arc<Device> { self.owner.commands() }
}

impl crate::HndScope<vk::Image> for vk::core {
    type Impl = _hs_Image::Image;
}

mod _hs_Image {
    use super::*;

    #[repr(transparent)]
    #[derive(Debug)]
    pub struct Image(pub(crate) crate::handle::Hnd<vk::Image, ::alloc::sync::Arc<super::Device>>);

    impl Clone for Image {
        fn clone(&self) -> Self { Self(self.0.clone()) }
    }

    #[derive(Debug, Clone, Copy)]
    struct Inst;
    impl<Ctx, Dep> crate::handle::HndInst<vk::Image, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)> for Inst
        where Ctx: crate::HndCtx<vk::core, vk::Device>,
    {
        fn drop(this: &mut crate::handle::HndData<vk::Image, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)>) {
            unsafe {
                this.scope.destroy_image(this.dep.0.raw(), Some(this.raw))
            }
        }
    }

    impl crate::hnd::Image<vk::core>
    {
        pub unsafe fn new(ctx: &impl crate::HndCtx<vk::core, vk::Device>, raw: vk::Image) -> Self {
            unsafe { Self::new_with(ctx, raw, || ()) }
        }
        pub unsafe fn new_with<Dep: Send + Sync + 'static>(ctx: &impl crate::HndCtx<vk::core, vk::Device>, raw: vk::Image, dep: impl FnOnce() -> Dep) -> Self {
            Self(Image(crate::handle::Hnd::new(
                ctx.commands().clone(),
                raw,
                (ctx.ctx(), dep()),
                Inst,
            )))
        }
    }

    impl crate::hnd::Image<vk::core> {
        pub fn raw(&self) -> vk::Image { self.0.0.raw }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0.scope }
    }

    impl ::core::fmt::Debug for crate::hnd::Image<vk::core> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Image({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Image<vk::core> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl<Ctx> crate::MakeHnd<Ctx, vk::core> for vk::Image
        where Ctx: crate::HndCtx<vk::core, vk::Device>,
    {
        type Output = crate::hnd::Image<vk::core>;
        unsafe fn hnd_with<Dep: Send + Sync + 'static>(self, ctx: &Ctx, dep: impl FnOnce() -> Dep) -> Self::Output {
            unsafe { crate::hnd::Image::<vk::core>::new_with(ctx, self, dep) }
        }
    }
}

impl<O: crate::HndCtx<vk::core, vk::Device>> crate::Owner<vk::ImageView, vk::core> for O {
    fn drop(&mut self, raw: vk::ImageView) {
        unsafe {
            self.commands().destroy_image_view(self.raw(), Some(raw))
        }
    }
}
impl<O> crate::HndCtx<vk::core, vk::Device> for ::alloc::sync::Arc<crate::Unique<vk::ImageView, O, vk::core>>
    where O: crate::HndCtx<vk::core, vk::Device> + Send + Sync + 'static,
{
    type Ctx = Self;
    fn ctx(&self) -> Self::Ctx { self.clone() }
    fn raw(&self) -> vk::Device { self.owner.raw() }
    fn commands(&self) -> &::alloc::sync::Arc<Device> { self.owner.commands() }
}

impl crate::HndScope<vk::ImageView> for vk::core {
    type Impl = _hs_ImageView::ImageView;
}

mod _hs_ImageView {
    use super::*;

    #[repr(transparent)]
    #[derive(Debug)]
    pub struct ImageView(pub(crate) crate::handle::Hnd<vk::ImageView, ::alloc::sync::Arc<super::Device>>);

    impl Clone for ImageView {
        fn clone(&self) -> Self { Self(self.0.clone()) }
    }

    #[derive(Debug, Clone, Copy)]
    struct Inst;
    impl<Ctx, Dep> crate::handle::HndInst<vk::ImageView, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)> for Inst
        where Ctx: crate::HndCtx<vk::core, vk::Device>,
    {
        fn drop(this: &mut crate::handle::HndData<vk::ImageView, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)>) {
            unsafe {
                this.scope.destroy_image_view(this.dep.0.raw(), Some(this.raw))
            }
        }
    }

    impl crate::hnd::ImageView<vk::core>
    {
        pub unsafe fn new(ctx: &impl crate::HndCtx<vk::core, vk::Device>, raw: vk::ImageView) -> Self {
            unsafe { Self::new_with(ctx, raw, || ()) }
        }
        pub unsafe fn new_with<Dep: Send + Sync + 'static>(ctx: &impl crate::HndCtx<vk::core, vk::Device>, raw: vk::ImageView, dep: impl FnOnce() -> Dep) -> Self {
            Self(ImageView(crate::handle::Hnd::new(
                ctx.commands().clone(),
                raw,
                (ctx.ctx(), dep()),
                Inst,
            )))
        }
    }

    impl crate::hnd::ImageView<vk::core> {
        pub fn raw(&self) -> vk::ImageView { self.0.0.raw }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0.scope }
    }

    impl ::core::fmt::Debug for crate::hnd::ImageView<vk::core> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("ImageView({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::ImageView<vk::core> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl<Ctx> crate::MakeHnd<Ctx, vk::core> for vk::ImageView
        where Ctx: crate::HndCtx<vk::core, vk::Device>,
    {
        type Output = crate::hnd::ImageView<vk::core>;
        unsafe fn hnd_with<Dep: Send + Sync + 'static>(self, ctx: &Ctx, dep: impl FnOnce() -> Dep) -> Self::Output {
            unsafe { crate::hnd::ImageView::<vk::core>::new_with(ctx, self, dep) }
        }
    }
}

impl<O: crate::HndCtx<vk::core, vk::Device>> crate::Owner<vk::Pipeline, vk::core> for O {
    fn drop(&mut self, raw: vk::Pipeline) {
        unsafe {
            self.commands().destroy_pipeline(self.raw(), Some(raw))
        }
    }
}
impl<O> crate::HndCtx<vk::core, vk::Device> for ::alloc::sync::Arc<crate::Unique<vk::Pipeline, O, vk::core>>
    where O: crate::HndCtx<vk::core, vk::Device> + Send + Sync + 'static,
{
    type Ctx = Self;
    fn ctx(&self) -> Self::Ctx { self.clone() }
    fn raw(&self) -> vk::Device { self.owner.raw() }
    fn commands(&self) -> &::alloc::sync::Arc<Device> { self.owner.commands() }
}

impl crate::HndScope<vk::Pipeline> for vk::core {
    type Impl = _hs_Pipeline::Pipeline;
}

mod _hs_Pipeline {
    use super::*;

    #[repr(transparent)]
    #[derive(Debug)]
    pub struct Pipeline(pub(crate) crate::handle::Hnd<vk::Pipeline, ::alloc::sync::Arc<super::Device>>);

    impl Clone for Pipeline {
        fn clone(&self) -> Self { Self(self.0.clone()) }
    }

    #[derive(Debug, Clone, Copy)]
    struct Inst;
    impl<Ctx, Dep> crate::handle::HndInst<vk::Pipeline, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)> for Inst
        where Ctx: crate::HndCtx<vk::core, vk::Device>,
    {
        fn drop(this: &mut crate::handle::HndData<vk::Pipeline, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)>) {
            unsafe {
                this.scope.destroy_pipeline(this.dep.0.raw(), Some(this.raw))
            }
        }
    }

    impl crate::hnd::Pipeline<vk::core>
    {
        pub unsafe fn new(ctx: &impl crate::HndCtx<vk::core, vk::Device>, raw: vk::Pipeline) -> Self {
            unsafe { Self::new_with(ctx, raw, || ()) }
        }
        pub unsafe fn new_with<Dep: Send + Sync + 'static>(ctx: &impl crate::HndCtx<vk::core, vk::Device>, raw: vk::Pipeline, dep: impl FnOnce() -> Dep) -> Self {
            Self(Pipeline(crate::handle::Hnd::new(
                ctx.commands().clone(),
                raw,
                (ctx.ctx(), dep()),
                Inst,
            )))
        }
    }

    impl crate::hnd::Pipeline<vk::core> {
        pub fn raw(&self) -> vk::Pipeline { self.0.0.raw }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0.scope }
    }

    impl ::core::fmt::Debug for crate::hnd::Pipeline<vk::core> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Pipeline({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Pipeline<vk::core> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl<Ctx> crate::MakeHnd<Ctx, vk::core> for vk::Pipeline
        where Ctx: crate::HndCtx<vk::core, vk::Device>,
    {
        type Output = crate::hnd::Pipeline<vk::core>;
        unsafe fn hnd_with<Dep: Send + Sync + 'static>(self, ctx: &Ctx, dep: impl FnOnce() -> Dep) -> Self::Output {
            unsafe { crate::hnd::Pipeline::<vk::core>::new_with(ctx, self, dep) }
        }
    }
}

impl<O: crate::HndCtx<vk::core, vk::Device>> crate::Owner<vk::PipelineCache, vk::core> for O {
    fn drop(&mut self, raw: vk::PipelineCache) {
        unsafe {
            self.commands().destroy_pipeline_cache(self.raw(), Some(raw))
        }
    }
}
impl<O> crate::HndCtx<vk::core, vk::Device> for ::alloc::sync::Arc<crate::Unique<vk::PipelineCache, O, vk::core>>
    where O: crate::HndCtx<vk::core, vk::Device> + Send + Sync + 'static,
{
    type Ctx = Self;
    fn ctx(&self) -> Self::Ctx { self.clone() }
    fn raw(&self) -> vk::Device { self.owner.raw() }
    fn commands(&self) -> &::alloc::sync::Arc<Device> { self.owner.commands() }
}

impl crate::HndScope<vk::PipelineCache> for vk::core {
    type Impl = _hs_PipelineCache::PipelineCache;
}

mod _hs_PipelineCache {
    use super::*;

    #[repr(transparent)]
    #[derive(Debug)]
    pub struct PipelineCache(pub(crate) crate::handle::Hnd<vk::PipelineCache, ::alloc::sync::Arc<super::Device>>);

    impl Clone for PipelineCache {
        fn clone(&self) -> Self { Self(self.0.clone()) }
    }

    #[derive(Debug, Clone, Copy)]
    struct Inst;
    impl<Ctx, Dep> crate::handle::HndInst<vk::PipelineCache, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)> for Inst
        where Ctx: crate::HndCtx<vk::core, vk::Device>,
    {
        fn drop(this: &mut crate::handle::HndData<vk::PipelineCache, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)>) {
            unsafe {
                this.scope.destroy_pipeline_cache(this.dep.0.raw(), Some(this.raw))
            }
        }
    }

    impl crate::hnd::PipelineCache<vk::core>
    {
        pub unsafe fn new(ctx: &impl crate::HndCtx<vk::core, vk::Device>, raw: vk::PipelineCache) -> Self {
            unsafe { Self::new_with(ctx, raw, || ()) }
        }
        pub unsafe fn new_with<Dep: Send + Sync + 'static>(ctx: &impl crate::HndCtx<vk::core, vk::Device>, raw: vk::PipelineCache, dep: impl FnOnce() -> Dep) -> Self {
            Self(PipelineCache(crate::handle::Hnd::new(
                ctx.commands().clone(),
                raw,
                (ctx.ctx(), dep()),
                Inst,
            )))
        }
    }

    impl crate::hnd::PipelineCache<vk::core> {
        pub fn raw(&self) -> vk::PipelineCache { self.0.0.raw }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0.scope }
    }

    impl ::core::fmt::Debug for crate::hnd::PipelineCache<vk::core> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("PipelineCache({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::PipelineCache<vk::core> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl<Ctx> crate::MakeHnd<Ctx, vk::core> for vk::PipelineCache
        where Ctx: crate::HndCtx<vk::core, vk::Device>,
    {
        type Output = crate::hnd::PipelineCache<vk::core>;
        unsafe fn hnd_with<Dep: Send + Sync + 'static>(self, ctx: &Ctx, dep: impl FnOnce() -> Dep) -> Self::Output {
            unsafe { crate::hnd::PipelineCache::<vk::core>::new_with(ctx, self, dep) }
        }
    }
}

impl<O: crate::HndCtx<vk::core, vk::Device>> crate::Owner<vk::PipelineLayout, vk::core> for O {
    fn drop(&mut self, raw: vk::PipelineLayout) {
        unsafe {
            self.commands().destroy_pipeline_layout(self.raw(), Some(raw))
        }
    }
}
impl<O> crate::HndCtx<vk::core, vk::Device> for ::alloc::sync::Arc<crate::Unique<vk::PipelineLayout, O, vk::core>>
    where O: crate::HndCtx<vk::core, vk::Device> + Send + Sync + 'static,
{
    type Ctx = Self;
    fn ctx(&self) -> Self::Ctx { self.clone() }
    fn raw(&self) -> vk::Device { self.owner.raw() }
    fn commands(&self) -> &::alloc::sync::Arc<Device> { self.owner.commands() }
}

impl crate::HndScope<vk::PipelineLayout> for vk::core {
    type Impl = _hs_PipelineLayout::PipelineLayout;
}

mod _hs_PipelineLayout {
    use super::*;

    #[repr(transparent)]
    #[derive(Debug)]
    pub struct PipelineLayout(pub(crate) crate::handle::Hnd<vk::PipelineLayout, ::alloc::sync::Arc<super::Device>>);

    impl Clone for PipelineLayout {
        fn clone(&self) -> Self { Self(self.0.clone()) }
    }

    #[derive(Debug, Clone, Copy)]
    struct Inst;
    impl<Ctx, Dep> crate::handle::HndInst<vk::PipelineLayout, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)> for Inst
        where Ctx: crate::HndCtx<vk::core, vk::Device>,
    {
        fn drop(this: &mut crate::handle::HndData<vk::PipelineLayout, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)>) {
            unsafe {
                this.scope.destroy_pipeline_layout(this.dep.0.raw(), Some(this.raw))
            }
        }
    }

    impl crate::hnd::PipelineLayout<vk::core>
    {
        pub unsafe fn new(ctx: &impl crate::HndCtx<vk::core, vk::Device>, raw: vk::PipelineLayout) -> Self {
            unsafe { Self::new_with(ctx, raw, || ()) }
        }
        pub unsafe fn new_with<Dep: Send + Sync + 'static>(ctx: &impl crate::HndCtx<vk::core, vk::Device>, raw: vk::PipelineLayout, dep: impl FnOnce() -> Dep) -> Self {
            Self(PipelineLayout(crate::handle::Hnd::new(
                ctx.commands().clone(),
                raw,
                (ctx.ctx(), dep()),
                Inst,
            )))
        }
    }

    impl crate::hnd::PipelineLayout<vk::core> {
        pub fn raw(&self) -> vk::PipelineLayout { self.0.0.raw }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0.scope }
    }

    impl ::core::fmt::Debug for crate::hnd::PipelineLayout<vk::core> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("PipelineLayout({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::PipelineLayout<vk::core> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl<Ctx> crate::MakeHnd<Ctx, vk::core> for vk::PipelineLayout
        where Ctx: crate::HndCtx<vk::core, vk::Device>,
    {
        type Output = crate::hnd::PipelineLayout<vk::core>;
        unsafe fn hnd_with<Dep: Send + Sync + 'static>(self, ctx: &Ctx, dep: impl FnOnce() -> Dep) -> Self::Output {
            unsafe { crate::hnd::PipelineLayout::<vk::core>::new_with(ctx, self, dep) }
        }
    }
}

impl<O: crate::HndCtx<vk::core, vk::Device>> crate::Owner<vk::PrivateDataSlot, vk::core> for O {
    fn drop(&mut self, raw: vk::PrivateDataSlot) {
        unsafe {
            self.commands().destroy_private_data_slot(self.raw(), Some(raw))
        }
    }
}
impl<O> crate::HndCtx<vk::core, vk::Device> for ::alloc::sync::Arc<crate::Unique<vk::PrivateDataSlot, O, vk::core>>
    where O: crate::HndCtx<vk::core, vk::Device> + Send + Sync + 'static,
{
    type Ctx = Self;
    fn ctx(&self) -> Self::Ctx { self.clone() }
    fn raw(&self) -> vk::Device { self.owner.raw() }
    fn commands(&self) -> &::alloc::sync::Arc<Device> { self.owner.commands() }
}

impl crate::HndScope<vk::PrivateDataSlot> for vk::core {
    type Impl = _hs_PrivateDataSlot::PrivateDataSlot;
}

mod _hs_PrivateDataSlot {
    use super::*;

    #[repr(transparent)]
    #[derive(Debug)]
    pub struct PrivateDataSlot(pub(crate) crate::handle::Hnd<vk::PrivateDataSlot, ::alloc::sync::Arc<super::Device>>);

    impl Clone for PrivateDataSlot {
        fn clone(&self) -> Self { Self(self.0.clone()) }
    }

    #[derive(Debug, Clone, Copy)]
    struct Inst;
    impl<Ctx, Dep> crate::handle::HndInst<vk::PrivateDataSlot, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)> for Inst
        where Ctx: crate::HndCtx<vk::core, vk::Device>,
    {
        fn drop(this: &mut crate::handle::HndData<vk::PrivateDataSlot, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)>) {
            unsafe {
                this.scope.destroy_private_data_slot(this.dep.0.raw(), Some(this.raw))
            }
        }
    }

    impl crate::hnd::PrivateDataSlot<vk::core>
    {
        pub unsafe fn new(ctx: &impl crate::HndCtx<vk::core, vk::Device>, raw: vk::PrivateDataSlot) -> Self {
            unsafe { Self::new_with(ctx, raw, || ()) }
        }
        pub unsafe fn new_with<Dep: Send + Sync + 'static>(ctx: &impl crate::HndCtx<vk::core, vk::Device>, raw: vk::PrivateDataSlot, dep: impl FnOnce() -> Dep) -> Self {
            Self(PrivateDataSlot(crate::handle::Hnd::new(
                ctx.commands().clone(),
                raw,
                (ctx.ctx(), dep()),
                Inst,
            )))
        }
    }

    impl crate::hnd::PrivateDataSlot<vk::core> {
        pub fn raw(&self) -> vk::PrivateDataSlot { self.0.0.raw }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0.scope }
    }

    impl ::core::fmt::Debug for crate::hnd::PrivateDataSlot<vk::core> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("PrivateDataSlot({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::PrivateDataSlot<vk::core> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl<Ctx> crate::MakeHnd<Ctx, vk::core> for vk::PrivateDataSlot
        where Ctx: crate::HndCtx<vk::core, vk::Device>,
    {
        type Output = crate::hnd::PrivateDataSlot<vk::core>;
        unsafe fn hnd_with<Dep: Send + Sync + 'static>(self, ctx: &Ctx, dep: impl FnOnce() -> Dep) -> Self::Output {
            unsafe { crate::hnd::PrivateDataSlot::<vk::core>::new_with(ctx, self, dep) }
        }
    }
}

impl<O: crate::HndCtx<vk::core, vk::Device>> crate::Owner<vk::QueryPool, vk::core> for O {
    fn drop(&mut self, raw: vk::QueryPool) {
        unsafe {
            self.commands().destroy_query_pool(self.raw(), Some(raw))
        }
    }
}
impl<O> crate::HndCtx<vk::core, vk::Device> for ::alloc::sync::Arc<crate::Unique<vk::QueryPool, O, vk::core>>
    where O: crate::HndCtx<vk::core, vk::Device> + Send + Sync + 'static,
{
    type Ctx = Self;
    fn ctx(&self) -> Self::Ctx { self.clone() }
    fn raw(&self) -> vk::Device { self.owner.raw() }
    fn commands(&self) -> &::alloc::sync::Arc<Device> { self.owner.commands() }
}

impl crate::HndScope<vk::QueryPool> for vk::core {
    type Impl = _hs_QueryPool::QueryPool;
}

mod _hs_QueryPool {
    use super::*;

    #[repr(transparent)]
    #[derive(Debug)]
    pub struct QueryPool(pub(crate) crate::handle::Hnd<vk::QueryPool, ::alloc::sync::Arc<super::Device>>);

    impl Clone for QueryPool {
        fn clone(&self) -> Self { Self(self.0.clone()) }
    }

    #[derive(Debug, Clone, Copy)]
    struct Inst;
    impl<Ctx, Dep> crate::handle::HndInst<vk::QueryPool, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)> for Inst
        where Ctx: crate::HndCtx<vk::core, vk::Device>,
    {
        fn drop(this: &mut crate::handle::HndData<vk::QueryPool, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)>) {
            unsafe {
                this.scope.destroy_query_pool(this.dep.0.raw(), Some(this.raw))
            }
        }
    }

    impl crate::hnd::QueryPool<vk::core>
    {
        pub unsafe fn new(ctx: &impl crate::HndCtx<vk::core, vk::Device>, raw: vk::QueryPool) -> Self {
            unsafe { Self::new_with(ctx, raw, || ()) }
        }
        pub unsafe fn new_with<Dep: Send + Sync + 'static>(ctx: &impl crate::HndCtx<vk::core, vk::Device>, raw: vk::QueryPool, dep: impl FnOnce() -> Dep) -> Self {
            Self(QueryPool(crate::handle::Hnd::new(
                ctx.commands().clone(),
                raw,
                (ctx.ctx(), dep()),
                Inst,
            )))
        }
    }

    impl crate::hnd::QueryPool<vk::core> {
        pub fn raw(&self) -> vk::QueryPool { self.0.0.raw }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0.scope }
    }

    impl ::core::fmt::Debug for crate::hnd::QueryPool<vk::core> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("QueryPool({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::QueryPool<vk::core> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl<Ctx> crate::MakeHnd<Ctx, vk::core> for vk::QueryPool
        where Ctx: crate::HndCtx<vk::core, vk::Device>,
    {
        type Output = crate::hnd::QueryPool<vk::core>;
        unsafe fn hnd_with<Dep: Send + Sync + 'static>(self, ctx: &Ctx, dep: impl FnOnce() -> Dep) -> Self::Output {
            unsafe { crate::hnd::QueryPool::<vk::core>::new_with(ctx, self, dep) }
        }
    }
}

impl<O: crate::HndCtx<vk::core, vk::Device>> crate::Owner<vk::RenderPass, vk::core> for O {
    fn drop(&mut self, raw: vk::RenderPass) {
        unsafe {
            self.commands().destroy_render_pass(self.raw(), Some(raw))
        }
    }
}
impl<O> crate::HndCtx<vk::core, vk::Device> for ::alloc::sync::Arc<crate::Unique<vk::RenderPass, O, vk::core>>
    where O: crate::HndCtx<vk::core, vk::Device> + Send + Sync + 'static,
{
    type Ctx = Self;
    fn ctx(&self) -> Self::Ctx { self.clone() }
    fn raw(&self) -> vk::Device { self.owner.raw() }
    fn commands(&self) -> &::alloc::sync::Arc<Device> { self.owner.commands() }
}

impl crate::HndScope<vk::RenderPass> for vk::core {
    type Impl = _hs_RenderPass::RenderPass;
}

mod _hs_RenderPass {
    use super::*;

    #[repr(transparent)]
    #[derive(Debug)]
    pub struct RenderPass(pub(crate) crate::handle::Hnd<vk::RenderPass, ::alloc::sync::Arc<super::Device>>);

    impl Clone for RenderPass {
        fn clone(&self) -> Self { Self(self.0.clone()) }
    }

    #[derive(Debug, Clone, Copy)]
    struct Inst;
    impl<Ctx, Dep> crate::handle::HndInst<vk::RenderPass, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)> for Inst
        where Ctx: crate::HndCtx<vk::core, vk::Device>,
    {
        fn drop(this: &mut crate::handle::HndData<vk::RenderPass, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)>) {
            unsafe {
                this.scope.destroy_render_pass(this.dep.0.raw(), Some(this.raw))
            }
        }
    }

    impl crate::hnd::RenderPass<vk::core>
    {
        pub unsafe fn new(ctx: &impl crate::HndCtx<vk::core, vk::Device>, raw: vk::RenderPass) -> Self {
            unsafe { Self::new_with(ctx, raw, || ()) }
        }
        pub unsafe fn new_with<Dep: Send + Sync + 'static>(ctx: &impl crate::HndCtx<vk::core, vk::Device>, raw: vk::RenderPass, dep: impl FnOnce() -> Dep) -> Self {
            Self(RenderPass(crate::handle::Hnd::new(
                ctx.commands().clone(),
                raw,
                (ctx.ctx(), dep()),
                Inst,
            )))
        }
    }

    impl crate::hnd::RenderPass<vk::core> {
        pub fn raw(&self) -> vk::RenderPass { self.0.0.raw }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0.scope }
    }

    impl ::core::fmt::Debug for crate::hnd::RenderPass<vk::core> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("RenderPass({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::RenderPass<vk::core> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl<Ctx> crate::MakeHnd<Ctx, vk::core> for vk::RenderPass
        where Ctx: crate::HndCtx<vk::core, vk::Device>,
    {
        type Output = crate::hnd::RenderPass<vk::core>;
        unsafe fn hnd_with<Dep: Send + Sync + 'static>(self, ctx: &Ctx, dep: impl FnOnce() -> Dep) -> Self::Output {
            unsafe { crate::hnd::RenderPass::<vk::core>::new_with(ctx, self, dep) }
        }
    }
}

impl<O: crate::HndCtx<vk::core, vk::Device>> crate::Owner<vk::Sampler, vk::core> for O {
    fn drop(&mut self, raw: vk::Sampler) {
        unsafe {
            self.commands().destroy_sampler(self.raw(), Some(raw))
        }
    }
}
impl<O> crate::HndCtx<vk::core, vk::Device> for ::alloc::sync::Arc<crate::Unique<vk::Sampler, O, vk::core>>
    where O: crate::HndCtx<vk::core, vk::Device> + Send + Sync + 'static,
{
    type Ctx = Self;
    fn ctx(&self) -> Self::Ctx { self.clone() }
    fn raw(&self) -> vk::Device { self.owner.raw() }
    fn commands(&self) -> &::alloc::sync::Arc<Device> { self.owner.commands() }
}

impl crate::HndScope<vk::Sampler> for vk::core {
    type Impl = _hs_Sampler::Sampler;
}

mod _hs_Sampler {
    use super::*;

    #[repr(transparent)]
    #[derive(Debug)]
    pub struct Sampler(pub(crate) crate::handle::Hnd<vk::Sampler, ::alloc::sync::Arc<super::Device>>);

    impl Clone for Sampler {
        fn clone(&self) -> Self { Self(self.0.clone()) }
    }

    #[derive(Debug, Clone, Copy)]
    struct Inst;
    impl<Ctx, Dep> crate::handle::HndInst<vk::Sampler, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)> for Inst
        where Ctx: crate::HndCtx<vk::core, vk::Device>,
    {
        fn drop(this: &mut crate::handle::HndData<vk::Sampler, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)>) {
            unsafe {
                this.scope.destroy_sampler(this.dep.0.raw(), Some(this.raw))
            }
        }
    }

    impl crate::hnd::Sampler<vk::core>
    {
        pub unsafe fn new(ctx: &impl crate::HndCtx<vk::core, vk::Device>, raw: vk::Sampler) -> Self {
            unsafe { Self::new_with(ctx, raw, || ()) }
        }
        pub unsafe fn new_with<Dep: Send + Sync + 'static>(ctx: &impl crate::HndCtx<vk::core, vk::Device>, raw: vk::Sampler, dep: impl FnOnce() -> Dep) -> Self {
            Self(Sampler(crate::handle::Hnd::new(
                ctx.commands().clone(),
                raw,
                (ctx.ctx(), dep()),
                Inst,
            )))
        }
    }

    impl crate::hnd::Sampler<vk::core> {
        pub fn raw(&self) -> vk::Sampler { self.0.0.raw }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0.scope }
    }

    impl ::core::fmt::Debug for crate::hnd::Sampler<vk::core> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Sampler({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Sampler<vk::core> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl<Ctx> crate::MakeHnd<Ctx, vk::core> for vk::Sampler
        where Ctx: crate::HndCtx<vk::core, vk::Device>,
    {
        type Output = crate::hnd::Sampler<vk::core>;
        unsafe fn hnd_with<Dep: Send + Sync + 'static>(self, ctx: &Ctx, dep: impl FnOnce() -> Dep) -> Self::Output {
            unsafe { crate::hnd::Sampler::<vk::core>::new_with(ctx, self, dep) }
        }
    }
}

impl<O: crate::HndCtx<vk::core, vk::Device>> crate::Owner<vk::SamplerYcbcrConversion, vk::core> for O {
    fn drop(&mut self, raw: vk::SamplerYcbcrConversion) {
        unsafe {
            self.commands().destroy_sampler_ycbcr_conversion(self.raw(), Some(raw))
        }
    }
}
impl<O> crate::HndCtx<vk::core, vk::Device> for ::alloc::sync::Arc<crate::Unique<vk::SamplerYcbcrConversion, O, vk::core>>
    where O: crate::HndCtx<vk::core, vk::Device> + Send + Sync + 'static,
{
    type Ctx = Self;
    fn ctx(&self) -> Self::Ctx { self.clone() }
    fn raw(&self) -> vk::Device { self.owner.raw() }
    fn commands(&self) -> &::alloc::sync::Arc<Device> { self.owner.commands() }
}

impl crate::HndScope<vk::SamplerYcbcrConversion> for vk::core {
    type Impl = _hs_SamplerYcbcrConversion::SamplerYcbcrConversion;
}

mod _hs_SamplerYcbcrConversion {
    use super::*;

    #[repr(transparent)]
    #[derive(Debug)]
    pub struct SamplerYcbcrConversion(pub(crate) crate::handle::Hnd<vk::SamplerYcbcrConversion, ::alloc::sync::Arc<super::Device>>);

    impl Clone for SamplerYcbcrConversion {
        fn clone(&self) -> Self { Self(self.0.clone()) }
    }

    #[derive(Debug, Clone, Copy)]
    struct Inst;
    impl<Ctx, Dep> crate::handle::HndInst<vk::SamplerYcbcrConversion, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)> for Inst
        where Ctx: crate::HndCtx<vk::core, vk::Device>,
    {
        fn drop(this: &mut crate::handle::HndData<vk::SamplerYcbcrConversion, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)>) {
            unsafe {
                this.scope.destroy_sampler_ycbcr_conversion(this.dep.0.raw(), Some(this.raw))
            }
        }
    }

    impl crate::hnd::SamplerYcbcrConversion<vk::core>
    {
        pub unsafe fn new(ctx: &impl crate::HndCtx<vk::core, vk::Device>, raw: vk::SamplerYcbcrConversion) -> Self {
            unsafe { Self::new_with(ctx, raw, || ()) }
        }
        pub unsafe fn new_with<Dep: Send + Sync + 'static>(ctx: &impl crate::HndCtx<vk::core, vk::Device>, raw: vk::SamplerYcbcrConversion, dep: impl FnOnce() -> Dep) -> Self {
            Self(SamplerYcbcrConversion(crate::handle::Hnd::new(
                ctx.commands().clone(),
                raw,
                (ctx.ctx(), dep()),
                Inst,
            )))
        }
    }

    impl crate::hnd::SamplerYcbcrConversion<vk::core> {
        pub fn raw(&self) -> vk::SamplerYcbcrConversion { self.0.0.raw }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0.scope }
    }

    impl ::core::fmt::Debug for crate::hnd::SamplerYcbcrConversion<vk::core> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("SamplerYcbcrConversion({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::SamplerYcbcrConversion<vk::core> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl<Ctx> crate::MakeHnd<Ctx, vk::core> for vk::SamplerYcbcrConversion
        where Ctx: crate::HndCtx<vk::core, vk::Device>,
    {
        type Output = crate::hnd::SamplerYcbcrConversion<vk::core>;
        unsafe fn hnd_with<Dep: Send + Sync + 'static>(self, ctx: &Ctx, dep: impl FnOnce() -> Dep) -> Self::Output {
            unsafe { crate::hnd::SamplerYcbcrConversion::<vk::core>::new_with(ctx, self, dep) }
        }
    }
}

impl<O: crate::HndCtx<vk::core, vk::Device>> crate::Owner<vk::Semaphore, vk::core> for O {
    fn drop(&mut self, raw: vk::Semaphore) {
        unsafe {
            self.commands().destroy_semaphore(self.raw(), Some(raw))
        }
    }
}
impl<O> crate::HndCtx<vk::core, vk::Device> for ::alloc::sync::Arc<crate::Unique<vk::Semaphore, O, vk::core>>
    where O: crate::HndCtx<vk::core, vk::Device> + Send + Sync + 'static,
{
    type Ctx = Self;
    fn ctx(&self) -> Self::Ctx { self.clone() }
    fn raw(&self) -> vk::Device { self.owner.raw() }
    fn commands(&self) -> &::alloc::sync::Arc<Device> { self.owner.commands() }
}

impl crate::HndScope<vk::Semaphore> for vk::core {
    type Impl = _hs_Semaphore::Semaphore;
}

mod _hs_Semaphore {
    use super::*;

    #[repr(transparent)]
    #[derive(Debug)]
    pub struct Semaphore(pub(crate) crate::handle::Hnd<vk::Semaphore, ::alloc::sync::Arc<super::Device>>);

    impl Clone for Semaphore {
        fn clone(&self) -> Self { Self(self.0.clone()) }
    }

    #[derive(Debug, Clone, Copy)]
    struct Inst;
    impl<Ctx, Dep> crate::handle::HndInst<vk::Semaphore, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)> for Inst
        where Ctx: crate::HndCtx<vk::core, vk::Device>,
    {
        fn drop(this: &mut crate::handle::HndData<vk::Semaphore, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)>) {
            unsafe {
                this.scope.destroy_semaphore(this.dep.0.raw(), Some(this.raw))
            }
        }
    }

    impl crate::hnd::Semaphore<vk::core>
    {
        pub unsafe fn new(ctx: &impl crate::HndCtx<vk::core, vk::Device>, raw: vk::Semaphore) -> Self {
            unsafe { Self::new_with(ctx, raw, || ()) }
        }
        pub unsafe fn new_with<Dep: Send + Sync + 'static>(ctx: &impl crate::HndCtx<vk::core, vk::Device>, raw: vk::Semaphore, dep: impl FnOnce() -> Dep) -> Self {
            Self(Semaphore(crate::handle::Hnd::new(
                ctx.commands().clone(),
                raw,
                (ctx.ctx(), dep()),
                Inst,
            )))
        }
    }

    impl crate::hnd::Semaphore<vk::core> {
        pub fn raw(&self) -> vk::Semaphore { self.0.0.raw }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0.scope }
    }

    impl ::core::fmt::Debug for crate::hnd::Semaphore<vk::core> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Semaphore({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Semaphore<vk::core> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl<Ctx> crate::MakeHnd<Ctx, vk::core> for vk::Semaphore
        where Ctx: crate::HndCtx<vk::core, vk::Device>,
    {
        type Output = crate::hnd::Semaphore<vk::core>;
        unsafe fn hnd_with<Dep: Send + Sync + 'static>(self, ctx: &Ctx, dep: impl FnOnce() -> Dep) -> Self::Output {
            unsafe { crate::hnd::Semaphore::<vk::core>::new_with(ctx, self, dep) }
        }
    }
}

impl<O: crate::HndCtx<vk::core, vk::Device>> crate::Owner<vk::ShaderModule, vk::core> for O {
    fn drop(&mut self, raw: vk::ShaderModule) {
        unsafe {
            self.commands().destroy_shader_module(self.raw(), Some(raw))
        }
    }
}
impl<O> crate::HndCtx<vk::core, vk::Device> for ::alloc::sync::Arc<crate::Unique<vk::ShaderModule, O, vk::core>>
    where O: crate::HndCtx<vk::core, vk::Device> + Send + Sync + 'static,
{
    type Ctx = Self;
    fn ctx(&self) -> Self::Ctx { self.clone() }
    fn raw(&self) -> vk::Device { self.owner.raw() }
    fn commands(&self) -> &::alloc::sync::Arc<Device> { self.owner.commands() }
}

impl crate::HndScope<vk::ShaderModule> for vk::core {
    type Impl = _hs_ShaderModule::ShaderModule;
}

mod _hs_ShaderModule {
    use super::*;

    #[repr(transparent)]
    #[derive(Debug)]
    pub struct ShaderModule(pub(crate) crate::handle::Hnd<vk::ShaderModule, ::alloc::sync::Arc<super::Device>>);

    impl Clone for ShaderModule {
        fn clone(&self) -> Self { Self(self.0.clone()) }
    }

    #[derive(Debug, Clone, Copy)]
    struct Inst;
    impl<Ctx, Dep> crate::handle::HndInst<vk::ShaderModule, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)> for Inst
        where Ctx: crate::HndCtx<vk::core, vk::Device>,
    {
        fn drop(this: &mut crate::handle::HndData<vk::ShaderModule, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)>) {
            unsafe {
                this.scope.destroy_shader_module(this.dep.0.raw(), Some(this.raw))
            }
        }
    }

    impl crate::hnd::ShaderModule<vk::core>
    {
        pub unsafe fn new(ctx: &impl crate::HndCtx<vk::core, vk::Device>, raw: vk::ShaderModule) -> Self {
            unsafe { Self::new_with(ctx, raw, || ()) }
        }
        pub unsafe fn new_with<Dep: Send + Sync + 'static>(ctx: &impl crate::HndCtx<vk::core, vk::Device>, raw: vk::ShaderModule, dep: impl FnOnce() -> Dep) -> Self {
            Self(ShaderModule(crate::handle::Hnd::new(
                ctx.commands().clone(),
                raw,
                (ctx.ctx(), dep()),
                Inst,
            )))
        }
    }

    impl crate::hnd::ShaderModule<vk::core> {
        pub fn raw(&self) -> vk::ShaderModule { self.0.0.raw }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0.scope }
    }

    impl ::core::fmt::Debug for crate::hnd::ShaderModule<vk::core> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("ShaderModule({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::ShaderModule<vk::core> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl<Ctx> crate::MakeHnd<Ctx, vk::core> for vk::ShaderModule
        where Ctx: crate::HndCtx<vk::core, vk::Device>,
    {
        type Output = crate::hnd::ShaderModule<vk::core>;
        unsafe fn hnd_with<Dep: Send + Sync + 'static>(self, ctx: &Ctx, dep: impl FnOnce() -> Dep) -> Self::Output {
            unsafe { crate::hnd::ShaderModule::<vk::core>::new_with(ctx, self, dep) }
        }
    }
}
