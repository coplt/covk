// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_NVX_binary_import` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::nvx::binary_import::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::nvx::binary_import::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// void vkCmdCuLaunchKernelNVX(VkCommandBuffer commandBuffer, VkCuLaunchInfoNVX const* pLaunchInfo)
    /// ```
    pub unsafe fn cmd_cu_launch_kernel(
        &self,
        command_buffer: vk::CommandBuffer,
        launch_info: &CuLaunchInfoNVX,
    ) -> () {
        unsafe {
            self.0.CmdCuLaunchKernelNVX(
                command_buffer.abi(), 
                launch_info.abi(), 
            );
        }
    }
    /// ```c
    /// VkResult vkCreateCuFunctionNVX(VkDevice device, VkCuFunctionCreateInfoNVX const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkCuFunctionNVX* pFunction)
    /// ```
    pub unsafe fn create_cu_function(
        &self,
        device: vk::Device,
        create_info: &CuFunctionCreateInfoNVX,
    ) -> crate::Result<vk::CuFunctionNVX> {
        unsafe {
            let mut _v: Option<vk::CuFunctionNVX> = Default::default();
            let _r = self.0.CreateCuFunctionNVX(
                device.abi(), 
                create_info.abi(), 
                Default::default(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| _v)
        }
    }
    /// ```c
    /// VkResult vkCreateCuModuleNVX(VkDevice device, VkCuModuleCreateInfoNVX const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkCuModuleNVX* pModule)
    /// ```
    pub unsafe fn create_cu_module(
        &self,
        device: vk::Device,
        create_info: &CuModuleCreateInfoNVX,
    ) -> crate::Result<vk::CuModuleNVX> {
        unsafe {
            let mut _v: Option<vk::CuModuleNVX> = Default::default();
            let _r = self.0.CreateCuModuleNVX(
                device.abi(), 
                create_info.abi(), 
                Default::default(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| _v)
        }
    }
    /// ```c
    /// void vkDestroyCuFunctionNVX(VkDevice device, VkCuFunctionNVX function, VkAllocationCallbacks const* pAllocator)
    /// ```
    pub unsafe fn destroy_cu_function(
        &self,
        device: vk::Device,
        function: vk::CuFunctionNVX,
    ) -> () {
        unsafe {
            self.0.DestroyCuFunctionNVX(
                device.abi(), 
                function.abi(), 
                Default::default(), 
            );
        }
    }
    /// ```c
    /// void vkDestroyCuModuleNVX(VkDevice device, VkCuModuleNVX module, VkAllocationCallbacks const* pAllocator)
    /// ```
    pub unsafe fn destroy_cu_module(
        &self,
        device: vk::Device,
        module: vk::CuModuleNVX,
    ) -> () {
        unsafe {
            self.0.DestroyCuModuleNVX(
                device.abi(), 
                module.abi(), 
                Default::default(), 
            );
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::nvx::binary_import {
    type Commands = Device;
}

/// Device object
pub trait NvxBinaryImportDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkCreateCuFunctionNVX(VkDevice device, VkCuFunctionCreateInfoNVX const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkCuFunctionNVX* pFunction)
    /// ```
    unsafe fn create_cu_function(
        &self,
        create_info: &CuFunctionCreateInfoNVX,
    ) -> crate::Result<vk::CuFunctionNVX> {
        unsafe {
            self.commands().create_cu_function(
                self.raw(),
                create_info,
            )
        }
    }
    /// ```c
    /// VkResult vkCreateCuModuleNVX(VkDevice device, VkCuModuleCreateInfoNVX const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkCuModuleNVX* pModule)
    /// ```
    unsafe fn create_cu_module(
        &self,
        create_info: &CuModuleCreateInfoNVX,
    ) -> crate::Result<vk::CuModuleNVX> {
        unsafe {
            self.commands().create_cu_module(
                self.raw(),
                create_info,
            )
        }
    }
    /// ```c
    /// void vkDestroyCuFunctionNVX(VkDevice device, VkCuFunctionNVX function, VkAllocationCallbacks const* pAllocator)
    /// ```
    unsafe fn destroy_cu_function(
        &self,
        function: vk::CuFunctionNVX,
    ) -> () {
        unsafe {
            self.commands().destroy_cu_function(
                self.raw(),
                function,
            )
        }
    }
    /// ```c
    /// void vkDestroyCuModuleNVX(VkDevice device, VkCuModuleNVX module, VkAllocationCallbacks const* pAllocator)
    /// ```
    unsafe fn destroy_cu_module(
        &self,
        module: vk::CuModuleNVX,
    ) -> () {
        unsafe {
            self.commands().destroy_cu_module(
                self.raw(),
                module,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::nvx::binary_import {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::nvx::binary_import> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::nvx::binary_import {
        type Output = crate::hnd::Device<vk::extensions::nvx::binary_import>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::nvx::binary_import>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::nvx::binary_import> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::nvx::binary_import> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::nvx::binary_import> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::nvx::binary_import> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::NvxBinaryImportDevice for crate::hnd::Device<vk::extensions::nvx::binary_import> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::nvx::binary_import, vk::Device> for crate::hnd::Device<vk::extensions::nvx::binary_import> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

/// CommandBuffer object
pub trait NvxBinaryImportCommandBuffer {
    fn raw(&self) -> vk::CommandBuffer;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkCmdCuLaunchKernelNVX(VkCommandBuffer commandBuffer, VkCuLaunchInfoNVX const* pLaunchInfo)
    /// ```
    unsafe fn cu_launch_kernel(
        &self,
        launch_info: &CuLaunchInfoNVX,
    ) -> () {
        unsafe {
            self.commands().cmd_cu_launch_kernel(
                self.raw(),
                launch_info,
            )
        }
    }
}
