// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_NV_cuda_kernel_launch` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::nv::cuda_kernel_launch::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::nv::cuda_kernel_launch::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// void vkCmdCudaLaunchKernelNV(VkCommandBuffer commandBuffer, VkCudaLaunchInfoNV const* pLaunchInfo)
    /// ```
    pub unsafe fn cmd_cuda_launch_kernel(
        &self,
        command_buffer: vk::CommandBuffer,
        launch_info: &CudaLaunchInfoNV,
    ) -> () {
        unsafe {
            self.0.CmdCudaLaunchKernelNV(
                command_buffer.abi(), 
                launch_info.abi(), 
            );
        }
    }
    /// ```c
    /// VkResult vkCreateCudaFunctionNV(VkDevice device, VkCudaFunctionCreateInfoNV const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkCudaFunctionNV* pFunction)
    /// ```
    pub unsafe fn create_cuda_function(
        &self,
        device: vk::Device,
        create_info: &CudaFunctionCreateInfoNV,
    ) -> crate::Result<vk::CudaFunctionNV> {
        unsafe {
            let mut _v: Option<vk::CudaFunctionNV> = Default::default();
            let _r = self.0.CreateCudaFunctionNV(
                device.abi(), 
                create_info.abi(), 
                Default::default(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| _v)
        }
    }
    /// ```c
    /// VkResult vkCreateCudaModuleNV(VkDevice device, VkCudaModuleCreateInfoNV const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkCudaModuleNV* pModule)
    /// ```
    pub unsafe fn create_cuda_module(
        &self,
        device: vk::Device,
        create_info: &CudaModuleCreateInfoNV,
    ) -> crate::Result<vk::CudaModuleNV> {
        unsafe {
            let mut _v: Option<vk::CudaModuleNV> = Default::default();
            let _r = self.0.CreateCudaModuleNV(
                device.abi(), 
                create_info.abi(), 
                Default::default(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| _v)
        }
    }
    /// ```c
    /// void vkDestroyCudaFunctionNV(VkDevice device, VkCudaFunctionNV function, VkAllocationCallbacks const* pAllocator)
    /// ```
    pub unsafe fn destroy_cuda_function(
        &self,
        device: vk::Device,
        function: vk::CudaFunctionNV,
    ) -> () {
        unsafe {
            self.0.DestroyCudaFunctionNV(
                device.abi(), 
                function.abi(), 
                Default::default(), 
            );
        }
    }
    /// ```c
    /// void vkDestroyCudaModuleNV(VkDevice device, VkCudaModuleNV module, VkAllocationCallbacks const* pAllocator)
    /// ```
    pub unsafe fn destroy_cuda_module(
        &self,
        device: vk::Device,
        module: vk::CudaModuleNV,
    ) -> () {
        unsafe {
            self.0.DestroyCudaModuleNV(
                device.abi(), 
                module.abi(), 
                Default::default(), 
            );
        }
    }
    /// ```c
    /// VkResult vkGetCudaModuleCacheNV(VkDevice device, VkCudaModuleNV module, size_t* pCacheSize, void* pCacheData)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    pub unsafe fn get_cuda_module_cache(
        &self,
        device: vk::Device,
        module: vk::CudaModuleNV,
        cache_size: *mut size_t,
        cache_data: *mut void,
    ) -> crate::Result<Result> {
        unsafe {
            let _r = self.0.GetCudaModuleCacheNV(
                device.abi(), 
                module.abi(), 
                cache_size.abi(), 
                cache_data.abi(), 
            ).vk();
            _r.result(|| Some(_r))
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::nv::cuda_kernel_launch {
    type Commands = Device;
}

/// Device object
pub trait NvCudaKernelLaunchDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkCreateCudaFunctionNV(VkDevice device, VkCudaFunctionCreateInfoNV const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkCudaFunctionNV* pFunction)
    /// ```
    unsafe fn create_cuda_function(
        &self,
        create_info: &CudaFunctionCreateInfoNV,
    ) -> crate::Result<vk::CudaFunctionNV> {
        unsafe {
            self.commands().create_cuda_function(
                self.raw(),
                create_info,
            )
        }
    }
    /// ```c
    /// VkResult vkCreateCudaModuleNV(VkDevice device, VkCudaModuleCreateInfoNV const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkCudaModuleNV* pModule)
    /// ```
    unsafe fn create_cuda_module(
        &self,
        create_info: &CudaModuleCreateInfoNV,
    ) -> crate::Result<vk::CudaModuleNV> {
        unsafe {
            self.commands().create_cuda_module(
                self.raw(),
                create_info,
            )
        }
    }
    /// ```c
    /// void vkDestroyCudaFunctionNV(VkDevice device, VkCudaFunctionNV function, VkAllocationCallbacks const* pAllocator)
    /// ```
    unsafe fn destroy_cuda_function(
        &self,
        function: vk::CudaFunctionNV,
    ) -> () {
        unsafe {
            self.commands().destroy_cuda_function(
                self.raw(),
                function,
            )
        }
    }
    /// ```c
    /// void vkDestroyCudaModuleNV(VkDevice device, VkCudaModuleNV module, VkAllocationCallbacks const* pAllocator)
    /// ```
    unsafe fn destroy_cuda_module(
        &self,
        module: vk::CudaModuleNV,
    ) -> () {
        unsafe {
            self.commands().destroy_cuda_module(
                self.raw(),
                module,
            )
        }
    }
    /// ```c
    /// VkResult vkGetCudaModuleCacheNV(VkDevice device, VkCudaModuleNV module, size_t* pCacheSize, void* pCacheData)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    unsafe fn get_cuda_module_cache(
        &self,
        module: vk::CudaModuleNV,
        cache_size: *mut size_t,
        cache_data: *mut void,
    ) -> crate::Result<Result> {
        unsafe {
            self.commands().get_cuda_module_cache(
                self.raw(),
                module,
                cache_size,
                cache_data,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::nv::cuda_kernel_launch {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::nv::cuda_kernel_launch> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::nv::cuda_kernel_launch {
        type Output = crate::hnd::Device<vk::extensions::nv::cuda_kernel_launch>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::nv::cuda_kernel_launch>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::nv::cuda_kernel_launch> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::nv::cuda_kernel_launch> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::nv::cuda_kernel_launch> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::nv::cuda_kernel_launch> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::NvCudaKernelLaunchDevice for crate::hnd::Device<vk::extensions::nv::cuda_kernel_launch> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::nv::cuda_kernel_launch, vk::Device> for crate::hnd::Device<vk::extensions::nv::cuda_kernel_launch> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

/// CommandBuffer object
pub trait NvCudaKernelLaunchCommandBuffer {
    fn raw(&self) -> vk::CommandBuffer;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkCmdCudaLaunchKernelNV(VkCommandBuffer commandBuffer, VkCudaLaunchInfoNV const* pLaunchInfo)
    /// ```
    unsafe fn cuda_launch_kernel(
        &self,
        launch_info: &CudaLaunchInfoNV,
    ) -> () {
        unsafe {
            self.commands().cmd_cuda_launch_kernel(
                self.raw(),
                launch_info,
            )
        }
    }
}
