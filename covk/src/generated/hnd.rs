// generated file, do not modify manually

use crate::vk::*;
use crate::generated::enums::*;
use ::core::ptr::NonNull;
use ::core::num::NonZeroU64;
use ::alloc::sync::Arc;
use crate::sys::ffi::*;
use crate::{sys, vk, Sys, Vk};

/// `VkAccelerationStructureKHR` : `VkDevice`
#[repr(transparent)]
pub struct AccelerationStructureKHR<S: crate::HndScope<vk::AccelerationStructureKHR> = vk::extensions::khr::acceleration_structure>(pub(crate) S::Impl);
impl_object! { AccelerationStructureKHR(vk::AccelerationStructureKHR) }

/// `VkAccelerationStructureNV` : `VkDevice`
#[repr(transparent)]
pub struct AccelerationStructureNV<S: crate::HndScope<vk::AccelerationStructureNV> = vk::extensions::nv::ray_tracing>(pub(crate) S::Impl);
impl_object! { AccelerationStructureNV(vk::AccelerationStructureNV) }

/// `VkBuffer` : `VkDevice`
#[repr(transparent)]
pub struct Buffer<S: crate::HndScope<vk::Buffer> = vk::core>(pub(crate) S::Impl);
impl_object! { Buffer(vk::Buffer) }

/// `VkBufferCollectionFUCHSIA` : `VkDevice`
#[repr(transparent)]
pub struct BufferCollectionFUCHSIA<S: crate::HndScope<vk::BufferCollectionFUCHSIA>>(pub(crate) S::Impl);
impl_object! { BufferCollectionFUCHSIA(vk::BufferCollectionFUCHSIA) }

/// `VkBufferView` : `VkDevice`
#[repr(transparent)]
pub struct BufferView<S: crate::HndScope<vk::BufferView> = vk::core>(pub(crate) S::Impl);
impl_object! { BufferView(vk::BufferView) }

/// `VkCommandPool` : `VkDevice`
#[repr(transparent)]
pub struct CommandPool<S: crate::HndScope<vk::CommandPool> = vk::core>(pub(crate) S::Impl);
impl_object! { CommandPool(vk::CommandPool) }

/// `VkCuFunctionNVX` : `VkDevice`
#[repr(transparent)]
pub struct CuFunctionNVX<S: crate::HndScope<vk::CuFunctionNVX>>(pub(crate) S::Impl);
impl_object! { CuFunctionNVX(vk::CuFunctionNVX) }

/// `VkCuModuleNVX` : `VkDevice`
#[repr(transparent)]
pub struct CuModuleNVX<S: crate::HndScope<vk::CuModuleNVX>>(pub(crate) S::Impl);
impl_object! { CuModuleNVX(vk::CuModuleNVX) }

/// `VkCudaFunctionNV` : `VkDevice`
#[repr(transparent)]
pub struct CudaFunctionNV<S: crate::HndScope<vk::CudaFunctionNV>>(pub(crate) S::Impl);
impl_object! { CudaFunctionNV(vk::CudaFunctionNV) }

/// `VkCudaModuleNV` : `VkDevice`
#[repr(transparent)]
pub struct CudaModuleNV<S: crate::HndScope<vk::CudaModuleNV>>(pub(crate) S::Impl);
impl_object! { CudaModuleNV(vk::CudaModuleNV) }

/// `VkDataGraphPipelineSessionARM` : `VkDevice`
#[repr(transparent)]
pub struct DataGraphPipelineSessionARM<S: crate::HndScope<vk::DataGraphPipelineSessionARM>>(pub(crate) S::Impl);
impl_object! { DataGraphPipelineSessionARM(vk::DataGraphPipelineSessionARM) }

/// `VkDebugReportCallbackEXT` : `VkInstance`
#[repr(transparent)]
pub struct DebugReportCallbackEXT<S: crate::HndScope<vk::DebugReportCallbackEXT> = vk::extensions::ext::debug_report>(pub(crate) S::Impl);
impl_object! { DebugReportCallbackEXT(vk::DebugReportCallbackEXT) }

