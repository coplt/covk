// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_KHR_swapchain` InstanceCommands
#[derive(Debug, Clone, Copy)]
pub struct Instance(pub sys::khr::swapchain::InstanceCommands);

impl Instance {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::khr::swapchain::InstanceCommands::load(get) })
    }
}

impl Instance {
    /// ```c
    /// VkResult vkGetPhysicalDevicePresentRectanglesKHR(VkPhysicalDevice physicalDevice, VkSurfaceKHR surface, uint32_t* pRectCount, VkRect2D* pRects)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    pub unsafe fn get_physical_device_present_rectangles(
        &self,
        physical_device: vk::PhysicalDevice,
        surface: vk::SurfaceKHR,
        rects: Option<&mut ::alloc::vec::Vec<Rect2D>>,
    ) -> crate::Result<(uint32_t, Result)> {
        unsafe {
            let mut _c: uint32_t = Default::default();
            let mut _r = self.0.GetPhysicalDevicePresentRectanglesKHR(
                physical_device.abi(), 
                surface.abi(), 
                &mut _c, 
                Default::default(), 
            ).vk();
            if let Some(_b) = rects {
                _r.result(|| Some(()))?;
                _b.reserve(_c as usize);
                _r = self.0.GetPhysicalDevicePresentRectanglesKHR(
                    physical_device.abi(), 
                    surface.abi(), 
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

impl crate::CommandScope<vk::Instance> for vk::extensions::khr::swapchain {
    type Commands = Instance;
}

/// Instance object
pub trait KhrSwapchainInstance {
    fn raw(&self) -> vk::Instance;
    fn commands(&self) -> &Instance;

}

impl crate::HndScope<vk::Instance> for vk::extensions::khr::swapchain {
    type Impl = _hs_Instance::Instance;
}

mod _hs_Instance {
    use super::*;
    #[derive(Debug)]
    pub struct Instance(pub(crate) ::alloc::sync::Arc<super::Instance>, pub(crate) crate::hnd::Instance<vk::core>);

    impl Clone for Instance {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Instance<vk::extensions::khr::swapchain> {
        pub unsafe fn new(base: &crate::hnd::Instance<vk::core>) -> Self {
            unsafe {
                Self(Instance(
                   ::alloc::sync::Arc::new(super::Instance::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Instance<vk::core>> for vk::extensions::khr::swapchain {
        type Output = crate::hnd::Instance<vk::extensions::khr::swapchain>;
        unsafe fn make(target: &crate::hnd::Instance<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Instance::<vk::extensions::khr::swapchain>::new(target) }
        }
    }

    impl crate::hnd::Instance<vk::extensions::khr::swapchain> {
        pub fn raw(&self) -> vk::Instance { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Instance<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Instance<vk::extensions::khr::swapchain> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Instance({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Instance<vk::extensions::khr::swapchain> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Instance<vk::extensions::khr::swapchain> {
        type Target = super::Instance;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::KhrSwapchainInstance for crate::hnd::Instance<vk::extensions::khr::swapchain> {
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &super::Instance { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::khr::swapchain, vk::Instance> for crate::hnd::Instance<vk::extensions::khr::swapchain> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { self.commands() }
    }
}

/// PhysicalDevice object
pub trait KhrSwapchainPhysicalDevice {
    fn raw(&self) -> vk::PhysicalDevice;
    fn commands(&self) -> &Instance;

    /// ```c
    /// VkResult vkGetPhysicalDevicePresentRectanglesKHR(VkPhysicalDevice physicalDevice, VkSurfaceKHR surface, uint32_t* pRectCount, VkRect2D* pRects)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    unsafe fn get_present_rectangles(
        &self,
        surface: vk::SurfaceKHR,
        rects: Option<&mut ::alloc::vec::Vec<Rect2D>>,
    ) -> crate::Result<(uint32_t, Result)> {
        unsafe {
            self.commands().get_physical_device_present_rectangles(
                self.raw(),
                surface,
                rects,
            )
        }
    }
}

/// `VK_KHR_swapchain` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::khr::swapchain::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::khr::swapchain::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// VkResult vkAcquireNextImage2KHR(VkDevice device, VkAcquireNextImageInfoKHR const* pAcquireInfo, uint32_t* pImageIndex)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Timeout], [Result::NotReady], [Result::SuboptimalKhr]
    pub unsafe fn acquire_next_image2(
        &self,
        device: vk::Device,
        acquire_info: &AcquireNextImageInfoKHR,
        image_index: *mut uint32_t,
    ) -> crate::Result<Result> {
        unsafe {
            let _r = self.0.AcquireNextImage2KHR(
                device.abi(), 
                acquire_info.abi(), 
                image_index.abi(), 
            ).vk();
            _r.result(|| Some(_r))
        }
    }
    /// ```c
    /// VkResult vkAcquireNextImageKHR(VkDevice device, VkSwapchainKHR swapchain, uint64_t timeout, VkSemaphore semaphore, VkFence fence, uint32_t* pImageIndex)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Timeout], [Result::NotReady], [Result::SuboptimalKhr]
    pub unsafe fn acquire_next_image(
        &self,
        device: vk::Device,
        swapchain: vk::SwapchainKHR,
        timeout: uint64_t,
        semaphore: Option<vk::Semaphore>,
        fence: Option<vk::Fence>,
        image_index: *mut uint32_t,
    ) -> crate::Result<Result> {
        unsafe {
            let _r = self.0.AcquireNextImageKHR(
                device.abi(), 
                swapchain.abi(), 
                timeout.abi(), 
                semaphore.abi(), 
                fence.abi(), 
                image_index.abi(), 
            ).vk();
            _r.result(|| Some(_r))
        }
    }
    /// ```c
    /// VkResult vkCreateSwapchainKHR(VkDevice device, VkSwapchainCreateInfoKHR const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkSwapchainKHR* pSwapchain)
    /// ```
    pub unsafe fn create_swapchain(
        &self,
        device: vk::Device,
        create_info: &SwapchainCreateInfoKHR,
    ) -> crate::Result<vk::SwapchainKHR> {
        unsafe {
            let mut _v: Option<vk::SwapchainKHR> = Default::default();
            let _r = self.0.CreateSwapchainKHR(
                device.abi(), 
                create_info.abi(), 
                Default::default(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| _v)
        }
    }
    /// ```c
    /// void vkDestroySwapchainKHR(VkDevice device, VkSwapchainKHR swapchain, VkAllocationCallbacks const* pAllocator)
    /// ```
    pub unsafe fn destroy_swapchain(
        &self,
        device: vk::Device,
        swapchain: Option<vk::SwapchainKHR>,
    ) -> () {
        unsafe {
            self.0.DestroySwapchainKHR(
                device.abi(), 
                swapchain.abi(), 
                Default::default(), 
            );
        }
    }
    /// ```c
    /// VkResult vkGetDeviceGroupPresentCapabilitiesKHR(VkDevice device, VkDeviceGroupPresentCapabilitiesKHR* pDeviceGroupPresentCapabilities)
    /// ```
    pub unsafe fn get_device_group_present_capabilities(
        &self,
        device: vk::Device,
        device_group_present_capabilities: &mut DeviceGroupPresentCapabilitiesKHR,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.GetDeviceGroupPresentCapabilitiesKHR(
                device.abi(), 
                device_group_present_capabilities.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkResult vkGetDeviceGroupSurfacePresentModesKHR(VkDevice device, VkSurfaceKHR surface, VkDeviceGroupPresentModeFlagsKHR* pModes)
    /// ```
    pub unsafe fn get_device_group_surface_present_modes(
        &self,
        device: vk::Device,
        surface: vk::SurfaceKHR,
    ) -> crate::Result<DeviceGroupPresentModeFlagsKHR> {
        unsafe {
            let mut _v: DeviceGroupPresentModeFlagsKHR = Default::default();
            let _r = self.0.GetDeviceGroupSurfacePresentModesKHR(
                device.abi(), 
                surface.abi(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| Some(_v))
        }
    }
    /// ```c
    /// VkResult vkGetSwapchainImagesKHR(VkDevice device, VkSwapchainKHR swapchain, uint32_t* pSwapchainImageCount, VkImage* pSwapchainImages)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    pub unsafe fn get_swapchain_images(
        &self,
        device: vk::Device,
        swapchain: vk::SwapchainKHR,
        swapchain_images: Option<&mut ::alloc::vec::Vec<Option<vk::Image>>>,
    ) -> crate::Result<(uint32_t, Result)> {
        unsafe {
            let mut _c: uint32_t = Default::default();
            let mut _r = self.0.GetSwapchainImagesKHR(
                device.abi(), 
                swapchain.abi(), 
                &mut _c, 
                Default::default(), 
            ).vk();
            if let Some(_b) = swapchain_images {
                _r.result(|| Some(()))?;
                _b.reserve(_c as usize);
                _r = self.0.GetSwapchainImagesKHR(
                    device.abi(), 
                    swapchain.abi(), 
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
    /// VkResult vkQueuePresentKHR(VkQueue queue, VkPresentInfoKHR const* pPresentInfo)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::SuboptimalKhr]
    pub unsafe fn queue_present(
        &self,
        queue: vk::Queue,
        present_info: &PresentInfoKHR,
    ) -> crate::Result<Result> {
        unsafe {
            let _r = self.0.QueuePresentKHR(
                queue.abi(), 
                present_info.abi(), 
            ).vk();
            _r.result(|| Some(_r))
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::khr::swapchain {
    type Commands = Device;
}

/// Device object
pub trait KhrSwapchainDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkAcquireNextImage2KHR(VkDevice device, VkAcquireNextImageInfoKHR const* pAcquireInfo, uint32_t* pImageIndex)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Timeout], [Result::NotReady], [Result::SuboptimalKhr]
    unsafe fn acquire_next_image2(
        &self,
        acquire_info: &AcquireNextImageInfoKHR,
        image_index: *mut uint32_t,
    ) -> crate::Result<Result> {
        unsafe {
            self.commands().acquire_next_image2(
                self.raw(),
                acquire_info,
                image_index,
            )
        }
    }
    /// ```c
    /// VkResult vkAcquireNextImageKHR(VkDevice device, VkSwapchainKHR swapchain, uint64_t timeout, VkSemaphore semaphore, VkFence fence, uint32_t* pImageIndex)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Timeout], [Result::NotReady], [Result::SuboptimalKhr]
    unsafe fn acquire_next_image(
        &self,
        swapchain: vk::SwapchainKHR,
        timeout: uint64_t,
        semaphore: Option<vk::Semaphore>,
        fence: Option<vk::Fence>,
        image_index: *mut uint32_t,
    ) -> crate::Result<Result> {
        unsafe {
            self.commands().acquire_next_image(
                self.raw(),
                swapchain,
                timeout,
                semaphore,
                fence,
                image_index,
            )
        }
    }
    /// ```c
    /// VkResult vkCreateSwapchainKHR(VkDevice device, VkSwapchainCreateInfoKHR const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkSwapchainKHR* pSwapchain)
    /// ```
    unsafe fn create_swapchain(
        &self,
        create_info: &SwapchainCreateInfoKHR,
    ) -> crate::Result<vk::SwapchainKHR> {
        unsafe {
            self.commands().create_swapchain(
                self.raw(),
                create_info,
            )
        }
    }
    /// ```c
    /// void vkDestroySwapchainKHR(VkDevice device, VkSwapchainKHR swapchain, VkAllocationCallbacks const* pAllocator)
    /// ```
    unsafe fn destroy_swapchain(
        &self,
        swapchain: Option<vk::SwapchainKHR>,
    ) -> () {
        unsafe {
            self.commands().destroy_swapchain(
                self.raw(),
                swapchain,
            )
        }
    }
    /// ```c
    /// VkResult vkGetDeviceGroupPresentCapabilitiesKHR(VkDevice device, VkDeviceGroupPresentCapabilitiesKHR* pDeviceGroupPresentCapabilities)
    /// ```
    unsafe fn get_device_group_present_capabilities(
        &self,
        device_group_present_capabilities: &mut DeviceGroupPresentCapabilitiesKHR,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().get_device_group_present_capabilities(
                self.raw(),
                device_group_present_capabilities,
            )
        }
    }
    /// ```c
    /// VkResult vkGetDeviceGroupSurfacePresentModesKHR(VkDevice device, VkSurfaceKHR surface, VkDeviceGroupPresentModeFlagsKHR* pModes)
    /// ```
    unsafe fn get_device_group_surface_present_modes(
        &self,
        surface: vk::SurfaceKHR,
    ) -> crate::Result<DeviceGroupPresentModeFlagsKHR> {
        unsafe {
            self.commands().get_device_group_surface_present_modes(
                self.raw(),
                surface,
            )
        }
    }
    /// ```c
    /// VkResult vkGetSwapchainImagesKHR(VkDevice device, VkSwapchainKHR swapchain, uint32_t* pSwapchainImageCount, VkImage* pSwapchainImages)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    unsafe fn get_swapchain_images(
        &self,
        swapchain: vk::SwapchainKHR,
        swapchain_images: Option<&mut ::alloc::vec::Vec<Option<vk::Image>>>,
    ) -> crate::Result<(uint32_t, Result)> {
        unsafe {
            self.commands().get_swapchain_images(
                self.raw(),
                swapchain,
                swapchain_images,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::khr::swapchain {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::khr::swapchain> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::khr::swapchain {
        type Output = crate::hnd::Device<vk::extensions::khr::swapchain>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::khr::swapchain>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::khr::swapchain> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::khr::swapchain> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::khr::swapchain> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::khr::swapchain> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::KhrSwapchainDevice for crate::hnd::Device<vk::extensions::khr::swapchain> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::khr::swapchain, vk::Device> for crate::hnd::Device<vk::extensions::khr::swapchain> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

/// Queue object
pub trait KhrSwapchainQueue {
    fn raw(&self) -> vk::Queue;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkQueuePresentKHR(VkQueue queue, VkPresentInfoKHR const* pPresentInfo)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::SuboptimalKhr]
    unsafe fn present(
        &self,
        present_info: &PresentInfoKHR,
    ) -> crate::Result<Result> {
        unsafe {
            self.commands().queue_present(
                self.raw(),
                present_info,
            )
        }
    }
}

impl<O: crate::HndCtx<vk::extensions::khr::swapchain, vk::Device>> crate::Owner<vk::SwapchainKHR, vk::extensions::khr::swapchain> for O {
    fn drop(&mut self, raw: vk::SwapchainKHR) {
        unsafe {
            self.commands().destroy_swapchain(self.raw(), Some(raw))
        }
    }
}
impl<O> crate::HndCtx<vk::extensions::khr::swapchain, vk::Device> for ::alloc::sync::Arc<crate::Unique<vk::SwapchainKHR, O, vk::extensions::khr::swapchain>>
    where O: crate::HndCtx<vk::extensions::khr::swapchain, vk::Device> + Send + Sync + 'static,
{
    type Ctx = Self;
    fn ctx(&self) -> Self::Ctx { self.clone() }
    fn raw(&self) -> vk::Device { self.owner.raw() }
    fn commands(&self) -> &::alloc::sync::Arc<Device> { self.owner.commands() }
}

impl crate::HndScope<vk::SwapchainKHR> for vk::extensions::khr::swapchain {
    type Impl = _hs_SwapchainKHR::SwapchainKHR;
}


mod _hs_SwapchainKHR {
    use super::*;

    #[repr(transparent)]
    #[derive(Debug)]
    pub struct SwapchainKHR(pub(crate) crate::handle::Hnd<vk::SwapchainKHR, ::alloc::sync::Arc<super::Device>>);

    impl Clone for SwapchainKHR {
        fn clone(&self) -> Self { Self(self.0.clone()) }
    }

    #[derive(Debug, Clone, Copy)]
    struct Inst;
    impl<Ctx, Dep> crate::handle::HndInst<vk::SwapchainKHR, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)> for Inst
        where Ctx: crate::HndCtx<vk::extensions::khr::swapchain, vk::Device>,
    {
        fn drop(this: &mut crate::handle::HndData<vk::SwapchainKHR, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)>) {
            unsafe {
                this.scope.destroy_swapchain(this.dep.0.raw(), Some(this.raw))
            }
        }
    }

    impl crate::hnd::SwapchainKHR<vk::extensions::khr::swapchain>
    {
        pub unsafe fn new(ctx: &impl crate::HndCtx<vk::extensions::khr::swapchain, vk::Device>, raw: vk::SwapchainKHR) -> Self {
            unsafe { Self::new_with(ctx, raw, || ()) }
        }
        pub unsafe fn new_with<Dep: Send + Sync + 'static>(ctx: &impl crate::HndCtx<vk::extensions::khr::swapchain, vk::Device>, raw: vk::SwapchainKHR, dep: impl FnOnce() -> Dep) -> Self {
            Self(SwapchainKHR(crate::handle::Hnd::new(
                ctx.commands().clone(),
                raw,
                (ctx.ctx(), dep()),
                Inst,
            )))
        }
    }

    impl crate::hnd::SwapchainKHR<vk::extensions::khr::swapchain> {
        pub fn raw(&self) -> vk::SwapchainKHR { self.0.0.raw }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0.scope }
    }

    impl ::core::fmt::Debug for crate::hnd::SwapchainKHR<vk::extensions::khr::swapchain> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("SwapchainKHR({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::SwapchainKHR<vk::extensions::khr::swapchain> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl<Ctx> crate::MakeHnd<Ctx, vk::extensions::khr::swapchain> for vk::SwapchainKHR
        where Ctx: crate::HndCtx<vk::extensions::khr::swapchain, vk::Device>,
    {
        type Output = crate::hnd::SwapchainKHR<vk::extensions::khr::swapchain>;
        unsafe fn hnd_with<Dep: Send + Sync + 'static>(self, ctx: &Ctx, dep: impl FnOnce() -> Dep) -> Self::Output {
            unsafe { crate::hnd::SwapchainKHR::<vk::extensions::khr::swapchain>::new_with(ctx, self, dep) }
        }
    }
}
