use env_logger::Env;
use glam::{EulerRot, FloatExt, Mat4, Quat, Vec3A, Vec4};
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use std::collections::{HashSet, VecDeque};
use std::ffi::{CStr, c_void};
use std::fmt::{Debug, Formatter};
use std::mem::ManuallyDrop;
use std::num::NonZero;
use std::ops::Deref;
use std::sync::atomic::AtomicU64;
use std::sync::{Arc, Mutex};
use winit::application::ApplicationHandler;
use winit::dpi::{LogicalSize, PhysicalPosition};
use winit::event::WindowEvent;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;

use covk::{MakeUnique, hnd, prelude::*};
use covk_sys::c;
use covk_sys::ffi::void;

static CUBE_SPV: &[u8] = include_bytes!("../../assets/Cube.spv");
static IMAGE_VULKAN_LOGO: &[u8] = include_bytes!("../../assets/VulkanLogo.basis.zst");

/// ```text
/// 0..72 : index [u16; 36]
/// 72..768 : position [f32 x 3; 24], (+12) normal [f32 x 3; 24], (+24) uv [f32 x 2; 24]
/// ```
static CUBE_MESH: &[u8; 840] = {
    &[
        0, 0, 1, 0, 3, 0, 2, 0, 8, 0, 5, 0, 4, 0, 9, 0, 7, 0, 6, 0, 10, 0, 11, 0, 12, 0, 13, 0, 14,
        0, 15, 0, 16, 0, 17, 0, 0, 0, 3, 0, 18, 0, 2, 0, 5, 0, 19, 0, 4, 0, 7, 0, 20, 0, 6, 0, 11,
        0, 21, 0, 12, 0, 14, 0, 22, 0, 15, 0, 17, 0, 23, 0, 0, 0, 0, 191, 0, 0, 0, 191, 0, 0, 0,
        191, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 191, 0, 0, 0, 0, 0, 0, 128, 63, 0, 0, 0, 191, 0, 0,
        0, 63, 0, 0, 0, 191, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 191, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 63, 0, 0, 0, 191, 0, 0, 0, 191, 0, 0, 128, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        128, 63, 0, 0, 0, 63, 0, 0, 0, 63, 0, 0, 0, 191, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 191, 0,
        0, 128, 63, 0, 0, 0, 0, 0, 0, 0, 63, 0, 0, 0, 191, 0, 0, 0, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 128, 63, 0, 0, 0, 0, 0, 0, 128, 63, 0, 0, 0, 63, 0, 0, 0, 63, 0, 0, 0, 63, 0, 0, 128,
        63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 63, 0, 0, 0, 0, 0, 0, 0, 191, 0, 0, 0, 191, 0, 0, 0,
        63, 0, 0, 128, 191, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 63, 0, 0, 0, 191, 0, 0,
        0, 63, 0, 0, 0, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 63, 0, 0, 128, 63, 0, 0, 0, 0, 0, 0,
        0, 63, 0, 0, 0, 63, 0, 0, 0, 191, 0, 0, 128, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 63, 0, 0, 0, 63, 0, 0, 0, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 63, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 191, 0, 0, 0, 63, 0, 0, 0, 63, 0, 0, 128, 191, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 191, 0, 0, 0, 63, 0, 0, 0, 191, 0, 0, 128, 191, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 63, 0, 0, 0, 0, 0, 0, 0, 191, 0, 0, 0, 63, 0, 0, 0, 191, 0,
        0, 0, 0, 0, 0, 128, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 63, 0, 0, 0, 191, 0, 0, 0, 63,
        0, 0, 0, 63, 0, 0, 0, 0, 0, 0, 128, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 63, 0,
        0, 0, 63, 0, 0, 0, 63, 0, 0, 0, 0, 0, 0, 128, 63, 0, 0, 0, 0, 0, 0, 128, 63, 0, 0, 0, 0, 0,
        0, 0, 191, 0, 0, 0, 191, 0, 0, 0, 63, 0, 0, 0, 0, 0, 0, 128, 191, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 128, 63, 0, 0, 0, 191, 0, 0, 0, 191, 0, 0, 0, 191, 0, 0, 0, 0, 0, 0, 128, 191, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 63, 0, 0, 0, 191, 0, 0, 0, 191, 0, 0, 0, 0, 0, 0,
        128, 191, 0, 0, 0, 0, 0, 0, 128, 63, 0, 0, 0, 0, 0, 0, 0, 63, 0, 0, 0, 191, 0, 0, 0, 191,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 191, 0, 0, 128, 63, 0, 0, 128, 63, 0, 0, 0, 63, 0, 0, 0,
        191, 0, 0, 0, 63, 0, 0, 128, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 63, 0, 0, 128, 63, 0,
        0, 0, 191, 0, 0, 0, 191, 0, 0, 0, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 63, 0, 0, 128, 63,
        0, 0, 128, 63, 0, 0, 0, 191, 0, 0, 0, 191, 0, 0, 0, 191, 0, 0, 128, 191, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 128, 63, 0, 0, 128, 63, 0, 0, 0, 63, 0, 0, 0, 63, 0, 0, 0, 191, 0, 0, 0, 0, 0,
        0, 128, 63, 0, 0, 0, 0, 0, 0, 128, 63, 0, 0, 128, 63, 0, 0, 0, 63, 0, 0, 0, 191, 0, 0, 0,
        63, 0, 0, 0, 0, 0, 0, 128, 191, 0, 0, 0, 0, 0, 0, 128, 63, 0, 0, 128, 63,
    ]
};

fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    basis_universal::transcoder_init();
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = App::default();
    event_loop.run_app(&mut app)?;
    Ok(())
}

#[repr(C)]
#[derive(Debug)]
struct Entry {
    pub scene: u64,
    pub params: u64,
}

#[repr(C)]
#[derive(Debug)]
struct Scene {
    pub view: Mat4,
    pub proj: Mat4,
    pub light_dir: Vec3A,
    pub time: f32,
}

