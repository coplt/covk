#![allow(unused_parens)]

use covk_sys as vk;
use covk_sys::loader::Vulkan;
use covk_sys::{Chainable, c};
use env_logger::Env;
use glam::{EulerRot, FloatExt, Mat4, Quat, Vec3A, Vec4};
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use std::collections::{HashSet, VecDeque};
use std::ffi::c_void;
use std::fmt::{Debug, Formatter};
use std::mem::ManuallyDrop;
use std::sync::Mutex;
use std::sync::atomic::AtomicU64;
use std::{ffi::CStr, sync::Arc};
use winit::dpi::{LogicalSize, PhysicalPosition};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

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
    pub vk_logo_view: ImageView<Arc<Image>>,
    pub linear_repeat_sampler: Sampler,
    pub vk_logo_hnd: u32,
    pub linear_repeat_sampler_hnd: u32,
    pub depth: Option<(u32, u32, ImageView<Arc<Image>>)>,
}

impl Resources {
    pub fn ensure_depth(
        &mut self,
        fc: &mut FrameContext,
        swap_chain: &SwapChain,
        allocator: &Arc<Allocator>,
    ) -> anyhow::Result<()> {
        if let Some((w, h, _)) = &self.depth {
            if (*w, *h) == swap_chain.size {
                return Ok(());
            }
        }
        let (w, h) = swap_chain.size;
        let img = allocator.alloc_image(
            "Depth",
            &vk::VkImageCreateInfo {
                imageType: vk::VK_IMAGE_TYPE_2D,
                format: vk::VK_FORMAT_D32_SFLOAT,
                extent: vk::VkExtent3D {
                    width: w,
                    height: h,
                    depth: 1,
                },
                mipLevels: 1,
                arrayLayers: 1,
                samples: vk::VK_SAMPLE_COUNT_1_BIT,
                usage: vk::VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT,
                sharingMode: vk::VK_SHARING_MODE_EXCLUSIVE,
                queueFamilyIndexCount: 1,
                pQueueFamilyIndices: &allocator.device.main_queue_family,
                initialLayout: vk::VK_IMAGE_LAYOUT_UNDEFINED,
                ..Default::default()
            },
            None,
        )?;
        let hnd = img.hnd;
        let view = ImageView::new(
            &allocator.device,
            || img,
            &vk::VkImageViewCreateInfo {
                image: hnd,
                viewType: vk::VK_IMAGE_VIEW_TYPE_2D,
                format: vk::VK_FORMAT_D32_SFLOAT,
                subresourceRange: vk::VkImageSubresourceRange {
                    aspectMask: vk::VK_IMAGE_ASPECT_DEPTH_BIT,
                    baseMipLevel: 0,
                    levelCount: 1,
                    baseArrayLayer: 0,
                    layerCount: 1,
                },
                ..Default::default()
            },
        )?;
        if let Some(old) = self.depth.replace((w, h, view)) {
            fc.free.push(Box::new(move || drop(old)));
        }
        Ok(())
    }

    pub fn depth(&self) -> &ImageView<Arc<Image>> {
        &self.depth.as_ref().unwrap().2
    }
}

