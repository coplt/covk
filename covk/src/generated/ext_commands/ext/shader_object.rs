// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_EXT_shader_object` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::ext::shader_object::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::ext::shader_object::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// void vkCmdBindShadersEXT(VkCommandBuffer commandBuffer, uint32_t stageCount, VkShaderStageFlagBits const* pStages, VkShaderEXT const* pShaders)
    /// ```
    pub unsafe fn cmd_bind_shaders(
        &self,
        command_buffer: vk::CommandBuffer,
        stages: &[ShaderStageFlags],
        shaders: Option<&[Option<vk::ShaderEXT>]>,
    ) -> () {
        unsafe {
            self.0.CmdBindShadersEXT(
                command_buffer.abi(), 
                stages.len() as _, 
                stages.abi(), 
                shaders.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdBindVertexBuffers2(VkCommandBuffer commandBuffer, uint32_t firstBinding, uint32_t bindingCount, VkBuffer const* pBuffers, VkDeviceSize const* pOffsets, VkDeviceSize const* pSizes, VkDeviceSize const* pStrides)
    /// ```
    pub unsafe fn cmd_bind_vertex_buffers2(
        &self,
        command_buffer: vk::CommandBuffer,
        first_binding: uint32_t,
        buffers: &[Option<vk::Buffer>],
        offsets: &[DeviceSize],
        sizes: Option<&[DeviceSize]>,
        strides: Option<&[DeviceSize]>,
    ) -> () {
        unsafe {
            self.0.CmdBindVertexBuffers2EXT(
                command_buffer.abi(), 
                first_binding.abi(), 
                buffers.len() as _, 
                buffers.abi(), 
                offsets.abi(), 
                sizes.abi(), 
                strides.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetAlphaToCoverageEnableEXT(VkCommandBuffer commandBuffer, VkBool32 alphaToCoverageEnable)
    /// ```
    pub unsafe fn cmd_set_alpha_to_coverage_enable(
        &self,
        command_buffer: vk::CommandBuffer,
        alpha_to_coverage_enable: bool,
    ) -> () {
        unsafe {
            self.0.CmdSetAlphaToCoverageEnableEXT(
                command_buffer.abi(), 
                alpha_to_coverage_enable.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetAlphaToOneEnableEXT(VkCommandBuffer commandBuffer, VkBool32 alphaToOneEnable)
    /// ```
    pub unsafe fn cmd_set_alpha_to_one_enable(
        &self,
        command_buffer: vk::CommandBuffer,
        alpha_to_one_enable: bool,
    ) -> () {
        unsafe {
            self.0.CmdSetAlphaToOneEnableEXT(
                command_buffer.abi(), 
                alpha_to_one_enable.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetColorBlendAdvancedEXT(VkCommandBuffer commandBuffer, uint32_t firstAttachment, uint32_t attachmentCount, VkColorBlendAdvancedEXT const* pColorBlendAdvanced)
    /// ```
    pub unsafe fn cmd_set_color_blend_advanced(
        &self,
        command_buffer: vk::CommandBuffer,
        first_attachment: uint32_t,
        color_blend_advanced: &[ColorBlendAdvancedEXT],
    ) -> () {
        unsafe {
            self.0.CmdSetColorBlendAdvancedEXT(
                command_buffer.abi(), 
                first_attachment.abi(), 
                color_blend_advanced.len() as _, 
                color_blend_advanced.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetColorBlendEnableEXT(VkCommandBuffer commandBuffer, uint32_t firstAttachment, uint32_t attachmentCount, VkBool32 const* pColorBlendEnables)
    /// ```
    pub unsafe fn cmd_set_color_blend_enable(
        &self,
        command_buffer: vk::CommandBuffer,
        first_attachment: uint32_t,
        color_blend_enables: &[Bool],
    ) -> () {
        unsafe {
            self.0.CmdSetColorBlendEnableEXT(
                command_buffer.abi(), 
                first_attachment.abi(), 
                color_blend_enables.len() as _, 
                color_blend_enables.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetColorBlendEquationEXT(VkCommandBuffer commandBuffer, uint32_t firstAttachment, uint32_t attachmentCount, VkColorBlendEquationEXT const* pColorBlendEquations)
    /// ```
    pub unsafe fn cmd_set_color_blend_equation(
        &self,
        command_buffer: vk::CommandBuffer,
        first_attachment: uint32_t,
        color_blend_equations: &[ColorBlendEquationEXT],
    ) -> () {
        unsafe {
            self.0.CmdSetColorBlendEquationEXT(
                command_buffer.abi(), 
                first_attachment.abi(), 
                color_blend_equations.len() as _, 
                color_blend_equations.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetColorWriteMaskEXT(VkCommandBuffer commandBuffer, uint32_t firstAttachment, uint32_t attachmentCount, VkColorComponentFlags const* pColorWriteMasks)
    /// ```
    pub unsafe fn cmd_set_color_write_mask(
        &self,
        command_buffer: vk::CommandBuffer,
        first_attachment: uint32_t,
        color_write_masks: &[ColorComponentFlags],
    ) -> () {
        unsafe {
            self.0.CmdSetColorWriteMaskEXT(
                command_buffer.abi(), 
                first_attachment.abi(), 
                color_write_masks.len() as _, 
                color_write_masks.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetConservativeRasterizationModeEXT(VkCommandBuffer commandBuffer, VkConservativeRasterizationModeEXT conservativeRasterizationMode)
    /// ```
    pub unsafe fn cmd_set_conservative_rasterization_mode(
        &self,
        command_buffer: vk::CommandBuffer,
        conservative_rasterization_mode: ConservativeRasterizationModeEXT,
    ) -> () {
        unsafe {
            self.0.CmdSetConservativeRasterizationModeEXT(
                command_buffer.abi(), 
                conservative_rasterization_mode.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetCoverageModulationModeNV(VkCommandBuffer commandBuffer, VkCoverageModulationModeNV coverageModulationMode)
    /// ```
    pub unsafe fn cmd_set_coverage_modulation_mode_nv(
        &self,
        command_buffer: vk::CommandBuffer,
        coverage_modulation_mode: CoverageModulationModeNV,
    ) -> () {
        unsafe {
            self.0.CmdSetCoverageModulationModeNV(
                command_buffer.abi(), 
                coverage_modulation_mode.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetCoverageModulationTableEnableNV(VkCommandBuffer commandBuffer, VkBool32 coverageModulationTableEnable)
    /// ```
    pub unsafe fn cmd_set_coverage_modulation_table_enable_nv(
        &self,
        command_buffer: vk::CommandBuffer,
        coverage_modulation_table_enable: bool,
    ) -> () {
        unsafe {
            self.0.CmdSetCoverageModulationTableEnableNV(
                command_buffer.abi(), 
                coverage_modulation_table_enable.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetCoverageModulationTableNV(VkCommandBuffer commandBuffer, uint32_t coverageModulationTableCount, float const* pCoverageModulationTable)
    /// ```
    pub unsafe fn cmd_set_coverage_modulation_table_nv(
        &self,
        command_buffer: vk::CommandBuffer,
        coverage_modulation_table: &[float],
    ) -> () {
        unsafe {
            self.0.CmdSetCoverageModulationTableNV(
                command_buffer.abi(), 
                coverage_modulation_table.len() as _, 
                coverage_modulation_table.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetCoverageReductionModeNV(VkCommandBuffer commandBuffer, VkCoverageReductionModeNV coverageReductionMode)
    /// ```
    pub unsafe fn cmd_set_coverage_reduction_mode_nv(
        &self,
        command_buffer: vk::CommandBuffer,
        coverage_reduction_mode: CoverageReductionModeNV,
    ) -> () {
        unsafe {
            self.0.CmdSetCoverageReductionModeNV(
                command_buffer.abi(), 
                coverage_reduction_mode.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetCoverageToColorEnableNV(VkCommandBuffer commandBuffer, VkBool32 coverageToColorEnable)
    /// ```
    pub unsafe fn cmd_set_coverage_to_color_enable_nv(
        &self,
        command_buffer: vk::CommandBuffer,
        coverage_to_color_enable: bool,
    ) -> () {
        unsafe {
            self.0.CmdSetCoverageToColorEnableNV(
                command_buffer.abi(), 
                coverage_to_color_enable.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetCoverageToColorLocationNV(VkCommandBuffer commandBuffer, uint32_t coverageToColorLocation)
    /// ```
    pub unsafe fn cmd_set_coverage_to_color_location_nv(
        &self,
        command_buffer: vk::CommandBuffer,
        coverage_to_color_location: uint32_t,
    ) -> () {
        unsafe {
            self.0.CmdSetCoverageToColorLocationNV(
                command_buffer.abi(), 
                coverage_to_color_location.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetCullMode(VkCommandBuffer commandBuffer, VkCullModeFlags cullMode)
    /// ```
    pub unsafe fn cmd_set_cull_mode(
        &self,
        command_buffer: vk::CommandBuffer,
        cull_mode: CullModeFlags,
    ) -> () {
        unsafe {
            self.0.CmdSetCullModeEXT(
                command_buffer.abi(), 
                cull_mode.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetDepthBiasEnable(VkCommandBuffer commandBuffer, VkBool32 depthBiasEnable)
    /// ```
    pub unsafe fn cmd_set_depth_bias_enable(
        &self,
        command_buffer: vk::CommandBuffer,
        depth_bias_enable: bool,
    ) -> () {
        unsafe {
            self.0.CmdSetDepthBiasEnableEXT(
                command_buffer.abi(), 
                depth_bias_enable.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetDepthBoundsTestEnable(VkCommandBuffer commandBuffer, VkBool32 depthBoundsTestEnable)
    /// ```
    pub unsafe fn cmd_set_depth_bounds_test_enable(
        &self,
        command_buffer: vk::CommandBuffer,
        depth_bounds_test_enable: bool,
    ) -> () {
        unsafe {
            self.0.CmdSetDepthBoundsTestEnableEXT(
                command_buffer.abi(), 
                depth_bounds_test_enable.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetDepthClampEnableEXT(VkCommandBuffer commandBuffer, VkBool32 depthClampEnable)
    /// ```
    pub unsafe fn cmd_set_depth_clamp_enable(
        &self,
        command_buffer: vk::CommandBuffer,
        depth_clamp_enable: bool,
    ) -> () {
        unsafe {
            self.0.CmdSetDepthClampEnableEXT(
                command_buffer.abi(), 
                depth_clamp_enable.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetDepthClampRangeEXT(VkCommandBuffer commandBuffer, VkDepthClampModeEXT depthClampMode, VkDepthClampRangeEXT const* pDepthClampRange)
    /// ```
    pub unsafe fn cmd_set_depth_clamp_range(
        &self,
        command_buffer: vk::CommandBuffer,
        depth_clamp_mode: DepthClampModeEXT,
        depth_clamp_range: Option<&DepthClampRangeEXT>,
    ) -> () {
        unsafe {
            self.0.CmdSetDepthClampRangeEXT(
                command_buffer.abi(), 
                depth_clamp_mode.abi(), 
                depth_clamp_range.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetDepthClipEnableEXT(VkCommandBuffer commandBuffer, VkBool32 depthClipEnable)
    /// ```
    pub unsafe fn cmd_set_depth_clip_enable(
        &self,
        command_buffer: vk::CommandBuffer,
        depth_clip_enable: bool,
    ) -> () {
        unsafe {
            self.0.CmdSetDepthClipEnableEXT(
                command_buffer.abi(), 
                depth_clip_enable.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetDepthClipNegativeOneToOneEXT(VkCommandBuffer commandBuffer, VkBool32 negativeOneToOne)
    /// ```
    pub unsafe fn cmd_set_depth_clip_negative_one_to_one(
        &self,
        command_buffer: vk::CommandBuffer,
        negative_one_to_one: bool,
    ) -> () {
        unsafe {
            self.0.CmdSetDepthClipNegativeOneToOneEXT(
                command_buffer.abi(), 
                negative_one_to_one.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetDepthCompareOp(VkCommandBuffer commandBuffer, VkCompareOp depthCompareOp)
    /// ```
    pub unsafe fn cmd_set_depth_compare_op(
        &self,
        command_buffer: vk::CommandBuffer,
        depth_compare_op: CompareOp,
    ) -> () {
        unsafe {
            self.0.CmdSetDepthCompareOpEXT(
                command_buffer.abi(), 
                depth_compare_op.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetDepthTestEnable(VkCommandBuffer commandBuffer, VkBool32 depthTestEnable)
    /// ```
    pub unsafe fn cmd_set_depth_test_enable(
        &self,
        command_buffer: vk::CommandBuffer,
        depth_test_enable: bool,
    ) -> () {
        unsafe {
            self.0.CmdSetDepthTestEnableEXT(
                command_buffer.abi(), 
                depth_test_enable.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetDepthWriteEnable(VkCommandBuffer commandBuffer, VkBool32 depthWriteEnable)
    /// ```
    pub unsafe fn cmd_set_depth_write_enable(
        &self,
        command_buffer: vk::CommandBuffer,
        depth_write_enable: bool,
    ) -> () {
        unsafe {
            self.0.CmdSetDepthWriteEnableEXT(
                command_buffer.abi(), 
                depth_write_enable.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetExtraPrimitiveOverestimationSizeEXT(VkCommandBuffer commandBuffer, float extraPrimitiveOverestimationSize)
    /// ```
    pub unsafe fn cmd_set_extra_primitive_overestimation_size(
        &self,
        command_buffer: vk::CommandBuffer,
        extra_primitive_overestimation_size: float,
    ) -> () {
        unsafe {
            self.0.CmdSetExtraPrimitiveOverestimationSizeEXT(
                command_buffer.abi(), 
                extra_primitive_overestimation_size.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetFrontFace(VkCommandBuffer commandBuffer, VkFrontFace frontFace)
    /// ```
    pub unsafe fn cmd_set_front_face(
        &self,
        command_buffer: vk::CommandBuffer,
        front_face: FrontFace,
    ) -> () {
        unsafe {
            self.0.CmdSetFrontFaceEXT(
                command_buffer.abi(), 
                front_face.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetLineRasterizationModeEXT(VkCommandBuffer commandBuffer, VkLineRasterizationModeEXT lineRasterizationMode)
    /// ```
    pub unsafe fn cmd_set_line_rasterization_mode(
        &self,
        command_buffer: vk::CommandBuffer,
        line_rasterization_mode: LineRasterizationModeEXT,
    ) -> () {
        unsafe {
            self.0.CmdSetLineRasterizationModeEXT(
                command_buffer.abi(), 
                line_rasterization_mode.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetLineStippleEnableEXT(VkCommandBuffer commandBuffer, VkBool32 stippledLineEnable)
    /// ```
    pub unsafe fn cmd_set_line_stipple_enable(
        &self,
        command_buffer: vk::CommandBuffer,
        stippled_line_enable: bool,
    ) -> () {
        unsafe {
            self.0.CmdSetLineStippleEnableEXT(
                command_buffer.abi(), 
                stippled_line_enable.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetLogicOpEXT(VkCommandBuffer commandBuffer, VkLogicOp logicOp)
    /// ```
    pub unsafe fn cmd_set_logic_op(
        &self,
        command_buffer: vk::CommandBuffer,
        logic_op: LogicOp,
    ) -> () {
        unsafe {
            self.0.CmdSetLogicOpEXT(
                command_buffer.abi(), 
                logic_op.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetLogicOpEnableEXT(VkCommandBuffer commandBuffer, VkBool32 logicOpEnable)
    /// ```
    pub unsafe fn cmd_set_logic_op_enable(
        &self,
        command_buffer: vk::CommandBuffer,
        logic_op_enable: bool,
    ) -> () {
        unsafe {
            self.0.CmdSetLogicOpEnableEXT(
                command_buffer.abi(), 
                logic_op_enable.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetPatchControlPointsEXT(VkCommandBuffer commandBuffer, uint32_t patchControlPoints)
    /// ```
    pub unsafe fn cmd_set_patch_control_points(
        &self,
        command_buffer: vk::CommandBuffer,
        patch_control_points: uint32_t,
    ) -> () {
        unsafe {
            self.0.CmdSetPatchControlPointsEXT(
                command_buffer.abi(), 
                patch_control_points.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetPolygonModeEXT(VkCommandBuffer commandBuffer, VkPolygonMode polygonMode)
    /// ```
    pub unsafe fn cmd_set_polygon_mode(
        &self,
        command_buffer: vk::CommandBuffer,
        polygon_mode: PolygonMode,
    ) -> () {
        unsafe {
            self.0.CmdSetPolygonModeEXT(
                command_buffer.abi(), 
                polygon_mode.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetPrimitiveRestartEnable(VkCommandBuffer commandBuffer, VkBool32 primitiveRestartEnable)
    /// ```
    pub unsafe fn cmd_set_primitive_restart_enable(
        &self,
        command_buffer: vk::CommandBuffer,
        primitive_restart_enable: bool,
    ) -> () {
        unsafe {
            self.0.CmdSetPrimitiveRestartEnableEXT(
                command_buffer.abi(), 
                primitive_restart_enable.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetPrimitiveTopology(VkCommandBuffer commandBuffer, VkPrimitiveTopology primitiveTopology)
    /// ```
    pub unsafe fn cmd_set_primitive_topology(
        &self,
        command_buffer: vk::CommandBuffer,
        primitive_topology: PrimitiveTopology,
    ) -> () {
        unsafe {
            self.0.CmdSetPrimitiveTopologyEXT(
                command_buffer.abi(), 
                primitive_topology.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetProvokingVertexModeEXT(VkCommandBuffer commandBuffer, VkProvokingVertexModeEXT provokingVertexMode)
    /// ```
    pub unsafe fn cmd_set_provoking_vertex_mode(
        &self,
        command_buffer: vk::CommandBuffer,
        provoking_vertex_mode: ProvokingVertexModeEXT,
    ) -> () {
        unsafe {
            self.0.CmdSetProvokingVertexModeEXT(
                command_buffer.abi(), 
                provoking_vertex_mode.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetRasterizationSamplesEXT(VkCommandBuffer commandBuffer, VkSampleCountFlagBits rasterizationSamples)
    /// ```
    pub unsafe fn cmd_set_rasterization_samples(
        &self,
        command_buffer: vk::CommandBuffer,
        rasterization_samples: SampleCountFlags,
    ) -> () {
        unsafe {
            self.0.CmdSetRasterizationSamplesEXT(
                command_buffer.abi(), 
                rasterization_samples.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetRasterizationStreamEXT(VkCommandBuffer commandBuffer, uint32_t rasterizationStream)
    /// ```
    pub unsafe fn cmd_set_rasterization_stream(
        &self,
        command_buffer: vk::CommandBuffer,
        rasterization_stream: uint32_t,
    ) -> () {
        unsafe {
            self.0.CmdSetRasterizationStreamEXT(
                command_buffer.abi(), 
                rasterization_stream.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetRasterizerDiscardEnable(VkCommandBuffer commandBuffer, VkBool32 rasterizerDiscardEnable)
    /// ```
    pub unsafe fn cmd_set_rasterizer_discard_enable(
        &self,
        command_buffer: vk::CommandBuffer,
        rasterizer_discard_enable: bool,
    ) -> () {
        unsafe {
            self.0.CmdSetRasterizerDiscardEnableEXT(
                command_buffer.abi(), 
                rasterizer_discard_enable.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetRepresentativeFragmentTestEnableNV(VkCommandBuffer commandBuffer, VkBool32 representativeFragmentTestEnable)
    /// ```
    pub unsafe fn cmd_set_representative_fragment_test_enable_nv(
        &self,
        command_buffer: vk::CommandBuffer,
        representative_fragment_test_enable: bool,
    ) -> () {
        unsafe {
            self.0.CmdSetRepresentativeFragmentTestEnableNV(
                command_buffer.abi(), 
                representative_fragment_test_enable.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetSampleLocationsEnableEXT(VkCommandBuffer commandBuffer, VkBool32 sampleLocationsEnable)
    /// ```
    pub unsafe fn cmd_set_sample_locations_enable(
        &self,
        command_buffer: vk::CommandBuffer,
        sample_locations_enable: bool,
    ) -> () {
        unsafe {
            self.0.CmdSetSampleLocationsEnableEXT(
                command_buffer.abi(), 
                sample_locations_enable.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetSampleMaskEXT(VkCommandBuffer commandBuffer, VkSampleCountFlagBits samples, VkSampleMask const* pSampleMask)
    /// ```
    pub unsafe fn cmd_set_sample_mask(
        &self,
        command_buffer: vk::CommandBuffer,
        samples: SampleCountFlags,
        sample_mask: *const SampleMask,
    ) -> () {
        unsafe {
            self.0.CmdSetSampleMaskEXT(
                command_buffer.abi(), 
                samples.abi(), 
                sample_mask.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetScissorWithCount(VkCommandBuffer commandBuffer, uint32_t scissorCount, VkRect2D const* pScissors)
    /// ```
    pub unsafe fn cmd_set_scissor_with_count(
        &self,
        command_buffer: vk::CommandBuffer,
        scissors: &[Rect2D],
    ) -> () {
        unsafe {
            self.0.CmdSetScissorWithCountEXT(
                command_buffer.abi(), 
                scissors.len() as _, 
                scissors.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetShadingRateImageEnableNV(VkCommandBuffer commandBuffer, VkBool32 shadingRateImageEnable)
    /// ```
    pub unsafe fn cmd_set_shading_rate_image_enable_nv(
        &self,
        command_buffer: vk::CommandBuffer,
        shading_rate_image_enable: bool,
    ) -> () {
        unsafe {
            self.0.CmdSetShadingRateImageEnableNV(
                command_buffer.abi(), 
                shading_rate_image_enable.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetStencilOp(VkCommandBuffer commandBuffer, VkStencilFaceFlags faceMask, VkStencilOp failOp, VkStencilOp passOp, VkStencilOp depthFailOp, VkCompareOp compareOp)
    /// ```
    pub unsafe fn cmd_set_stencil_op(
        &self,
        command_buffer: vk::CommandBuffer,
        face_mask: StencilFaceFlags,
        fail_op: StencilOp,
        pass_op: StencilOp,
        depth_fail_op: StencilOp,
        compare_op: CompareOp,
    ) -> () {
        unsafe {
            self.0.CmdSetStencilOpEXT(
                command_buffer.abi(), 
                face_mask.abi(), 
                fail_op.abi(), 
                pass_op.abi(), 
                depth_fail_op.abi(), 
                compare_op.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetStencilTestEnable(VkCommandBuffer commandBuffer, VkBool32 stencilTestEnable)
    /// ```
    pub unsafe fn cmd_set_stencil_test_enable(
        &self,
        command_buffer: vk::CommandBuffer,
        stencil_test_enable: bool,
    ) -> () {
        unsafe {
            self.0.CmdSetStencilTestEnableEXT(
                command_buffer.abi(), 
                stencil_test_enable.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetTessellationDomainOriginEXT(VkCommandBuffer commandBuffer, VkTessellationDomainOrigin domainOrigin)
    /// ```
    pub unsafe fn cmd_set_tessellation_domain_origin(
        &self,
        command_buffer: vk::CommandBuffer,
        domain_origin: TessellationDomainOrigin,
    ) -> () {
        unsafe {
            self.0.CmdSetTessellationDomainOriginEXT(
                command_buffer.abi(), 
                domain_origin.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetVertexInputEXT(VkCommandBuffer commandBuffer, uint32_t vertexBindingDescriptionCount, VkVertexInputBindingDescription2EXT const* pVertexBindingDescriptions, uint32_t vertexAttributeDescriptionCount, VkVertexInputAttributeDescription2EXT const* pVertexAttributeDescriptions)
    /// ```
    pub unsafe fn cmd_set_vertex_input(
        &self,
        command_buffer: vk::CommandBuffer,
        vertex_binding_descriptions: &[VertexInputBindingDescription2EXT],
        vertex_attribute_descriptions: &[VertexInputAttributeDescription2EXT],
    ) -> () {
        unsafe {
            self.0.CmdSetVertexInputEXT(
                command_buffer.abi(), 
                vertex_binding_descriptions.len() as _, 
                vertex_binding_descriptions.abi(), 
                vertex_attribute_descriptions.len() as _, 
                vertex_attribute_descriptions.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetViewportSwizzleNV(VkCommandBuffer commandBuffer, uint32_t firstViewport, uint32_t viewportCount, VkViewportSwizzleNV const* pViewportSwizzles)
    /// ```
    pub unsafe fn cmd_set_viewport_swizzle_nv(
        &self,
        command_buffer: vk::CommandBuffer,
        first_viewport: uint32_t,
        viewport_swizzles: &[ViewportSwizzleNV],
    ) -> () {
        unsafe {
            self.0.CmdSetViewportSwizzleNV(
                command_buffer.abi(), 
                first_viewport.abi(), 
                viewport_swizzles.len() as _, 
                viewport_swizzles.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetViewportWScalingEnableNV(VkCommandBuffer commandBuffer, VkBool32 viewportWScalingEnable)
    /// ```
    pub unsafe fn cmd_set_viewport_w_scaling_enable_nv(
        &self,
        command_buffer: vk::CommandBuffer,
        viewport_w_scaling_enable: bool,
    ) -> () {
        unsafe {
            self.0.CmdSetViewportWScalingEnableNV(
                command_buffer.abi(), 
                viewport_w_scaling_enable.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetViewportWithCount(VkCommandBuffer commandBuffer, uint32_t viewportCount, VkViewport const* pViewports)
    /// ```
    pub unsafe fn cmd_set_viewport_with_count(
        &self,
        command_buffer: vk::CommandBuffer,
        viewports: &[Viewport],
    ) -> () {
        unsafe {
            self.0.CmdSetViewportWithCountEXT(
                command_buffer.abi(), 
                viewports.len() as _, 
                viewports.abi(), 
            );
        }
    }
    /// ```c
    /// VkResult vkCreateShadersEXT(VkDevice device, uint32_t createInfoCount, VkShaderCreateInfoEXT const* pCreateInfos, VkAllocationCallbacks const* pAllocator, VkShaderEXT* pShaders)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::IncompatibleShaderBinaryExt]
    pub unsafe fn create_shaders(
        &self,
        device: vk::Device,
        create_infos: &[ShaderCreateInfoEXT],
        shaders: &mut [Option<vk::ShaderEXT>],
    ) -> crate::Result<Result> {
        unsafe {
            let _r = self.0.CreateShadersEXT(
                device.abi(), 
                create_infos.len() as _, 
                create_infos.abi(), 
                Default::default(), 
                shaders.abi(), 
            ).vk();
            _r.result(|| Some(_r))
        }
    }
    /// ```c
    /// void vkDestroyShaderEXT(VkDevice device, VkShaderEXT shader, VkAllocationCallbacks const* pAllocator)
    /// ```
    pub unsafe fn destroy_shader(
        &self,
        device: vk::Device,
        shader: Option<vk::ShaderEXT>,
    ) -> () {
        unsafe {
            self.0.DestroyShaderEXT(
                device.abi(), 
                shader.abi(), 
                Default::default(), 
            );
        }
    }
    /// ```c
    /// VkResult vkGetShaderBinaryDataEXT(VkDevice device, VkShaderEXT shader, size_t* pDataSize, void* pData)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    pub unsafe fn get_shader_binary_data(
        &self,
        device: vk::Device,
        shader: vk::ShaderEXT,
        data_size: *mut size_t,
        data: *mut void,
    ) -> crate::Result<Result> {
        unsafe {
            let _r = self.0.GetShaderBinaryDataEXT(
                device.abi(), 
                shader.abi(), 
                data_size.abi(), 
                data.abi(), 
            ).vk();
            _r.result(|| Some(_r))
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::ext::shader_object {
    type Commands = Device;
}

/// Device object
pub trait ExtShaderObjectDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkCreateShadersEXT(VkDevice device, uint32_t createInfoCount, VkShaderCreateInfoEXT const* pCreateInfos, VkAllocationCallbacks const* pAllocator, VkShaderEXT* pShaders)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::IncompatibleShaderBinaryExt]
    unsafe fn create_shaders(
        &self,
        create_infos: &[ShaderCreateInfoEXT],
        shaders: &mut [Option<vk::ShaderEXT>],
    ) -> crate::Result<Result> {
        unsafe {
            self.commands().create_shaders(
                self.raw(),
                create_infos,
                shaders,
            )
        }
    }
    /// ```c
    /// void vkDestroyShaderEXT(VkDevice device, VkShaderEXT shader, VkAllocationCallbacks const* pAllocator)
    /// ```
    unsafe fn destroy_shader(
        &self,
        shader: Option<vk::ShaderEXT>,
    ) -> () {
        unsafe {
            self.commands().destroy_shader(
                self.raw(),
                shader,
            )
        }
    }
    /// ```c
    /// VkResult vkGetShaderBinaryDataEXT(VkDevice device, VkShaderEXT shader, size_t* pDataSize, void* pData)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    unsafe fn get_shader_binary_data(
        &self,
        shader: vk::ShaderEXT,
        data_size: *mut size_t,
        data: *mut void,
    ) -> crate::Result<Result> {
        unsafe {
            self.commands().get_shader_binary_data(
                self.raw(),
                shader,
                data_size,
                data,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::ext::shader_object {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::ext::shader_object> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::ext::shader_object {
        type Output = crate::hnd::Device<vk::extensions::ext::shader_object>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::ext::shader_object>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::ext::shader_object> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::ext::shader_object> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::ext::shader_object> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::ext::shader_object> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::ExtShaderObjectDevice for crate::hnd::Device<vk::extensions::ext::shader_object> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::ext::shader_object, vk::Device> for crate::hnd::Device<vk::extensions::ext::shader_object> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

/// CommandBuffer object
pub trait ExtShaderObjectCommandBuffer {
    fn raw(&self) -> vk::CommandBuffer;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkCmdBindShadersEXT(VkCommandBuffer commandBuffer, uint32_t stageCount, VkShaderStageFlagBits const* pStages, VkShaderEXT const* pShaders)
    /// ```
    unsafe fn bind_shaders(
        &self,
        stages: &[ShaderStageFlags],
        shaders: Option<&[Option<vk::ShaderEXT>]>,
    ) -> () {
        unsafe {
            self.commands().cmd_bind_shaders(
                self.raw(),
                stages,
                shaders,
            )
        }
    }
    /// ```c
    /// void vkCmdBindVertexBuffers2(VkCommandBuffer commandBuffer, uint32_t firstBinding, uint32_t bindingCount, VkBuffer const* pBuffers, VkDeviceSize const* pOffsets, VkDeviceSize const* pSizes, VkDeviceSize const* pStrides)
    /// ```
    unsafe fn bind_vertex_buffers2(
        &self,
        first_binding: uint32_t,
        buffers: &[Option<vk::Buffer>],
        offsets: &[DeviceSize],
        sizes: Option<&[DeviceSize]>,
        strides: Option<&[DeviceSize]>,
    ) -> () {
        unsafe {
            self.commands().cmd_bind_vertex_buffers2(
                self.raw(),
                first_binding,
                buffers,
                offsets,
                sizes,
                strides,
            )
        }
    }
    /// ```c
    /// void vkCmdSetAlphaToCoverageEnableEXT(VkCommandBuffer commandBuffer, VkBool32 alphaToCoverageEnable)
    /// ```
    unsafe fn set_alpha_to_coverage_enable(
        &self,
        alpha_to_coverage_enable: bool,
    ) -> () {
        unsafe {
            self.commands().cmd_set_alpha_to_coverage_enable(
                self.raw(),
                alpha_to_coverage_enable,
            )
        }
    }
    /// ```c
    /// void vkCmdSetAlphaToOneEnableEXT(VkCommandBuffer commandBuffer, VkBool32 alphaToOneEnable)
    /// ```
    unsafe fn set_alpha_to_one_enable(
        &self,
        alpha_to_one_enable: bool,
    ) -> () {
        unsafe {
            self.commands().cmd_set_alpha_to_one_enable(
                self.raw(),
                alpha_to_one_enable,
            )
        }
    }
    /// ```c
    /// void vkCmdSetColorBlendAdvancedEXT(VkCommandBuffer commandBuffer, uint32_t firstAttachment, uint32_t attachmentCount, VkColorBlendAdvancedEXT const* pColorBlendAdvanced)
    /// ```
    unsafe fn set_color_blend_advanced(
        &self,
        first_attachment: uint32_t,
        color_blend_advanced: &[ColorBlendAdvancedEXT],
    ) -> () {
        unsafe {
            self.commands().cmd_set_color_blend_advanced(
                self.raw(),
                first_attachment,
                color_blend_advanced,
            )
        }
    }
    /// ```c
    /// void vkCmdSetColorBlendEnableEXT(VkCommandBuffer commandBuffer, uint32_t firstAttachment, uint32_t attachmentCount, VkBool32 const* pColorBlendEnables)
    /// ```
    unsafe fn set_color_blend_enable(
        &self,
        first_attachment: uint32_t,
        color_blend_enables: &[Bool],
    ) -> () {
        unsafe {
            self.commands().cmd_set_color_blend_enable(
                self.raw(),
                first_attachment,
                color_blend_enables,
            )
        }
    }
    /// ```c
    /// void vkCmdSetColorBlendEquationEXT(VkCommandBuffer commandBuffer, uint32_t firstAttachment, uint32_t attachmentCount, VkColorBlendEquationEXT const* pColorBlendEquations)
    /// ```
    unsafe fn set_color_blend_equation(
        &self,
        first_attachment: uint32_t,
        color_blend_equations: &[ColorBlendEquationEXT],
    ) -> () {
        unsafe {
            self.commands().cmd_set_color_blend_equation(
                self.raw(),
                first_attachment,
                color_blend_equations,
            )
        }
    }
    /// ```c
    /// void vkCmdSetColorWriteMaskEXT(VkCommandBuffer commandBuffer, uint32_t firstAttachment, uint32_t attachmentCount, VkColorComponentFlags const* pColorWriteMasks)
    /// ```
    unsafe fn set_color_write_mask(
        &self,
        first_attachment: uint32_t,
        color_write_masks: &[ColorComponentFlags],
    ) -> () {
        unsafe {
            self.commands().cmd_set_color_write_mask(
                self.raw(),
                first_attachment,
                color_write_masks,
            )
        }
    }
    /// ```c
    /// void vkCmdSetConservativeRasterizationModeEXT(VkCommandBuffer commandBuffer, VkConservativeRasterizationModeEXT conservativeRasterizationMode)
    /// ```
    unsafe fn set_conservative_rasterization_mode(
        &self,
        conservative_rasterization_mode: ConservativeRasterizationModeEXT,
    ) -> () {
        unsafe {
            self.commands().cmd_set_conservative_rasterization_mode(
                self.raw(),
                conservative_rasterization_mode,
            )
        }
    }
    /// ```c
    /// void vkCmdSetCoverageModulationModeNV(VkCommandBuffer commandBuffer, VkCoverageModulationModeNV coverageModulationMode)
    /// ```
    unsafe fn set_coverage_modulation_mode_nv(
        &self,
        coverage_modulation_mode: CoverageModulationModeNV,
    ) -> () {
        unsafe {
            self.commands().cmd_set_coverage_modulation_mode_nv(
                self.raw(),
                coverage_modulation_mode,
            )
        }
    }
    /// ```c
    /// void vkCmdSetCoverageModulationTableEnableNV(VkCommandBuffer commandBuffer, VkBool32 coverageModulationTableEnable)
    /// ```
    unsafe fn set_coverage_modulation_table_enable_nv(
        &self,
        coverage_modulation_table_enable: bool,
    ) -> () {
        unsafe {
            self.commands().cmd_set_coverage_modulation_table_enable_nv(
                self.raw(),
                coverage_modulation_table_enable,
            )
        }
    }
    /// ```c
    /// void vkCmdSetCoverageModulationTableNV(VkCommandBuffer commandBuffer, uint32_t coverageModulationTableCount, float const* pCoverageModulationTable)
    /// ```
    unsafe fn set_coverage_modulation_table_nv(
        &self,
        coverage_modulation_table: &[float],
    ) -> () {
        unsafe {
            self.commands().cmd_set_coverage_modulation_table_nv(
                self.raw(),
                coverage_modulation_table,
            )
        }
    }
    /// ```c
    /// void vkCmdSetCoverageReductionModeNV(VkCommandBuffer commandBuffer, VkCoverageReductionModeNV coverageReductionMode)
    /// ```
    unsafe fn set_coverage_reduction_mode_nv(
        &self,
        coverage_reduction_mode: CoverageReductionModeNV,
    ) -> () {
        unsafe {
            self.commands().cmd_set_coverage_reduction_mode_nv(
                self.raw(),
                coverage_reduction_mode,
            )
        }
    }
    /// ```c
    /// void vkCmdSetCoverageToColorEnableNV(VkCommandBuffer commandBuffer, VkBool32 coverageToColorEnable)
    /// ```
    unsafe fn set_coverage_to_color_enable_nv(
        &self,
        coverage_to_color_enable: bool,
    ) -> () {
        unsafe {
            self.commands().cmd_set_coverage_to_color_enable_nv(
                self.raw(),
                coverage_to_color_enable,
            )
        }
    }
    /// ```c
    /// void vkCmdSetCoverageToColorLocationNV(VkCommandBuffer commandBuffer, uint32_t coverageToColorLocation)
    /// ```
    unsafe fn set_coverage_to_color_location_nv(
        &self,
        coverage_to_color_location: uint32_t,
    ) -> () {
        unsafe {
            self.commands().cmd_set_coverage_to_color_location_nv(
                self.raw(),
                coverage_to_color_location,
            )
        }
    }
    /// ```c
    /// void vkCmdSetCullMode(VkCommandBuffer commandBuffer, VkCullModeFlags cullMode)
    /// ```
    unsafe fn set_cull_mode(
        &self,
        cull_mode: CullModeFlags,
    ) -> () {
        unsafe {
            self.commands().cmd_set_cull_mode(
                self.raw(),
                cull_mode,
            )
        }
    }
    /// ```c
    /// void vkCmdSetDepthBiasEnable(VkCommandBuffer commandBuffer, VkBool32 depthBiasEnable)
    /// ```
    unsafe fn set_depth_bias_enable(
        &self,
        depth_bias_enable: bool,
    ) -> () {
        unsafe {
            self.commands().cmd_set_depth_bias_enable(
                self.raw(),
                depth_bias_enable,
            )
        }
    }
    /// ```c
    /// void vkCmdSetDepthBoundsTestEnable(VkCommandBuffer commandBuffer, VkBool32 depthBoundsTestEnable)
    /// ```
    unsafe fn set_depth_bounds_test_enable(
        &self,
        depth_bounds_test_enable: bool,
    ) -> () {
        unsafe {
            self.commands().cmd_set_depth_bounds_test_enable(
                self.raw(),
                depth_bounds_test_enable,
            )
        }
    }
    /// ```c
    /// void vkCmdSetDepthClampEnableEXT(VkCommandBuffer commandBuffer, VkBool32 depthClampEnable)
    /// ```
    unsafe fn set_depth_clamp_enable(
        &self,
        depth_clamp_enable: bool,
    ) -> () {
        unsafe {
            self.commands().cmd_set_depth_clamp_enable(
                self.raw(),
                depth_clamp_enable,
            )
        }
    }
    /// ```c
    /// void vkCmdSetDepthClampRangeEXT(VkCommandBuffer commandBuffer, VkDepthClampModeEXT depthClampMode, VkDepthClampRangeEXT const* pDepthClampRange)
    /// ```
    unsafe fn set_depth_clamp_range(
        &self,
        depth_clamp_mode: DepthClampModeEXT,
        depth_clamp_range: Option<&DepthClampRangeEXT>,
    ) -> () {
        unsafe {
            self.commands().cmd_set_depth_clamp_range(
                self.raw(),
                depth_clamp_mode,
                depth_clamp_range,
            )
        }
    }
    /// ```c
    /// void vkCmdSetDepthClipEnableEXT(VkCommandBuffer commandBuffer, VkBool32 depthClipEnable)
    /// ```
    unsafe fn set_depth_clip_enable(
        &self,
        depth_clip_enable: bool,
    ) -> () {
        unsafe {
            self.commands().cmd_set_depth_clip_enable(
                self.raw(),
                depth_clip_enable,
            )
        }
    }
    /// ```c
    /// void vkCmdSetDepthClipNegativeOneToOneEXT(VkCommandBuffer commandBuffer, VkBool32 negativeOneToOne)
    /// ```
    unsafe fn set_depth_clip_negative_one_to_one(
        &self,
        negative_one_to_one: bool,
    ) -> () {
        unsafe {
            self.commands().cmd_set_depth_clip_negative_one_to_one(
                self.raw(),
                negative_one_to_one,
            )
        }
    }
    /// ```c
    /// void vkCmdSetDepthCompareOp(VkCommandBuffer commandBuffer, VkCompareOp depthCompareOp)
    /// ```
    unsafe fn set_depth_compare_op(
        &self,
        depth_compare_op: CompareOp,
    ) -> () {
        unsafe {
            self.commands().cmd_set_depth_compare_op(
                self.raw(),
                depth_compare_op,
            )
        }
    }
    /// ```c
    /// void vkCmdSetDepthTestEnable(VkCommandBuffer commandBuffer, VkBool32 depthTestEnable)
    /// ```
    unsafe fn set_depth_test_enable(
        &self,
        depth_test_enable: bool,
    ) -> () {
        unsafe {
            self.commands().cmd_set_depth_test_enable(
                self.raw(),
                depth_test_enable,
            )
        }
    }
    /// ```c
    /// void vkCmdSetDepthWriteEnable(VkCommandBuffer commandBuffer, VkBool32 depthWriteEnable)
    /// ```
    unsafe fn set_depth_write_enable(
        &self,
        depth_write_enable: bool,
    ) -> () {
        unsafe {
            self.commands().cmd_set_depth_write_enable(
                self.raw(),
                depth_write_enable,
            )
        }
    }
    /// ```c
    /// void vkCmdSetExtraPrimitiveOverestimationSizeEXT(VkCommandBuffer commandBuffer, float extraPrimitiveOverestimationSize)
    /// ```
    unsafe fn set_extra_primitive_overestimation_size(
        &self,
        extra_primitive_overestimation_size: float,
    ) -> () {
        unsafe {
            self.commands().cmd_set_extra_primitive_overestimation_size(
                self.raw(),
                extra_primitive_overestimation_size,
            )
        }
    }
    /// ```c
    /// void vkCmdSetFrontFace(VkCommandBuffer commandBuffer, VkFrontFace frontFace)
    /// ```
    unsafe fn set_front_face(
        &self,
        front_face: FrontFace,
    ) -> () {
        unsafe {
            self.commands().cmd_set_front_face(
                self.raw(),
                front_face,
            )
        }
    }
    /// ```c
    /// void vkCmdSetLineRasterizationModeEXT(VkCommandBuffer commandBuffer, VkLineRasterizationModeEXT lineRasterizationMode)
    /// ```
    unsafe fn set_line_rasterization_mode(
        &self,
        line_rasterization_mode: LineRasterizationModeEXT,
    ) -> () {
        unsafe {
            self.commands().cmd_set_line_rasterization_mode(
                self.raw(),
                line_rasterization_mode,
            )
        }
    }
    /// ```c
    /// void vkCmdSetLineStippleEnableEXT(VkCommandBuffer commandBuffer, VkBool32 stippledLineEnable)
    /// ```
    unsafe fn set_line_stipple_enable(
        &self,
        stippled_line_enable: bool,
    ) -> () {
        unsafe {
            self.commands().cmd_set_line_stipple_enable(
                self.raw(),
                stippled_line_enable,
            )
        }
    }
    /// ```c
    /// void vkCmdSetLogicOpEXT(VkCommandBuffer commandBuffer, VkLogicOp logicOp)
    /// ```
    unsafe fn set_logic_op(
        &self,
        logic_op: LogicOp,
    ) -> () {
        unsafe {
            self.commands().cmd_set_logic_op(
                self.raw(),
                logic_op,
            )
        }
    }
    /// ```c
    /// void vkCmdSetLogicOpEnableEXT(VkCommandBuffer commandBuffer, VkBool32 logicOpEnable)
    /// ```
    unsafe fn set_logic_op_enable(
        &self,
        logic_op_enable: bool,
    ) -> () {
        unsafe {
            self.commands().cmd_set_logic_op_enable(
                self.raw(),
                logic_op_enable,
            )
        }
    }
    /// ```c
    /// void vkCmdSetPatchControlPointsEXT(VkCommandBuffer commandBuffer, uint32_t patchControlPoints)
    /// ```
    unsafe fn set_patch_control_points(
        &self,
        patch_control_points: uint32_t,
    ) -> () {
        unsafe {
            self.commands().cmd_set_patch_control_points(
                self.raw(),
                patch_control_points,
            )
        }
    }
    /// ```c
    /// void vkCmdSetPolygonModeEXT(VkCommandBuffer commandBuffer, VkPolygonMode polygonMode)
    /// ```
    unsafe fn set_polygon_mode(
        &self,
        polygon_mode: PolygonMode,
    ) -> () {
        unsafe {
            self.commands().cmd_set_polygon_mode(
                self.raw(),
                polygon_mode,
            )
        }
    }
    /// ```c
    /// void vkCmdSetPrimitiveRestartEnable(VkCommandBuffer commandBuffer, VkBool32 primitiveRestartEnable)
    /// ```
    unsafe fn set_primitive_restart_enable(
        &self,
        primitive_restart_enable: bool,
    ) -> () {
        unsafe {
            self.commands().cmd_set_primitive_restart_enable(
                self.raw(),
                primitive_restart_enable,
            )
        }
    }
    /// ```c
    /// void vkCmdSetPrimitiveTopology(VkCommandBuffer commandBuffer, VkPrimitiveTopology primitiveTopology)
    /// ```
    unsafe fn set_primitive_topology(
        &self,
        primitive_topology: PrimitiveTopology,
    ) -> () {
        unsafe {
            self.commands().cmd_set_primitive_topology(
                self.raw(),
                primitive_topology,
            )
        }
    }
    /// ```c
    /// void vkCmdSetProvokingVertexModeEXT(VkCommandBuffer commandBuffer, VkProvokingVertexModeEXT provokingVertexMode)
    /// ```
    unsafe fn set_provoking_vertex_mode(
        &self,
        provoking_vertex_mode: ProvokingVertexModeEXT,
    ) -> () {
        unsafe {
            self.commands().cmd_set_provoking_vertex_mode(
                self.raw(),
                provoking_vertex_mode,
            )
        }
    }
    /// ```c
    /// void vkCmdSetRasterizationSamplesEXT(VkCommandBuffer commandBuffer, VkSampleCountFlagBits rasterizationSamples)
    /// ```
    unsafe fn set_rasterization_samples(
        &self,
        rasterization_samples: SampleCountFlags,
    ) -> () {
        unsafe {
            self.commands().cmd_set_rasterization_samples(
                self.raw(),
                rasterization_samples,
            )
        }
    }
    /// ```c
    /// void vkCmdSetRasterizationStreamEXT(VkCommandBuffer commandBuffer, uint32_t rasterizationStream)
    /// ```
    unsafe fn set_rasterization_stream(
        &self,
        rasterization_stream: uint32_t,
    ) -> () {
        unsafe {
            self.commands().cmd_set_rasterization_stream(
                self.raw(),
                rasterization_stream,
            )
        }
    }
    /// ```c
    /// void vkCmdSetRasterizerDiscardEnable(VkCommandBuffer commandBuffer, VkBool32 rasterizerDiscardEnable)
    /// ```
    unsafe fn set_rasterizer_discard_enable(
        &self,
        rasterizer_discard_enable: bool,
    ) -> () {
        unsafe {
            self.commands().cmd_set_rasterizer_discard_enable(
                self.raw(),
                rasterizer_discard_enable,
            )
        }
    }
    /// ```c
    /// void vkCmdSetRepresentativeFragmentTestEnableNV(VkCommandBuffer commandBuffer, VkBool32 representativeFragmentTestEnable)
    /// ```
    unsafe fn set_representative_fragment_test_enable_nv(
        &self,
        representative_fragment_test_enable: bool,
    ) -> () {
        unsafe {
            self.commands().cmd_set_representative_fragment_test_enable_nv(
                self.raw(),
                representative_fragment_test_enable,
            )
        }
    }
    /// ```c
    /// void vkCmdSetSampleLocationsEnableEXT(VkCommandBuffer commandBuffer, VkBool32 sampleLocationsEnable)
    /// ```
    unsafe fn set_sample_locations_enable(
        &self,
        sample_locations_enable: bool,
    ) -> () {
        unsafe {
            self.commands().cmd_set_sample_locations_enable(
                self.raw(),
                sample_locations_enable,
            )
        }
    }
    /// ```c
    /// void vkCmdSetSampleMaskEXT(VkCommandBuffer commandBuffer, VkSampleCountFlagBits samples, VkSampleMask const* pSampleMask)
    /// ```
    unsafe fn set_sample_mask(
        &self,
        samples: SampleCountFlags,
        sample_mask: *const SampleMask,
    ) -> () {
        unsafe {
            self.commands().cmd_set_sample_mask(
                self.raw(),
                samples,
                sample_mask,
            )
        }
    }
    /// ```c
    /// void vkCmdSetScissorWithCount(VkCommandBuffer commandBuffer, uint32_t scissorCount, VkRect2D const* pScissors)
    /// ```
    unsafe fn set_scissor_with_count(
        &self,
        scissors: &[Rect2D],
    ) -> () {
        unsafe {
            self.commands().cmd_set_scissor_with_count(
                self.raw(),
                scissors,
            )
        }
    }
    /// ```c
    /// void vkCmdSetShadingRateImageEnableNV(VkCommandBuffer commandBuffer, VkBool32 shadingRateImageEnable)
    /// ```
    unsafe fn set_shading_rate_image_enable_nv(
        &self,
        shading_rate_image_enable: bool,
    ) -> () {
        unsafe {
            self.commands().cmd_set_shading_rate_image_enable_nv(
                self.raw(),
                shading_rate_image_enable,
            )
        }
    }
    /// ```c
    /// void vkCmdSetStencilOp(VkCommandBuffer commandBuffer, VkStencilFaceFlags faceMask, VkStencilOp failOp, VkStencilOp passOp, VkStencilOp depthFailOp, VkCompareOp compareOp)
    /// ```
    unsafe fn set_stencil_op(
        &self,
        face_mask: StencilFaceFlags,
        fail_op: StencilOp,
        pass_op: StencilOp,
        depth_fail_op: StencilOp,
        compare_op: CompareOp,
    ) -> () {
        unsafe {
            self.commands().cmd_set_stencil_op(
                self.raw(),
                face_mask,
                fail_op,
                pass_op,
                depth_fail_op,
                compare_op,
            )
        }
    }
    /// ```c
    /// void vkCmdSetStencilTestEnable(VkCommandBuffer commandBuffer, VkBool32 stencilTestEnable)
    /// ```
    unsafe fn set_stencil_test_enable(
        &self,
        stencil_test_enable: bool,
    ) -> () {
        unsafe {
            self.commands().cmd_set_stencil_test_enable(
                self.raw(),
                stencil_test_enable,
            )
        }
    }
    /// ```c
    /// void vkCmdSetTessellationDomainOriginEXT(VkCommandBuffer commandBuffer, VkTessellationDomainOrigin domainOrigin)
    /// ```
    unsafe fn set_tessellation_domain_origin(
        &self,
        domain_origin: TessellationDomainOrigin,
    ) -> () {
        unsafe {
            self.commands().cmd_set_tessellation_domain_origin(
                self.raw(),
                domain_origin,
            )
        }
    }
    /// ```c
    /// void vkCmdSetVertexInputEXT(VkCommandBuffer commandBuffer, uint32_t vertexBindingDescriptionCount, VkVertexInputBindingDescription2EXT const* pVertexBindingDescriptions, uint32_t vertexAttributeDescriptionCount, VkVertexInputAttributeDescription2EXT const* pVertexAttributeDescriptions)
    /// ```
    unsafe fn set_vertex_input(
        &self,
        vertex_binding_descriptions: &[VertexInputBindingDescription2EXT],
        vertex_attribute_descriptions: &[VertexInputAttributeDescription2EXT],
    ) -> () {
        unsafe {
            self.commands().cmd_set_vertex_input(
                self.raw(),
                vertex_binding_descriptions,
                vertex_attribute_descriptions,
            )
        }
    }
    /// ```c
    /// void vkCmdSetViewportSwizzleNV(VkCommandBuffer commandBuffer, uint32_t firstViewport, uint32_t viewportCount, VkViewportSwizzleNV const* pViewportSwizzles)
    /// ```
    unsafe fn set_viewport_swizzle_nv(
        &self,
        first_viewport: uint32_t,
        viewport_swizzles: &[ViewportSwizzleNV],
    ) -> () {
        unsafe {
            self.commands().cmd_set_viewport_swizzle_nv(
                self.raw(),
                first_viewport,
                viewport_swizzles,
            )
        }
    }
    /// ```c
    /// void vkCmdSetViewportWScalingEnableNV(VkCommandBuffer commandBuffer, VkBool32 viewportWScalingEnable)
    /// ```
    unsafe fn set_viewport_w_scaling_enable_nv(
        &self,
        viewport_w_scaling_enable: bool,
    ) -> () {
        unsafe {
            self.commands().cmd_set_viewport_w_scaling_enable_nv(
                self.raw(),
                viewport_w_scaling_enable,
            )
        }
    }
    /// ```c
    /// void vkCmdSetViewportWithCount(VkCommandBuffer commandBuffer, uint32_t viewportCount, VkViewport const* pViewports)
    /// ```
    unsafe fn set_viewport_with_count(
        &self,
        viewports: &[Viewport],
    ) -> () {
        unsafe {
            self.commands().cmd_set_viewport_with_count(
                self.raw(),
                viewports,
            )
        }
    }
}

impl<O: crate::HndCtx<vk::extensions::ext::shader_object, vk::Device>> crate::Owner<vk::ShaderEXT, vk::extensions::ext::shader_object> for O {
    fn drop(&mut self, raw: vk::ShaderEXT) {
        unsafe {
            self.commands().destroy_shader(self.raw(), Some(raw))
        }
    }
}
impl<O> crate::HndCtx<vk::extensions::ext::shader_object, vk::Device> for ::alloc::sync::Arc<crate::Unique<vk::ShaderEXT, O, vk::extensions::ext::shader_object>>
    where O: crate::HndCtx<vk::extensions::ext::shader_object, vk::Device> + Send + Sync + 'static,
{
    type Ctx = Self;
    fn ctx(&self) -> Self::Ctx { self.clone() }
    fn raw(&self) -> vk::Device { self.owner.raw() }
    fn commands(&self) -> &::alloc::sync::Arc<Device> { self.owner.commands() }
}

impl crate::HndScope<vk::ShaderEXT> for vk::extensions::ext::shader_object {
    type Impl = _hs_ShaderEXT::ShaderEXT;
}


mod _hs_ShaderEXT {
    use super::*;

    #[repr(transparent)]
    #[derive(Debug)]
    pub struct ShaderEXT(pub(crate) crate::handle::Hnd<vk::ShaderEXT, ::alloc::sync::Arc<super::Device>>);

    impl Clone for ShaderEXT {
        fn clone(&self) -> Self { Self(self.0.clone()) }
    }

    #[derive(Debug, Clone, Copy)]
    struct Inst;
    impl<Ctx, Dep> crate::handle::HndInst<vk::ShaderEXT, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)> for Inst
        where Ctx: crate::HndCtx<vk::extensions::ext::shader_object, vk::Device>,
    {
        fn drop(this: &mut crate::handle::HndData<vk::ShaderEXT, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)>) {
            unsafe {
                this.scope.destroy_shader(this.dep.0.raw(), Some(this.raw))
            }
        }
    }

    impl crate::hnd::ShaderEXT<vk::extensions::ext::shader_object>
    {
        pub unsafe fn new(ctx: &impl crate::HndCtx<vk::extensions::ext::shader_object, vk::Device>, raw: vk::ShaderEXT) -> Self {
            unsafe { Self::new_with(ctx, raw, || ()) }
        }
        pub unsafe fn new_with<Dep: Send + Sync + 'static>(ctx: &impl crate::HndCtx<vk::extensions::ext::shader_object, vk::Device>, raw: vk::ShaderEXT, dep: impl FnOnce() -> Dep) -> Self {
            Self(ShaderEXT(crate::handle::Hnd::new(
                ctx.commands().clone(),
                raw,
                (ctx.ctx(), dep()),
                Inst,
            )))
        }
    }

    impl crate::hnd::ShaderEXT<vk::extensions::ext::shader_object> {
        pub fn raw(&self) -> vk::ShaderEXT { self.0.0.raw }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0.scope }
    }

    impl ::core::fmt::Debug for crate::hnd::ShaderEXT<vk::extensions::ext::shader_object> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("ShaderEXT({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::ShaderEXT<vk::extensions::ext::shader_object> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl<Ctx> crate::MakeHnd<Ctx, vk::extensions::ext::shader_object> for vk::ShaderEXT
        where Ctx: crate::HndCtx<vk::extensions::ext::shader_object, vk::Device>,
    {
        type Output = crate::hnd::ShaderEXT<vk::extensions::ext::shader_object>;
        unsafe fn hnd_with<Dep: Send + Sync + 'static>(self, ctx: &Ctx, dep: impl FnOnce() -> Dep) -> Self::Output {
            unsafe { crate::hnd::ShaderEXT::<vk::extensions::ext::shader_object>::new_with(ctx, self, dep) }
        }
    }
}
