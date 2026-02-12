// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_KHR_deferred_host_operations` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::khr::deferred_host_operations::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::khr::deferred_host_operations::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// VkResult vkCreateDeferredOperationKHR(VkDevice device, VkAllocationCallbacks const* pAllocator, VkDeferredOperationKHR* pDeferredOperation)
    /// ```
    pub unsafe fn create_deferred_operation(
        &self,
        device: vk::Device,
    ) -> crate::Result<vk::DeferredOperationKHR> {
        unsafe {
            let mut _v: Option<vk::DeferredOperationKHR> = Default::default();
            let _r = self.0.CreateDeferredOperationKHR(
                device.abi(), 
                Default::default(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| _v)
        }
    }
    /// ```c
    /// VkResult vkDeferredOperationJoinKHR(VkDevice device, VkDeferredOperationKHR operation)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::ThreadDoneKhr], [Result::ThreadIdleKhr]
    pub unsafe fn deferred_operation_join(
        &self,
        device: vk::Device,
        operation: vk::DeferredOperationKHR,
    ) -> crate::Result<Result> {
        unsafe {
            let _r = self.0.DeferredOperationJoinKHR(
                device.abi(), 
                operation.abi(), 
            ).vk();
            _r.result(|| Some(_r))
        }
    }
    /// ```c
    /// void vkDestroyDeferredOperationKHR(VkDevice device, VkDeferredOperationKHR operation, VkAllocationCallbacks const* pAllocator)
    /// ```
    pub unsafe fn destroy_deferred_operation(
        &self,
        device: vk::Device,
        operation: Option<vk::DeferredOperationKHR>,
    ) -> () {
        unsafe {
            self.0.DestroyDeferredOperationKHR(
                device.abi(), 
                operation.abi(), 
                Default::default(), 
            );
        }
    }
    /// ```c
    /// uint32_t vkGetDeferredOperationMaxConcurrencyKHR(VkDevice device, VkDeferredOperationKHR operation)
    /// ```
    pub unsafe fn get_deferred_operation_max_concurrency(
        &self,
        device: vk::Device,
        operation: vk::DeferredOperationKHR,
    ) -> uint32_t {
        unsafe {
            let _r = self.0.GetDeferredOperationMaxConcurrencyKHR(
                device.abi(), 
                operation.abi(), 
            );
            _r
        }
    }
    /// ```c
    /// VkResult vkGetDeferredOperationResultKHR(VkDevice device, VkDeferredOperationKHR operation)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::NotReady]
    pub unsafe fn get_deferred_operation_result(
        &self,
        device: vk::Device,
        operation: vk::DeferredOperationKHR,
    ) -> crate::Result<Result> {
        unsafe {
            let _r = self.0.GetDeferredOperationResultKHR(
                device.abi(), 
                operation.abi(), 
            ).vk();
            _r.result(|| Some(_r))
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::khr::deferred_host_operations {
    type Commands = Device;
}

/// Device object
pub trait KhrDeferredHostOperationsDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkCreateDeferredOperationKHR(VkDevice device, VkAllocationCallbacks const* pAllocator, VkDeferredOperationKHR* pDeferredOperation)
    /// ```
    unsafe fn create_deferred_operation(
        &self,
    ) -> crate::Result<vk::DeferredOperationKHR> {
        unsafe {
            self.commands().create_deferred_operation(
                self.raw(),
            )
        }
    }
    /// ```c
    /// VkResult vkDeferredOperationJoinKHR(VkDevice device, VkDeferredOperationKHR operation)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::ThreadDoneKhr], [Result::ThreadIdleKhr]
    unsafe fn deferred_operation_join(
        &self,
        operation: vk::DeferredOperationKHR,
    ) -> crate::Result<Result> {
        unsafe {
            self.commands().deferred_operation_join(
                self.raw(),
                operation,
            )
        }
    }
    /// ```c
    /// void vkDestroyDeferredOperationKHR(VkDevice device, VkDeferredOperationKHR operation, VkAllocationCallbacks const* pAllocator)
    /// ```
    unsafe fn destroy_deferred_operation(
        &self,
        operation: Option<vk::DeferredOperationKHR>,
    ) -> () {
        unsafe {
            self.commands().destroy_deferred_operation(
                self.raw(),
                operation,
            )
        }
    }
    /// ```c
    /// uint32_t vkGetDeferredOperationMaxConcurrencyKHR(VkDevice device, VkDeferredOperationKHR operation)
    /// ```
    unsafe fn get_deferred_operation_max_concurrency(
        &self,
        operation: vk::DeferredOperationKHR,
    ) -> uint32_t {
        unsafe {
            self.commands().get_deferred_operation_max_concurrency(
                self.raw(),
                operation,
            )
        }
    }
    /// ```c
    /// VkResult vkGetDeferredOperationResultKHR(VkDevice device, VkDeferredOperationKHR operation)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::NotReady]
    unsafe fn get_deferred_operation_result(
        &self,
        operation: vk::DeferredOperationKHR,
    ) -> crate::Result<Result> {
        unsafe {
            self.commands().get_deferred_operation_result(
                self.raw(),
                operation,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::khr::deferred_host_operations {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::khr::deferred_host_operations> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::khr::deferred_host_operations {
        type Output = crate::hnd::Device<vk::extensions::khr::deferred_host_operations>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::khr::deferred_host_operations>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::khr::deferred_host_operations> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::khr::deferred_host_operations> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::khr::deferred_host_operations> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::khr::deferred_host_operations> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::KhrDeferredHostOperationsDevice for crate::hnd::Device<vk::extensions::khr::deferred_host_operations> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::khr::deferred_host_operations, vk::Device> for crate::hnd::Device<vk::extensions::khr::deferred_host_operations> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

impl<O: crate::HndCtx<vk::extensions::khr::deferred_host_operations, vk::Device>> crate::Owner<vk::DeferredOperationKHR, vk::extensions::khr::deferred_host_operations> for O {
    fn drop(&mut self, raw: vk::DeferredOperationKHR) {
        unsafe {
            self.commands().destroy_deferred_operation(self.raw(), Some(raw))
        }
    }
}
impl<O> crate::HndCtx<vk::extensions::khr::deferred_host_operations, vk::Device> for ::alloc::sync::Arc<crate::Unique<vk::DeferredOperationKHR, O, vk::extensions::khr::deferred_host_operations>>
    where O: crate::HndCtx<vk::extensions::khr::deferred_host_operations, vk::Device> + Send + Sync + 'static,
{
    type Ctx = Self;
    fn ctx(&self) -> Self::Ctx { self.clone() }
    fn raw(&self) -> vk::Device { self.owner.raw() }
    fn commands(&self) -> &::alloc::sync::Arc<Device> { self.owner.commands() }
}

impl crate::HndScope<vk::DeferredOperationKHR> for vk::extensions::khr::deferred_host_operations {
    type Impl = _hs_DeferredOperationKHR::DeferredOperationKHR;
}


mod _hs_DeferredOperationKHR {
    use super::*;

    #[repr(transparent)]
    #[derive(Debug)]
    pub struct DeferredOperationKHR(pub(crate) crate::handle::Hnd<vk::DeferredOperationKHR, ::alloc::sync::Arc<super::Device>>);

    impl Clone for DeferredOperationKHR {
        fn clone(&self) -> Self { Self(self.0.clone()) }
    }

    #[derive(Debug, Clone, Copy)]
    struct Inst;
    impl<Ctx, Dep> crate::handle::HndInst<vk::DeferredOperationKHR, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)> for Inst
        where Ctx: crate::HndCtx<vk::extensions::khr::deferred_host_operations, vk::Device>,
    {
        fn drop(this: &mut crate::handle::HndData<vk::DeferredOperationKHR, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)>) {
            unsafe {
                this.scope.destroy_deferred_operation(this.dep.0.raw(), Some(this.raw))
            }
        }
    }

    impl crate::hnd::DeferredOperationKHR<vk::extensions::khr::deferred_host_operations>
    {
        pub unsafe fn new(ctx: &impl crate::HndCtx<vk::extensions::khr::deferred_host_operations, vk::Device>, raw: vk::DeferredOperationKHR) -> Self {
            unsafe { Self::new_with(ctx, raw, || ()) }
        }
        pub unsafe fn new_with<Dep: Send + Sync + 'static>(ctx: &impl crate::HndCtx<vk::extensions::khr::deferred_host_operations, vk::Device>, raw: vk::DeferredOperationKHR, dep: impl FnOnce() -> Dep) -> Self {
            Self(DeferredOperationKHR(crate::handle::Hnd::new(
                ctx.commands().clone(),
                raw,
                (ctx.ctx(), dep()),
                Inst,
            )))
        }
    }

    impl crate::hnd::DeferredOperationKHR<vk::extensions::khr::deferred_host_operations> {
        pub fn raw(&self) -> vk::DeferredOperationKHR { self.0.0.raw }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0.scope }
    }

    impl ::core::fmt::Debug for crate::hnd::DeferredOperationKHR<vk::extensions::khr::deferred_host_operations> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("DeferredOperationKHR({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::DeferredOperationKHR<vk::extensions::khr::deferred_host_operations> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl<Ctx> crate::MakeHnd<Ctx, vk::extensions::khr::deferred_host_operations> for vk::DeferredOperationKHR
        where Ctx: crate::HndCtx<vk::extensions::khr::deferred_host_operations, vk::Device>,
    {
        type Output = crate::hnd::DeferredOperationKHR<vk::extensions::khr::deferred_host_operations>;
        unsafe fn hnd_with<Dep: Send + Sync + 'static>(self, ctx: &Ctx, dep: impl FnOnce() -> Dep) -> Self::Output {
            unsafe { crate::hnd::DeferredOperationKHR::<vk::extensions::khr::deferred_host_operations>::new_with(ctx, self, dep) }
        }
    }
}
