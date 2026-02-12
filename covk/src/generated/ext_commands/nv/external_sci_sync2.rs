// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_NV_external_sci_sync2` InstanceCommands
#[derive(Debug, Clone, Copy)]
pub struct Instance(pub sys::nv::external_sci_sync2::InstanceCommands);

impl Instance {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::nv::external_sci_sync2::InstanceCommands::load(get) })
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

impl crate::CommandScope<vk::Instance> for vk::extensions::nv::external_sci_sync2 {
    type Commands = Instance;
}

/// Instance object
pub trait NvExternalSciSync2Instance {
    fn raw(&self) -> vk::Instance;
    fn commands(&self) -> &Instance;

}

impl crate::HndScope<vk::Instance> for vk::extensions::nv::external_sci_sync2 {
    type Impl = _hs_Instance::Instance;
}

mod _hs_Instance {
    use super::*;
    #[derive(Debug)]
    pub struct Instance(pub(crate) ::alloc::sync::Arc<super::Instance>, pub(crate) crate::hnd::Instance<vk::core>);

    impl Clone for Instance {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Instance<vk::extensions::nv::external_sci_sync2> {
        pub unsafe fn new(base: &crate::hnd::Instance<vk::core>) -> Self {
            unsafe {
                Self(Instance(
                   ::alloc::sync::Arc::new(super::Instance::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Instance<vk::core>> for vk::extensions::nv::external_sci_sync2 {
        type Output = crate::hnd::Instance<vk::extensions::nv::external_sci_sync2>;
        unsafe fn make(target: &crate::hnd::Instance<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Instance::<vk::extensions::nv::external_sci_sync2>::new(target) }
        }
    }

    impl crate::hnd::Instance<vk::extensions::nv::external_sci_sync2> {
        pub fn raw(&self) -> vk::Instance { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Instance<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Instance<vk::extensions::nv::external_sci_sync2> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Instance({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Instance<vk::extensions::nv::external_sci_sync2> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Instance<vk::extensions::nv::external_sci_sync2> {
        type Target = super::Instance;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::NvExternalSciSync2Instance for crate::hnd::Instance<vk::extensions::nv::external_sci_sync2> {
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &super::Instance { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::nv::external_sci_sync2, vk::Instance> for crate::hnd::Instance<vk::extensions::nv::external_sci_sync2> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { self.commands() }
    }
}

/// PhysicalDevice object
pub trait NvExternalSciSync2PhysicalDevice {
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

/// `VK_NV_external_sci_sync2` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::nv::external_sci_sync2::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::nv::external_sci_sync2::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// VkResult vkCreateSemaphoreSciSyncPoolNV(VkDevice device, VkSemaphoreSciSyncPoolCreateInfoNV const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkSemaphoreSciSyncPoolNV* pSemaphorePool)
    /// ```
    pub unsafe fn create_semaphore_sci_sync_pool(
        &self,
        device: vk::Device,
        create_info: &SemaphoreSciSyncPoolCreateInfoNV,
    ) -> crate::Result<vk::SemaphoreSciSyncPoolNV> {
        unsafe {
            let mut _v: Option<vk::SemaphoreSciSyncPoolNV> = Default::default();
            let _r = self.0.CreateSemaphoreSciSyncPoolNV(
                device.abi(), 
                create_info.abi(), 
                Default::default(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| _v)
        }
    }
    /// ```c
    /// void vkDestroySemaphoreSciSyncPoolNV(VkDevice device, VkSemaphoreSciSyncPoolNV semaphorePool, VkAllocationCallbacks const* pAllocator)
    /// ```
    pub unsafe fn destroy_semaphore_sci_sync_pool(
        &self,
        device: vk::Device,
        semaphore_pool: Option<vk::SemaphoreSciSyncPoolNV>,
    ) -> () {
        unsafe {
            self.0.DestroySemaphoreSciSyncPoolNV(
                device.abi(), 
                semaphore_pool.abi(), 
                Default::default(), 
            );
        }
    }
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
}

impl crate::CommandScope<vk::Device> for vk::extensions::nv::external_sci_sync2 {
    type Commands = Device;
}

/// Device object
pub trait NvExternalSciSync2Device {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkCreateSemaphoreSciSyncPoolNV(VkDevice device, VkSemaphoreSciSyncPoolCreateInfoNV const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkSemaphoreSciSyncPoolNV* pSemaphorePool)
    /// ```
    unsafe fn create_semaphore_sci_sync_pool(
        &self,
        create_info: &SemaphoreSciSyncPoolCreateInfoNV,
    ) -> crate::Result<vk::SemaphoreSciSyncPoolNV> {
        unsafe {
            self.commands().create_semaphore_sci_sync_pool(
                self.raw(),
                create_info,
            )
        }
    }
    /// ```c
    /// void vkDestroySemaphoreSciSyncPoolNV(VkDevice device, VkSemaphoreSciSyncPoolNV semaphorePool, VkAllocationCallbacks const* pAllocator)
    /// ```
    unsafe fn destroy_semaphore_sci_sync_pool(
        &self,
        semaphore_pool: Option<vk::SemaphoreSciSyncPoolNV>,
    ) -> () {
        unsafe {
            self.commands().destroy_semaphore_sci_sync_pool(
                self.raw(),
                semaphore_pool,
            )
        }
    }
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
}

impl crate::HndScope<vk::Device> for vk::extensions::nv::external_sci_sync2 {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::nv::external_sci_sync2> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::nv::external_sci_sync2 {
        type Output = crate::hnd::Device<vk::extensions::nv::external_sci_sync2>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::nv::external_sci_sync2>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::nv::external_sci_sync2> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::nv::external_sci_sync2> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::nv::external_sci_sync2> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::nv::external_sci_sync2> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::NvExternalSciSync2Device for crate::hnd::Device<vk::extensions::nv::external_sci_sync2> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::nv::external_sci_sync2, vk::Device> for crate::hnd::Device<vk::extensions::nv::external_sci_sync2> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

impl<O: crate::HndCtx<vk::extensions::nv::external_sci_sync2, vk::Device>> crate::Owner<vk::SemaphoreSciSyncPoolNV, vk::extensions::nv::external_sci_sync2> for O {
    fn drop(&mut self, raw: vk::SemaphoreSciSyncPoolNV) {
        unsafe {
            self.commands().destroy_semaphore_sci_sync_pool(self.raw(), Some(raw))
        }
    }
}
impl<O> crate::HndCtx<vk::extensions::nv::external_sci_sync2, vk::Device> for ::alloc::sync::Arc<crate::Unique<vk::SemaphoreSciSyncPoolNV, O, vk::extensions::nv::external_sci_sync2>>
    where O: crate::HndCtx<vk::extensions::nv::external_sci_sync2, vk::Device> + Send + Sync + 'static,
{
    type Ctx = Self;
    fn ctx(&self) -> Self::Ctx { self.clone() }
    fn raw(&self) -> vk::Device { self.owner.raw() }
    fn commands(&self) -> &::alloc::sync::Arc<Device> { self.owner.commands() }
}

impl crate::HndScope<vk::SemaphoreSciSyncPoolNV> for vk::extensions::nv::external_sci_sync2 {
    type Impl = _hs_SemaphoreSciSyncPoolNV::SemaphoreSciSyncPoolNV;
}


mod _hs_SemaphoreSciSyncPoolNV {
    use super::*;

    #[repr(transparent)]
    #[derive(Debug)]
    pub struct SemaphoreSciSyncPoolNV(pub(crate) crate::handle::Hnd<vk::SemaphoreSciSyncPoolNV, ::alloc::sync::Arc<super::Device>>);

    impl Clone for SemaphoreSciSyncPoolNV {
        fn clone(&self) -> Self { Self(self.0.clone()) }
    }

    #[derive(Debug, Clone, Copy)]
    struct Inst;
    impl<Ctx, Dep> crate::handle::HndInst<vk::SemaphoreSciSyncPoolNV, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)> for Inst
        where Ctx: crate::HndCtx<vk::extensions::nv::external_sci_sync2, vk::Device>,
    {
        fn drop(this: &mut crate::handle::HndData<vk::SemaphoreSciSyncPoolNV, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)>) {
            unsafe {
                this.scope.destroy_semaphore_sci_sync_pool(this.dep.0.raw(), Some(this.raw))
            }
        }
    }

    impl crate::hnd::SemaphoreSciSyncPoolNV<vk::extensions::nv::external_sci_sync2>
    {
        pub unsafe fn new(ctx: &impl crate::HndCtx<vk::extensions::nv::external_sci_sync2, vk::Device>, raw: vk::SemaphoreSciSyncPoolNV) -> Self {
            unsafe { Self::new_with(ctx, raw, || ()) }
        }
        pub unsafe fn new_with<Dep: Send + Sync + 'static>(ctx: &impl crate::HndCtx<vk::extensions::nv::external_sci_sync2, vk::Device>, raw: vk::SemaphoreSciSyncPoolNV, dep: impl FnOnce() -> Dep) -> Self {
            Self(SemaphoreSciSyncPoolNV(crate::handle::Hnd::new(
                ctx.commands().clone(),
                raw,
                (ctx.ctx(), dep()),
                Inst,
            )))
        }
    }

    impl crate::hnd::SemaphoreSciSyncPoolNV<vk::extensions::nv::external_sci_sync2> {
        pub fn raw(&self) -> vk::SemaphoreSciSyncPoolNV { self.0.0.raw }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0.scope }
    }

    impl ::core::fmt::Debug for crate::hnd::SemaphoreSciSyncPoolNV<vk::extensions::nv::external_sci_sync2> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("SemaphoreSciSyncPoolNV({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::SemaphoreSciSyncPoolNV<vk::extensions::nv::external_sci_sync2> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl<Ctx> crate::MakeHnd<Ctx, vk::extensions::nv::external_sci_sync2> for vk::SemaphoreSciSyncPoolNV
        where Ctx: crate::HndCtx<vk::extensions::nv::external_sci_sync2, vk::Device>,
    {
        type Output = crate::hnd::SemaphoreSciSyncPoolNV<vk::extensions::nv::external_sci_sync2>;
        unsafe fn hnd_with<Dep: Send + Sync + 'static>(self, ctx: &Ctx, dep: impl FnOnce() -> Dep) -> Self::Output {
            unsafe { crate::hnd::SemaphoreSciSyncPoolNV::<vk::extensions::nv::external_sci_sync2>::new_with(ctx, self, dep) }
        }
    }
}
