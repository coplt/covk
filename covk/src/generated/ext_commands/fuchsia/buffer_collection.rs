// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_FUCHSIA_buffer_collection` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::fuchsia::buffer_collection::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::fuchsia::buffer_collection::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// VkResult vkCreateBufferCollectionFUCHSIA(VkDevice device, VkBufferCollectionCreateInfoFUCHSIA const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkBufferCollectionFUCHSIA* pCollection)
    /// ```
    pub unsafe fn create_buffer_collection(
        &self,
        device: vk::Device,
        create_info: &BufferCollectionCreateInfoFUCHSIA,
    ) -> crate::Result<vk::BufferCollectionFUCHSIA> {
        unsafe {
            let mut _v: Option<vk::BufferCollectionFUCHSIA> = Default::default();
            let _r = self.0.CreateBufferCollectionFUCHSIA(
                device.abi(), 
                create_info.abi(), 
                Default::default(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| _v)
        }
    }
    /// ```c
    /// void vkDestroyBufferCollectionFUCHSIA(VkDevice device, VkBufferCollectionFUCHSIA collection, VkAllocationCallbacks const* pAllocator)
    /// ```
    pub unsafe fn destroy_buffer_collection(
        &self,
        device: vk::Device,
        collection: vk::BufferCollectionFUCHSIA,
    ) -> () {
        unsafe {
            self.0.DestroyBufferCollectionFUCHSIA(
                device.abi(), 
                collection.abi(), 
                Default::default(), 
            );
        }
    }
    /// ```c
    /// VkResult vkGetBufferCollectionPropertiesFUCHSIA(VkDevice device, VkBufferCollectionFUCHSIA collection, VkBufferCollectionPropertiesFUCHSIA* pProperties)
    /// ```
    pub unsafe fn get_buffer_collection_properties(
        &self,
        device: vk::Device,
        collection: vk::BufferCollectionFUCHSIA,
        properties: &mut BufferCollectionPropertiesFUCHSIA,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.GetBufferCollectionPropertiesFUCHSIA(
                device.abi(), 
                collection.abi(), 
                properties.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkResult vkSetBufferCollectionBufferConstraintsFUCHSIA(VkDevice device, VkBufferCollectionFUCHSIA collection, VkBufferConstraintsInfoFUCHSIA const* pBufferConstraintsInfo)
    /// ```
    pub unsafe fn set_buffer_collection_buffer_constraints(
        &self,
        device: vk::Device,
        collection: vk::BufferCollectionFUCHSIA,
        buffer_constraints_info: &BufferConstraintsInfoFUCHSIA,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.SetBufferCollectionBufferConstraintsFUCHSIA(
                device.abi(), 
                collection.abi(), 
                buffer_constraints_info.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkResult vkSetBufferCollectionImageConstraintsFUCHSIA(VkDevice device, VkBufferCollectionFUCHSIA collection, VkImageConstraintsInfoFUCHSIA const* pImageConstraintsInfo)
    /// ```
    pub unsafe fn set_buffer_collection_image_constraints(
        &self,
        device: vk::Device,
        collection: vk::BufferCollectionFUCHSIA,
        image_constraints_info: &ImageConstraintsInfoFUCHSIA,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.SetBufferCollectionImageConstraintsFUCHSIA(
                device.abi(), 
                collection.abi(), 
                image_constraints_info.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::fuchsia::buffer_collection {
    type Commands = Device;
}

/// Device object
pub trait FuchsiaBufferCollectionDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkCreateBufferCollectionFUCHSIA(VkDevice device, VkBufferCollectionCreateInfoFUCHSIA const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkBufferCollectionFUCHSIA* pCollection)
    /// ```
    unsafe fn create_buffer_collection(
        &self,
        create_info: &BufferCollectionCreateInfoFUCHSIA,
    ) -> crate::Result<vk::BufferCollectionFUCHSIA> {
        unsafe {
            self.commands().create_buffer_collection(
                self.raw(),
                create_info,
            )
        }
    }
    /// ```c
    /// void vkDestroyBufferCollectionFUCHSIA(VkDevice device, VkBufferCollectionFUCHSIA collection, VkAllocationCallbacks const* pAllocator)
    /// ```
    unsafe fn destroy_buffer_collection(
        &self,
        collection: vk::BufferCollectionFUCHSIA,
    ) -> () {
        unsafe {
            self.commands().destroy_buffer_collection(
                self.raw(),
                collection,
            )
        }
    }
    /// ```c
    /// VkResult vkGetBufferCollectionPropertiesFUCHSIA(VkDevice device, VkBufferCollectionFUCHSIA collection, VkBufferCollectionPropertiesFUCHSIA* pProperties)
    /// ```
    unsafe fn get_buffer_collection_properties(
        &self,
        collection: vk::BufferCollectionFUCHSIA,
        properties: &mut BufferCollectionPropertiesFUCHSIA,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().get_buffer_collection_properties(
                self.raw(),
                collection,
                properties,
            )
        }
    }
    /// ```c
    /// VkResult vkSetBufferCollectionBufferConstraintsFUCHSIA(VkDevice device, VkBufferCollectionFUCHSIA collection, VkBufferConstraintsInfoFUCHSIA const* pBufferConstraintsInfo)
    /// ```
    unsafe fn set_buffer_collection_buffer_constraints(
        &self,
        collection: vk::BufferCollectionFUCHSIA,
        buffer_constraints_info: &BufferConstraintsInfoFUCHSIA,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().set_buffer_collection_buffer_constraints(
                self.raw(),
                collection,
                buffer_constraints_info,
            )
        }
    }
    /// ```c
    /// VkResult vkSetBufferCollectionImageConstraintsFUCHSIA(VkDevice device, VkBufferCollectionFUCHSIA collection, VkImageConstraintsInfoFUCHSIA const* pImageConstraintsInfo)
    /// ```
    unsafe fn set_buffer_collection_image_constraints(
        &self,
        collection: vk::BufferCollectionFUCHSIA,
        image_constraints_info: &ImageConstraintsInfoFUCHSIA,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().set_buffer_collection_image_constraints(
                self.raw(),
                collection,
                image_constraints_info,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::fuchsia::buffer_collection {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::fuchsia::buffer_collection> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::fuchsia::buffer_collection {
        type Output = crate::hnd::Device<vk::extensions::fuchsia::buffer_collection>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::fuchsia::buffer_collection>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::fuchsia::buffer_collection> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::fuchsia::buffer_collection> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::fuchsia::buffer_collection> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::fuchsia::buffer_collection> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::FuchsiaBufferCollectionDevice for crate::hnd::Device<vk::extensions::fuchsia::buffer_collection> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::fuchsia::buffer_collection, vk::Device> for crate::hnd::Device<vk::extensions::fuchsia::buffer_collection> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}
