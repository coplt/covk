// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_EXT_private_data` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::ext::private_data::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::ext::private_data::DeviceCommands::load(get) })
    }
}

impl Device {
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
            let _r = self.0.CreatePrivateDataSlotEXT(
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
            self.0.DestroyPrivateDataSlotEXT(
                device.abi(), 
                private_data_slot.abi(), 
                Default::default(), 
            );
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
            self.0.GetPrivateDataEXT(
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
            let _r = self.0.SetPrivateDataEXT(
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

impl crate::CommandScope<vk::Device> for vk::extensions::ext::private_data {
    type Commands = Device;
}

/// Device object
pub trait ExtPrivateDataDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

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
}

impl crate::HndScope<vk::Device> for vk::extensions::ext::private_data {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::ext::private_data> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::ext::private_data {
        type Output = crate::hnd::Device<vk::extensions::ext::private_data>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::ext::private_data>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::ext::private_data> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::ext::private_data> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::ext::private_data> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::ext::private_data> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::ExtPrivateDataDevice for crate::hnd::Device<vk::extensions::ext::private_data> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::ext::private_data, vk::Device> for crate::hnd::Device<vk::extensions::ext::private_data> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

impl<O: crate::HndCtx<vk::extensions::ext::private_data, vk::Device>> crate::Owner<vk::PrivateDataSlot, vk::extensions::ext::private_data> for O {
    fn drop(&mut self, raw: vk::PrivateDataSlot) {
        unsafe {
            self.commands().destroy_private_data_slot(self.raw(), Some(raw))
        }
    }
}
impl<O> crate::HndCtx<vk::extensions::ext::private_data, vk::Device> for ::alloc::sync::Arc<crate::Unique<vk::PrivateDataSlot, O, vk::extensions::ext::private_data>>
    where O: crate::HndCtx<vk::extensions::ext::private_data, vk::Device> + Send + Sync + 'static,
{
    type Ctx = Self;
    fn ctx(&self) -> Self::Ctx { self.clone() }
    fn raw(&self) -> vk::Device { self.owner.raw() }
    fn commands(&self) -> &::alloc::sync::Arc<Device> { self.owner.commands() }
}

impl crate::HndScope<vk::PrivateDataSlot> for vk::extensions::ext::private_data {
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
        where Ctx: crate::HndCtx<vk::extensions::ext::private_data, vk::Device>,
    {
        fn drop(this: &mut crate::handle::HndData<vk::PrivateDataSlot, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)>) {
            unsafe {
                this.scope.destroy_private_data_slot(this.dep.0.raw(), Some(this.raw))
            }
        }
    }

    impl crate::hnd::PrivateDataSlot<vk::extensions::ext::private_data>
    {
        pub unsafe fn new(ctx: &impl crate::HndCtx<vk::extensions::ext::private_data, vk::Device>, raw: vk::PrivateDataSlot) -> Self {
            unsafe { Self::new_with(ctx, raw, || ()) }
        }
        pub unsafe fn new_with<Dep: Send + Sync + 'static>(ctx: &impl crate::HndCtx<vk::extensions::ext::private_data, vk::Device>, raw: vk::PrivateDataSlot, dep: impl FnOnce() -> Dep) -> Self {
            Self(PrivateDataSlot(crate::handle::Hnd::new(
                ctx.commands().clone(),
                raw,
                (ctx.ctx(), dep()),
                Inst,
            )))
        }
    }

    impl crate::hnd::PrivateDataSlot<vk::extensions::ext::private_data> {
        pub fn raw(&self) -> vk::PrivateDataSlot { self.0.0.raw }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0.scope }
    }

    impl ::core::fmt::Debug for crate::hnd::PrivateDataSlot<vk::extensions::ext::private_data> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("PrivateDataSlot({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::PrivateDataSlot<vk::extensions::ext::private_data> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl<Ctx> crate::MakeHnd<Ctx, vk::extensions::ext::private_data> for vk::PrivateDataSlot
        where Ctx: crate::HndCtx<vk::extensions::ext::private_data, vk::Device>,
    {
        type Output = crate::hnd::PrivateDataSlot<vk::extensions::ext::private_data>;
        unsafe fn hnd_with<Dep: Send + Sync + 'static>(self, ctx: &Ctx, dep: impl FnOnce() -> Dep) -> Self::Output {
            unsafe { crate::hnd::PrivateDataSlot::<vk::extensions::ext::private_data>::new_with(ctx, self, dep) }
        }
    }
}