#[repr(C)]
#[derive(Debug)]
struct Parameters {
    pub model: Mat4,
    pub inv_model: Mat4,
    /// hsv
    pub color: Vec4,
    pub texture: u32,
    pub sampler: u32,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Assets {
    pub vulkan_logo: ImageAsset,
}

impl Assets {
    pub fn load() -> anyhow::Result<Self> {
        Ok(Self {
            vulkan_logo: ImageAsset::load(IMAGE_VULKAN_LOGO)?,
        })
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Resources {
    pub cube_mesh: Arc<Buffer>,
    pub vk_logo: Arc<Image>,
    pub vk_logo_view: hnd::ImageView,
    pub linear_repeat_sampler: hnd::Sampler,
    pub vk_logo_hnd: u32,
    pub linear_repeat_sampler_hnd: u32,
    pub depth: Option<(u32, u32, (Arc<Image>, hnd::ImageView))>,
}

impl Resources {
    pub fn ensure_depth(
        &mut self,
        fc: &mut FrameContext,
        swap_chain: &SwapChain,
        allocator: &Arc<Allocator>,
    ) -> anyhow::Result<()> {
        unsafe {
            if let Some((w, h, _)) = &self.depth {
                if (*w, *h) == swap_chain.size {
                    return Ok(());
                }
            }
            let (w, h) = swap_chain.size;
            let device = &allocator.device;
            let queue_family_indices = [device.main_queue_family];
            let img = allocator.alloc_image(
                "Depth",
                &vk::ImageCreateInfo::default()
                    .image_type(vk::ImageType::_2d)
                    .format(vk::Format::D32_SFloat)
                    .extent(vk::Extent3D {
                        width: w,
                        height: h,
                        depth: 1,
                    })
                    .mip_levels(1)
                    .array_layers(1)
                    .samples(vk::SampleCountFlags::_1)
                    .usage(vk::ImageUsageFlags::DepthStencilAttachment)
                    .queue_family_indices(&queue_family_indices),
                None,
            )?;
            let view = device
                .create_image_view(
                    &vk::ImageViewCreateInfo::default()
                        .image(img.image().raw())
                        .view_type(vk::ImageViewType::_2d)
                        .format(vk::Format::D32_SFloat)
                        .subresource_range(vk::ImageSubresourceRange {
                            aspect_mask: vk::ImageAspectFlags::Depth,
                            base_mip_level: 0,
                            level_count: 1,
                            base_array_layer: 0,
                            layer_count: 1,
                        }),
                )?
                .hnd_with(&device.hnd, || img.clone());
            if let Some(old) = self.depth.replace((w, h, (img, view))) {
                fc.free.push(Box::new(move || drop(old)));
            }
            Ok(())
        }
    }
    pub fn depth(&self) -> &(Arc<Image>, hnd::ImageView) {
        &self.depth.as_ref().unwrap().2
    }
}

impl RenderContext {
    pub fn init(&mut self, window: &Window, assets: Assets) -> anyhow::Result<()> {
        unsafe {
            let queue_family_indices = [self.allocator.device.main_queue_family];
            let cube_mesh = self.allocator.alloc_buffer(
                "Cube",
                &vk::BufferCreateInfo::default()
                    .size(CUBE_MESH.len() as u64)
                    .usage(
                        vk::BufferUsageFlags::TransferDst
                            | vk::BufferUsageFlags::IndexBuffer
                            | vk::BufferUsageFlags::VertexBuffer,
                    )
                    .queue_family_indices(&queue_family_indices),
                None,
            )?;
            let vk_logo = self.allocator.alloc_image(
                "Vulkan Logo",
                &vk::ImageCreateInfo::default()
                    .image_type(vk::ImageType::_2d)
                    .format(assets.vulkan_logo.format)
                    .extent(vk::Extent3D {
                        width: assets.vulkan_logo.width,
                        height: assets.vulkan_logo.height,
                        depth: 1,
                    })
                    .mip_levels(assets.vulkan_logo.mips.len() as u32)
                    .array_layers(1)
                    .samples(vk::SampleCountFlags::_1)
                    .usage(vk::ImageUsageFlags::TransferDst | vk::ImageUsageFlags::Sampled)
                    .queue_family_indices(&queue_family_indices),
                None,
            )?;
            let vk_logo_view = self
                .device
                .create_image_view(
                    &vk::ImageViewCreateInfo::default()
                        .image(vk_logo.image().raw())
                        .view_type(vk::ImageViewType::_2d)
                        .format(assets.vulkan_logo.format)
                        .subresource_range(vk::ImageSubresourceRange {
                            aspect_mask: vk::ImageAspectFlags::Color,
                            base_mip_level: 0,
                            level_count: assets.vulkan_logo.mips.len() as u32,
                            base_array_layer: 0,
                            layer_count: 1,
                        }),
                )?
                .hnd_with(&self.device.hnd, || vk_logo.clone());
            let linear_repeat_sampler = self
                .device
                .create_sampler(
                    &vk::SamplerCreateInfo::default()
                        .mag_filter(vk::Filter::Linear)
                        .min_filter(vk::Filter::Linear)
                        .mipmap_mode(vk::SamplerMipmapMode::Linear)
                        .address_mode_u(vk::SamplerAddressMode::Repeat)
                        .address_mode_v(vk::SamplerAddressMode::Repeat)
                        .address_mode_w(vk::SamplerAddressMode::Repeat)
                        .min_lod(0.0)
                        .max_lod(f32::MAX),
                )?
                .hnd(&self.device.hnd);

            {
                let image_info0 = [vk::DescriptorImageInfo::default()
                    .sampler(linear_repeat_sampler.raw())];
                let image_info1 = [vk::DescriptorImageInfo::default()
                    .image_view(vk_logo_view.raw())
                    .image_layout(vk::ImageLayout::ShaderReadOnlyOptimal)];
                let writers = [
                    vk::WriteDescriptorSet::default()
                        .dst_set(self.desc_set.hnd)
                        .dst_binding(0)
                        .dst_array_element(0)
                        .descriptor_count(1)
                        .descriptor_type(vk::DescriptorType::Sampler)
                        .image_info(&image_info0),
                    vk::WriteDescriptorSet::default()
                        .dst_set(self.desc_set.hnd)
                        .dst_binding(1)
                        .dst_array_element(0)
                        .descriptor_count(1)
                        .descriptor_type(vk::DescriptorType::SampledImage)
                        .image_info(&image_info1),
                ];
                self.device.update_descriptor_sets(&writers, &[]);
            }

            self.assets = Some(assets);
            self.resources = Some(Resources {
                cube_mesh,
                vk_logo,
                vk_logo_view,
                linear_repeat_sampler,
                vk_logo_hnd: 0,
                linear_repeat_sampler_hnd: 0,
                depth: None,
            });



            self.render(true, 0.0)?;
            window.set_visible(true);
            Ok(())
        }
    }
    pub fn first_frame(&mut self, fc: &mut FrameContext) -> anyhow::Result<()> {
        unsafe {
            let assets = self.assets.as_ref().unwrap();
            let resources = self.resources.as_ref().unwrap();
            let device = &*fc.cmd.device;
            let cmd = &fc.cmd;

            // barrier
            {
                let img_barriers = [vk::ImageMemoryBarrier2::default()
                    .src_stage_mask(vk::PipelineStageFlags2::TopOfPipe)
                    .src_access_mask(vk::AccessFlags2::None)
                    .dst_stage_mask(vk::PipelineStageFlags2::Transfer)
                    .dst_access_mask(vk::AccessFlags2::TransferWrite)
                    .old_layout(vk::ImageLayout::Undefined)
                    .new_layout(vk::ImageLayout::TransferDstOptimal)
                    .src_queue_family_index(device.main_queue_family)
                    .dst_queue_family_index(device.main_queue_family)
                    .image(resources.vk_logo.image().raw())
                    .subresource_range(vk::ImageSubresourceRange {
                        aspect_mask: vk::ImageAspectFlags::Color,
                        base_mip_level: 0,
                        level_count: assets.vulkan_logo.mips.len() as u32,
                        base_array_layer: 0,
                        layer_count: 1,
                    })];
                let dep = vk::DependencyInfo::default().image_memory_barriers(&img_barriers);
                cmd.pipeline_barrier2(&dep);
            }

            let cube_mesh_loc = fc.upload.write_bytes(4, CUBE_MESH)?;
            let cube_mesh_size = cube_mesh_loc.size;
            cmd.copy_buffer(
                cube_mesh_loc.buffer.buffer().raw(),
                resources.cube_mesh.buffer().raw(),
                &[vk::BufferCopy {
                    src_offset: cube_mesh_loc.offset,
                    dst_offset: 0,
                    size: cube_mesh_size,
                }],
            );

            for (mip, data) in assets.vulkan_logo.mips.iter().enumerate() {
                let loc = fc.upload.write_bytes(16, data)?;
                cmd.copy_buffer_to_image(
                    loc.buffer.buffer().raw(),
                    resources.vk_logo.image().raw(),
                    vk::ImageLayout::TransferDstOptimal,
                    &[vk::BufferImageCopy {
                        buffer_offset: loc.offset,
                        buffer_row_length: 0,
                        buffer_image_height: 0,
                        image_subresource: vk::ImageSubresourceLayers {
                            aspect_mask: vk::ImageAspectFlags::Color,
                            mip_level: mip as u32,
                            base_array_layer: 0,
                            layer_count: 1,
                        },
                        image_offset: Default::default(),
                        image_extent: vk::Extent3D {
                            width: (assets.vulkan_logo.width >> mip).max(1),
                            height: (assets.vulkan_logo.height >> mip).max(1),
                            depth: 1,
                        },
                    }],
                )
            }

            // barrier
            {
                let buffer_barriers = [vk::BufferMemoryBarrier2::default()
                    .src_stage_mask(vk::PipelineStageFlags2::Transfer)
                    .src_access_mask(vk::AccessFlags2::TransferWrite)
                    .dst_stage_mask(
                        vk::PipelineStageFlags2::IndexInput
                            | vk::PipelineStageFlags2::VertexAttributeInput,
                    )
                    .dst_access_mask(
                        vk::AccessFlags2::IndexRead | vk::AccessFlags2::VertexAttributeRead,
                    )
                    .src_queue_family_index(device.main_queue_family)
                    .dst_queue_family_index(device.main_queue_family)
                    .buffer(resources.cube_mesh.buffer().raw())
                    .offset(0)
                    .size(cube_mesh_size)];
                let img_barriers = [vk::ImageMemoryBarrier2::default()
                    .src_stage_mask(vk::PipelineStageFlags2::Transfer)
                    .src_access_mask(vk::AccessFlags2::TransferWrite)
                    .dst_stage_mask(vk::PipelineStageFlags2::FragmentShader)
                    .dst_access_mask(vk::AccessFlags2::ShaderSampledRead)
                    .old_layout(vk::ImageLayout::TransferDstOptimal)
                    .new_layout(vk::ImageLayout::ShaderReadOnlyOptimal)
                    .src_queue_family_index(device.main_queue_family)
                    .dst_queue_family_index(device.main_queue_family)
                    .image(resources.vk_logo.image().raw())
                    .subresource_range(vk::ImageSubresourceRange {
                        aspect_mask: vk::ImageAspectFlags::Color,
                        base_mip_level: 0,
                        level_count: assets.vulkan_logo.mips.len() as u32,
                        base_array_layer: 0,
                        layer_count: 1,
                    })];
                let dep = vk::DependencyInfo::default()
                    .buffer_memory_barriers(&buffer_barriers)
                    .image_memory_barriers(&img_barriers);
                cmd.pipeline_barrier2(&dep);
            }

            Ok(())
        }
    }
    pub fn render(&mut self, first: bool, time: f32) -> anyhow::Result<()> {
        unsafe {
            self.swap_chain.acquire_image(&self.device)?;
            let mut fc = self.start_frame()?;

            if first {
                self.first_frame(&mut *fc)?;
            }

            self.resources.as_mut().unwrap().ensure_depth(
                &mut fc,
                &self.swap_chain.swap_chain,
                &self.allocator,
            )?;

            let device = &self.device;
            let resources = self.resources.as_ref().unwrap();
            let cmd = &fc.cmd;

            let swap_chain_image = self.swap_chain.images[self.swap_chain.cur_image as usize];
            let swap_chain_view = &self.swap_chain.views[self.swap_chain.cur_image as usize];
            let (width, height) = self.swap_chain.swap_chain.size;

            let depth = resources.depth();

            let camera = (
                Vec3A::new(0.0, 1.0, -2.0),
                Quat::from_euler(EulerRot::XYZ, std::f32::consts::FRAC_PI_6, 0.0, 0.0),
            );
            let view = {
                let inv_rot = camera.1.conjugate();
                let inv_pos = inv_rot * -camera.0;
                Mat4::from_rotation_translation(inv_rot, inv_pos.into())
            };
            let proj = Mat4::perspective_lh(45.0, width as f32 / height as f32, 1000.0, 0.01);

            // barrier
            {
                let img_barriers = [
                    vk::ImageMemoryBarrier2::default()
                        .src_stage_mask(vk::PipelineStageFlags2::TopOfPipe)
                        .src_access_mask(vk::AccessFlags2::None)
                        .dst_stage_mask(vk::PipelineStageFlags2::ColorAttachmentOutput)
                        .dst_access_mask(vk::AccessFlags2::ColorAttachmentWrite)
                        .old_layout(vk::ImageLayout::Undefined)
                        .new_layout(vk::ImageLayout::ColorAttachmentOptimal)
                        .src_queue_family_index(device.main_queue_family)
                        .dst_queue_family_index(device.main_queue_family)
                        .image(swap_chain_image.unwrap())
                        .subresource_range(vk::ImageSubresourceRange {
                            aspect_mask: vk::ImageAspectFlags::Color,
                            base_mip_level: 0,
                            level_count: 1,
                            base_array_layer: 0,
                            layer_count: 1,
                        }),
                    vk::ImageMemoryBarrier2::default()
                        .src_stage_mask(vk::PipelineStageFlags2::TopOfPipe)
                        .src_access_mask(vk::AccessFlags2::None)
                        .dst_stage_mask(
                            vk::PipelineStageFlags2::EarlyFragmentTests
                                | vk::PipelineStageFlags2::LateFragmentTests,
                        )
                        .dst_access_mask(
                            vk::AccessFlags2::DepthStencilAttachmentRead
                                | vk::AccessFlags2::DepthStencilAttachmentWrite,
                        )
                        .old_layout(vk::ImageLayout::Undefined)
                        .new_layout(vk::ImageLayout::DepthAttachmentOptimal)
                        .src_queue_family_index(device.main_queue_family)
                        .dst_queue_family_index(device.main_queue_family)
                        .image(depth.0.image().raw())
                        .subresource_range(vk::ImageSubresourceRange {
                            aspect_mask: vk::ImageAspectFlags::Depth,
                            base_mip_level: 0,
                            level_count: 1,
                            base_array_layer: 0,
                            layer_count: 1,
                        }),
                ];
                let dep = vk::DependencyInfo::default().image_memory_barriers(&img_barriers);
                cmd.pipeline_barrier2(&dep);
            }

            // begin_rendering
            {
                let color = [vk::RenderingAttachmentInfo::default()
                    .image_view(swap_chain_view.raw())
                    .image_layout(vk::ImageLayout::ColorAttachmentOptimal)
                    .load_op(vk::AttachmentLoadOp::Clear)
                    .store_op(vk::AttachmentStoreOp::Store)
                    .clear_value(
                        vk::new::ClearValue::Color(
                            vk::new::ClearColorValue::Float32([0.3, 0.3, 0.3, 1.0]).new(),
                        )
                        .new(),
                    )];
                let depth = vk::RenderingAttachmentInfo::default()
                    .image_view(depth.1.raw())
                    .image_layout(vk::ImageLayout::DepthAttachmentOptimal)
                    .load_op(vk::AttachmentLoadOp::Clear)
                    .store_op(vk::AttachmentStoreOp::Store)
                    .clear_value(
                        vk::new::ClearValue::DepthStencil(vk::ClearDepthStencilValue {
                            depth: 0.0,
                            stencil: 0,
                        })
                        .new(),
                    );
                let info = vk::RenderingInfo::default()
                    .render_area(vk::Rect2D {
                        offset: Default::default(),
                        extent: vk::Extent2D { width, height },
                    })
                    .view_mask(1)
                    .color_attachments(&color)
                    .depth_attachment(&depth);
                cmd.begin_rendering(&info);
            }

            // set view
            {
                cmd.set_viewport(
                    0,
                    &[vk::Viewport {
                        x: 0.0,
                        y: height as f32,
                        width: width as f32,
                        height: -(height as f32),
                        min_depth: 0.0,
                        max_depth: 1.0,
                    }],
                );
                cmd.set_scissor(
                    0,
                    &[vk::Rect2D {
                        offset: Default::default(),
                        extent: vk::Extent2D { width, height },
                    }],
                );
            }

            // draw
            {
                cmd.bind_descriptor_sets(
                    vk::PipelineBindPoint::Graphics,
                    self.pipeline_layout.raw(),
                    0,
                    &[Some(self.desc_set.hnd)],
                    &[],
                );
                cmd.bind_pipeline(vk::PipelineBindPoint::Graphics, self.pipeline_cube.raw());

                cmd.bind_index_buffer(
                    Some(resources.cube_mesh.buffer().raw()),
                    0,
                    vk::IndexType::Uint16,
                );
                cmd.bind_vertex_buffers(0, &[Some(resources.cube_mesh.buffer().raw())], &[72]);

                let rot_speed = 1.0;
                let float_speed = 3.0;
                let object = (
                    Vec3A::new(
                        0.0,
                        (time * float_speed)
                            .cos()
                            .remap(-HALF_PI, HALF_PI, -0.1, 0.1),
                        0.0,
                    ),
                    Quat::from_euler(EulerRot::XYZ, 0.0, time * rot_speed, 0.0),
                );
                let model = Mat4::from_rotation_translation(object.1, object.0.into());
                let inv_model = model.inverse();

                const HALF_PI: f32 = std::f32::consts::FRAC_PI_2;

                let param = Parameters {
                    model,
                    inv_model,
                    // hsv
                    color: Vec4::new(time.sin().remap(-HALF_PI, HALF_PI, 0.0, 1.0), 0.6, 0.8, 1.0),
                    texture: resources.vk_logo_hnd,
                    sampler: resources.linear_repeat_sampler_hnd,
                };
                let scene = Scene {
                    view,
                    proj,
                    light_dir: (camera.0 - object.0).normalize(),
                    time: 0.0,
                };
                let param_loc = fc.upload.write_t(16, &param)?.gpu_ptr();
                let scene_loc = fc.upload.write_t(16, &scene)?.gpu_ptr();

                let entry = Entry {
                    scene: scene_loc,
                    params: param_loc,
                };
                cmd.push_constants(
                    self.pipeline_layout.raw(),
                    vk::ShaderStageFlags::All,
                    0,
                    size_of::<Entry>() as u32,
                    &entry as *const Entry as *const c_void,
                );
                cmd.draw_indexed(36, 1, 0, 0, 0);
            }

            // end_rendering
            {
                cmd.end_rendering();
            }

            // barrier
            {
                let img_barriers = [vk::ImageMemoryBarrier2::default()
                    .src_stage_mask(vk::PipelineStageFlags2::ColorAttachmentOutput)
                    .src_access_mask(vk::AccessFlags2::ColorAttachmentWrite)
                    .dst_stage_mask(vk::PipelineStageFlags2::BottomOfPipe)
                    .dst_access_mask(vk::AccessFlags2::None)
                    .old_layout(vk::ImageLayout::ColorAttachmentOptimal)
                    .new_layout(vk::ImageLayout::PresentSrcKhr)
                    .src_queue_family_index(device.main_queue_family)
                    .dst_queue_family_index(device.main_queue_family)
                    .image(swap_chain_image.unwrap())
                    .subresource_range(vk::ImageSubresourceRange {
                        aspect_mask: vk::ImageAspectFlags::Color,
                        base_mip_level: 0,
                        level_count: 1,
                        base_array_layer: 0,
                        layer_count: 1,
                    })];
                let dep = vk::DependencyInfo::default().image_memory_barriers(&img_barriers);
                cmd.pipeline_barrier2(&dep);
            }

            self.end_frame(fc)?;
            self.swap_chain.present(&self.device)?;
            Ok(())
        }
    }
}

#[derive(Debug, Default)]
struct App {
    window: Option<Window>,
    ctx: Option<RenderContext>,
    start: Option<std::time::Instant>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.start = Some(std::time::Instant::now());
        let assets_thread = std::thread::spawn(|| Assets::load());
        self.window = Some(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_visible(false)
                        .with_title("Cube")
                        .with_inner_size(LogicalSize::new(960, 540)),
                )
                .unwrap(),
        );
        let window = self.window.as_ref().unwrap();
        if let Some(monitor) = window.current_monitor() {
            let monitor_size = monitor.size();
            let window_size = window.outer_size();
            let x = (monitor_size.width - window_size.width) / 2;
            let y = (monitor_size.height - window_size.height) / 2;
            window.set_outer_position(PhysicalPosition::new(x as i32, y as i32));
        }
        self.ctx = Some(RenderContext::new(window).unwrap());
        let assets = assets_thread.join().unwrap().unwrap();
        self.ctx.as_mut().unwrap().init(window, assets).unwrap();
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                let now = std::time::Instant::now();
                let time = now.duration_since(self.start.unwrap()).as_secs_f32();
                if let Some(ctx) = &mut self.ctx {
                    ctx.render(false, time).unwrap();
                }
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct RenderContext {
    pub assets: Option<Assets>,
    pub resources: Option<Resources>,
    pub inst: Arc<Instance>,
    pub surface: hnd::SurfaceKHR,
    pub device: Arc<Device>,
    pub allocator: Arc<Allocator>,
    pub desc_pool: hnd::DescriptorPool,
    pub desc_set: Arc<DescriptorSet>,
    pub timeline: Arc<TimelineSemaphore>,
    pub fcs: VecDeque<Box<FrameContext>>,
    pub swap_chain: SwapChainContext,
    pub pipeline_layout: hnd::PipelineLayout,
    pub shader_cube: hnd::ShaderModule,
    pub pipeline_cube: hnd::Pipeline,
}

impl Drop for RenderContext {
    fn drop(&mut self) {
        unsafe {
            _ = self.device.device_wait_idle();
        }
    }
}

impl RenderContext {
    fn new(window: &Window) -> anyhow::Result<Self> {
        unsafe {
            let inst = Instance::new()?;
            let surface = match window.window_handle()?.as_raw() {
                RawWindowHandle::Win32(hnd) => inst.sf_win32.create_win32surface(
                    &vk::new::Win32SurfaceCreateInfoKHR {
                        hinstance: Default::default(),
                        hwnd: hnd.hwnd.get(),
                    }
                    .new(),
                )?,
                _ => Err(anyhow::anyhow!("Platform not support"))?,
            }
            .hnd(&inst.sf);
            let device = Device::new(&inst, &surface)?;
            let allocator = Allocator::new(&inst, &device)?;
            let swap_chain = SwapChainContext::new(&device, &surface)?;
            let desc_pool = crate_desc_pool(&device, 1)?;
            let desc_set = DescriptorSet::new(&device, &desc_pool)?;
            let timeline = TimelineSemaphore::new(&device, 0)?;
            let pipeline_layout = crate_pipeline_layout(&device, &desc_set.layout)?;
            let shader_cube = load_shader(&device, CUBE_SPV)?;
            let pipeline_cube = Self::create_cube_pipeline(
                &device,
                &pipeline_layout,
                &shader_cube,
                &swap_chain.swap_chain,
            )?;

            Ok(Self {
                assets: None,
                resources: None,
                inst,
                surface,
                device,
                allocator,
                desc_pool,
                desc_set,
                timeline,
                fcs: VecDeque::new(),
                swap_chain,
                pipeline_layout,
                shader_cube,
                pipeline_cube,
            })
        }
    }

    fn create_cube_pipeline(
        device: &Arc<Device>,
        layout: &hnd::PipelineLayout,
        module: &hnd::ShaderModule,
        swap_chain: &SwapChain,
    ) -> anyhow::Result<hnd::Pipeline> {
        unsafe {
            let stages = [
                vk::PipelineShaderStageCreateInfo::default()
                    .stage(vk::ShaderStageFlags::Vertex)
                    .module(module.raw())
                    .name(c!("Vertex")),
                vk::PipelineShaderStageCreateInfo::default()
                    .stage(vk::ShaderStageFlags::Fragment)
                    .module(module.raw())
                    .name(c!("Pixel")),
            ];
            let dyn_states = [vk::DynamicState::Viewport, vk::DynamicState::Scissor];
            let vertex_input_state = vk::PipelineVertexInputStateCreateInfo::default()
                .vertex_binding_descriptions(&[vk::VertexInputBindingDescription {
                    binding: 0,
                    stride: ((3 + 3 + 2) * size_of::<f32>()) as u32,
                    input_rate: vk::VertexInputRate::Vertex,
                }])
                .vertex_attribute_descriptions(&[
                    vk::VertexInputAttributeDescription {
                        location: 0,
                        binding: 0,
                        format: vk::Format::R32G32B32_SFloat,
                        offset: 0,
                    },
                    vk::VertexInputAttributeDescription {
                        location: 1,
                        binding: 0,
                        format: vk::Format::R32G32B32_SFloat,
                        offset: 12,
                    },
                    vk::VertexInputAttributeDescription {
                        location: 2,
                        binding: 0,
                        format: vk::Format::R32G32_SFloat,
                        offset: 24,
                    },
                ]);
            let input_assembly_state = vk::PipelineInputAssemblyStateCreateInfo::default()
                .topology(vk::PrimitiveTopology::TriangleList);
            let viewport_state = vk::PipelineViewportStateCreateInfo::default()
                .viewport_count(1)
                .scissor_count(1);
            let rasterization_state = vk::PipelineRasterizationStateCreateInfo::default()
                .polygon_mode(vk::PolygonMode::Fill)
                .cull_mode(vk::CullModeFlags::Back)
                .front_face(vk::FrontFace::Clockwise);
            let multisample_state = vk::PipelineMultisampleStateCreateInfo::default()
                .rasterization_samples(vk::SampleCountFlags::_1);
            let depth_stencil_state = vk::PipelineDepthStencilStateCreateInfo::default()
                .depth_test_enable(true)
                .depth_write_enable(true)
                .depth_compare_op(vk::CompareOp::GreaterOrEqual)
                .depth_bounds_test_enable(true)
                .min_depth_bounds(0.0)
                .max_depth_bounds(1.0);
            let attachments = [
                vk::PipelineColorBlendAttachmentState::default().color_write_mask(
                    vk::ColorComponentFlags::R
                        | vk::ColorComponentFlags::G
                        | vk::ColorComponentFlags::B,
                ),
            ];
            let color_blend_state =
                vk::PipelineColorBlendStateCreateInfo::default().attachments(&attachments);
            let dynamic_state =
                vk::PipelineDynamicStateCreateInfo::default().dynamic_states(&dyn_states);
            let mut info = vk::GraphicsPipelineCreateInfo::default()
                .stages(&stages)
                .vertex_input_state(&vertex_input_state)
                .input_assembly_state(&input_assembly_state)
                .viewport_state(&viewport_state)
                .rasterization_state(&rasterization_state)
                .multisample_state(&multisample_state)
                .depth_stencil_state(&depth_stencil_state)
                .color_blend_state(&color_blend_state)
                .dynamic_state(&dynamic_state)
                .layout(layout.raw());
            let formats = [swap_chain.format];
            let mut dyn_info = vk::PipelineRenderingCreateInfo::default()
                .view_mask(1)
                .color_attachment_formats(&formats)
                .depth_attachment_format(vk::Format::D32_SFloat);
            info.push_next(&mut dyn_info);
            let mut pipelines = [None];
            device.create_graphics_pipelines(None, &[info], &mut pipelines)?;
            Ok(pipelines[0]
                .unwrap()
                .hnd_with(&device.hnd, || (layout.clone(), module.clone())))
        }
    }
}

struct FrameContext {
    pub signal: u64,
    pub cmd: CmdBuf,
    pub upload: UploadBuffer,
    pub free: Vec<Box<dyn FnOnce()>>,
}

impl Debug for FrameContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FrameContext")
            .field("signal", &self.signal)
            .finish()
    }
}

impl FrameContext {
    pub fn new(device: &Arc<Device>, allocator: &Arc<Allocator>) -> anyhow::Result<Box<Self>> {
        Ok(Box::new(Self {
            signal: 0,
            cmd: CmdBuf::new(device)?,
            upload: UploadBuffer::new(allocator),
            free: Vec::new(),
        }))
    }
    pub fn start(&mut self) -> anyhow::Result<()> {
        unsafe {
            self.upload.recycle();
            for free in self.free.drain(..) {
                free();
            }
            let device = &*self.cmd.device;
            device.reset_command_pool(self.cmd.pool.raw(), Default::default())?;
            let info = vk::CommandBufferBeginInfo::default()
                .flags(vk::CommandBufferUsageFlags::OneTimeSubmit);
            self.cmd.begin(&info)?;
            Ok(())
        }
    }
    pub fn end(&mut self, ctx: &mut RenderContext) -> anyhow::Result<()> {
        unsafe {
            let device = &*self.cmd.device;
            self.cmd.end()?;
            let cmd_info = [vk::CommandBufferSubmitInfo::default().command_buffer(self.cmd.raw())];
            let signal = ctx.timeline.alloc_signal();
            let signal_infos = [
                vk::SemaphoreSubmitInfo::default()
                    .semaphore(ctx.timeline.hnd.raw())
                    .value(signal)
                    .stage_mask(vk::PipelineStageFlags2::BottomOfPipe),
                vk::SemaphoreSubmitInfo::default()
                    .semaphore(ctx.swap_chain.alloc_present_signal()?)
                    .stage_mask(vk::PipelineStageFlags2::ColorAttachmentOutput),
            ];
            let submit = vk::SubmitInfo2::default()
                .command_buffer_infos(&cmd_info)
                .signal_semaphore_infos(&signal_infos);
            device.queue_submit2(device.queue, &[submit], None)?;
            self.signal = signal;
            Ok(())
        }
    }
}

impl RenderContext {
    pub fn start_frame(&mut self) -> anyhow::Result<Box<FrameContext>> {
        let mut fc = loop {
            if let Some(a) = self.fcs.front() {
                if a.signal >= self.timeline.cur_value()? {
                    break self.fcs.pop_front().unwrap();
                }
            }
            break FrameContext::new(&self.device, &self.allocator)?;
        };
        fc.start()?;
        Ok(fc)
    }
    pub fn end_frame(&mut self, mut fc: Box<FrameContext>) -> anyhow::Result<()> {
        fc.end(self)?;
        self.fcs.push_back(fc);
        Ok(())
    }
}

#[derive(Debug)]
struct SwapChainContext {
    pub fence: Arc<Fence>,
    pub swap_chain: Box<SwapChain>,
    pub images: Vec<Option<vk::Image>>,
    pub views: Vec<hnd::ImageView>,
    pub present_signals: Vec<Arc<BinarySemaphore>>,
    pub present_signal_handles: Vec<vk::Semaphore>,
    pub present_signals_pos: usize,
    pub cur_image: u32,
}

impl SwapChainContext {
    pub fn new(device: &Arc<Device>, surface: &hnd::SurfaceKHR) -> anyhow::Result<Self> {
        let fence = Fence::new(device)?;
        let swap_chain = SwapChain::new(&device, &surface, true)?;
        let mut images = vec![];
        let mut views = vec![];

        Self::get_images(device, &swap_chain, &mut images, &mut views)?;

        Ok(Self {
            swap_chain,
            fence,
            images,
            views,
            present_signals: vec![],
            present_signal_handles: vec![],
            present_signals_pos: 0,
            cur_image: 0,
        })
    }