/// `VkDebugUtilsMessengerEXT` : `VkInstance`
#[repr(transparent)]
pub struct DebugUtilsMessengerEXT<S: crate::HndScope<vk::DebugUtilsMessengerEXT> = vk::extensions::ext::debug_utils>(pub(crate) S::Impl);
impl_object! { DebugUtilsMessengerEXT(vk::DebugUtilsMessengerEXT) }

/// `VkDeferredOperationKHR` : `VkDevice`
#[repr(transparent)]
pub struct DeferredOperationKHR<S: crate::HndScope<vk::DeferredOperationKHR> = vk::extensions::khr::deferred_host_operations>(pub(crate) S::Impl);
impl_object! { DeferredOperationKHR(vk::DeferredOperationKHR) }

/// `VkDescriptorPool` : `VkDevice`
#[repr(transparent)]
pub struct DescriptorPool<S: crate::HndScope<vk::DescriptorPool> = vk::core>(pub(crate) S::Impl);
impl_object! { DescriptorPool(vk::DescriptorPool) }

/// `VkDescriptorSetLayout` : `VkDevice`
#[repr(transparent)]
pub struct DescriptorSetLayout<S: crate::HndScope<vk::DescriptorSetLayout> = vk::core>(pub(crate) S::Impl);
impl_object! { DescriptorSetLayout(vk::DescriptorSetLayout) }

/// `VkDescriptorUpdateTemplate` : `VkDevice`
#[repr(transparent)]
pub struct DescriptorUpdateTemplate<S: crate::HndScope<vk::DescriptorUpdateTemplate> = vk::core>(pub(crate) S::Impl);
impl_object! { DescriptorUpdateTemplate(vk::DescriptorUpdateTemplate) }

pub use DescriptorUpdateTemplate as DescriptorUpdateTemplateKHR;

/// `VkDevice` : `VkPhysicalDevice`
#[repr(transparent)]
pub struct Device<S: crate::HndScope<vk::Device> = vk::core>(pub(crate) S::Impl);
impl_object! { Device(vk::Device) }

/// `VkEvent` : `VkDevice`
#[repr(transparent)]
pub struct Event<S: crate::HndScope<vk::Event> = vk::core>(pub(crate) S::Impl);
impl_object! { Event(vk::Event) }

/// `VkExternalComputeQueueNV` : `VkDevice`
#[repr(transparent)]
pub struct ExternalComputeQueueNV<S: crate::HndScope<vk::ExternalComputeQueueNV>>(pub(crate) S::Impl);
impl_object! { ExternalComputeQueueNV(vk::ExternalComputeQueueNV) }

/// `VkFence` : `VkDevice`
#[repr(transparent)]
pub struct Fence<S: crate::HndScope<vk::Fence> = vk::core>(pub(crate) S::Impl);
impl_object! { Fence(vk::Fence) }

/// `VkFramebuffer` : `VkDevice`
#[repr(transparent)]
pub struct Framebuffer<S: crate::HndScope<vk::Framebuffer> = vk::core>(pub(crate) S::Impl);
impl_object! { Framebuffer(vk::Framebuffer) }

/// `VkImage` : `VkDevice`
#[repr(transparent)]
pub struct Image<S: crate::HndScope<vk::Image> = vk::core>(pub(crate) S::Impl);
impl_object! { Image(vk::Image) }

/// `VkImageView` : `VkDevice`
#[repr(transparent)]
pub struct ImageView<S: crate::HndScope<vk::ImageView> = vk::core>(pub(crate) S::Impl);
impl_object! { ImageView(vk::ImageView) }

/// `VkIndirectCommandsLayoutEXT` : `VkDevice`
#[repr(transparent)]
pub struct IndirectCommandsLayoutEXT<S: crate::HndScope<vk::IndirectCommandsLayoutEXT> = vk::extensions::ext::device_generated_commands>(pub(crate) S::Impl);
impl_object! { IndirectCommandsLayoutEXT(vk::IndirectCommandsLayoutEXT) }

