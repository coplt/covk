// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_NV_external_sci_sync` InstanceCommands
#[derive(Debug, Clone, Copy)]
pub struct Instance(pub sys::nv::external_sci_sync::InstanceCommands);

impl Instance {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::nv::external_sci_sync::InstanceCommands::load(get) })
    }
}

impl Instance {
    /// ```c
    /// VkResult vkGetPhysicalDeviceSciSyncAttributesNV(VkPhysicalDevice physicalDevice, VkSciSyncAttributesInfoNV const* pSciSyncAttributesInfo, NvSciSyncAttrList pAttributes)
    /// ```
    pub unsafe fn get_physical_device_sci_sync_attributes(
        &self,
        physical_device: vk::PhysicalDevice,
        sci_sync_attributes_info: &SciSyncAttributesInfoNV,
        attributes: NvSciSyncAttrList,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.GetPhysicalDeviceSciSyncAttributesNV(
                physical_device.abi(), 
                sci_sync_attributes_info.abi(), 
                attributes.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
}

impl crate::CommandScope<vk::Instance> for vk::extensions::nv::external_sci_sync {
    type Commands = Instance;
}

/// Instance object
pub trait NvExternalSciSyncInstance {
    fn raw(&self) -> vk::Instance;
    fn commands(&self) -> &Instance;

}

impl crate::HndScope<vk::Instance> for vk::extensions::nv::external_sci_sync {
    type Impl = _hs_Instance::Instance;
}

mod _hs_Instance {
    use super::*;
    #[derive(Debug)]
    pub struct Instance(pub(crate) ::alloc::sync::Arc<super::Instance>, pub(crate) crate::hnd::Instance<vk::core>);

    impl Clone for Instance {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Instance<vk::extensions::nv::external_sci_sync> {
        pub unsafe fn new(base: &crate::hnd::Instance<vk::core>) -> Self {
            unsafe {
                Self(Instance(
                   ::alloc::sync::Arc::new(super::Instance::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Instance<vk::core>> for vk::extensions::nv::external_sci_sync {
        type Output = crate::hnd::Instance<vk::extensions::nv::external_sci_sync>;
        unsafe fn make(target: &crate::hnd::Instance<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Instance::<vk::extensions::nv::external_sci_sync>::new(target) }
        }
    }

    impl crate::hnd::Instance<vk::extensions::nv::external_sci_sync> {
        pub fn raw(&self) -> vk::Instance { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Instance<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Instance<vk::extensions::nv::external_sci_sync> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Instance({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Instance<vk::extensions::nv::external_sci_sync> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Instance<vk::extensions::nv::external_sci_sync> {
        type Target = super::Instance;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::NvExternalSciSyncInstance for crate::hnd::Instance<vk::extensions::nv::external_sci_sync> {
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &super::Instance { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::nv::external_sci_sync, vk::Instance> for crate::hnd::Instance<vk::extensions::nv::external_sci_sync> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { self.commands() }
    }
}

/// PhysicalDevice object
pub trait NvExternalSciSyncPhysicalDevice {
    fn raw(&self) -> vk::PhysicalDevice;
    fn commands(&self) -> &Instance;

    /// ```c
    /// VkResult vkGetPhysicalDeviceSciSyncAttributesNV(VkPhysicalDevice physicalDevice, VkSciSyncAttributesInfoNV const* pSciSyncAttributesInfo, NvSciSyncAttrList pAttributes)
    /// ```
    unsafe fn get_sci_sync_attributes(
        &self,
        sci_sync_attributes_info: &SciSyncAttributesInfoNV,
        attributes: NvSciSyncAttrList,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().get_physical_device_sci_sync_attributes(
                self.raw(),
                sci_sync_attributes_info,
                attributes,
            )
        }
    }
}

/// `VK_NV_external_sci_sync` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::nv::external_sci_sync::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::nv::external_sci_sync::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// VkResult vkGetFenceSciSyncFenceNV(VkDevice device, VkFenceGetSciSyncInfoNV const* pGetSciSyncHandleInfo, void* pHandle)
    /// ```
    pub unsafe fn get_fence_sci_sync_fence(
        &self,
        device: vk::Device,
        get_sci_sync_handle_info: &FenceGetSciSyncInfoNV,
        handle: *mut void,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.GetFenceSciSyncFenceNV(
                device.abi(), 
                get_sci_sync_handle_info.abi(), 
                handle.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkResult vkGetFenceSciSyncObjNV(VkDevice device, VkFenceGetSciSyncInfoNV const* pGetSciSyncHandleInfo, void* pHandle)
    /// ```
    pub unsafe fn get_fence_sci_sync_obj(
        &self,
        device: vk::Device,
        get_sci_sync_handle_info: &FenceGetSciSyncInfoNV,
        handle: *mut void,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.GetFenceSciSyncObjNV(
                device.abi(), 
                get_sci_sync_handle_info.abi(), 
                handle.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkResult vkGetSemaphoreSciSyncObjNV(VkDevice device, VkSemaphoreGetSciSyncInfoNV const* pGetSciSyncInfo, void* pHandle)
    /// ```
    pub unsafe fn get_semaphore_sci_sync_obj(
        &self,
        device: vk::Device,
        get_sci_sync_info: &SemaphoreGetSciSyncInfoNV,
        handle: *mut void,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.GetSemaphoreSciSyncObjNV(
                device.abi(), 
                get_sci_sync_info.abi(), 
                handle.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkResult vkImportFenceSciSyncFenceNV(VkDevice device, VkImportFenceSciSyncInfoNV const* pImportFenceSciSyncInfo)
    /// ```
    pub unsafe fn import_fence_sci_sync_fence(
        &self,
        device: vk::Device,
        import_fence_sci_sync_info: &ImportFenceSciSyncInfoNV,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.ImportFenceSciSyncFenceNV(
                device.abi(), 
                import_fence_sci_sync_info.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkResult vkImportFenceSciSyncObjNV(VkDevice device, VkImportFenceSciSyncInfoNV const* pImportFenceSciSyncInfo)
    /// ```
    pub unsafe fn import_fence_sci_sync_obj(
        &self,
        device: vk::Device,
        import_fence_sci_sync_info: &ImportFenceSciSyncInfoNV,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.ImportFenceSciSyncObjNV(
                device.abi(), 
                import_fence_sci_sync_info.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkResult vkImportSemaphoreSciSyncObjNV(VkDevice device, VkImportSemaphoreSciSyncInfoNV const* pImportSemaphoreSciSyncInfo)
    /// ```
    pub unsafe fn import_semaphore_sci_sync_obj(
        &self,
        device: vk::Device,
        import_semaphore_sci_sync_info: &ImportSemaphoreSciSyncInfoNV,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.ImportSemaphoreSciSyncObjNV(
                device.abi(), 
                import_semaphore_sci_sync_info.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::nv::external_sci_sync {
    type Commands = Device;
}

/// Device object
pub trait NvExternalSciSyncDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkGetFenceSciSyncFenceNV(VkDevice device, VkFenceGetSciSyncInfoNV const* pGetSciSyncHandleInfo, void* pHandle)
    /// ```
    unsafe fn get_fence_sci_sync_fence(
        &self,
        get_sci_sync_handle_info: &FenceGetSciSyncInfoNV,
        handle: *mut void,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().get_fence_sci_sync_fence(
                self.raw(),
                get_sci_sync_handle_info,
                handle,
            )
        }
    }
    /// ```c
    /// VkResult vkGetFenceSciSyncObjNV(VkDevice device, VkFenceGetSciSyncInfoNV const* pGetSciSyncHandleInfo, void* pHandle)
    /// ```
    unsafe fn get_fence_sci_sync_obj(
        &self,
        get_sci_sync_handle_info: &FenceGetSciSyncInfoNV,
        handle: *mut void,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().get_fence_sci_sync_obj(
                self.raw(),
                get_sci_sync_handle_info,
                handle,
            )
        }
    }
    /// ```c
    /// VkResult vkGetSemaphoreSciSyncObjNV(VkDevice device, VkSemaphoreGetSciSyncInfoNV const* pGetSciSyncInfo, void* pHandle)
    /// ```
    unsafe fn get_semaphore_sci_sync_obj(
        &self,
        get_sci_sync_info: &SemaphoreGetSciSyncInfoNV,
        handle: *mut void,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().get_semaphore_sci_sync_obj(
                self.raw(),
                get_sci_sync_info,
                handle,
            )
        }
    }
    /// ```c
    /// VkResult vkImportFenceSciSyncFenceNV(VkDevice device, VkImportFenceSciSyncInfoNV const* pImportFenceSciSyncInfo)
    /// ```
    unsafe fn import_fence_sci_sync_fence(
        &self,
        import_fence_sci_sync_info: &ImportFenceSciSyncInfoNV,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().import_fence_sci_sync_fence(
                self.raw(),
                import_fence_sci_sync_info,
            )
        }
    }
    /// ```c
    /// VkResult vkImportFenceSciSyncObjNV(VkDevice device, VkImportFenceSciSyncInfoNV const* pImportFenceSciSyncInfo)
    /// ```
    unsafe fn import_fence_sci_sync_obj(
        &self,
        import_fence_sci_sync_info: &ImportFenceSciSyncInfoNV,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().import_fence_sci_sync_obj(
                self.raw(),
                import_fence_sci_sync_info,
            )
        }
    }
    /// ```c
    /// VkResult vkImportSemaphoreSciSyncObjNV(VkDevice device, VkImportSemaphoreSciSyncInfoNV const* pImportSemaphoreSciSyncInfo)
    /// ```
    unsafe fn import_semaphore_sci_sync_obj(
        &self,
        import_semaphore_sci_sync_info: &ImportSemaphoreSciSyncInfoNV,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().import_semaphore_sci_sync_obj(
                self.raw(),
                import_semaphore_sci_sync_info,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::nv::external_sci_sync {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::nv::external_sci_sync> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::nv::external_sci_sync {
        type Output = crate::hnd::Device<vk::extensions::nv::external_sci_sync>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::nv::external_sci_sync>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::nv::external_sci_sync> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::nv::external_sci_sync> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::nv::external_sci_sync> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::nv::external_sci_sync> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::NvExternalSciSyncDevice for crate::hnd::Device<vk::extensions::nv::external_sci_sync> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::nv::external_sci_sync, vk::Device> for crate::hnd::Device<vk::extensions::nv::external_sci_sync> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}