    fn get_images(
        device: &Arc<Device>,
        swap_chain: &SwapChain,
        images: &mut Vec<Option<vk::Image>>,
        views: &mut Vec<hnd::ImageView>,
    ) -> anyhow::Result<()> {
        unsafe {
            (device.sc).get_swapchain_images(swap_chain.hnd.raw(), Some(images))?;
            views.reserve(images.len());
            for image in images.iter() {
                let info = vk::new::ImageViewCreateInfo {
                    image: image.unwrap(),
                    view_type: vk::ImageViewType::_2d,
                    format: swap_chain.format,
                    components: vk::ComponentMapping::default(),
                    subresource_range: vk::ImageSubresourceRange {
                        aspect_mask: vk::ImageAspectFlags::Color,
                        base_mip_level: 0,
                        level_count: 1,
                        base_array_layer: 0,
                        layer_count: 1,
                    },
                }
                .new();
                let view = device.create_image_view(&info)?.hnd(&device.hnd);
                views.push(view);
            }
            Ok(())
        }
    }

    fn acquire_image(&mut self, device: &Arc<Device>) -> anyhow::Result<()> {
        unsafe {
            let mut image_index = 0;
            let r = (device.sc).acquire_next_image(
                self.swap_chain.hnd.raw(),
                u64::MAX,
                None,
                Some(self.fence.hnd.raw()),
                &mut image_index,
            );
            match r {
                Ok(vk::Result::Success) => {}
                Ok(vk::Result::SuboptimalKhr) | Err(vk::Result::ErrorOutOfDateKhr) => {
                    if matches!(r, Ok(vk::Result::SuboptimalKhr)) {
                        self.fence.wait(None)?;
                    }
                    self.images.clear();
                    self.views.clear();
                    self.swap_chain.re_create()?;
                    Self::get_images(device, &self.swap_chain, &mut self.images, &mut self.views)?;
                    match (device.sc).acquire_next_image(
                        self.swap_chain.hnd.raw(),
                        u64::MAX,
                        None,
                        Some(self.fence.hnd.raw()),
                        &mut image_index,
                    ) {
                        Ok(vk::Result::Success) => {}
                        Ok(r) => Err(r)?,
                        Err(e) => Err(e)?,
                    }
                }
                Ok(r) => Err(r)?,
                Err(e) => Err(e)?,
            }
            self.fence.wait(None)?;
            self.cur_image = image_index;

            Ok(())
        }
    }