/// `VkIndirectCommandsLayoutNV` : `VkDevice`
#[repr(transparent)]
pub struct IndirectCommandsLayoutNV<S: crate::HndScope<vk::IndirectCommandsLayoutNV> = vk::extensions::nv::device_generated_commands>(pub(crate) S::Impl);
impl_object! { IndirectCommandsLayoutNV(vk::IndirectCommandsLayoutNV) }

/// `VkIndirectExecutionSetEXT` : `VkDevice`
#[repr(transparent)]
pub struct IndirectExecutionSetEXT<S: crate::HndScope<vk::IndirectExecutionSetEXT> = vk::extensions::ext::device_generated_commands>(pub(crate) S::Impl);
impl_object! { IndirectExecutionSetEXT(vk::IndirectExecutionSetEXT) }

/// `VkInstance`
#[repr(transparent)]
pub struct Instance<S: crate::HndScope<vk::Instance> = vk::core>(pub(crate) S::Impl);
impl_object! { Instance(vk::Instance) }

/// `VkMicromapEXT` : `VkDevice`
#[repr(transparent)]
pub struct MicromapEXT<S: crate::HndScope<vk::MicromapEXT> = vk::extensions::ext::opacity_micromap>(pub(crate) S::Impl);
impl_object! { MicromapEXT(vk::MicromapEXT) }

/// `VkOpticalFlowSessionNV` : `VkDevice`
#[repr(transparent)]
pub struct OpticalFlowSessionNV<S: crate::HndScope<vk::OpticalFlowSessionNV>>(pub(crate) S::Impl);
impl_object! { OpticalFlowSessionNV(vk::OpticalFlowSessionNV) }

/// `VkPipeline` : `VkDevice`
#[repr(transparent)]
pub struct Pipeline<S: crate::HndScope<vk::Pipeline> = vk::core>(pub(crate) S::Impl);
impl_object! { Pipeline(vk::Pipeline) }

/// `VkPipelineBinaryKHR` : `VkDevice`
#[repr(transparent)]
pub struct PipelineBinaryKHR<S: crate::HndScope<vk::PipelineBinaryKHR> = vk::extensions::khr::pipeline_binary>(pub(crate) S::Impl);
impl_object! { PipelineBinaryKHR(vk::PipelineBinaryKHR) }

/// `VkPipelineCache` : `VkDevice`
#[repr(transparent)]
pub struct PipelineCache<S: crate::HndScope<vk::PipelineCache> = vk::core>(pub(crate) S::Impl);
impl_object! { PipelineCache(vk::PipelineCache) }

/// `VkPipelineLayout` : `VkDevice`
#[repr(transparent)]
pub struct PipelineLayout<S: crate::HndScope<vk::PipelineLayout> = vk::core>(pub(crate) S::Impl);
impl_object! { PipelineLayout(vk::PipelineLayout) }

/// `VkPrivateDataSlot` : `VkDevice`
#[repr(transparent)]
pub struct PrivateDataSlot<S: crate::HndScope<vk::PrivateDataSlot> = vk::core>(pub(crate) S::Impl);
impl_object! { PrivateDataSlot(vk::PrivateDataSlot) }

pub use PrivateDataSlot as PrivateDataSlotEXT;

/// `VkQueryPool` : `VkDevice`
#[repr(transparent)]
pub struct QueryPool<S: crate::HndScope<vk::QueryPool> = vk::core>(pub(crate) S::Impl);
impl_object! { QueryPool(vk::QueryPool) }

/// `VkRenderPass` : `VkDevice`
#[repr(transparent)]
pub struct RenderPass<S: crate::HndScope<vk::RenderPass> = vk::core>(pub(crate) S::Impl);
impl_object! { RenderPass(vk::RenderPass) }

/// `VkSampler` : `VkDevice`
#[repr(transparent)]
pub struct Sampler<S: crate::HndScope<vk::Sampler> = vk::core>(pub(crate) S::Impl);
impl_object! { Sampler(vk::Sampler) }

/// `VkSamplerYcbcrConversion` : `VkDevice`
#[repr(transparent)]
pub struct SamplerYcbcrConversion<S: crate::HndScope<vk::SamplerYcbcrConversion> = vk::core>(pub(crate) S::Impl);
impl_object! { SamplerYcbcrConversion(vk::SamplerYcbcrConversion) }

