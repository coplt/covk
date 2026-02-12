// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_KHR_pipeline_binary` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::khr::pipeline_binary::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::khr::pipeline_binary::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// VkResult vkCreatePipelineBinariesKHR(VkDevice device, VkPipelineBinaryCreateInfoKHR const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkPipelineBinaryHandlesInfoKHR* pBinaries)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete], [Result::PipelineBinaryMissingKhr]
    pub unsafe fn create_pipeline_binaries(
        &self,
        device: vk::Device,
        create_info: &PipelineBinaryCreateInfoKHR,
        binaries: &mut PipelineBinaryHandlesInfoKHR,
    ) -> crate::Result<Result> {
        unsafe {
            let _r = self.0.CreatePipelineBinariesKHR(
                device.abi(), 
                create_info.abi(), 
                Default::default(), 
                binaries.abi(), 
            ).vk();
            _r.result(|| Some(_r))
        }
    }
    /// ```c
    /// void vkDestroyPipelineBinaryKHR(VkDevice device, VkPipelineBinaryKHR pipelineBinary, VkAllocationCallbacks const* pAllocator)
    /// ```
    pub unsafe fn destroy_pipeline_binary(
        &self,
        device: vk::Device,
        pipeline_binary: Option<vk::PipelineBinaryKHR>,
    ) -> () {
        unsafe {
            self.0.DestroyPipelineBinaryKHR(
                device.abi(), 
                pipeline_binary.abi(), 
                Default::default(), 
            );
        }
    }
    /// ```c
    /// VkResult vkGetPipelineBinaryDataKHR(VkDevice device, VkPipelineBinaryDataInfoKHR const* pInfo, VkPipelineBinaryKeyKHR* pPipelineBinaryKey, size_t* pPipelineBinaryDataSize, void* pPipelineBinaryData)
    /// ```
    pub unsafe fn get_pipeline_binary_data(
        &self,
        device: vk::Device,
        info: &PipelineBinaryDataInfoKHR,
        pipeline_binary_key: &mut PipelineBinaryKeyKHR,
        pipeline_binary_data_size: *mut size_t,
        pipeline_binary_data: *mut void,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.GetPipelineBinaryDataKHR(
                device.abi(), 
                info.abi(), 
                pipeline_binary_key.abi(), 
                pipeline_binary_data_size.abi(), 
                pipeline_binary_data.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkResult vkGetPipelineKeyKHR(VkDevice device, VkPipelineCreateInfoKHR const* pPipelineCreateInfo, VkPipelineBinaryKeyKHR* pPipelineKey)
    /// ```
    pub unsafe fn get_pipeline_key(
        &self,
        device: vk::Device,
        pipeline_create_info: Option<&PipelineCreateInfoKHR>,
        pipeline_key: &mut PipelineBinaryKeyKHR,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.GetPipelineKeyKHR(
                device.abi(), 
                pipeline_create_info.abi(), 
                pipeline_key.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkResult vkReleaseCapturedPipelineDataKHR(VkDevice device, VkReleaseCapturedPipelineDataInfoKHR const* pInfo, VkAllocationCallbacks const* pAllocator)
    /// ```
    pub unsafe fn release_captured_pipeline_data(
        &self,
        device: vk::Device,
        info: &ReleaseCapturedPipelineDataInfoKHR,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.ReleaseCapturedPipelineDataKHR(
                device.abi(), 
                info.abi(), 
                Default::default(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::khr::pipeline_binary {
    type Commands = Device;
}

/// Device object
pub trait KhrPipelineBinaryDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkCreatePipelineBinariesKHR(VkDevice device, VkPipelineBinaryCreateInfoKHR const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkPipelineBinaryHandlesInfoKHR* pBinaries)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete], [Result::PipelineBinaryMissingKhr]
    unsafe fn create_pipeline_binaries(
        &self,
        create_info: &PipelineBinaryCreateInfoKHR,
        binaries: &mut PipelineBinaryHandlesInfoKHR,
    ) -> crate::Result<Result> {
        unsafe {
            self.commands().create_pipeline_binaries(
                self.raw(),
                create_info,
                binaries,
            )
        }
    }
    /// ```c
    /// void vkDestroyPipelineBinaryKHR(VkDevice device, VkPipelineBinaryKHR pipelineBinary, VkAllocationCallbacks const* pAllocator)
    /// ```
    unsafe fn destroy_pipeline_binary(
        &self,
        pipeline_binary: Option<vk::PipelineBinaryKHR>,
    ) -> () {
        unsafe {
            self.commands().destroy_pipeline_binary(
                self.raw(),
                pipeline_binary,
            )
        }
    }
    /// ```c
    /// VkResult vkGetPipelineBinaryDataKHR(VkDevice device, VkPipelineBinaryDataInfoKHR const* pInfo, VkPipelineBinaryKeyKHR* pPipelineBinaryKey, size_t* pPipelineBinaryDataSize, void* pPipelineBinaryData)
    /// ```
    unsafe fn get_pipeline_binary_data(
        &self,
        info: &PipelineBinaryDataInfoKHR,
        pipeline_binary_key: &mut PipelineBinaryKeyKHR,
        pipeline_binary_data_size: *mut size_t,
        pipeline_binary_data: *mut void,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().get_pipeline_binary_data(
                self.raw(),
                info,
                pipeline_binary_key,
                pipeline_binary_data_size,
                pipeline_binary_data,
            )
        }
    }
    /// ```c
    /// VkResult vkGetPipelineKeyKHR(VkDevice device, VkPipelineCreateInfoKHR const* pPipelineCreateInfo, VkPipelineBinaryKeyKHR* pPipelineKey)
    /// ```
    unsafe fn get_pipeline_key(
        &self,
        pipeline_create_info: Option<&PipelineCreateInfoKHR>,
        pipeline_key: &mut PipelineBinaryKeyKHR,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().get_pipeline_key(
                self.raw(),
                pipeline_create_info,
                pipeline_key,
            )
        }
    }
    /// ```c
    /// VkResult vkReleaseCapturedPipelineDataKHR(VkDevice device, VkReleaseCapturedPipelineDataInfoKHR const* pInfo, VkAllocationCallbacks const* pAllocator)
    /// ```
    unsafe fn release_captured_pipeline_data(
        &self,
        info: &ReleaseCapturedPipelineDataInfoKHR,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().release_captured_pipeline_data(
                self.raw(),
                info,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::khr::pipeline_binary {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::khr::pipeline_binary> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::khr::pipeline_binary {
        type Output = crate::hnd::Device<vk::extensions::khr::pipeline_binary>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::khr::pipeline_binary>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::khr::pipeline_binary> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::khr::pipeline_binary> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::khr::pipeline_binary> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::khr::pipeline_binary> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::KhrPipelineBinaryDevice for crate::hnd::Device<vk::extensions::khr::pipeline_binary> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::khr::pipeline_binary, vk::Device> for crate::hnd::Device<vk::extensions::khr::pipeline_binary> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

impl<O: crate::HndCtx<vk::extensions::khr::pipeline_binary, vk::Device>> crate::Owner<vk::PipelineBinaryKHR, vk::extensions::khr::pipeline_binary> for O {
    fn drop(&mut self, raw: vk::PipelineBinaryKHR) {
        unsafe {
            self.commands().destroy_pipeline_binary(self.raw(), Some(raw))
        }
    }
}
impl<O> crate::HndCtx<vk::extensions::khr::pipeline_binary, vk::Device> for ::alloc::sync::Arc<crate::Unique<vk::PipelineBinaryKHR, O, vk::extensions::khr::pipeline_binary>>
    where O: crate::HndCtx<vk::extensions::khr::pipeline_binary, vk::Device> + Send + Sync + 'static,
{
    type Ctx = Self;
    fn ctx(&self) -> Self::Ctx { self.clone() }
    fn raw(&self) -> vk::Device { self.owner.raw() }
    fn commands(&self) -> &::alloc::sync::Arc<Device> { self.owner.commands() }
}

impl crate::HndScope<vk::PipelineBinaryKHR> for vk::extensions::khr::pipeline_binary {
    type Impl = _hs_PipelineBinaryKHR::PipelineBinaryKHR;
}


mod _hs_PipelineBinaryKHR {
    use super::*;

    #[repr(transparent)]
    #[derive(Debug)]
    pub struct PipelineBinaryKHR(pub(crate) crate::handle::Hnd<vk::PipelineBinaryKHR, ::alloc::sync::Arc<super::Device>>);

    impl Clone for PipelineBinaryKHR {
        fn clone(&self) -> Self { Self(self.0.clone()) }
    }

    #[derive(Debug, Clone, Copy)]
    struct Inst;
    impl<Ctx, Dep> crate::handle::HndInst<vk::PipelineBinaryKHR, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)> for Inst
        where Ctx: crate::HndCtx<vk::extensions::khr::pipeline_binary, vk::Device>,
    {
        fn drop(this: &mut crate::handle::HndData<vk::PipelineBinaryKHR, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)>) {
            unsafe {
                this.scope.destroy_pipeline_binary(this.dep.0.raw(), Some(this.raw))
            }
        }
    }

    impl crate::hnd::PipelineBinaryKHR<vk::extensions::khr::pipeline_binary>
    {
        pub unsafe fn new(ctx: &impl crate::HndCtx<vk::extensions::khr::pipeline_binary, vk::Device>, raw: vk::PipelineBinaryKHR) -> Self {
            unsafe { Self::new_with(ctx, raw, || ()) }
        }
        pub unsafe fn new_with<Dep: Send + Sync + 'static>(ctx: &impl crate::HndCtx<vk::extensions::khr::pipeline_binary, vk::Device>, raw: vk::PipelineBinaryKHR, dep: impl FnOnce() -> Dep) -> Self {
            Self(PipelineBinaryKHR(crate::handle::Hnd::new(
                ctx.commands().clone(),
                raw,
                (ctx.ctx(), dep()),
                Inst,
            )))
        }
    }

    impl crate::hnd::PipelineBinaryKHR<vk::extensions::khr::pipeline_binary> {
        pub fn raw(&self) -> vk::PipelineBinaryKHR { self.0.0.raw }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0.scope }
    }

    impl ::core::fmt::Debug for crate::hnd::PipelineBinaryKHR<vk::extensions::khr::pipeline_binary> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("PipelineBinaryKHR({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::PipelineBinaryKHR<vk::extensions::khr::pipeline_binary> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl<Ctx> crate::MakeHnd<Ctx, vk::extensions::khr::pipeline_binary> for vk::PipelineBinaryKHR
        where Ctx: crate::HndCtx<vk::extensions::khr::pipeline_binary, vk::Device>,
    {
        type Output = crate::hnd::PipelineBinaryKHR<vk::extensions::khr::pipeline_binary>;
        unsafe fn hnd_with<Dep: Send + Sync + 'static>(self, ctx: &Ctx, dep: impl FnOnce() -> Dep) -> Self::Output {
            unsafe { crate::hnd::PipelineBinaryKHR::<vk::extensions::khr::pipeline_binary>::new_with(ctx, self, dep) }
        }
    }
}