    fn present(&mut self, device: &Arc<Device>) -> anyhow::Result<()> {
        unsafe {
            let swapchains = [self.swap_chain.hnd.raw()];
            let image_indices = [self.cur_image];
            let info = vk::new::PresentInfoKHR {
                wait_semaphores: &self.present_signal_handles[..self.present_signals_pos],
                swapchains: &swapchains,
                image_indices: &image_indices,
            }
            .new();
            (device.sc).queue_present(device.queue, &info)?;
            self.present_signals_pos = 0;
            Ok(())
        }
    }

    pub fn alloc_present_signal(&mut self) -> anyhow::Result<vk::Semaphore> {
        if self.present_signals_pos >= self.present_signals.len() {
            self.present_signals
                .push(BinarySemaphore::new(&self.swap_chain.device)?);
            self.present_signal_handles
                .push(self.present_signals[self.present_signals_pos].hnd.raw());
        }
        let r = self.present_signals[self.present_signals_pos].hnd.raw();
        self.present_signals_pos += 1;
        Ok(r)
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Instance {
    pub vk: Vulkan,
    pub hnd: hnd::Instance,
    pub sf: hnd::Instance<vk::khr::surface>,
    #[cfg(windows)]
    pub sf_win32: hnd::Instance<vk::khr::win32_surface>,
    pub debug_utils: bool,
}

impl Deref for Instance {
    type Target = hnd::Instance;

    fn deref(&self) -> &Self::Target {
        &self.hnd
    }
}

impl Instance {
    fn new() -> anyhow::Result<Arc<Self>> {
        unsafe {
            let vk = Vulkan::new()?;
            let mut layers = vec![];
            vk.enumerate_instance_layer_properties(Some(&mut layers))?;
            let mut exts = vec![];
            vk.enumerate_instance_extension_properties(None, Some(&mut exts))?;

            log::info!(
                "Available instance layers: {:?}",
                layers
                    .iter()
                    .map(|a| CStr::from_ptr(&a.layer_name as _))
                    .collect::<Vec<_>>()
            );
            log::info!(
                "Available instance extensions: {:?}",
                exts.iter()
                    .map(|a| CStr::from_ptr(&a.extension_name as _))
                    .collect::<Vec<_>>()
            );

            let mut enable_layers = vec![];
            let validation = layers
                .iter()
                .any(|a| CStr::from_ptr(&a.layer_name as _) == c!("VK_LAYER_KHRONOS_validation"));
            if validation {
                enable_layers.push(c!("VK_LAYER_KHRONOS_validation"));
            }
            let mut enabled_exts = vec![];
            let debug_utils = exts
                .iter()
                .any(|a| CStr::from_ptr(&a.extension_name as _) == vk::ext::debug_utils);
            if debug_utils {
                enabled_exts.push(vk::ext::debug_utils::NAME);
            }
            if cfg!(windows) {
                enabled_exts.push(vk::khr::win32_surface::NAME);
            } else {
                panic!("not support platform")
            }
            enabled_exts.push(vk::khr::surface::NAME);
            log::info!("Enabled layers: {:?}", enable_layers);
            log::info!("Enabled extensions: {:?}", enabled_exts);

            let enable_layers = enable_layers.iter().map(|a| a.as_ptr()).collect::<Vec<_>>();
            let enabled_exts = enabled_exts.iter().map(|a| a.as_ptr()).collect::<Vec<_>>();

            let app_info = vk::new::ApplicationInfo {
                application_version: 0,
                engine_version: 0,
                api_version: vk::API_VERSION_1_4,
            }
            .new();
            let mut info = vk::new::InstanceCreateInfo {
                p_enabled_layer_names: &enable_layers,
                p_enabled_extension_names: &enabled_exts,
            }
            .application_info(&app_info);
            let mut dbg_info;
            if debug_utils {
                dbg_info = vk::new::DebugUtilsMessengerCreateInfoEXT {
                    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT::WarningExt
                        | vk::DebugUtilsMessageSeverityFlagsEXT::ErrorExt,
                    message_type: vk::DebugUtilsMessageTypeFlagsEXT::GeneralExt
                        | vk::DebugUtilsMessageTypeFlagsEXT::ValidationExt
                        | vk::DebugUtilsMessageTypeFlagsEXT::PerformanceExt,
                    pfn_user_callback: Self::debug_messenger_callback,
                }
                .new();
                info.push_next(&mut dbg_info);
            }
            let inst = vk.create_instance(&info)?.hnd(&vk);
            let sf = inst.ext::<vk::khr::surface>();
            #[cfg(windows)]
            let sf_win32 = inst.ext::<vk::khr::win32_surface>();
            Ok(Arc::new(Self {
                vk,
                hnd: inst,
                sf,
                #[cfg(windows)]
                sf_win32,
                debug_utils,
            }))
        }
    }

    unsafe extern "system" fn debug_messenger_callback(
        message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
        _message_types: vk::DebugUtilsMessageTypeFlagsEXT,
        callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
        _user_data: *mut void,
    ) -> vk::Bool {
        let level = match message_severity {
            vk::DebugUtilsMessageSeverityFlagsEXT::VerboseExt => log::Level::Debug,
            vk::DebugUtilsMessageSeverityFlagsEXT::WarningExt => log::Level::Warn,
            vk::DebugUtilsMessageSeverityFlagsEXT::ErrorExt => log::Level::Error,
            _ => log::Level::Info,
        };
        let msg = unsafe { CStr::from_ptr((*callback_data).p_message) }.to_string_lossy();
        log::log!(level, "{msg}");
        vk::FALSE
    }
}

#[derive(Debug)]
struct Device {
    pub inst: Arc<Instance>,
    pub adp: vk::PhysicalDevice,
    pub hnd: hnd::Device,
    pub sc: hnd::Device<vk::khr::swapchain>,
    pub main_queue_family: u32,
    pub queue: vk::Queue,
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe {
            _ = self.device_wait_idle();
        }
    }
}

impl Deref for Device {
    type Target = hnd::Device;