pub use SamplerYcbcrConversion as SamplerYcbcrConversionKHR;

/// `VkSemaphore` : `VkDevice`
#[repr(transparent)]
pub struct Semaphore<S: crate::HndScope<vk::Semaphore> = vk::core>(pub(crate) S::Impl);
impl_object! { Semaphore(vk::Semaphore) }

/// `VkSemaphoreSciSyncPoolNV` : `VkDevice`
#[repr(transparent)]
pub struct SemaphoreSciSyncPoolNV<S: crate::HndScope<vk::SemaphoreSciSyncPoolNV> = vk::extensions::nv::external_sci_sync2>(pub(crate) S::Impl);
impl_object! { SemaphoreSciSyncPoolNV(vk::SemaphoreSciSyncPoolNV) }

/// `VkShaderEXT` : `VkDevice`
#[repr(transparent)]
pub struct ShaderEXT<S: crate::HndScope<vk::ShaderEXT> = vk::extensions::ext::shader_object>(pub(crate) S::Impl);
impl_object! { ShaderEXT(vk::ShaderEXT) }

/// `VkShaderInstrumentationARM` : `VkDevice`
#[repr(transparent)]
pub struct ShaderInstrumentationARM<S: crate::HndScope<vk::ShaderInstrumentationARM> = vk::extensions::arm::shader_instrumentation>(pub(crate) S::Impl);
impl_object! { ShaderInstrumentationARM(vk::ShaderInstrumentationARM) }

/// `VkShaderModule` : `VkDevice`
#[repr(transparent)]
pub struct ShaderModule<S: crate::HndScope<vk::ShaderModule> = vk::core>(pub(crate) S::Impl);
impl_object! { ShaderModule(vk::ShaderModule) }

/// `VkSurfaceKHR` : `VkInstance`
#[repr(transparent)]
pub struct SurfaceKHR<S: crate::HndScope<vk::SurfaceKHR> = vk::extensions::khr::surface>(pub(crate) S::Impl);
impl_object! { SurfaceKHR(vk::SurfaceKHR) }

/// `VkSwapchainKHR` : `VkDevice`
#[repr(transparent)]
pub struct SwapchainKHR<S: crate::HndScope<vk::SwapchainKHR> = vk::extensions::khr::swapchain>(pub(crate) S::Impl);
impl_object! { SwapchainKHR(vk::SwapchainKHR) }

/// `VkTensorARM` : `VkDevice`
#[repr(transparent)]
pub struct TensorARM<S: crate::HndScope<vk::TensorARM> = vk::extensions::arm::tensors>(pub(crate) S::Impl);
impl_object! { TensorARM(vk::TensorARM) }

/// `VkTensorViewARM` : `VkDevice`
#[repr(transparent)]
pub struct TensorViewARM<S: crate::HndScope<vk::TensorViewARM> = vk::extensions::arm::tensors>(pub(crate) S::Impl);
impl_object! { TensorViewARM(vk::TensorViewARM) }

/// `VkValidationCacheEXT` : `VkDevice`
#[repr(transparent)]
pub struct ValidationCacheEXT<S: crate::HndScope<vk::ValidationCacheEXT> = vk::extensions::ext::validation_cache>(pub(crate) S::Impl);
impl_object! { ValidationCacheEXT(vk::ValidationCacheEXT) }

/// `VkVideoSessionKHR` : `VkDevice`
#[repr(transparent)]
pub struct VideoSessionKHR<S: crate::HndScope<vk::VideoSessionKHR> = vk::extensions::khr::video_queue>(pub(crate) S::Impl);
impl_object! { VideoSessionKHR(vk::VideoSessionKHR) }

/// `VkVideoSessionParametersKHR` : `VkDevice`
#[repr(transparent)]
pub struct VideoSessionParametersKHR<S: crate::HndScope<vk::VideoSessionParametersKHR> = vk::extensions::khr::video_queue>(pub(crate) S::Impl);
impl_object! { VideoSessionParametersKHR(vk::VideoSessionParametersKHR) }