impl RenderContext {
    pub fn init(&mut self, window: &Window, assets: Assets) -> anyhow::Result<()> {
        let cube_mesh = self.allocator.alloc_buffer(
            "Cube",
            &vk::VkBufferCreateInfo {
                size: CUBE_MESH.len() as u64,
                usage: vk::VK_BUFFER_USAGE_TRANSFER_DST_BIT
                    | vk::VK_BUFFER_USAGE_INDEX_BUFFER_BIT
                    | vk::VK_BUFFER_USAGE_VERTEX_BUFFER_BIT,
                sharingMode: vk::VK_SHARING_MODE_EXCLUSIVE,
                queueFamilyIndexCount: 1,
                pQueueFamilyIndices: &self.device.main_queue_family,
                ..Default::default()
            },
            None,
        )?;
        let vk_logo = self.allocator.alloc_image(
            "Vulkan Logo",
            &vk::VkImageCreateInfo {
                imageType: vk::VK_IMAGE_TYPE_2D,
                format: assets.vulkan_logo.format,
                extent: vk::VkExtent3D {
                    width: assets.vulkan_logo.width,
                    height: assets.vulkan_logo.height,
                    depth: 1,
                },
                mipLevels: assets.vulkan_logo.mips.len() as u32,
                arrayLayers: 1,
                samples: vk::VK_SAMPLE_COUNT_1_BIT,
                tiling: vk::VK_IMAGE_TILING_OPTIMAL,
                usage: vk::VK_IMAGE_USAGE_TRANSFER_DST_BIT | vk::VK_IMAGE_USAGE_SAMPLED_BIT,
                sharingMode: vk::VK_SHARING_MODE_EXCLUSIVE,
                queueFamilyIndexCount: 1,
                pQueueFamilyIndices: &self.device.main_queue_family,
                initialLayout: vk::VK_IMAGE_LAYOUT_UNDEFINED,
                ..Default::default()
            },
            None,
        )?;
        let vk_logo_view = ImageView::new(
            &self.device,
            || vk_logo.clone(),
            &vk::VkImageViewCreateInfo {
                image: vk_logo.hnd,
                viewType: vk::VK_IMAGE_VIEW_TYPE_2D,
                format: assets.vulkan_logo.format,
                subresourceRange: vk::VkImageSubresourceRange {
                    aspectMask: vk::VK_IMAGE_ASPECT_COLOR_BIT,
                    baseMipLevel: 0,
                    levelCount: assets.vulkan_logo.mips.len() as u32,
                    baseArrayLayer: 0,
                    layerCount: 1,
                },
                ..Default::default()
            },
        )?;
        let linear_repeat_sampler = Sampler::new(
            &self.device,
            &vk::VkSamplerCreateInfo {
                magFilter: vk::VK_FILTER_LINEAR,
                minFilter: vk::VK_FILTER_LINEAR,
                mipmapMode: vk::VK_SAMPLER_MIPMAP_MODE_NEAREST,
                addressModeU: vk::VK_SAMPLER_ADDRESS_MODE_REPEAT,
                addressModeV: vk::VK_SAMPLER_ADDRESS_MODE_REPEAT,
                addressModeW: vk::VK_SAMPLER_ADDRESS_MODE_REPEAT,
                minLod: 0.0,
                maxLod: f32::MAX,
                ..Default::default()
            },
        )?;
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

        unsafe {
            let device = &*self.device;
            let resources = self.resources.as_ref().unwrap();
            let writers = [
                vk::VkWriteDescriptorSet {
                    dstSet: self.desc_set.hnd,
                    dstBinding: 0,
                    dstArrayElement: 0,
                    descriptorCount: 1,
                    descriptorType: vk::VK_DESCRIPTOR_TYPE_SAMPLER,
                    pImageInfo: &vk::VkDescriptorImageInfo {
                        sampler: resources.linear_repeat_sampler.hnd,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                vk::VkWriteDescriptorSet {
                    dstSet: self.desc_set.hnd,
                    dstBinding: 1,
                    dstArrayElement: 0,
                    descriptorCount: 1,
                    descriptorType: vk::VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE,
                    pImageInfo: &vk::VkDescriptorImageInfo {
                        imageView: resources.vk_logo_view.hnd,
                        imageLayout: vk::VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ];
            (device._1_0).UpdateDescriptorSets(
                device.hnd,
                writers.len() as u32,
                writers.as_ptr(),
                0,
                std::ptr::null(),
            );
        }

        self.render(true, 0.0)?;
        window.set_visible(true);
        Ok(())
    }
    pub fn first_frame(&mut self, fc: &mut FrameContext) -> anyhow::Result<()> {
        unsafe {
            let assets = self.assets.as_ref().unwrap();
            let resources = self.resources.as_ref().unwrap();
            let device = &*fc.cmd.device;
            let cmd = fc.cmd.buf;

            // barrier
            {
                let img_barriers = [vk::VkImageMemoryBarrier2 {
                    srcStageMask: vk::VK_PIPELINE_STAGE_2_TOP_OF_PIPE_BIT,
                    srcAccessMask: vk::VK_ACCESS_2_NONE,
                    dstStageMask: vk::VK_PIPELINE_STAGE_2_TRANSFER_BIT,
                    dstAccessMask: vk::VK_ACCESS_2_TRANSFER_WRITE_BIT,
                    oldLayout: vk::VK_IMAGE_LAYOUT_UNDEFINED,
                    newLayout: vk::VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL,
                    srcQueueFamilyIndex: device.main_queue_family,
                    dstQueueFamilyIndex: device.main_queue_family,
                    image: resources.vk_logo.hnd,
                    subresourceRange: vk::VkImageSubresourceRange {
                        aspectMask: vk::VK_IMAGE_ASPECT_COLOR_BIT,
                        baseMipLevel: 0,
                        levelCount: assets.vulkan_logo.mips.len() as u32,
                        baseArrayLayer: 0,
                        layerCount: 1,
                    },
                    ..Default::default()
                }];
                let dep = vk::VkDependencyInfo {
                    imageMemoryBarrierCount: img_barriers.len() as u32,
                    pImageMemoryBarriers: img_barriers.as_ptr(),
                    ..Default::default()
                };
                (device._1_3).CmdPipelineBarrier2(cmd, &dep);
            }

            let cube_mesh_loc = fc.upload.write_bytes(4, CUBE_MESH)?;
            let cube_mesh_size = cube_mesh_loc.size;
            (device._1_0).CmdCopyBuffer(
                cmd,
                cube_mesh_loc.buffer.hnd,
                resources.cube_mesh.hnd,
                1,
                &vk::VkBufferCopy {
                    srcOffset: cube_mesh_loc.offset,
                    dstOffset: 0,
                    size: cube_mesh_size,
                },
            );

            for (mip, data) in assets.vulkan_logo.mips.iter().enumerate() {
                let loc = fc.upload.write_bytes(16, data)?;
                (device._1_0).CmdCopyBufferToImage(
                    cmd,
                    loc.buffer.hnd,
                    resources.vk_logo.hnd,
                    vk::VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL,
                    1,
                    &vk::VkBufferImageCopy {
                        bufferOffset: loc.offset,
                        bufferRowLength: 0,
                        bufferImageHeight: 0,
                        imageSubresource: vk::VkImageSubresourceLayers {
                            aspectMask: vk::VK_IMAGE_ASPECT_COLOR_BIT,
                            mipLevel: mip as u32,
                            baseArrayLayer: 0,
                            layerCount: 1,
                        },
                        imageOffset: Default::default(),
                        imageExtent: vk::VkExtent3D {
                            width: (assets.vulkan_logo.width >> mip).max(1),
                            height: (assets.vulkan_logo.height >> mip).max(1),
                            depth: 1,
                        },
                    },
                );
            }

            // barrier
            {
                let buffer_barriers = [vk::VkBufferMemoryBarrier2 {
                    srcStageMask: vk::VK_PIPELINE_STAGE_2_TRANSFER_BIT,
                    srcAccessMask: vk::VK_ACCESS_2_TRANSFER_WRITE_BIT,
                    dstStageMask: vk::VK_PIPELINE_STAGE_2_INDEX_INPUT_BIT
                        | vk::VK_PIPELINE_STAGE_2_VERTEX_ATTRIBUTE_INPUT_BIT,
                    dstAccessMask: vk::VK_ACCESS_2_INDEX_READ_BIT
                        | vk::VK_ACCESS_2_VERTEX_ATTRIBUTE_READ_BIT,
                    srcQueueFamilyIndex: device.main_queue_family,
                    dstQueueFamilyIndex: device.main_queue_family,
                    buffer: resources.cube_mesh.hnd,
                    offset: 0,
                    size: cube_mesh_size,
                    ..Default::default()
                }];
                let img_barriers = [vk::VkImageMemoryBarrier2 {
                    srcStageMask: vk::VK_PIPELINE_STAGE_2_TRANSFER_BIT,
                    srcAccessMask: vk::VK_ACCESS_2_TRANSFER_WRITE_BIT,
                    dstStageMask: vk::VK_PIPELINE_STAGE_2_FRAGMENT_SHADER_BIT,
                    dstAccessMask: vk::VK_ACCESS_2_SHADER_SAMPLED_READ_BIT,
                    oldLayout: vk::VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL,
                    newLayout: vk::VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL,
                    srcQueueFamilyIndex: device.main_queue_family,
                    dstQueueFamilyIndex: device.main_queue_family,
                    image: resources.vk_logo.hnd,
                    subresourceRange: vk::VkImageSubresourceRange {
                        aspectMask: vk::VK_IMAGE_ASPECT_COLOR_BIT,
                        baseMipLevel: 0,
                        levelCount: assets.vulkan_logo.mips.len() as u32,
                        baseArrayLayer: 0,
                        layerCount: 1,
                    },
                    ..Default::default()
                }];
                let dep = vk::VkDependencyInfo {
                    imageMemoryBarrierCount: img_barriers.len() as u32,
                    pImageMemoryBarriers: img_barriers.as_ptr(),
                    bufferMemoryBarrierCount: buffer_barriers.len() as u32,
                    pBufferMemoryBarriers: buffer_barriers.as_ptr(),
                    ..Default::default()
                };
                (device._1_3).CmdPipelineBarrier2(cmd, &dep);
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
            let cmd = fc.cmd.buf;

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
                    vk::VkImageMemoryBarrier2 {
                        srcStageMask: vk::VK_PIPELINE_STAGE_2_TOP_OF_PIPE_BIT,
                        srcAccessMask: vk::VK_ACCESS_2_NONE,
                        dstStageMask: vk::VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT,
                        dstAccessMask: vk::VK_ACCESS_2_COLOR_ATTACHMENT_WRITE_BIT,
                        oldLayout: vk::VK_IMAGE_LAYOUT_UNDEFINED,
                        newLayout: vk::VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL,
                        srcQueueFamilyIndex: device.main_queue_family,
                        dstQueueFamilyIndex: device.main_queue_family,
                        image: swap_chain_image,
                        subresourceRange: vk::VkImageSubresourceRange {
                            aspectMask: vk::VK_IMAGE_ASPECT_COLOR_BIT,
                            baseMipLevel: 0,
                            levelCount: 1,
                            baseArrayLayer: 0,
                            layerCount: 1,
                        },
                        ..Default::default()
                    },
                    vk::VkImageMemoryBarrier2 {
                        srcStageMask: vk::VK_PIPELINE_STAGE_2_TOP_OF_PIPE_BIT,
                        srcAccessMask: vk::VK_ACCESS_2_NONE,
                        dstStageMask: vk::VK_PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS_BIT
                            | vk::VK_PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT,
                        dstAccessMask: vk::VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_READ_BIT
                            | vk::VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT,
                        oldLayout: vk::VK_IMAGE_LAYOUT_UNDEFINED,
                        newLayout: vk::VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL,
                        srcQueueFamilyIndex: device.main_queue_family,
                        dstQueueFamilyIndex: device.main_queue_family,
                        image: depth.dep.hnd,
                        subresourceRange: vk::VkImageSubresourceRange {
                            aspectMask: vk::VK_IMAGE_ASPECT_DEPTH_BIT,
                            baseMipLevel: 0,
                            levelCount: 1,
                            baseArrayLayer: 0,
                            layerCount: 1,
                        },
                        ..Default::default()
                    },
                ];
                let dep = vk::VkDependencyInfo {
                    imageMemoryBarrierCount: img_barriers.len() as u32,
                    pImageMemoryBarriers: img_barriers.as_ptr(),
                    ..Default::default()
                };
                (device._1_3).CmdPipelineBarrier2(cmd, &dep);
            }

            // CmdBeginRendering
            {
                let color = vk::VkRenderingAttachmentInfo {
                    imageView: swap_chain_view.hnd,
                    imageLayout: vk::VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL,
                    loadOp: vk::VK_ATTACHMENT_LOAD_OP_CLEAR,
                    storeOp: vk::VK_ATTACHMENT_STORE_OP_STORE,
                    clearValue: vk::VkClearValue {
                        color: vk::VkClearColorValue {
                            float32: [0.3, 0.3, 0.3, 1.0],
                        },
                    },
                    ..Default::default()
                };
                let depth = vk::VkRenderingAttachmentInfo {
                    imageView: depth.hnd,
                    imageLayout: vk::VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL,
                    loadOp: vk::VK_ATTACHMENT_LOAD_OP_CLEAR,
                    storeOp: vk::VK_ATTACHMENT_STORE_OP_STORE,
                    clearValue: vk::VkClearValue {
                        depthStencil: vk::VkClearDepthStencilValue {
                            depth: 0.0,
                            stencil: 0,
                        },
                    },
                    ..Default::default()
                };
                let info = vk::VkRenderingInfo {
                    renderArea: vk::VkRect2D {
                        offset: Default::default(),
                        extent: vk::VkExtent2D { width, height },
                    },
                    layerCount: 0,
                    viewMask: 1,
                    colorAttachmentCount: 1,
                    pColorAttachments: &color,
                    pDepthAttachment: &depth,
                    ..Default::default()
                };
                (device._1_3).CmdBeginRendering(cmd, &info);
            }

            // set view
            {
                (device._1_0).CmdSetViewport(
                    cmd,
                    0,
                    1,
                    &vk::VkViewport {
                        x: 0.0,
                        y: height as f32,
                        width: width as f32,
                        height: -(height as f32),
                        minDepth: 0.0,
                        maxDepth: 1.0,
                    },
                );
                (device._1_0).CmdSetScissor(
                    cmd,
                    0,
                    1,
                    &vk::VkRect2D {
                        offset: Default::default(),
                        extent: vk::VkExtent2D { width, height },
                    },
                );
            }

            // draw
            {
                (device._1_0).CmdBindDescriptorSets(
                    cmd,
                    vk::VK_PIPELINE_BIND_POINT_GRAPHICS,
                    self.pipeline_cube.layout.hnd,
                    0,
                    1,
                    &self.desc_set.hnd,
                    0,
                    std::ptr::null(),
                );
                (device._1_0).CmdBindPipeline(
                    cmd,
                    vk::VK_PIPELINE_BIND_POINT_GRAPHICS,
                    self.pipeline_cube.hnd,
                );

                (device._1_0).CmdBindIndexBuffer(
                    cmd,
                    resources.cube_mesh.hnd,
                    0,
                    vk::VK_INDEX_TYPE_UINT16,
                );
                (device._1_0).CmdBindVertexBuffers(cmd, 0, 1, &resources.cube_mesh.hnd, &72);

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
                (device._1_0).CmdPushConstants(
                    cmd,
                    self.pipeline_cube.layout.hnd,
                    vk::VK_SHADER_STAGE_ALL,
                    0,
                    size_of::<Entry>() as u32,
                    &entry as *const Entry as *const c_void,
                );
                (device._1_0).CmdDrawIndexed(cmd, 36, 1, 0, 0, 0);
            }

            // CmdEndRendering
            {
                (device._1_3).CmdEndRendering(cmd);
            }

            // barrier
            {
                let img_barriers = [vk::VkImageMemoryBarrier2 {
                    srcStageMask: vk::VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT,
                    srcAccessMask: vk::VK_ACCESS_2_COLOR_ATTACHMENT_WRITE_BIT,
                    dstStageMask: vk::VK_PIPELINE_STAGE_2_BOTTOM_OF_PIPE_BIT,
                    dstAccessMask: vk::VK_ACCESS_2_NONE,
                    oldLayout: vk::VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL,
                    newLayout: vk::VK_IMAGE_LAYOUT_PRESENT_SRC_KHR,
                    srcQueueFamilyIndex: device.main_queue_family,
                    dstQueueFamilyIndex: device.main_queue_family,
                    image: swap_chain_image,
                    subresourceRange: vk::VkImageSubresourceRange {
                        aspectMask: vk::VK_IMAGE_ASPECT_COLOR_BIT,
                        baseMipLevel: 0,
                        levelCount: 1,
                        baseArrayLayer: 0,
                        layerCount: 1,
                    },
                    ..Default::default()
                }];
                let dep = vk::VkDependencyInfo {
                    imageMemoryBarrierCount: img_barriers.len() as u32,
                    pImageMemoryBarriers: img_barriers.as_ptr(),
                    ..Default::default()
                };
                (device._1_3).CmdPipelineBarrier2(cmd, &dep);
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
    pub surface: Arc<Surface>,
    pub device: Arc<Device>,
    pub allocator: Arc<Allocator>,
    pub desc_pool: Arc<DescriptorPool>,
    pub desc_set: Arc<DescriptorSet>,
    pub timeline: Arc<TimelineSemaphore>,
    pub fcs: VecDeque<Box<FrameContext>>,
    pub swap_chain: SwapChainContext,
    pub pipeline_layout: Arc<PipelineLayout>,
    pub shader_cube: Arc<ShaderModule>,
    pub pipeline_cube: Arc<Pipeline>,
}

impl Drop for RenderContext {
    fn drop(&mut self) {
        unsafe {
            (self.device._1_0).DeviceWaitIdle(self.device.hnd);
        }
    }
}

impl RenderContext {
    fn new(window: &Window) -> anyhow::Result<Self> {
        let inst = Instance::new()?;
        let surface = Surface::new(&inst, window)?;
        let device = Device::new(&inst, &surface)?;
        let allocator = Allocator::new(&inst, &device)?;
        let desc_pool = DescriptorPool::new(&device, 1)?;
        let desc_set = DescriptorSet::new(&desc_pool)?;
        let swap_chain = SwapChainContext::new(&device, &surface)?;
        let timeline = TimelineSemaphore::new(&device, 0)?;
        let pipeline_layout = PipelineLayout::new(&device, &desc_set.layout)?;
        let shader_cube = ShaderModule::new(&device, CUBE_SPV)?;
        let pipeline_cube =
            Self::create_cube_pipeline(&pipeline_layout, &shader_cube, &swap_chain.swap_chain)?;
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

    fn create_cube_pipeline(
        layout: &Arc<PipelineLayout>,
        module: &ShaderModule,
        swap_chain: &SwapChain,
    ) -> anyhow::Result<Arc<Pipeline>> {
        unsafe {
            let stages = [
                vk::VkPipelineShaderStageCreateInfo {
                    stage: vk::VK_SHADER_STAGE_VERTEX_BIT,
                    module: module.hnd,
                    pName: c!("Vertex").as_ptr(),
                    ..Default::default()
                },
                vk::VkPipelineShaderStageCreateInfo {
                    stage: vk::VK_SHADER_STAGE_FRAGMENT_BIT,
                    module: module.hnd,
                    pName: c!("Pixel").as_ptr(),
                    ..Default::default()
                },
            ];
            let dyn_states = [vk::VK_DYNAMIC_STATE_VIEWPORT, vk::VK_DYNAMIC_STATE_SCISSOR];
            let mut info = vk::VkGraphicsPipelineCreateInfo {
                stageCount: stages.len() as u32,
                pStages: stages.as_ptr(),
                pVertexInputState: &vk::VkPipelineVertexInputStateCreateInfo {
                    vertexBindingDescriptionCount: 1,
                    pVertexBindingDescriptions: [vk::VkVertexInputBindingDescription {
                        binding: 0,
                        stride: ((3 + 3 + 2) * size_of::<f32>()) as u32,
                        inputRate: vk::VK_VERTEX_INPUT_RATE_VERTEX,
                    }]
                    .as_ptr(),
                    vertexAttributeDescriptionCount: 3,
                    pVertexAttributeDescriptions: [
                        vk::VkVertexInputAttributeDescription {
                            location: 0,
                            binding: 0,
                            format: vk::VK_FORMAT_R32G32B32_SFLOAT,
                            offset: 0,
                        },
                        vk::VkVertexInputAttributeDescription {
                            location: 1,
                            binding: 0,
                            format: vk::VK_FORMAT_R32G32B32_SFLOAT,
                            offset: 12,
                        },
                        vk::VkVertexInputAttributeDescription {
                            location: 2,
                            binding: 0,
                            format: vk::VK_FORMAT_R32G32_SFLOAT,
                            offset: 24,
                        },
                    ]
                    .as_ptr(),
                    ..Default::default()
                },
                pInputAssemblyState: &vk::VkPipelineInputAssemblyStateCreateInfo {
                    topology: vk::VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST,
                    ..Default::default()
                },
                pViewportState: &vk::VkPipelineViewportStateCreateInfo {
                    viewportCount: 1,
                    pViewports: std::ptr::null(),
                    scissorCount: 1,
                    pScissors: std::ptr::null(),
                    ..Default::default()
                },
                pRasterizationState: &vk::VkPipelineRasterizationStateCreateInfo {
                    polygonMode: vk::VK_POLYGON_MODE_FILL,
                    cullMode: vk::VK_CULL_MODE_BACK_BIT,
                    frontFace: vk::VK_FRONT_FACE_CLOCKWISE,
                    ..Default::default()
                },
                pMultisampleState: &vk::VkPipelineMultisampleStateCreateInfo {
                    rasterizationSamples: vk::VK_SAMPLE_COUNT_1_BIT,
                    pSampleMask: std::ptr::null(),
                    ..Default::default()
                },
                pDepthStencilState: &vk::VkPipelineDepthStencilStateCreateInfo {
                    depthTestEnable: vk::VK_TRUE,
                    depthWriteEnable: vk::VK_TRUE,
                    depthCompareOp: vk::VK_COMPARE_OP_GREATER_OR_EQUAL,
                    depthBoundsTestEnable: vk::VK_TRUE,
                    stencilTestEnable: vk::VK_FALSE,
                    minDepthBounds: 0.0,
                    maxDepthBounds: 1.0,
                    ..Default::default()
                },
                pColorBlendState: &vk::VkPipelineColorBlendStateCreateInfo {
                    attachmentCount: 1,
                    pAttachments: &vk::VkPipelineColorBlendAttachmentState {
                        colorWriteMask: vk::VK_COLOR_COMPONENT_R_BIT
                            | vk::VK_COLOR_COMPONENT_G_BIT
                            | vk::VK_COLOR_COMPONENT_B_BIT,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                pDynamicState: &vk::VkPipelineDynamicStateCreateInfo {
                    dynamicStateCount: dyn_states.len() as u32,
                    pDynamicStates: dyn_states.as_ptr(),
                    ..Default::default()
                },
                layout: layout.hnd,
                ..Default::default()
            };
            let mut dyn_info = vk::VkPipelineRenderingCreateInfo {
                viewMask: 1,
                colorAttachmentCount: 1,
                pColorAttachmentFormats: &swap_chain.format,
                depthAttachmentFormat: vk::VK_FORMAT_D32_SFLOAT,
                ..Default::default()
            };
            info.push_next(&mut dyn_info);
            Pipeline::new_graphics(layout, &info)
        }
    }
}

#[allow(dead_code)]
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
            let r = (device._1_0).ResetCommandPool(device.hnd, self.cmd.pool, Default::default());
            if r != vk::VK_SUCCESS {
                Err(anyhow::anyhow!("ResetCommandPool failed: {r}"))?
            }
            let info = vk::VkCommandBufferBeginInfo {
                flags: vk::VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT,
                ..Default::default()
            };
            let r = (device._1_0).BeginCommandBuffer(self.cmd.buf, &info);
            if r != vk::VK_SUCCESS {
                Err(anyhow::anyhow!("BeginCommandBuffer failed: {r}"))?
            }
            Ok(())
        }
    }
    pub fn end(&mut self, ctx: &mut RenderContext) -> anyhow::Result<()> {
        unsafe {
            let device = &*self.cmd.device;
            let r = (device._1_0).EndCommandBuffer(self.cmd.buf);
            if r != vk::VK_SUCCESS {
                Err(anyhow::anyhow!("EndCommandBuffer failed: {r}"))?
            }
            let cmd_info = vk::VkCommandBufferSubmitInfo {
                commandBuffer: self.cmd.buf,
                deviceMask: 0,
                ..Default::default()
            };
            let signal = ctx.timeline.alloc_signal();
            let signal_infos = [
                vk::VkSemaphoreSubmitInfo {
                    semaphore: ctx.timeline.hnd,
                    value: signal,
                    stageMask: vk::VK_PIPELINE_STAGE_2_BOTTOM_OF_PIPE_BIT,
                    ..Default::default()
                },
                vk::VkSemaphoreSubmitInfo {
                    semaphore: ctx.swap_chain.alloc_present_signal()?,
                    value: 0,
                    stageMask: vk::VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT_KHR,
                    ..Default::default()
                },
            ];
            let submit = vk::VkSubmitInfo2 {
                waitSemaphoreInfoCount: 0,
                pWaitSemaphoreInfos: std::ptr::null(),
                commandBufferInfoCount: 1,
                pCommandBufferInfos: &cmd_info,
                signalSemaphoreInfoCount: signal_infos.len() as u32,
                pSignalSemaphoreInfos: signal_infos.as_ptr(),
                ..Default::default()
            };
            let r = (device._1_3).QueueSubmit2(device.queue, 1, &submit, vk::VkFence::null());
            if r != vk::VK_SUCCESS {
                Err(anyhow::anyhow!("QueueSubmit2 failed: {r}"))?
            }
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
#[allow(dead_code)]
struct SwapChainContext {
    pub fence: Arc<Fence>,
    pub swap_chain: Box<SwapChain>,
    pub images: Vec<vk::VkImage>,
    pub views: Vec<ImageView<()>>,
    pub present_signals: Vec<Arc<BinarySemaphore>>,
    pub present_signal_handles: Vec<vk::VkSemaphore>,
    pub present_signals_pos: usize,
    pub cur_image: u32,
}

impl SwapChainContext {
    pub fn new(device: &Arc<Device>, surface: &Arc<Surface>) -> anyhow::Result<Self> {
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
        images: &mut Vec<vk::VkImage>,
        views: &mut Vec<ImageView<()>>,
    ) -> anyhow::Result<()> {
        unsafe {
            let mut len = 0;
            device._sc.GetSwapchainImagesKHR(
                device.hnd,
                swap_chain.hnd,
                &mut len,
                std::ptr::null_mut(),
            );
            images.reserve(len as usize);
            device._sc.GetSwapchainImagesKHR(
                device.hnd,
                swap_chain.hnd,
                &mut len,
                images.as_mut_ptr().add(images.len()),
            );
            images.set_len(images.len() + len as usize);
            views.reserve(len as usize);
            for image in images {
                let view_info = vk::VkImageViewCreateInfo {
                    image: *image,
                    viewType: vk::VK_IMAGE_VIEW_TYPE_2D,
                    format: swap_chain.format,
                    subresourceRange: vk::VkImageSubresourceRange {
                        aspectMask: vk::VK_IMAGE_ASPECT_COLOR_BIT,
                        baseMipLevel: 0,
                        levelCount: 1,
                        baseArrayLayer: 0,
                        layerCount: 1,
                    },
                    ..Default::default()
                };
                views.push(ImageView::new(device, || (), &view_info)?)
            }
            Ok(())
        }
    }

    pub fn acquire_image(&mut self, device: &Arc<Device>) -> anyhow::Result<()> {
        unsafe {
            let mut image_index = 0;
            let r = (device._sc).AcquireNextImageKHR(
                device.hnd,
                self.swap_chain.hnd,
                u64::MAX,
                vk::VkSemaphore::null(),
                self.fence.hnd,
                &mut image_index,
            );
            match r {
                vk::VK_SUCCESS => {}
                vk::VK_SUBOPTIMAL_KHR | vk::VK_ERROR_OUT_OF_DATE_KHR => {
                    if r == vk::VK_SUBOPTIMAL_KHR {
                        self.fence.wait(None)?;
                    }
                    self.images.clear();
                    self.views.clear();
                    self.swap_chain.re_create()?;
                    Self::get_images(device, &self.swap_chain, &mut self.images, &mut self.views)?;
                    match (device._sc).AcquireNextImageKHR(
                        device.hnd,
                        self.swap_chain.hnd,
                        u64::MAX,
                        vk::VkSemaphore::null(),
                        self.fence.hnd,
                        &mut image_index,
                    ) {
                        vk::VK_SUCCESS => {}
                        r => Err(anyhow::anyhow!("AcquireNextImageKHR failed: {r}"))?,
                    }
                }
                r => Err(anyhow::anyhow!("AcquireNextImageKHR failed: {r}"))?,
            }
            self.fence.wait(None)?;
            self.cur_image = image_index;

            Ok(())
        }
    }

    pub fn present(&mut self, device: &Arc<Device>) -> anyhow::Result<()> {
        unsafe {
            let info = vk::VkPresentInfoKHR {
                waitSemaphoreCount: self.present_signals_pos as u32,
                pWaitSemaphores: self.present_signal_handles.as_ptr(),
                swapchainCount: 1,
                pSwapchains: &self.swap_chain.hnd,
                pImageIndices: &self.cur_image,
                ..Default::default()
            };
            match device._sc.QueuePresentKHR(device.queue, &info) {
                vk::VK_SUCCESS | vk::VK_SUBOPTIMAL_KHR | vk::VK_ERROR_OUT_OF_DATE_KHR => {}
                r => Err(anyhow::anyhow!("QueuePresentKHR failed: {r}"))?,
            }
            self.present_signals_pos = 0;
            Ok(())
        }
    }

    pub fn alloc_present_signal(&mut self) -> anyhow::Result<vk::VkSemaphore> {
        if self.present_signals_pos >= self.present_signals.len() {
            self.present_signals
                .push(BinarySemaphore::new(&self.swap_chain.device)?);
            self.present_signal_handles
                .push(self.present_signals[self.present_signals_pos].hnd);
        }
        let r = self.present_signals[self.present_signals_pos].hnd;
        self.present_signals_pos += 1;
        Ok(r)
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Instance {
    pub vk: Arc<Vulkan>,
    pub hnd: vk::VkInstance,
    pub _1_0: vk::vtbl::InstanceCommands_1_0,
    pub _1_1: vk::vtbl::InstanceCommands_1_1,
    pub _1_2: vk::vtbl::InstanceCommands_1_2,
    pub _1_3: vk::vtbl::InstanceCommands_1_3,
    pub _1_4: vk::vtbl::InstanceCommands_1_4,
    pub _sf: vk::khr::surface::InstanceCommands,
    #[cfg(windows)]
    pub _sf_win32: vk::khr::win32_surface::InstanceCommands,
    pub debug_utils: bool,
}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe {
            self._1_0.DestroyInstance(self.hnd, std::ptr::null());
        }
    }
}

impl Instance {
    fn new() -> anyhow::Result<Arc<Self>> {
        unsafe {
            let vk = Vulkan::new()?;

            let layers = {
                let mut len = 0;
                vk.EnumerateInstanceLayerProperties(&mut len, std::ptr::null_mut());
                let mut layers = Vec::with_capacity(len as usize);
                vk.EnumerateInstanceLayerProperties(&mut len, layers.as_mut_ptr());
                layers.set_len(len as usize);
                layers
            };
            let exts = {
                let mut len = 0;
                vk.EnumerateInstanceExtensionProperties(
                    std::ptr::null_mut(),
                    &mut len,
                    std::ptr::null_mut(),
                );
                let mut exts = Vec::with_capacity(len as usize);
                vk.EnumerateInstanceExtensionProperties(
                    std::ptr::null_mut(),
                    &mut len,
                    exts.as_mut_ptr(),
                );
                exts.set_len(len as usize);
                exts
            };
            log::info!(
                "Available instance layers: {:?}",
                layers
                    .iter()
                    .map(|a| CStr::from_ptr(&a.layerName as _))
                    .collect::<Vec<_>>()
            );
            log::info!(
                "Available instance extensions: {:?}",
                exts.iter()
                    .map(|a| CStr::from_ptr(&a.extensionName as _))
                    .collect::<Vec<_>>()
            );

            let mut enable_layers = vec![];
            let validation = layers
                .iter()
                .any(|a| CStr::from_ptr(&a.layerName as _) == c!("VK_LAYER_KHRONOS_validation"));
            if validation {
                enable_layers.push(c!("VK_LAYER_KHRONOS_validation"));
            }
            let mut enabled_exts = vec![];
            let debug_utils = exts
                .iter()
                .any(|a| CStr::from_ptr(&a.extensionName as _) == vk::ext::debug_utils::NAME);
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

            let app_info = vk::VkApplicationInfo {
                pApplicationName: c!("rust cube").as_ptr(),
                applicationVersion: 0,
                pEngineName: std::ptr::null(),
                engineVersion: 0,
                apiVersion: vk::VK_API_VERSION_1_4,
                ..Default::default()
            };
            let mut info = vk::VkInstanceCreateInfo {
                pApplicationInfo: &app_info,
                enabledLayerCount: enable_layers.len() as u32,
                ppEnabledLayerNames: enable_layers.as_ptr(),
                enabledExtensionCount: enabled_exts.len() as u32,
                ppEnabledExtensionNames: enabled_exts.as_ptr(),
                ..Default::default()
            };

            let mut dbg_info;
            if debug_utils {
                dbg_info = vk::VkDebugUtilsMessengerCreateInfoEXT {
                    messageSeverity: vk::VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT
                        | vk::VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT,
                    messageType: vk::VK_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT
                        | vk::VK_DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT
                        | vk::VK_DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT,
                    pfnUserCallback: Some(Self::debug_messenger_callback),
                    pUserData: std::ptr::null_mut(),
                    ..Default::default()
                };
                info.push_next(&mut dbg_info);
            }

            let mut inst = vk::VkInstance::null();
            let err = vk.CreateInstance(&info, std::ptr::null(), &mut inst);
            if err != vk::VK_SUCCESS {
                Err(anyhow::anyhow!("{err}"))?
            }

            let load_inst_proc_addr = |name: &CStr| vk.GetInstanceProcAddr(inst, name.as_ptr());
            let _1_0 = vk::vtbl::InstanceCommands_1_0::load(load_inst_proc_addr);
            let _1_1 = vk::vtbl::InstanceCommands_1_1::load(load_inst_proc_addr);
            let _1_2 = vk::vtbl::InstanceCommands_1_2::load(load_inst_proc_addr);
            let _1_3 = vk::vtbl::InstanceCommands_1_3::load(load_inst_proc_addr);
            let _1_4 = vk::vtbl::InstanceCommands_1_4::load(load_inst_proc_addr);

            let _sf = vk::khr::surface::InstanceCommands::load(load_inst_proc_addr);

            #[cfg(windows)]
            let _sf_win32 = vk::khr::win32_surface::InstanceCommands::load(load_inst_proc_addr);

            Ok(Arc::new(Self {
                vk,
                hnd: inst,
                _1_0,
                _1_1,
                _1_2,
                _1_3,
                _1_4,
                _sf,
                #[cfg(windows)]
                _sf_win32,
                debug_utils,
            }))
        }
    }

    unsafe extern "system" fn debug_messenger_callback(
        message_severity: vk::VkDebugUtilsMessageSeverityFlagsEXT,
        _message_types: vk::VkDebugUtilsMessageTypeFlagsEXT,
        p_callback_data: *const vk::VkDebugUtilsMessengerCallbackDataEXT,
        _p_user_data: *mut c_void,
    ) -> vk::VkBool32 {
        let level = match message_severity {
            vk::VK_DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT => log::Level::Debug,
            vk::VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT => log::Level::Warn,
            vk::VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT => log::Level::Error,
            _ => log::Level::Info,
        };
        let msg = unsafe { CStr::from_ptr((*p_callback_data).pMessage) }.to_string_lossy();
        log::log!(level, "{msg}");
        vk::VK_FALSE
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Surface {
    pub inst: Arc<Instance>,
    pub hnd: vk::VkSurfaceKHR,
}

impl Drop for Surface {
    fn drop(&mut self) {
        unsafe {
            (self.inst._sf).DestroySurfaceKHR(self.inst.hnd, self.hnd, std::ptr::null());
        }
    }
}

impl Surface {
    pub fn new(inst: &Arc<Instance>, window: &Window) -> anyhow::Result<Arc<Self>> {
        unsafe {
            Ok(Arc::new(Self {
                hnd: match window.window_handle()?.as_raw() {
                    #[cfg(windows)]
                    RawWindowHandle::Win32(hnd) => {
                        let mut surface = vk::VkSurfaceKHR::null();
                        let info = vk::VkWin32SurfaceCreateInfoKHR {
                            hwnd: hnd.hwnd.get(),
                            ..Default::default()
                        };
                        let r = inst._sf_win32.CreateWin32SurfaceKHR(
                            inst.hnd,
                            &info,
                            std::ptr::null(),
                            &mut surface,
                        );
                        if r != vk::VK_SUCCESS {
                            Err(anyhow::anyhow!("CreateWin32SurfaceKHR failed: {r}"))?
                        }
                        surface
                    }
                    _ => Err(anyhow::anyhow!("Platform not support"))?,
                },
                inst: inst.clone(),
            }))
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Device {
    pub inst: Arc<Instance>,
    pub adp: vk::VkPhysicalDevice,
    pub hnd: vk::VkDevice,
    pub _1_0: vk::vtbl::DeviceCommands_1_0,
    pub _1_1: vk::vtbl::DeviceCommands_1_1,
    pub _1_2: vk::vtbl::DeviceCommands_1_2,
    pub _1_3: vk::vtbl::DeviceCommands_1_3,
    pub _1_4: vk::vtbl::DeviceCommands_1_4,
    pub _sc: vk::khr::swapchain::DeviceCommands,
    pub main_queue_family: u32,
    pub queue: vk::VkQueue,
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe {
            (self._1_0).DeviceWaitIdle(self.hnd);
            (self._1_0).DestroyDevice(self.hnd, std::ptr::null());
        }
    }
}

impl Device {
    fn new(inst: &Arc<Instance>, surface: &Surface) -> anyhow::Result<Arc<Self>> {
        unsafe {
            let physical_devices = {
                let mut devices = Vec::new();
                let mut len = 0;
                (inst._1_0).EnumeratePhysicalDevices(inst.hnd, &mut len, std::ptr::null_mut());
                devices.reserve(len as usize);
                (inst._1_0).EnumeratePhysicalDevices(inst.hnd, &mut len, devices.as_mut_ptr());
                devices.set_len(len as usize);
                devices
            };
            assert!(physical_devices.len() > 0, "No physical devices found");

            let physical_device = physical_devices
                .iter()
                .flat_map(|physical_device| {
                    let mut priority = u32::MAX;
                    let mut props = vk::VkPhysicalDeviceProperties::default();
                    (inst._1_0).GetPhysicalDeviceProperties(*physical_device, &mut props);

                    let mut support = 0;
                    let r = inst._sf.GetPhysicalDeviceSurfaceSupportKHR(
                        *physical_device,
                        0,
                        surface.hnd,
                        &mut support,
                    );
                    if r != vk::VK_SUCCESS || support == vk::VK_FALSE {
                        return None;
                    }

                    match props.deviceType {
                        vk::VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU => priority = 0,
                        vk::VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU => priority = 1,
                        vk::VK_PHYSICAL_DEVICE_TYPE_VIRTUAL_GPU => priority = 2,
                        vk::VK_PHYSICAL_DEVICE_TYPE_CPU => priority = 3,
                        vk::VK_PHYSICAL_DEVICE_TYPE_OTHER => priority = 4,
                        _ => {}
                    }

                    Some((*physical_device, priority, props))
                })
                .min_by_key(|(_, priority, _)| *priority);
            if physical_device.is_none() {
                Err(anyhow::anyhow!("No suitable physical device found"))?
            }
            let (adapter, _, props) = physical_device.unwrap();
            log::info!(
                "Select device ({:?}) {{ Type = {} }}",
                CStr::from_ptr(&props.deviceName as _),
                props.deviceType,
            );

            let exts = {
                let mut len = 0;
                inst._1_0.EnumerateDeviceExtensionProperties(
                    adapter,
                    std::ptr::null(),
                    &mut len,
                    std::ptr::null_mut(),
                );
                let mut exts = Vec::with_capacity(len as usize);
                inst._1_0.EnumerateDeviceExtensionProperties(
                    adapter,
                    std::ptr::null(),
                    &mut len,
                    exts.as_mut_ptr(),
                );
                exts.set_len(len as usize);
                exts
            };
            let exts = exts
                .iter()
                .map(|a| CStr::from_ptr(&a.extensionName as _))
                .collect::<HashSet<_>>();
            log::info!("Available device extensions: {:?}", exts);

            let mut enabled_exts = vec![];
            enabled_exts.push(vk::khr::swapchain::NAME);

            log::info!("enabled device extensions: {:?}", enabled_exts);
            let enabled_exts = enabled_exts.iter().map(|a| a.as_ptr()).collect::<Vec<_>>();

            let queue_familys = {
                let mut len = 0;
                inst._1_0.GetPhysicalDeviceQueueFamilyProperties(
                    adapter,
                    &mut len,
                    std::ptr::null_mut(),
                );
                let mut queue_families = Vec::with_capacity(len as usize);
                inst._1_0.GetPhysicalDeviceQueueFamilyProperties(
                    adapter,
                    &mut len,
                    queue_families.as_mut_ptr(),
                );
                queue_families.set_len(len as usize);
                queue_families
            };
            let main_queue_family = queue_familys
                .iter()
                .position(|a| a.queueFlags.has_flags(vk::VK_QUEUE_GRAPHICS_BIT))
                .expect("No queue family found") as u32;

            let queue_priorities = queue_familys
                .iter()
                .map(|a| {
                    (0..a.queueCount)
                        .into_iter()
                        .map(|_| 0.0)
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            let queue_infos = queue_priorities
                .iter()
                .enumerate()
                .map(|(i, a)| vk::VkDeviceQueueCreateInfo {
                    queueFamilyIndex: i as u32,
                    queueCount: a.len() as u32,
                    pQueuePriorities: a.as_ptr(),
                    ..Default::default()
                })
                .collect::<Vec<_>>();

            let mut features = vk::VkPhysicalDeviceFeatures2::default();
            let mut features_1_1 = vk::VkPhysicalDeviceVulkan11Features::default();
            let mut features_1_2 = vk::VkPhysicalDeviceVulkan12Features::default();
            let mut features_1_3 = vk::VkPhysicalDeviceVulkan13Features::default();
            let mut features_1_4 = vk::VkPhysicalDeviceVulkan14Features::default();
            features.push_next(&mut features_1_1);
            features.push_next(&mut features_1_2);
            features.push_next(&mut features_1_3);
            features.push_next(&mut features_1_4);
            (inst._1_1).GetPhysicalDeviceFeatures2(adapter, &mut features);

            features.pNext = std::ptr::null_mut();
            features.features.robustBufferAccess = vk::VK_FALSE;

            features_1_1.multiview = vk::VK_TRUE;
            features_1_2.timelineSemaphore = vk::VK_TRUE;
            features_1_2.bufferDeviceAddress = vk::VK_TRUE;
            features_1_2.descriptorIndexing = vk::VK_TRUE;
            features_1_2.shaderSampledImageArrayNonUniformIndexing = vk::VK_TRUE;
            features_1_2.shaderStorageImageArrayNonUniformIndexing = vk::VK_TRUE;
            features_1_2.shaderStorageBufferArrayNonUniformIndexing = vk::VK_TRUE;
            features_1_2.descriptorBindingSampledImageUpdateAfterBind = vk::VK_TRUE;
            features_1_2.descriptorBindingStorageImageUpdateAfterBind = vk::VK_TRUE;
            features_1_2.descriptorBindingStorageBufferUpdateAfterBind = vk::VK_TRUE;
            features_1_2.descriptorBindingPartiallyBound = vk::VK_TRUE;
            features_1_3.robustImageAccess = vk::VK_FALSE;
            features_1_3.dynamicRendering = vk::VK_TRUE;
            features_1_3.synchronization2 = vk::VK_TRUE;
            features_1_3.maintenance4 = vk::VK_TRUE;
            features_1_4.pipelineRobustness = vk::VK_FALSE;
            features_1_4.pushDescriptor = vk::VK_TRUE;

            let mut info = vk::VkDeviceCreateInfo {
                queueCreateInfoCount: queue_infos.len() as u32,
                pQueueCreateInfos: queue_infos.as_ptr(),
                enabledExtensionCount: enabled_exts.len() as u32,
                ppEnabledExtensionNames: enabled_exts.as_ptr(),
                pEnabledFeatures: &features.features,
                ..Default::default()
            };
            info.push_next(&mut features_1_1);
            info.push_next(&mut features_1_2);
            info.push_next(&mut features_1_3);
            info.push_next(&mut features_1_4);
            let mut device = vk::VkDevice::null();
            let r = (inst._1_0).CreateDevice(adapter, &info, std::ptr::null(), &mut device);
            if r != vk::VK_SUCCESS {
                Err(anyhow::anyhow!("CreateDevice failed: {r}"))?
            }

            let get_dev_proc_addr = |name: &CStr| (inst._1_0).GetDeviceProcAddr(device, name);

            let _1_0 = vk::vtbl::DeviceCommands_1_0::load(get_dev_proc_addr);
            let _1_1 = vk::vtbl::DeviceCommands_1_1::load(get_dev_proc_addr);
            let _1_2 = vk::vtbl::DeviceCommands_1_2::load(get_dev_proc_addr);
            let _1_3 = vk::vtbl::DeviceCommands_1_3::load(get_dev_proc_addr);
            let _1_4 = vk::vtbl::DeviceCommands_1_4::load(get_dev_proc_addr);
            let _sc = vk::khr::swapchain::DeviceCommands::load(get_dev_proc_addr);

            let mut queue = vk::VkQueue::null();
            _1_0.GetDeviceQueue(device, main_queue_family, 0, &mut queue);

            Ok(Arc::new(Self {
                inst: inst.clone(),
                adp: adapter,
                hnd: device,
                _1_0,
                _1_1,
                _1_2,
                _1_3,
                _1_4,
                _sc,
                main_queue_family,
                queue,
            }))
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Fence {
    pub device: Arc<Device>,
    pub hnd: vk::VkFence,
}

impl Drop for Fence {
    fn drop(&mut self) {
        unsafe {
            (self.device._1_0).DestroyFence(self.device.hnd, self.hnd, std::ptr::null());
        }
    }
}

impl Fence {
    #[allow(dead_code)]
    pub fn new(device: &Arc<Device>) -> anyhow::Result<Arc<Self>> {
        unsafe {
            let mut hnd = vk::VkFence::null();
            let info = vk::VkFenceCreateInfo::default();
            let r = (device._1_0).CreateFence(device.hnd, &info, std::ptr::null(), &mut hnd);
            if r != vk::VK_SUCCESS {
                Err(anyhow::anyhow!("CreateFence failed: {r}"))?
            }

            Ok(Arc::new(Self {
                device: device.clone(),
                hnd,
            }))
        }
    }

    #[allow(dead_code)]
    pub unsafe fn wait(&self, timeout: Option<u64>) -> anyhow::Result<()> {
        unsafe {
            let r = self.device._1_0.WaitForFences(
                self.device.hnd,
                1,
                &self.hnd,
                vk::VK_TRUE,
                timeout.unwrap_or(u64::MAX),
            );
            if r != vk::VK_SUCCESS {
                Err(anyhow::anyhow!("WaitForFences failed: {r}"))?
            }
            let r = self.device._1_0.ResetFences(self.device.hnd, 1, &self.hnd);
            if r != vk::VK_SUCCESS {
                Err(anyhow::anyhow!("ResetFences failed: {r}"))?
            }
            Ok(())
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct BinarySemaphore {
    pub device: Arc<Device>,
    pub hnd: vk::VkSemaphore,
}

impl Drop for BinarySemaphore {
    fn drop(&mut self) {
        unsafe {
            (self.device._1_0).DestroySemaphore(self.device.hnd, self.hnd, std::ptr::null());
        }
    }
}

impl BinarySemaphore {
    #[allow(dead_code)]
    pub fn new(device: &Arc<Device>) -> anyhow::Result<Arc<Self>> {
        unsafe {
            let mut hnd = vk::VkSemaphore::null();
            let info = vk::VkSemaphoreCreateInfo::default();
            let r = (device._1_0).CreateSemaphore(device.hnd, &info, std::ptr::null(), &mut hnd);
            if r != vk::VK_SUCCESS {
                Err(anyhow::anyhow!("CreateSemaphore failed: {r}"))?
            }

            Ok(Arc::new(Self {
                device: device.clone(),
                hnd,
            }))
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct TimelineSemaphore {
    pub device: Arc<Device>,
    pub hnd: vk::VkSemaphore,
    pub value: AtomicU64,
}

impl Drop for TimelineSemaphore {
    fn drop(&mut self) {
        unsafe {
            _ = self.wait_signal(self.value.load(std::sync::atomic::Ordering::Relaxed), None);
            (self.device._1_0).DestroySemaphore(self.device.hnd, self.hnd, std::ptr::null());
        }
    }
}

impl TimelineSemaphore {
    #[allow(dead_code)]
    pub fn new(device: &Arc<Device>, init: u64) -> anyhow::Result<Arc<Self>> {
        unsafe {
            let mut hnd = vk::VkSemaphore::null();
            let mut info = vk::VkSemaphoreCreateInfo::default();
            let mut timeline_info = vk::VkSemaphoreTypeCreateInfo {
                semaphoreType: vk::VK_SEMAPHORE_TYPE_TIMELINE,
                initialValue: init,
                ..Default::default()
            };
            info.push_next(&mut timeline_info);
            let r = (device._1_0).CreateSemaphore(device.hnd, &info, std::ptr::null(), &mut hnd);
            if r != vk::VK_SUCCESS {
                Err(anyhow::anyhow!("CreateSemaphore failed: {r}"))?
            }

            Ok(Arc::new(Self {
                device: device.clone(),
                hnd,
                value: AtomicU64::new(init),
            }))
        }
    }

    pub fn cur_value(&self) -> anyhow::Result<u64> {
        unsafe {
            let mut value = 0;
            let r =
                (self.device._1_2).GetSemaphoreCounterValue(self.device.hnd, self.hnd, &mut value);
            if r != vk::VK_SUCCESS {
                return Err(anyhow::anyhow!("GetSemaphoreCounterValue failed: {r}"))?;
            }
            Ok(value)
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub fn alloc_signal(&self) -> u64 {
        self.value
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
            + 1
    }
    #[allow(dead_code)]
    pub unsafe fn send_signal(&self, signal: u64) -> anyhow::Result<()> {
        unsafe {
            let info = vk::VkSemaphoreSignalInfo {
                semaphore: self.hnd,
                value: signal,
                ..Default::default()
            };
            let r = self.device._1_2.SignalSemaphore(self.device.hnd, &info);
            if r != vk::VK_SUCCESS {
                Err(anyhow::anyhow!("SignalSemaphore failed: {r}"))?
            }
            Ok(())
        }
    }
    #[allow(dead_code)]
    pub unsafe fn wait_signal(&self, signal: u64, timeout: Option<u64>) -> anyhow::Result<()> {
        unsafe {
            let info = vk::VkSemaphoreWaitInfo {
                semaphoreCount: 1,
                pSemaphores: &self.hnd,
                pValues: &signal,
                ..Default::default()
            };
            let r = self.device._1_2.WaitSemaphores(
                self.device.hnd,
                &info,
                timeout.unwrap_or(u64::MAX),
            );
            if r != vk::VK_SUCCESS {
                Err(anyhow::anyhow!("WaitSemaphores failed: {r}"))?
            }
            Ok(())
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct CmdBuf {
    pub device: Arc<Device>,
    pub pool: vk::VkCommandPool,
    pub buf: vk::VkCommandBuffer,
}

impl Drop for CmdBuf {
    fn drop(&mut self) {
        unsafe {
            (self.device._1_0).FreeCommandBuffers(self.device.hnd, self.pool, 1, &self.buf);
            (self.device._1_0).DestroyCommandPool(self.device.hnd, self.pool, std::ptr::null());
        }
    }
}

impl CmdBuf {
    #[allow(dead_code)]
    fn new(device: &Arc<Device>) -> anyhow::Result<Self> {
        unsafe {
            let mut pool = vk::VkCommandPool::null();
            let info = vk::VkCommandPoolCreateInfo {
                queueFamilyIndex: device.main_queue_family,
                flags: vk::VK_COMMAND_POOL_CREATE_TRANSIENT_BIT,
                ..Default::default()
            };
            let r = (device._1_0).CreateCommandPool(device.hnd, &info, std::ptr::null(), &mut pool);
            if r != vk::VK_SUCCESS {
                Err(anyhow::anyhow!("CreateCommandPool failed: {r}"))?
            }

            let mut buf = vk::VkCommandBuffer::null();
            let info = vk::VkCommandBufferAllocateInfo {
                commandPool: pool,
                level: vk::VK_COMMAND_BUFFER_LEVEL_PRIMARY,
                commandBufferCount: 1,
                ..Default::default()
            };
            let r = (device._1_0).AllocateCommandBuffers(device.hnd, &info, &mut buf);
            if r != vk::VK_SUCCESS {
                (device._1_0).DestroyCommandPool(device.hnd, pool, std::ptr::null());
                Err(anyhow::anyhow!("AllocateCommandBuffers failed: {r}"))?
            }

            Ok(Self {
                device: device.clone(),
                pool,
                buf,
            })
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct SwapChain {
    pub device: Arc<Device>,
    pub surface: Arc<Surface>,
    pub hnd: vk::VkSwapchainKHR,
    pub v_sync: bool,
    pub format: vk::VkFormat,
    pub size: (u32, u32),
}

impl Drop for SwapChain {
    fn drop(&mut self) {
        unsafe {
            (self.device._1_0).QueueWaitIdle(self.device.queue);
            (self.device._sc).DestroySwapchainKHR(self.device.hnd, self.hnd, std::ptr::null());
        }
    }
}

impl SwapChain {
    pub fn new(
        device: &Arc<Device>,
        surface: &Arc<Surface>,
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
        unsafe {
            let (hnd, size, format) =
                Self::create(&self.device, &self.surface, self.v_sync, Some(self.hnd))?;
            (self.device._sc).DestroySwapchainKHR(self.device.hnd, self.hnd, std::ptr::null());
            self.hnd = hnd;
            self.size = size;
            self.format = format;
            Ok(())
        }
    }

    fn create(
        device: &Device,
        surface: &Surface,
        v_sync: bool,
        old: Option<vk::VkSwapchainKHR>,
    ) -> anyhow::Result<(vk::VkSwapchainKHR, (u32, u32), vk::VkFormat)> {
        unsafe {
            let inst = &*device.inst;
            let mut caps = vk::VkSurfaceCapabilitiesKHR::default();
            (inst._sf).GetPhysicalDeviceSurfaceCapabilitiesKHR(device.adp, surface.hnd, &mut caps);
            log::info!(
                "Surface size: ({}, {})",
                caps.currentExtent.width,
                caps.currentExtent.height
            );

            let formats = {
                let mut len = 0;
                (inst._sf).GetPhysicalDeviceSurfaceFormatsKHR(
                    device.adp,
                    surface.hnd,
                    &mut len,
                    std::ptr::null_mut(),
                );
                let mut formats = Vec::with_capacity(len as usize);
                (inst._sf).GetPhysicalDeviceSurfaceFormatsKHR(
                    device.adp,
                    surface.hnd,
                    &mut len,
                    formats.as_mut_ptr(),
                );
                formats.set_len(len as usize);
                formats
            };
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
                        vk::VK_FORMAT_R8G8B8A8_SRGB | vk::VK_FORMAT_B8G8R8A8_SRGB
                    )
                })
                .unwrap_or_else(|| &formats[0]);
            log::info!(
                "Selected format: {} ({})",
                selected_format.format,
                selected_format.colorSpace
            );

            let present_modes = {
                let mut len = 0;
                (inst._sf).GetPhysicalDeviceSurfacePresentModesKHR(
                    device.adp,
                    surface.hnd,
                    &mut len,
                    std::ptr::null_mut(),
                );
                let mut present_modes = Vec::with_capacity(len as usize);
                (inst._sf).GetPhysicalDeviceSurfacePresentModesKHR(
                    device.adp,
                    surface.hnd,
                    &mut len,
                    present_modes.as_mut_ptr(),
                );
                present_modes.set_len(len as usize);
                present_modes
            };
            log::info!("Available present modes: {:?}", present_modes);
            let present_mode = if v_sync {
                vk::VK_PRESENT_MODE_FIFO_KHR
            } else {
                if present_modes.contains(&vk::VK_PRESENT_MODE_MAILBOX_KHR) {
                    vk::VK_PRESENT_MODE_MAILBOX_KHR
                } else if present_modes.contains(&vk::VK_PRESENT_MODE_IMMEDIATE_KHR) {
                    vk::VK_PRESENT_MODE_IMMEDIATE_KHR
                } else {
                    vk::VK_PRESENT_MODE_FIFO_KHR
                }
            };
            log::info!("Selected present modes {:?}", present_mode);

            let mut hnd = vk::VkSwapchainKHR::null();
            let info = vk::VkSwapchainCreateInfoKHR {
                surface: surface.hnd,
                minImageCount: caps.minImageCount.max(3).min(caps.maxImageCount),
                imageFormat: selected_format.format,
                imageColorSpace: selected_format.colorSpace,
                imageExtent: caps.currentExtent,
                imageArrayLayers: 1,
                imageUsage: vk::VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT,
                imageSharingMode: vk::VK_SHARING_MODE_EXCLUSIVE,
                preTransform: vk::VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR,
                compositeAlpha: vk::VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR,
                presentMode: present_mode,
                clipped: vk::VK_TRUE,
                oldSwapchain: old.unwrap_or_default(),
                ..Default::default()
            };
            let r = (device._sc).CreateSwapchainKHR(device.hnd, &info, std::ptr::null(), &mut hnd);
            if r != vk::VK_SUCCESS {
                Err(anyhow::anyhow!("CreateSwapchainKHR failed: {r}"))?
            }

            Ok((
                hnd,
                (caps.currentExtent.width, caps.currentExtent.height),
                selected_format.format,
            ))
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct ImageView<Dep> {
    pub device: Arc<Device>,
    pub dep: Dep,
    pub hnd: vk::VkImageView,
}

impl<Dep> Drop for ImageView<Dep> {
    fn drop(&mut self) {
        unsafe {
            (self.device._1_0).DestroyImageView(self.device.hnd, self.hnd, std::ptr::null());
        }
    }
}

impl<Dep> ImageView<Dep> {
    pub fn new(
        device: &Arc<Device>,
        dep: impl FnOnce() -> Dep,
        info: &vk::VkImageViewCreateInfo,
    ) -> anyhow::Result<Self> {
        unsafe {
            let mut hnd = vk::VkImageView::null();
            let r = (device._1_0).CreateImageView(device.hnd, info, std::ptr::null(), &mut hnd);
            if r != vk::VK_SUCCESS {
                Err(anyhow::anyhow!("CreateImageView failed: {r}"))?
            }
            Ok(Self {
                device: device.clone(),
                dep: dep(),
                hnd,
            })
        }
    }
}

#[allow(dead_code)]
struct ImageAsset {
    pub format: vk::VkFormat,
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
                vk::VK_FORMAT_BC7_SRGB_BLOCK,
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

#[derive(Debug)]
#[allow(dead_code)]
struct ShaderModule {
    pub device: Arc<Device>,
    pub hnd: vk::VkShaderModule,
}

impl Drop for ShaderModule {
    fn drop(&mut self) {
        unsafe {
            (self.device._1_0).DestroyShaderModule(self.device.hnd, self.hnd, std::ptr::null());
        }
    }
}

impl ShaderModule {
    pub fn new(device: &Arc<Device>, bytes: &[u8]) -> anyhow::Result<Arc<Self>> {
        unsafe {
            let info = vk::VkShaderModuleCreateInfo {
                codeSize: bytes.len(),
                pCode: bytes.as_ptr() as *const u32,
                ..Default::default()
            };
            let mut hnd = vk::VkShaderModule::null();
            let r = (device._1_0).CreateShaderModule(device.hnd, &info, std::ptr::null(), &mut hnd);
            if r != vk::VK_SUCCESS {
                Err(anyhow::anyhow!("CreateShaderModule failed: {r}"))?
            }
            Ok(Arc::new(Self {
                device: device.clone(),
                hnd,
            }))
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct DescriptorPool {
    pub device: Arc<Device>,
    pub hnd: vk::VkDescriptorPool,
}

impl Drop for DescriptorPool {
    fn drop(&mut self) {
        unsafe {
            (self.device._1_0).DestroyDescriptorPool(self.device.hnd, self.hnd, std::ptr::null());
        }
    }
}

impl DescriptorPool {
    pub fn new(device: &Arc<Device>, max_sets: u32) -> anyhow::Result<Arc<Self>> {
        unsafe {
            let pool_sizes = [
                vk::VkDescriptorPoolSize {
                    r#type: vk::VK_DESCRIPTOR_TYPE_SAMPLER,
                    descriptorCount: 1024 * max_sets,
                },
                vk::VkDescriptorPoolSize {
                    r#type: vk::VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE,
                    descriptorCount: 1_000_000 * max_sets,
                },
            ];
            let info = vk::VkDescriptorPoolCreateInfo {
                flags: vk::VK_DESCRIPTOR_POOL_CREATE_UPDATE_AFTER_BIND_BIT
                    | vk::VK_DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT,
                maxSets: max_sets,
                poolSizeCount: pool_sizes.len() as u32,
                pPoolSizes: pool_sizes.as_ptr(),
                ..Default::default()
            };
            let mut hnd = vk::VkDescriptorPool::null();
            let r =
                (device._1_0).CreateDescriptorPool(device.hnd, &info, std::ptr::null(), &mut hnd);
            if r != vk::VK_SUCCESS {
                Err(anyhow::anyhow!("CreateDescriptorPool failed: {r}"))?
            }
            Ok(Arc::new(Self {
                device: device.clone(),
                hnd,
            }))
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct DescriptorSetLayout {
    pub device: Arc<Device>,
    pub hnd: vk::VkDescriptorSetLayout,
}

impl Drop for DescriptorSetLayout {
    fn drop(&mut self) {
        unsafe {
            (self.device._1_0).DestroyDescriptorSetLayout(
                self.device.hnd,
                self.hnd,
                std::ptr::null(),
            );
        }
    }
}

impl DescriptorSetLayout {
    pub fn new(
        device: &Arc<Device>,
        info: &vk::VkDescriptorSetLayoutCreateInfo,
    ) -> anyhow::Result<Arc<Self>> {
        unsafe {
            let mut hnd = vk::VkDescriptorSetLayout::null();
            let r = (device._1_0.CreateDescriptorSetLayout(
                device.hnd,
                info,
                std::ptr::null(),
                &mut hnd,
            ));
            if r != vk::VK_SUCCESS {
                Err(anyhow::anyhow!("CreateDescriptorSetLayout failed: {r}"))?
            }
            Ok(Arc::new(Self {
                device: device.clone(),
                hnd,
            }))
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct DescriptorSet {
    pub pool: Arc<DescriptorPool>,
    pub layout: Arc<DescriptorSetLayout>,
    pub hnd: vk::VkDescriptorSet,
}

impl Drop for DescriptorSet {
    fn drop(&mut self) {
        unsafe {
            (self.device()._1_0).FreeDescriptorSets(self.device().hnd, self.pool.hnd, 1, &self.hnd);
        }
    }
}

impl DescriptorSet {
    pub fn device(&self) -> &Arc<Device> {
        &self.pool.device
    }

    pub fn new(pool: &Arc<DescriptorPool>) -> anyhow::Result<Arc<Self>> {
        unsafe {
            let bindings = [
                vk::VkDescriptorSetLayoutBinding {
                    binding: 0,
                    descriptorType: vk::VK_DESCRIPTOR_TYPE_SAMPLER,
                    descriptorCount: 1024,
                    stageFlags: vk::VK_SHADER_STAGE_ALL,
                    pImmutableSamplers: std::ptr::null(),
                },
                vk::VkDescriptorSetLayoutBinding {
                    binding: 1,
                    descriptorType: vk::VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE,
                    descriptorCount: 1_000_000,
                    stageFlags: vk::VK_SHADER_STAGE_ALL,
                    pImmutableSamplers: std::ptr::null(),
                },
            ];
            let layout = DescriptorSetLayout::new(
                &pool.device,
                &vk::VkDescriptorSetLayoutCreateInfo {
                    flags: vk::VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT,
                    bindingCount: bindings.len() as u32,
                    pBindings: bindings.as_ptr(),
                    ..Default::default()
                },
            )?;
            let info = vk::VkDescriptorSetAllocateInfo {
                descriptorPool: pool.hnd,
                descriptorSetCount: 1,
                pSetLayouts: &layout.hnd,
                ..Default::default()
            };
            let mut hnd = vk::VkDescriptorSet::null();
            let r = (pool.device._1_0).AllocateDescriptorSets(pool.device.hnd, &info, &mut hnd);
            if r != vk::VK_SUCCESS {
                Err(anyhow::anyhow!("AllocateDescriptorSets failed: {r}"))?
            }
            Ok(Arc::new(Self {
                pool: pool.clone(),
                layout,
                hnd,
            }))
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct PipelineLayout {
    pub set: Arc<DescriptorSetLayout>,
    pub hnd: vk::VkPipelineLayout,
}

impl Drop for PipelineLayout {
    fn drop(&mut self) {
        unsafe {
            let device = self.device();
            (device._1_0).DestroyPipelineLayout(device.hnd, self.hnd, std::ptr::null());
        }
    }
}

impl PipelineLayout {
    pub fn new(device: &Arc<Device>, set: &Arc<DescriptorSetLayout>) -> anyhow::Result<Arc<Self>> {
        unsafe {
            let push_const = vk::VkPushConstantRange {
                stageFlags: vk::VK_SHADER_STAGE_ALL,
                offset: 0,
                size: (2 * size_of::<vk::VkDeviceSize>()) as u32,
            };
            let info = vk::VkPipelineLayoutCreateInfo {
                setLayoutCount: 1,
                pSetLayouts: &set.hnd,
                pushConstantRangeCount: 1,
                pPushConstantRanges: &push_const,
                ..Default::default()
            };
            let mut hnd = vk::VkPipelineLayout::null();
            let r =
                (device._1_0).CreatePipelineLayout(device.hnd, &info, std::ptr::null(), &mut hnd);
            if r != vk::VK_SUCCESS {
                Err(anyhow::anyhow!("CreatePipelineLayout failed: {r}"))?
            }
            Ok(Arc::new(Self {
                set: set.clone(),
                hnd,
            }))
        }
    }

    pub fn device(&self) -> &Arc<Device> {
        &self.set.device
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Pipeline {
    pub layout: Arc<PipelineLayout>,
    pub hnd: vk::VkPipeline,
}

impl Drop for Pipeline {
    fn drop(&mut self) {
        unsafe {
            let device = self.device();
            (device._1_0).DestroyPipeline(device.hnd, self.hnd, std::ptr::null());
        }
    }
}

impl Pipeline {
    pub fn device(&self) -> &Arc<Device> {
        self.layout.device()
    }

    pub fn new_graphics(
        layout: &Arc<PipelineLayout>,
        info: &vk::VkGraphicsPipelineCreateInfo,
    ) -> anyhow::Result<Arc<Self>> {
        unsafe {
            let mut hnd = vk::VkPipeline::null();
            let r = (layout.device()._1_0).CreateGraphicsPipelines(
                layout.device().hnd,
                vk::VkPipelineCache::null(),
                1,
                info,
                std::ptr::null(),
                &mut hnd,
            );
            if r != vk::VK_SUCCESS {
                Err(anyhow::anyhow!("CreateGraphicsPipelines failed: {r}"))?
            }
            Ok(Arc::new(Self {
                layout: layout.clone(),
                hnd,
            }))
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct BufferRaw {
    pub device: Arc<Device>,
    pub hnd: vk::VkBuffer,
}

impl Drop for BufferRaw {
    fn drop(&mut self) {
        unsafe {
            (self.device._1_0).DestroyBuffer(self.device.hnd, self.hnd, std::ptr::null());
        }
    }
}

impl BufferRaw {
    pub fn new(device: &Arc<Device>, info: &vk::VkBufferCreateInfo) -> anyhow::Result<Self> {
        unsafe {
            let mut hnd = vk::VkBuffer::null();
            let r = device
                ._1_0
                .CreateBuffer(device.hnd, info, std::ptr::null(), &mut hnd);
            if r != vk::VK_SUCCESS {
                Err(anyhow::anyhow!("CreateBuffer failed: {r}"))?
            }
            Ok(Self {
                device: device.clone(),
                hnd,
            })
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Buffer {
    pub allocator: Arc<Allocator>,
    pub allocation: Option<gpu_allocator::vulkan::Allocation>,
    pub hnd: vk::VkBuffer,
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            let device = self.device();
            (device._1_0).DestroyBuffer(device.hnd, self.hnd, std::ptr::null());
            if let Some(allocation) = self.allocation.take() {
                let mut alloc = self.allocator.inner.lock().unwrap();
                _ = alloc.free(allocation);
            }
        }
    }
}

impl Buffer {
    pub fn device(&self) -> &Arc<Device> {
        &self.allocator.device
    }

    pub fn new(
        allocator: &Arc<Allocator>,
        name: &str,
        info: &vk::VkBufferCreateInfo,
        location: Option<gpu_allocator::MemoryLocation>,
    ) -> anyhow::Result<Arc<Self>> {
        use ash::vk::Handle;
        unsafe {
            let device = &*allocator.device;
            let raw = BufferRaw::new(&allocator.device, info)?;
            let mut alloc = allocator.inner.lock().expect("Lock failed");
            let mut req = vk::VkMemoryRequirements::default();
            (device._1_0).GetBufferMemoryRequirements(device.hnd, raw.hnd, &mut req);
            let allocation = alloc.allocate(&gpu_allocator::vulkan::AllocationCreateDesc {
                name,
                requirements: std::mem::transmute(req),
                location: location.unwrap_or(gpu_allocator::MemoryLocation::Unknown),
                linear: true,
                allocation_scheme: gpu_allocator::vulkan::AllocationScheme::GpuAllocatorManaged,
            })?;
            let r = (device._1_0).BindBufferMemory(
                device.hnd,
                raw.hnd,
                vk::VkDeviceMemory(allocation.memory().as_raw()),
                allocation.offset(),
            );
            if r != vk::VK_SUCCESS {
                _ = alloc.free(allocation);
                return Err(anyhow::anyhow!("BindBufferMemory failed: {r}"))?;
            }
            let mut raw = ManuallyDrop::new(raw);
            std::ptr::drop_in_place(&mut raw.device);
            Ok(Arc::new(Self {
                allocator: allocator.clone(),
                allocation: Some(allocation),
                hnd: raw.hnd,
            }))
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct ImageRaw {
    pub device: Arc<Device>,
    pub hnd: vk::VkImage,
}

impl Drop for ImageRaw {
    fn drop(&mut self) {
        unsafe {
            (self.device._1_0).DestroyImage(self.device.hnd, self.hnd, std::ptr::null());
        }
    }
}

impl ImageRaw {
    pub fn new(device: &Arc<Device>, info: &vk::VkImageCreateInfo) -> anyhow::Result<Self> {
        unsafe {
            let mut hnd = vk::VkImage::null();
            let r = device
                ._1_0
                .CreateImage(device.hnd, info, std::ptr::null(), &mut hnd);
            if r != vk::VK_SUCCESS {
                Err(anyhow::anyhow!("CreateBuffer failed: {r}"))?
            }
            Ok(Self {
                device: device.clone(),
                hnd,
            })
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Image {
    pub allocator: Arc<Allocator>,
    pub allocation: Option<gpu_allocator::vulkan::Allocation>,
    pub hnd: vk::VkImage,
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe {
            let device = self.device();
            (device._1_0).DestroyImage(device.hnd, self.hnd, std::ptr::null());
            if let Some(allocation) = self.allocation.take() {
                let mut alloc = self.allocator.inner.lock().unwrap();
                _ = alloc.free(allocation);
            }
        }
    }
}

impl Image {
    pub fn device(&self) -> &Arc<Device> {
        &self.allocator.device
    }

    pub fn new(
        allocator: &Arc<Allocator>,
        name: &str,
        info: &vk::VkImageCreateInfo,
        location: Option<gpu_allocator::MemoryLocation>,
    ) -> anyhow::Result<Arc<Self>> {
        use ash::vk::Handle;
        unsafe {
            let device = &*allocator.device;
            let raw = ImageRaw::new(&allocator.device, info)?;
            let mut alloc = allocator.inner.lock().expect("Lock failed");
            let mut req = vk::VkMemoryRequirements::default();
            (device._1_0).GetImageMemoryRequirements(device.hnd, raw.hnd, &mut req);
            let allocation = alloc.allocate(&gpu_allocator::vulkan::AllocationCreateDesc {
                name,
                requirements: std::mem::transmute(req),
                location: location.unwrap_or(gpu_allocator::MemoryLocation::Unknown),
                linear: false,
                allocation_scheme: gpu_allocator::vulkan::AllocationScheme::GpuAllocatorManaged,
            })?;
            let r = (device._1_0).BindImageMemory(
                device.hnd,
                raw.hnd,
                vk::VkDeviceMemory(allocation.memory().as_raw()),
                allocation.offset(),
            );
            if r != vk::VK_SUCCESS {
                _ = alloc.free(allocation);
                return Err(anyhow::anyhow!("BindBufferMemory failed: {r}"))?;
            }
            let mut raw = ManuallyDrop::new(raw);
            std::ptr::drop_in_place(&mut raw.device);
            Ok(Arc::new(Self {
                allocator: allocator.clone(),
                allocation: Some(allocation),
                hnd: raw.hnd,
            }))
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
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
                    |name| std::mem::transmute(inst._1_0.GetInstanceProcAddr(inst.hnd, name)),
                    ash::vk::Instance::from_raw(inst.hnd.0 as u64),
                ),
                device: ash::Device::load_with(
                    |name| std::mem::transmute(inst._1_0.GetDeviceProcAddr(device.hnd, name)),
                    ash::vk::Device::from_raw(device.hnd.0 as u64),
                ),
                physical_device: ash::vk::PhysicalDevice::from_raw(device.adp.0 as u64),
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
        info: &vk::VkBufferCreateInfo,
        location: Option<gpu_allocator::MemoryLocation>,
    ) -> anyhow::Result<Arc<Buffer>> {
        Buffer::new(self, name, info, location)
    }

    pub fn alloc_image(
        self: &Arc<Allocator>,
        name: &str,
        info: &vk::VkImageCreateInfo,
        location: Option<gpu_allocator::MemoryLocation>,
    ) -> anyhow::Result<Arc<Image>> {
        Image::new(self, name, info, location)
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct UploadBuffer {
    pub allocator: Arc<Allocator>,
    pub chunks: VecDeque<UploadBufferChunk>,
}

#[derive(Debug)]
#[allow(dead_code)]
struct UploadBufferChunk {
    pub buffer: Arc<Buffer>,
    pub size: u64,
    pub used: u64,
}

#[allow(dead_code)]
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

        let buffer = Buffer::new(
            &self.allocator,
            "UploadBuffer",
            &vk::VkBufferCreateInfo {
                size: buf_size,
                usage: vk::VK_BUFFER_USAGE_TRANSFER_SRC_BIT
                    | vk::VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT
                    | vk::VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT,
                sharingMode: vk::VK_SHARING_MODE_EXCLUSIVE,
                queueFamilyIndexCount: 1,
                pQueueFamilyIndices: &self.allocator.device.main_queue_family,
                ..Default::default()
            },
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

    pub fn gpu_ptr(&self) -> vk::VkDeviceAddress {
        unsafe {
            let device = self.buffer.device();
            let addr = (device._1_2).GetBufferDeviceAddress(
                device.hnd,
                &vk::VkBufferDeviceAddressInfo {
                    buffer: self.buffer.hnd,
                    ..Default::default()
                },
            );
            addr + self.offset
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Sampler {
    pub device: Arc<Device>,
    pub hnd: vk::VkSampler,
}

impl Drop for Sampler {
    fn drop(&mut self) {
        unsafe {
            (self.device._1_0).DestroySampler(self.device.hnd, self.hnd, std::ptr::null());
        }
    }
}

impl Sampler {
    pub fn new(device: &Arc<Device>, info: &vk::VkSamplerCreateInfo) -> anyhow::Result<Self> {
        let mut hnd = vk::VkSampler::null();
        unsafe {
            let r = (device._1_0).CreateSampler(device.hnd, info, std::ptr::null(), &mut hnd);
            if r != vk::VK_SUCCESS {
                Err(anyhow::anyhow!("CreateSampler failed: {r}"))?
            }
            Ok(Self {
                device: device.clone(),
                hnd,
            })
        }
    }
}