    fn deref(&self) -> &Self::Target {
        &self.hnd
    }
}

impl Device {
    fn new(inst: &Arc<Instance>, surface: &hnd::SurfaceKHR) -> anyhow::Result<Arc<Self>> {
        unsafe {
            let mut physical_devices = vec![];
            inst.enumerate_physical_devices(Some(&mut physical_devices))?;

            let physical_device = physical_devices
                .iter()
                .map(|a| a.unwrap())
                .flat_map(|physical_device| {
                    let mut priority = u32::MAX;
                    let props = inst.get_physical_device_properties(physical_device);

                    inst.sf
                        .get_physical_device_surface_support(physical_device, 0, surface.raw())
                        .ok()
                        .filter(|a| a.bool())?;

                    match props.device_type {
                        vk::PhysicalDeviceType::DiscreteGpu => priority = 0,
                        vk::PhysicalDeviceType::IntegratedGpu => priority = 1,
                        vk::PhysicalDeviceType::VirtualGpu => priority = 2,
                        vk::PhysicalDeviceType::Cpu => priority = 3,
                        vk::PhysicalDeviceType::Other => priority = 4,
                        _ => {}
                    }

                    Some((physical_device, priority, props))
                })
                .min_by_key(|(_, priority, _)| *priority);
            if physical_device.is_none() {
                Err(anyhow::anyhow!("No suitable physical device found"))?
            }

            let (adapter, _, props) = physical_device.unwrap();
            log::info!(
                "Select device ({:?}) {{ Type = {} }}",
                CStr::from_ptr(&props.device_name as _),
                props.device_type,
            );

            let mut exts = vec![];
            inst.enumerate_device_extension_properties(adapter, None, Some(&mut exts))?;
            let exts = exts
                .iter()
                .map(|a| CStr::from_ptr(&a.extension_name as _))
                .collect::<HashSet<_>>();
            log::info!("Available device extensions: {:?}", exts);

            let mut enabled_exts = vec![];
            enabled_exts.push(vk::khr::swapchain::NAME);

            log::info!("enabled device extensions: {:?}", enabled_exts);
            let enabled_exts = enabled_exts.iter().map(|a| a.as_ptr()).collect::<Vec<_>>();

            let mut queue_familys = vec![];
            inst.get_physical_device_queue_family_properties(adapter, Some(&mut queue_familys));
            let main_queue_family = queue_familys
                .iter()
                .position(|a| a.queue_flags.has_flags(vk::QueueFlags::Graphics))
                .expect("No queue family found") as u32;
            let queue_priorities = queue_familys
                .iter()
                .map(|a| {
                    (0..a.queue_count)
                        .into_iter()
                        .map(|_| 0.0)
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            let queue_infos = queue_priorities
                .iter()
                .enumerate()
                .map(|(i, a)| {
                    vk::new::DeviceQueueCreateInfo {
                        queue_family_index: i as u32,
                        queue_priorities: a,
                    }
                    .new()
                })
                .collect::<Vec<_>>();

            let mut features = vk::PhysicalDeviceFeatures2::default();
            let mut features_1_1 = vk::PhysicalDeviceVulkan11Features::default();
            let mut features_1_2 = vk::PhysicalDeviceVulkan12Features::default();
            let mut features_1_3 = vk::PhysicalDeviceVulkan13Features::default();
            let mut features_1_4 = vk::PhysicalDeviceVulkan14Features::default();
            features.push_next(&mut features_1_1);
            features.push_next(&mut features_1_2);
            features.push_next(&mut features_1_3);
            features.push_next(&mut features_1_4);
            inst.get_physical_device_features2(adapter, &mut features);

            features.p_next = std::ptr::null_mut();
            features.features.robust_buffer_access = vk::FALSE;

            features_1_1.multiview = vk::TRUE;
            features_1_2.timeline_semaphore = vk::TRUE;
            features_1_2.buffer_device_address = vk::TRUE;
            features_1_2.descriptor_indexing = vk::TRUE;
            features_1_2.shader_sampled_image_array_non_uniform_indexing = vk::TRUE;
            features_1_2.shader_storage_image_array_non_uniform_indexing = vk::TRUE;
            features_1_2.shader_storage_buffer_array_non_uniform_indexing = vk::TRUE;
            features_1_2.descriptor_binding_sampled_image_update_after_bind = vk::TRUE;
            features_1_2.descriptor_binding_storage_image_update_after_bind = vk::TRUE;
            features_1_2.descriptor_binding_storage_buffer_update_after_bind = vk::TRUE;
            features_1_2.descriptor_binding_partially_bound = vk::TRUE;
            features_1_3.robust_image_access = vk::FALSE;
            features_1_3.dynamic_rendering = vk::TRUE;
            features_1_3.synchronization2 = vk::TRUE;
            features_1_3.maintenance4 = vk::TRUE;
            features_1_4.pipeline_robustness = vk::FALSE;
            features_1_4.push_descriptor = vk::TRUE;

            let mut info = vk::DeviceCreateInfo::default()
                .queue_create_infos(&queue_infos)
                .p_enabled_extension_names(&enabled_exts)
                .enabled_features(&mut features.features);
            info.push_next(&mut features_1_1);
            info.push_next(&mut features_1_2);
            info.push_next(&mut features_1_3);
            info.push_next(&mut features_1_4);

            let device = inst.create_device(adapter, &info)?.hnd(&inst.hnd);
            let sc = device.ext::<vk::khr::swapchain>();
            let queue = device.get_queue(main_queue_family, 0);

            Ok(Arc::new(Self {
                inst: inst.clone(),
                adp: adapter,
                hnd: device,
                sc,
                main_queue_family,
                queue,
            }))
        }
    }
}

impl<'a> covk::HndCtx<vk::core, vk::Device> for &'a Device {
    type Ctx = hnd::Device;

    fn ctx(&self) -> Self::Ctx {
        self.hnd.clone()
    }

    fn raw(&self) -> vk::Device {
        self.hnd.raw()
    }

    fn commands(&self) -> &Arc<covk::commands::Device> {
        self.hnd.commands()
    }
}

#[derive(Debug)]
struct Fence {
    pub device: Arc<Device>,
    pub hnd: hnd::Fence,
}

impl Deref for Fence {
    type Target = hnd::Fence;

    fn deref(&self) -> &Self::Target {
        &self.hnd
    }
}

impl Fence {
    pub fn new(device: &Arc<Device>) -> anyhow::Result<Arc<Self>> {
        unsafe {
            let hnd = device
                .create_fence(&vk::FenceCreateInfo::default())?
                .hnd(&device.hnd);
            Ok(Arc::new(Self {
                device: device.clone(),
                hnd,
            }))
        }
    }
    pub unsafe fn wait(&self, timeout: Option<u64>) -> anyhow::Result<()> {
        unsafe {
            self.device
                .wait_for_fences(&[self.hnd.raw()], true, timeout.unwrap_or(u64::MAX))?;
            self.device.reset_fences(&[self.hnd.raw()])?;
            Ok(())
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct BinarySemaphore {
    pub device: Arc<Device>,
    pub hnd: hnd::Semaphore,
}

impl Deref for BinarySemaphore {
    type Target = hnd::Semaphore;

    fn deref(&self) -> &Self::Target {
        &self.hnd
    }
}

impl BinarySemaphore {
    pub fn new(device: &Arc<Device>) -> anyhow::Result<Arc<Self>> {
        unsafe {
            let hnd = device
                .create_semaphore(&vk::SemaphoreCreateInfo::default())?
                .hnd(&device.hnd);
            Ok(Arc::new(Self {
                device: device.clone(),
                hnd,
            }))
        }
    }
}

#[derive(Debug)]
struct TimelineSemaphore {
    pub device: Arc<Device>,
    pub hnd: hnd::Semaphore,
    pub value: AtomicU64,
}

impl Deref for TimelineSemaphore {
    type Target = hnd::Semaphore;

    fn deref(&self) -> &Self::Target {
        &self.hnd
    }
}

#[allow(dead_code)]
impl TimelineSemaphore {
    pub fn new(device: &Arc<Device>, init: u64) -> anyhow::Result<Arc<Self>> {
        unsafe {
            let mut info = vk::SemaphoreCreateInfo::default();
            let mut timeline_info = vk::SemaphoreTypeCreateInfo::default()
                .semaphore_type(vk::SemaphoreType::Timeline)
                .initial_value(init);
            info.push_next(&mut timeline_info);
            let hnd = device.create_semaphore(&info)?.hnd(&device.hnd);
            Ok(Arc::new(Self {
                device: device.clone(),
                hnd,
                value: AtomicU64::new(init),
            }))
        }
    }

    pub fn cur_value(&self) -> anyhow::Result<u64> {
        unsafe {
            let value = self.device.get_semaphore_counter_value(self.hnd.raw())?;
            Ok(value)
        }
    }

    #[inline]
    pub fn alloc_signal(&self) -> u64 {
        self.value
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
            + 1
    }

    pub unsafe fn send_signal(&self, signal: u64) -> anyhow::Result<()> {
        unsafe {
            self.device.signal_semaphore(
                &vk::new::SemaphoreSignalInfo {
                    semaphore: self.hnd.raw(),
                    value: signal,
                }
                .new(),
            )?;
            Ok(())
        }
    }

    pub unsafe fn wait_signal(&self, signal: u64, timeout: Option<u64>) -> anyhow::Result<()> {
        unsafe {
            self.device.wait_semaphores(
                &vk::new::SemaphoreWaitInfo {
                    semaphores: &[self.hnd.raw()],
                    values: &[signal],
                }
                .new(),
                timeout.unwrap_or(u64::MAX),
            )?;
            Ok(())
        }
    }
}

#[derive(Debug)]
struct CmdBuf {
    pub device: Arc<Device>,
    pub pool: hnd::CommandPool,
    pub buf: vk::CommandBuffer,
}

impl CoreCommandBuffer for CmdBuf {
    fn raw(&self) -> vk::CommandBuffer {
        self.buf
    }

    fn commands(&self) -> &covk::commands::Device {
        self.device.commands()
    }
}

impl Drop for CmdBuf {
    fn drop(&mut self) {
        unsafe {
            self.device
                .free_command_buffers(self.pool.raw(), &[self.buf]);
        }
    }
}

impl CmdBuf {
    fn new(device: &Arc<Device>) -> anyhow::Result<Self> {
        unsafe {
            let info = vk::new::CommandPoolCreateInfo {
                queue_family_index: device.main_queue_family,
            }
            .flags(vk::CommandPoolCreateFlags::Transient);
            let pool = device.create_command_pool(&info)?.hnd(&device.hnd);

            let info = vk::new::CommandBufferAllocateInfo {
                command_pool: pool.raw(),
                level: vk::CommandBufferLevel::Primary,
                command_buffer_count: 1,
            }
            .new();
            let mut buf = [None];
            device.allocate_command_buffers(&info, &mut buf)?;
            Ok(Self {
                device: device.clone(),
                pool,
                buf: buf[0].unwrap(),
            })
        }
    }
}

#[derive(Debug)]
struct SwapChain {
    pub device: Arc<Device>,
    pub surface: hnd::SurfaceKHR,
    pub hnd: hnd::SwapchainKHR,
    pub v_sync: bool,
    pub format: vk::Format,
    pub size: (u32, u32),
}

impl Drop for SwapChain {
    fn drop(&mut self) {
        unsafe {
            _ = self.device.queue_wait_idle(self.device.queue);
        }
    }
}

impl SwapChain {
    pub fn new(
        device: &Arc<Device>,
        surface: &hnd::SurfaceKHR,
        v_sync: bool,
    ) -> anyhow::Result<Box<Self>> {
        let (hnd, size, format) = Self::create(device, surface, v_sync, None)?;
        Ok(Box::new(Self {
            device: device.clone(),
            surface: surface.clone(),
            hnd,
            v_sync,
            size,
            format,
        }))
    }

    pub fn re_create(&mut self) -> anyhow::Result<()> {
        let (hnd, size, format) = Self::create(
            &self.device,
            &self.surface,
            self.v_sync,
            Some(self.hnd.raw()),
        )?;
        self.hnd = hnd;
        self.size = size;
        self.format = format;
        Ok(())
    }

    fn create(
        device: &Arc<Device>,
        surface: &hnd::SurfaceKHR,
        v_sync: bool,
        old: Option<vk::SwapchainKHR>,
    ) -> anyhow::Result<(hnd::SwapchainKHR, (u32, u32), vk::Format)> {
        unsafe {
            let inst = &*device.inst;
            let caps =
                (inst.sf).get_physical_device_surface_capabilities(device.adp, surface.raw())?;
            log::info!(
                "Surface size: ({}, {})",
                caps.current_extent.width,
                caps.current_extent.height
            );

            let mut formats = vec![];
            (inst.sf).get_physical_device_surface_formats(
                device.adp,
                Some(surface.raw()),
                Some(&mut formats),
            )?;
            log::info!(
                "Available surface formats: {:?}",
                formats.iter().map(|a| a.format).collect::<Vec<_>>()
            );
            if formats.is_empty() {
                Err(anyhow::anyhow!("No surface formats found"))?
            }
            let selected_format = formats
                .iter()
                .find(|a| {
                    matches!(
                        a.format,
                        vk::Format::R8G8B8A8_Srgb | vk::Format::B8G8R8A8_Srgb
                    )
                })
                .unwrap_or_else(|| &formats[0]);
            log::info!(
                "Selected format: {} ({})",
                selected_format.format,
                selected_format.color_space
            );

            let mut present_modes = vec![];
            (inst.sf).get_physical_device_surface_present_modes(
                device.adp,
                Some(surface.raw()),
                Some(&mut present_modes),
            )?;
            log::info!("Available present modes: {:?}", present_modes);
            let present_mode = if v_sync {
                vk::PresentModeKHR::FifoKhr
            } else {
                if present_modes.contains(&vk::PresentModeKHR::MailboxKhr) {
                    vk::PresentModeKHR::MailboxKhr
                } else if present_modes.contains(&vk::PresentModeKHR::ImmediateKhr) {
                    vk::PresentModeKHR::ImmediateKhr
                } else {
                    vk::PresentModeKHR::FifoKhr
                }
            };
            log::info!("Selected present modes {:?}", present_mode);
            let mut info = vk::new::SwapchainCreateInfoKHR {
                surface: surface.raw(),
                min_image_count: caps.min_image_count.max(3).min(caps.max_image_count),
                image_format: selected_format.format,
                image_color_space: selected_format.color_space,
                image_extent: caps.current_extent,
                image_array_layers: 1,
                image_usage: vk::ImageUsageFlags::ColorAttachment,
                image_sharing_mode: vk::SharingMode::Exclusive,
                queue_family_indices: &[],
                pre_transform: vk::SurfaceTransformFlagsKHR::IdentityKhr,
                composite_alpha: vk::CompositeAlphaFlagsKHR::OpaqueKhr,
                present_mode,
                clipped: true,
            }
            .new();
            if let Some(old) = old {
                info = info.old_swapchain(old);
            }
            let hnd = device
                .sc
                .create_swapchain(&info)?
                .hnd_with(&device.sc, || surface.clone());
            Ok((
                hnd,
                (caps.current_extent.width, caps.current_extent.height),
                selected_format.format,
            ))
        }
    }
}

#[allow(dead_code)]
struct ImageAsset {
    pub format: vk::Format,
    pub width: u32,
    pub height: u32,
    pub mips: Vec<Vec<u8>>,
}

impl Debug for ImageAsset {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ImageAsset")
            .field("format", &self.format)
            .field("width", &self.width)
            .field("height", &self.height)
            .field("mips", &self.mips.len())
            .finish()
    }
}

impl ImageAsset {
    pub fn load(data: &[u8]) -> anyhow::Result<Self> {
        let (format, vk_format) = if cfg!(windows) {
            (
                basis_universal::TranscoderTextureFormat::BC7_RGBA,
                vk::Format::BC7_Srgb_Block,
            )
        } else {
            panic!("Not support platform")
        };
        let data = &zstd::stream::decode_all(data)?;
        let mut transcoder = basis_universal::Transcoder::new();
        let info = transcoder
            .image_info(data, 0)
            .ok_or_else(|| anyhow::anyhow!("Failed to get image info"))?;
        let width = info.m_width;
        let height = info.m_height;
        let mut mips = Vec::with_capacity(info.m_total_levels as usize);
        transcoder
            .prepare_transcoding(data)
            .map_err(|e| anyhow::anyhow!("{e:?}"))?;
        for nth_mip in 0..info.m_total_levels {
            let buf = transcoder
                .transcode_image_level(
                    data,
                    format,
                    basis_universal::TranscodeParameters {
                        image_index: 0,
                        level_index: nth_mip,
                        decode_flags: None,
                        output_row_pitch_in_blocks_or_pixels: None,
                        output_rows_in_pixels: None,
                    },
                )
                .map_err(|e| anyhow::anyhow!("{e:?}"))?;
            mips.push(buf);
        }
        Ok(Self {
            format: vk_format,
            width,
            height,
            mips,
        })
    }
}

unsafe fn crate_desc_pool(
    device: &Arc<Device>,
    max_sets: u32,
) -> anyhow::Result<hnd::DescriptorPool> {
    unsafe {
        let pool_sizes = [
            vk::DescriptorPoolSize {
                typ: vk::DescriptorType::Sampler,
                descriptor_count: 1024 * max_sets,
            },
            vk::DescriptorPoolSize {
                typ: vk::DescriptorType::SampledImage,
                descriptor_count: 1_000_000 * max_sets,
            },
        ];
        let desc_pool = device
            .create_descriptor_pool(
                &vk::new::DescriptorPoolCreateInfo {
                    max_sets,
                    pool_sizes: &pool_sizes,
                }
                .flags(
                    vk::DescriptorPoolCreateFlags::UpdateAfterBind
                        | vk::DescriptorPoolCreateFlags::FreeDescriptorSet,
                ),
            )?
            .hnd(&device.hnd);
        Ok(desc_pool)
    }
}

#[derive(Debug)]
struct DescriptorSet {
    pub device: Arc<Device>,
    pub pool: hnd::DescriptorPool,
    pub layout: hnd::DescriptorSetLayout,
    pub hnd: vk::DescriptorSet,
}

impl Drop for DescriptorSet {
    fn drop(&mut self) {
        unsafe {
            _ = (self.device).free_descriptor_sets(self.pool.raw(), &[self.hnd]);
        }
    }
}

impl DescriptorSet {
    pub fn new(device: &Arc<Device>, pool: &hnd::DescriptorPool) -> anyhow::Result<Arc<Self>> {
        unsafe {
            let bindings = [
                vk::DescriptorSetLayoutBinding::default()
                    .binding(0)
                    .descriptor_type(vk::DescriptorType::Sampler)
                    .descriptor_count(1024)
                    .stage_flags(vk::ShaderStageFlags::All),
                vk::DescriptorSetLayoutBinding::default()
                    .binding(1)
                    .descriptor_type(vk::DescriptorType::SampledImage)
                    .descriptor_count(1_000_000)
                    .stage_flags(vk::ShaderStageFlags::All),
            ];
            let layout = device
                .create_descriptor_set_layout(
                    &vk::DescriptorSetLayoutCreateInfo::default()
                        .flags(vk::DescriptorSetLayoutCreateFlags::UpdateAfterBindPool)
                        .bindings(&bindings),
                )?
                .hnd(&device.hnd);
            let mut sets = [None];
            device.allocate_descriptor_sets(
                &vk::DescriptorSetAllocateInfo::default()
                    .descriptor_pool(pool.raw())
                    .set_layouts(&[layout.raw()]),
                &mut sets,
            )?;
            Ok(Arc::new(Self {
                device: device.clone(),
                pool: pool.clone(),
                layout,
                hnd: sets[0].unwrap(),
            }))
        }
    }
}

unsafe fn crate_pipeline_layout(
    device: &Arc<Device>,
    set: &hnd::DescriptorSetLayout,
) -> anyhow::Result<hnd::PipelineLayout> {
    unsafe {
        let set_layouts = [Some(set.raw())];
        let info = vk::PipelineLayoutCreateInfo::default()
            .set_layouts(&set_layouts)
            .push_constant_ranges(&[vk::PushConstantRange {
                stage_flags: vk::ShaderStageFlags::All,
                offset: 0,
                size: (2 * size_of::<vk::DeviceSize>()) as u32,
            }]);
        let layout = device
            .create_pipeline_layout(&info)?
            .hnd_with(&device.hnd, || set.clone());
        Ok(layout)
    }
}

unsafe fn load_shader(device: &Arc<Device>, bytes: &[u8]) -> anyhow::Result<hnd::ShaderModule> {
    unsafe {
        let shader = device
            .create_shader_module(
                &vk::new::ShaderModuleCreateInfo {
                    code_size: bytes.len(),
                    code: bytes.as_ptr() as *const u32,
                }
                .new(),
            )?
            .hnd(&device.hnd);
        Ok(shader)
    }
}

#[derive(Debug)]
struct Buffer {
    pub allocator: Arc<Allocator>,
    pub allocation: Option<gpu_allocator::vulkan::Allocation>,
    pub buffer: Option<hnd::Buffer>,
}

impl Drop for Buffer {
    fn drop(&mut self) {
        drop(self.buffer.take());
        if let Some(allocation) = self.allocation.take() {
            let mut alloc = self.allocator.inner.lock().unwrap();
            _ = alloc.free(allocation);
        }
    }
}

impl Buffer {
    pub fn device(&self) -> &Arc<Device> {
        &self.allocator.device
    }
    pub fn buffer(&self) -> &hnd::Buffer {
        self.buffer.as_ref().unwrap()
    }

    pub fn new(
        allocator: &Arc<Allocator>,
        name: &str,
        info: &vk::BufferCreateInfo,
        location: Option<gpu_allocator::MemoryLocation>,
    ) -> anyhow::Result<Arc<Self>> {
        use ash::vk::Handle;
        unsafe {
            let device = &*allocator.device;
            let buffer = device.create_buffer(info)?.unique(device);
            let mut alloc = allocator.inner.lock().expect("Lock failed");
            let req = device.get_buffer_memory_requirements(buffer.raw);
            let allocation = alloc.allocate(&gpu_allocator::vulkan::AllocationCreateDesc {
                name,
                requirements: std::mem::transmute(req),
                location: location.unwrap_or(gpu_allocator::MemoryLocation::Unknown),
                linear: true,
                allocation_scheme: gpu_allocator::vulkan::AllocationScheme::GpuAllocatorManaged,
            })?;
            device.bind_buffer_memory(
                buffer.raw,
                vk::DeviceMemory(NonZero::new_unchecked(allocation.memory().as_raw())),
                allocation.offset(),
            )?;
            let buffer = ManuallyDrop::new(buffer);
            Ok(Arc::new(Self {
                allocator: allocator.clone(),
                allocation: Some(allocation),
                buffer: Some(buffer.raw.hnd(&device.hnd)),
            }))
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Image {
    pub allocator: Arc<Allocator>,
    pub allocation: Option<gpu_allocator::vulkan::Allocation>,
    pub image: Option<hnd::Image>,
}

impl Drop for Image {
    fn drop(&mut self) {
        drop(self.image.take());
        if let Some(allocation) = self.allocation.take() {
            let mut alloc = self.allocator.inner.lock().unwrap();
            _ = alloc.free(allocation);
        }
    }
}

#[allow(dead_code)]
impl Image {
    pub fn device(&self) -> &Arc<Device> {
        &self.allocator.device
    }
    pub fn image(&self) -> &hnd::Image {
        self.image.as_ref().unwrap()
    }

    pub fn new(
        allocator: &Arc<Allocator>,
        name: &str,
        info: &vk::ImageCreateInfo,
        location: Option<gpu_allocator::MemoryLocation>,
    ) -> anyhow::Result<Arc<Self>> {
        use ash::vk::Handle;
        unsafe {
            let device = &*allocator.device;
            let image = device.create_image(info)?.unique(device);
            let mut alloc = allocator.inner.lock().expect("Lock failed");
            let req = device.get_image_memory_requirements(image.raw);
            let allocation = alloc.allocate(&gpu_allocator::vulkan::AllocationCreateDesc {
                name,
                requirements: std::mem::transmute(req),
                location: location.unwrap_or(gpu_allocator::MemoryLocation::Unknown),
                linear: true,
                allocation_scheme: gpu_allocator::vulkan::AllocationScheme::GpuAllocatorManaged,
            })?;
            device.bind_image_memory(
                image.raw,
                vk::DeviceMemory(NonZero::new_unchecked(allocation.memory().as_raw())),
                allocation.offset(),
            )?;
            let image = ManuallyDrop::new(image);
            Ok(Arc::new(Self {
                allocator: allocator.clone(),
                allocation: Some(allocation),
                image: Some(image.raw.hnd(&device.hnd)),
            }))
        }
    }
}

#[derive(Debug)]
struct Allocator {
    pub device: Arc<Device>,
    pub inner: Mutex<gpu_allocator::vulkan::Allocator>,
}

impl Allocator {
    pub fn new(inst: &Arc<Instance>, device: &Arc<Device>) -> anyhow::Result<Arc<Self>> {
        use ash::vk::Handle;
        let allocator = unsafe {
            gpu_allocator::vulkan::Allocator::new(&gpu_allocator::vulkan::AllocatorCreateDesc {
                instance: ash::Instance::load_with(
                    |name| std::mem::transmute(inst.get_proc_addr(name)),
                    ash::vk::Instance::from_raw(inst.hnd.raw().0.as_ptr() as u64),
                ),
                device: ash::Device::load_with(
                    |name| std::mem::transmute(device.get_proc_addr(name)),
                    ash::vk::Device::from_raw(device.hnd.raw().0.as_ptr() as u64),
                ),
                physical_device: ash::vk::PhysicalDevice::from_raw(device.adp.0.as_ptr() as u64),
                debug_settings: Default::default(),
                buffer_device_address: true,
                allocation_sizes: Default::default(),
            })?
        };
        Ok(Arc::new(Self {
            device: device.clone(),
            inner: Mutex::new(allocator),
        }))
    }

    pub fn alloc_buffer(
        self: &Arc<Allocator>,
        name: &str,
        info: &vk::BufferCreateInfo,
        location: Option<gpu_allocator::MemoryLocation>,
    ) -> anyhow::Result<Arc<Buffer>> {
        Buffer::new(self, name, info, location)
    }

    pub fn alloc_image(
        self: &Arc<Allocator>,
        name: &str,
        info: &vk::ImageCreateInfo,
        location: Option<gpu_allocator::MemoryLocation>,
    ) -> anyhow::Result<Arc<Image>> {
        Image::new(self, name, info, location)
    }
}

#[derive(Debug)]
struct UploadBuffer {
    pub allocator: Arc<Allocator>,
    pub chunks: VecDeque<UploadBufferChunk>,
}

#[derive(Debug)]
struct UploadBufferChunk {
    pub buffer: Arc<Buffer>,
    pub size: u64,
    pub used: u64,
}

impl UploadBuffer {
    pub fn new(allocator: &Arc<Allocator>) -> Self {
        Self {
            allocator: allocator.clone(),
            chunks: VecDeque::new(),
        }
    }

    pub fn recycle(&mut self) {
        while self.chunks.len() > 1 {
            _ = self.chunks.pop_front();
        }
        if let Some(chunk) = self.chunks.front_mut() {
            chunk.used = 0;
        }
    }

    const INIT_SIZE: u64 = 1024;

    pub fn alloc(&'_ mut self, size: u64, align: u64) -> anyhow::Result<UploadPosRef<'_>> {
        let chunks: *mut VecDeque<UploadBufferChunk> = (&mut self.chunks) as *mut _;
        for chunk in unsafe { &mut *chunks } {
            let start = align_up_u64(chunk.used, align);
            let end = start + size;
            if end > chunk.size {
                continue;
            }
            chunk.used = end;
            return Ok(UploadPosRef::new(&chunk.buffer, start, size));
        }

        let chunks = unsafe { &mut *chunks };
        let buf_size = chunks
            .back()
            .map(|a| a.size * 2)
            .unwrap_or(Self::INIT_SIZE)
            .max(size.next_power_of_two());

        let queue_family_indices = [self.allocator.device.main_queue_family];
        let buffer = Buffer::new(
            &self.allocator,
            "UploadBuffer",
            &vk::BufferCreateInfo::default()
                .size(buf_size)
                .usage(
                    vk::BufferUsageFlags::TransferSrc
                        | vk::BufferUsageFlags::UniformBuffer
                        | vk::BufferUsageFlags::ShaderDeviceAddress,
                )
                .queue_family_indices(&queue_family_indices),
            Some(gpu_allocator::MemoryLocation::CpuToGpu),
        )?;
        chunks.push_back(UploadBufferChunk {
            buffer,
            size: buf_size,
            used: size,
        });
        let chunk = chunks.back().unwrap();
        Ok(UploadPosRef::new(&chunk.buffer, 0, size))
    }

    pub fn write_bytes(&'_ mut self, align: u64, bytes: &[u8]) -> anyhow::Result<UploadPosRef<'_>> {
        let size = bytes.len() as u64;
        let pos = self.alloc(size, align)?;
        unsafe {
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), pos.data, bytes.len());
        }
        Ok(pos)
    }

    pub unsafe fn write_t<T>(
        &'_ mut self,
        align: u64,
        data: &T,
    ) -> anyhow::Result<UploadPosRef<'_>> {
        let bytes =
            unsafe { std::slice::from_raw_parts((data as *const T).cast::<u8>(), size_of::<T>()) };
        self.write_bytes(align, bytes)
    }
}

#[inline]
#[allow(dead_code)]
fn align_up_u64(value: u64, align: u64) -> u64 {
    debug_assert!(align.is_power_of_two());
    (value + align - 1) & !(align - 1)
}

#[derive(Debug)]
#[allow(dead_code)]
struct UploadPosRef<'a> {
    pub buffer: &'a Arc<Buffer>,
    pub offset: u64,
    pub data: *mut u8,
    pub size: u64,
}

impl<'a> UploadPosRef<'a> {
    pub fn new(buffer: &'a Arc<Buffer>, offset: u64, size: u64) -> Self {
        Self {
            buffer: &buffer,
            offset,
            data: unsafe {
                buffer
                    .allocation
                    .as_ref()
                    .unwrap()
                    .mapped_ptr()
                    .unwrap()
                    .cast::<u8>()
                    .add(offset as usize)
                    .as_ptr()
            },
            size,
        }
    }

    pub fn gpu_ptr(&self) -> vk::DeviceAddress {
        unsafe {
            let device = self.buffer.device();
            let addr = device.get_buffer_device_address(
                &vk::BufferDeviceAddressInfo::default().buffer(self.buffer.buffer().raw()),
            );
            addr + self.offset
        }
    }
}
