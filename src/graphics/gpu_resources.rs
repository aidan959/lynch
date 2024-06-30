use ash::vk;

use super::{color_write_enabled, fill_mode, pipeline_stage, render_pass_operation, render_pass_type, resource_usage_type, texture_type, vertex_component_format, vertex_input_rate};

const K_INVALID_INDEX: u32 = 0xffffffff;

type ResourceHandle = u32;

#[derive(Debug, Copy, Clone)]
struct BufferHandle {
    index: ResourceHandle,
}
impl Default for BufferHandle {
    #[inline]
    fn default() -> Self {
        BufferHandle {
            index: Default::default(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct TextureHandle {
    index: ResourceHandle,
}
impl Default for TextureHandle{
    #[inline]
    fn default() -> Self {
        TextureHandle {
            index: Default::default(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct ShaderStateHandle {
    index: ResourceHandle,
}

#[derive(Debug, Copy, Clone)]
struct SamplerHandle {
    index: ResourceHandle,
}
impl  Default for SamplerHandle{
    #[inline]
    fn default() -> Self {
        SamplerHandle {
            index: Default::default(),
        }
    }
}
#[derive(Debug, Copy, Clone)]
struct DescriptorSetLayoutHandle {
    index: ResourceHandle,
}
impl Default for DescriptorSetLayoutHandle{
    #[inline]
    fn default() -> Self {
        DescriptorSetLayoutHandle {
            index: Default::default(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct DescriptorSetHandle {
    index: ResourceHandle,
}

#[derive(Debug, Copy, Clone)]
struct PipelineHandle {
    index: ResourceHandle,
}

#[derive(Debug, Copy, Clone)]
struct RenderPassHandle {
    index: ResourceHandle,
}

// Invalid handles
const K_INVALID_BUFFER: BufferHandle = BufferHandle { index: K_INVALID_INDEX };
const K_INVALID_TEXTURE: TextureHandle = TextureHandle { index: K_INVALID_INDEX };
const K_INVALID_SHADER: ShaderStateHandle = ShaderStateHandle { index: K_INVALID_INDEX };
const K_INVALID_SAMPLER: SamplerHandle = SamplerHandle { index: K_INVALID_INDEX };
const K_INVALID_LAYOUT: DescriptorSetLayoutHandle = DescriptorSetLayoutHandle { index: K_INVALID_INDEX };
const K_INVALID_SET: DescriptorSetHandle = DescriptorSetHandle { index: K_INVALID_INDEX };
const K_INVALID_PIPELINE: PipelineHandle = PipelineHandle { index: K_INVALID_INDEX };
const K_INVALID_PASS: RenderPassHandle = RenderPassHandle { index: K_INVALID_INDEX };


const K_MAX_IMAGE_OUTPUTS: usize = 8;               // Maximum number of images/render_targets/fbo attachments usable.
const K_MAX_DESCRIPTOR_SET_LAYOUTS: usize = 8;      // Maximum number of layouts in the pipeline.
const K_MAX_SHADER_STAGES: usize = 5;               // Maximum simultaneous shader stages. Applicable to all different types of pipelines.
const K_MAX_DESCRIPTORS_PER_SET: usize = 16;        // Maximum list elements for both descriptor set layout and descriptor sets.
const K_MAX_VERTEX_STREAMS: usize = 16;
const K_MAX_VERTEX_ATTRIBUTES: usize = 16;

const K_SUBMIT_HEADER_SENTINEL: u32 = 0xfefeb7ba;
const K_MAX_RESOURCE_DELETIONS: u32 = 64;


#[derive(Debug, Copy, Clone)]
struct Rect2D {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl Default for Rect2D {
    fn default() -> Self {
        Rect2D {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Rect2DInt {
    x: i16,
    y: i16,
    width: u16,
    height: u16,
}

impl Default for Rect2DInt {
    fn default() -> Self {
        Rect2DInt {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Viewport {
    rect: Rect2DInt,
    min_depth: f32,
    max_depth: f32,
}

impl Default for Viewport {
    fn default() -> Self {
        Viewport {
            rect: Rect2DInt::default(),
            min_depth: 0.0,
            max_depth: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
struct ViewportState {
    num_viewports: u32,
    num_scissors: u32,
    viewport: Option<Vec<Viewport>>,
    scissors: Option<Vec<Rect2DInt>>,
}

impl Default for ViewportState {
    fn default() -> Self {
        ViewportState {
            num_viewports: 0,
            num_scissors: 0,
            viewport: None,
            scissors: None,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct StencilOperationState {
    fail: vk::StencilOp,
    pass: vk::StencilOp,
    depth_fail: vk::StencilOp,
    compare: vk::CompareOp,
    compare_mask: u32,
    write_mask: u32,
    reference: u32,
}

impl Default for StencilOperationState {
    fn default() -> Self {
        StencilOperationState {
            fail: vk::StencilOp::KEEP,
            pass: vk::StencilOp::KEEP,
            depth_fail: vk::StencilOp::KEEP,
            compare: vk::CompareOp::ALWAYS,
            compare_mask: 0xff,
            write_mask: 0xff,
            reference: 0xff,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct DepthStencilCreation {
    front: StencilOperationState,
    back: StencilOperationState,
    depth_comparison: vk::CompareOp,
    depth_enable: bool,
    depth_write_enable: bool,
    stencil_enable: bool,
    pad: u8,
}

impl Default for DepthStencilCreation {
    fn default() -> Self {
        DepthStencilCreation {
            front: StencilOperationState::default(),
            back: StencilOperationState::default(),
            depth_comparison: vk::CompareOp::ALWAYS,
            depth_enable: false,
            depth_write_enable: false,
            stencil_enable: false,
            pad: 0,
        }
    }
}

impl DepthStencilCreation {
    fn set_depth(&mut self, write: bool, comparison_test: vk::CompareOp) -> &mut Self {
        self.depth_write_enable = write;
        self.depth_comparison = comparison_test;
        self
    }
}

#[derive(Debug, Copy, Clone)]
struct BlendState {
    source_color: vk::BlendFactor,
    destination_color: vk::BlendFactor,
    color_operation: vk::BlendOp,
    source_alpha: vk::BlendFactor,
    destination_alpha: vk::BlendFactor,
    alpha_operation: vk::BlendOp,
    color_write_mask: color_write_enabled::Mask,
    blend_enabled: bool,
    separate_blend: bool,
    pad: u8,
}

impl Default for BlendState {
    fn default() -> Self {
        BlendState {
            source_color: vk::BlendFactor::ONE,
            destination_color: vk::BlendFactor::ONE,
            color_operation: vk::BlendOp::ADD,
            source_alpha: vk::BlendFactor::ONE,
            destination_alpha: vk::BlendFactor::ONE,
            alpha_operation: vk::BlendOp::ADD,
            color_write_mask: color_write_enabled::Mask::AllMask,
            blend_enabled: false,
            separate_blend: false,
            pad: 0,
        }
    }
}

impl BlendState {
    fn set_color(&mut self, source_color: vk::BlendFactor, destination_color: vk::BlendFactor, color_operation: vk::BlendOp) -> &mut Self {
        self.source_color = source_color;
        self.destination_color = destination_color;
        self.color_operation = color_operation;
        self
    }

    fn set_alpha(&mut self, source_alpha: vk::BlendFactor, destination_alpha: vk::BlendFactor, alpha_operation: vk::BlendOp) -> &mut Self {
        self.source_alpha = source_alpha;
        self.destination_alpha = destination_alpha;
        self.alpha_operation = alpha_operation;
        self
    }

    fn set_color_write_mask(&mut self, value: color_write_enabled::Mask) -> &mut Self {
        self.color_write_mask = value;
        self
    }
}

#[derive(Debug, Clone)]
struct BlendStateCreation {
    blend_states: [BlendState; K_MAX_IMAGE_OUTPUTS as usize],
    active_states: u32,
}

impl Default for BlendStateCreation {
    fn default() -> Self {
        BlendStateCreation {
            blend_states: [BlendState::default(); K_MAX_IMAGE_OUTPUTS as usize],
            active_states: 0,
        }
    }
}

impl BlendStateCreation {
    fn reset(&mut self) -> &mut Self {
        self.active_states = 0;
        self
    }

    fn add_blend_state(&mut self) -> &mut BlendState {
        let state = &mut self.blend_states[self.active_states as usize];
        self.active_states += 1;
        state
    }
}

#[derive(Debug, Copy, Clone)]
struct RasterizationCreation {
    cull_mode: vk::CullModeFlags,
    front: vk::FrontFace,
    fill: fill_mode::Enum,
}

impl Default for RasterizationCreation {
    fn default() -> Self {
        RasterizationCreation {
            cull_mode: vk::CullModeFlags::NONE,
            front: vk::FrontFace::COUNTER_CLOCKWISE,
            fill: fill_mode::Enum::Solid,
        }
    }
}

#[derive(Debug, Clone)]
struct BufferCreation {
    type_flags: vk::BufferUsageFlags,
    usage: resource_usage_type::Enum,
    size: u32,
    initial_data: Option<*mut std::ffi::c_void>,
    name: Option<String>,
}

impl Default for BufferCreation {
    fn default() -> Self {
        BufferCreation {
            type_flags: vk::BufferUsageFlags::empty(),
            usage: resource_usage_type::Enum::Immutable,
            size: 0,
            initial_data: None,
            name: None,
        }
    }
}

impl BufferCreation {
    fn reset(&mut self) -> &mut Self {
        self.type_flags = vk::BufferUsageFlags::empty();
        self.usage = resource_usage_type::Enum::Immutable;
        self.size = 0;
        self.initial_data = None;
        self.name = None;
        self
    }

    fn set(&mut self, flags: vk::BufferUsageFlags, usage: resource_usage_type::Enum, size: u32) -> &mut Self {
        self.type_flags = flags;
        self.usage = usage;
        self.size = size;
        self
    }

    fn set_data(&mut self, data: *mut std::ffi::c_void) -> &mut Self {
        self.initial_data = Some(data);
        self
    }

    fn set_name(&mut self, name: &str) -> &mut Self {
        self.name = Some(name.to_string());
        self
    }
}

#[derive(Debug, Clone)]
struct TextureCreation {
    initial_data: Option<*mut std::ffi::c_void>,
    width: u16,
    height: u16,
    depth: u16,
    mipmaps: u8,
    flags: u8,
    format: vk::Format,
    texture_type: texture_type::Enum,
    name: Option<String>,
}

impl Default for TextureCreation {
    fn default() -> Self {
        TextureCreation {
            initial_data: None,
            width: 1,
            height: 1,
            depth: 1,
            mipmaps: 1,
            flags: 0,
            format: vk::Format::UNDEFINED,
            texture_type: texture_type::Enum::Texture2D,
            name: None,
        }
    }
}

impl TextureCreation {
    fn set_size(&mut self, width: u16, height: u16, depth: u16) -> &mut Self {
        self.width = width;
        self.height = height;
        self.depth = depth;
        self
    }

    fn set_flags(&mut self, mipmaps: u8, flags: u8) -> &mut Self {
        self.mipmaps = mipmaps;
        self.flags = flags;
        self
    }

    fn set_format_type(&mut self, format: vk::Format, texture_type: texture_type::Enum) -> &mut Self {
        self.format = format;
        self.texture_type = texture_type;
        self
    }

    fn set_name(&mut self, name: &str) -> &mut Self {
        self.name = Some(name.to_string());
        self
    }

    fn set_data(&mut self, data: *mut std::ffi::c_void) -> &mut Self {
        self.initial_data = Some(data);
        self
    }
}

pub struct SamplerCreation<'a> {
    min_filter: vk::Filter,
    mag_filter: vk::Filter,
    mip_filter: vk::SamplerMipmapMode,

    address_mode_u: vk::SamplerAddressMode,
    address_mode_v: vk::SamplerAddressMode,
    address_mode_w: vk::SamplerAddressMode,

    name: Option<&'a str>,
}

impl<'a> SamplerCreation<'a> {
    pub fn new() -> Self {
        SamplerCreation {
            min_filter: vk::Filter::NEAREST,
            mag_filter: vk::Filter::NEAREST,
            mip_filter: vk::SamplerMipmapMode::NEAREST,
            address_mode_u: vk::SamplerAddressMode::REPEAT,
            address_mode_v: vk::SamplerAddressMode::REPEAT,
            address_mode_w: vk::SamplerAddressMode::REPEAT,
            name: None,
        }
    }

    pub fn set_min_mag_mip(&mut self, min: vk::Filter, mag: vk::Filter, mip: vk::SamplerMipmapMode) -> &mut Self {
        self.min_filter = min;
        self.mag_filter = mag;
        self.mip_filter = mip;
        self
    }

    pub fn set_address_mode_u(&mut self, u: vk::SamplerAddressMode) -> &mut Self {
        self.address_mode_u = u;
        self
    }

    pub fn set_address_mode_uv(&mut self, u: vk::SamplerAddressMode, v: vk::SamplerAddressMode) -> &mut Self {
        self.address_mode_u = u;
        self.address_mode_v = v;
        self
    }

    pub fn set_address_mode_uvw(&mut self, u: vk::SamplerAddressMode, v: vk::SamplerAddressMode, w: vk::SamplerAddressMode) -> &mut Self {
        self.address_mode_u = u;
        self.address_mode_v = v;
        self.address_mode_w = w;
        self
    }

    pub fn set_name(&mut self, name: &'a str) -> &mut Self {
        self.name = Some(name);
        self
    }
}
pub struct ShaderStage<'a> {
    code: Option<&'a [u8]>,
    code_size: u32,
    ty: vk::ShaderStageFlags,
}

impl<'a> ShaderStage<'a> {
    pub fn new() -> Self {
        ShaderStage {
            code: None,
            code_size: 0,
            ty: vk::ShaderStageFlags::ALL,
        }
    }

    pub fn set_code(&mut self, code: &'a [u8], size: u32) -> &mut Self {
        self.code = Some(code);
        self.code_size = size;
        self
    }

    pub fn set_type(&mut self, ty: vk::ShaderStageFlags) -> &mut Self {
        self.ty = ty;
        self
    }
}

pub struct ShaderStateCreation<'a> {
    stages: [ShaderStage<'a>; K_MAX_SHADER_STAGES], 

    name: Option<&'a str>,

    stages_count: u32,
    spv_input: bool,
}

impl<'a> ShaderStateCreation<'a> {
    pub fn new() -> Self {
        ShaderStateCreation {
            stages: core::array::from_fn(|_| ShaderStage::new()),
            name: None,
            stages_count: 0,
            spv_input: false,
        }
    }

    pub fn reset(&mut self) -> &mut Self {
        self.stages.fill_with(|| ShaderStage::new());
        self.name = None;
        self.stages_count = 0;
        self.spv_input = false;
        self
    }

    pub fn set_name(&mut self, name: &'a str) -> &mut Self {
        self.name = Some(name);
        self
    }

    pub fn add_stage(&mut self, code: &'a [u8], code_size: u32, ty: vk::ShaderStageFlags) -> &mut Self {
        if self.stages_count < K_MAX_SHADER_STAGES as u32 {
            self.stages[self.stages_count as usize].set_code(code, code_size).set_type(ty);
            self.stages_count += 1;
        }
        self
    }

    pub fn set_spv_input(&mut self, value: bool) -> &mut Self {
        self.spv_input = value;
        self
    }
}

pub struct DescriptorSetLayoutCreation {
    bindings: [Binding; K_MAX_DESCRIPTORS_PER_SET],
    num_bindings: u32,
    set_index: u32,
    name: Option<&'static str>,
}

pub struct Binding {
    ty: Option<vk::DescriptorType>,
    start: u16,
    count: u16,
    name: Option<&'static str>,
}

impl<'a> DescriptorSetLayoutCreation {
    pub fn new() -> Self {
        DescriptorSetLayoutCreation {
            bindings: std::array::from_fn(|_|Binding::default()),
            num_bindings: 0,
            set_index: 0,
            name: None,
        }
    }

    pub fn reset(&mut self) -> &mut Self {
        self.bindings.fill_with(|| Binding::default());
        self.num_bindings = 0;
        self.set_index = 0;
        self.name = None;
        self
    }

    pub fn add_binding(&mut self, binding: Binding) -> &mut Self {
        if self.num_bindings < K_MAX_DESCRIPTORS_PER_SET as u32 {
            self.bindings[self.num_bindings as usize] = binding;
            self.num_bindings += 1;
        }
        self
    }
    // TODO may need lifetime?
    pub fn set_name(&mut self, name: &'static str) -> &mut Self {
        self.name = Some(name);
        self
    }

    pub fn set_set_index(&mut self, index: u32) -> &mut Self {
        self.set_index = index;
        self
    }
}

impl Default for Binding {
    fn default() -> Self {
        Binding {
            ty: None,
            start: 0,
            count: 0,
            name: None,
        }
    }
}

pub struct DescriptorSetCreation<'a> {
    resources: [ResourceHandle; K_MAX_DESCRIPTORS_PER_SET], 
    samplers: [SamplerHandle; K_MAX_DESCRIPTORS_PER_SET],   
    bindings: [u16; K_MAX_DESCRIPTORS_PER_SET],

    layout: DescriptorSetLayoutHandle,
    num_resources: u32,
    name: Option<&'a str>,
}

impl<'a> DescriptorSetCreation<'a> {
    pub fn new() -> Self {
        DescriptorSetCreation {
            resources: [ResourceHandle::default(); K_MAX_DESCRIPTORS_PER_SET],
            samplers: [SamplerHandle::default(); K_MAX_DESCRIPTORS_PER_SET],
            bindings: [0; K_MAX_DESCRIPTORS_PER_SET],
            layout: DescriptorSetLayoutHandle::default(),
            num_resources: 0,
            name: None,
        }
    }

    pub fn reset(&mut self) -> &mut Self {
        self.resources.fill(ResourceHandle::default());
        self.samplers.fill(SamplerHandle::default());
        self.bindings.fill(0);
        self.layout = DescriptorSetLayoutHandle::default();
        self.num_resources = 0;
        self.name = None;
        self
    }

    pub fn set_layout(&mut self, layout: DescriptorSetLayoutHandle) -> &mut Self {
        self.layout = layout;
        self
    }

    pub fn texture(&mut self, texture: TextureHandle, binding: u16) -> &mut Self {
        if binding < K_MAX_DESCRIPTORS_PER_SET as u16 {
            self.resources[binding as usize] = texture.index;
            self.bindings[binding as usize] = binding;
            self.num_resources += 1;
        }
        self
    }

    pub fn buffer(&mut self, buffer: BufferHandle, binding: u16) -> &mut Self {
        if binding < K_MAX_DESCRIPTORS_PER_SET as u16 {
            self.resources[binding as usize] = buffer.index;
            self.bindings[binding as usize] = binding;
            self.num_resources += 1;
        }
        self
    }

    pub fn texture_sampler(&mut self, texture: TextureHandle, sampler: SamplerHandle, binding: u16) -> &mut Self {
        if binding < K_MAX_DESCRIPTORS_PER_SET as u16 {
            self.resources[binding as usize] = texture.index;
            self.samplers[binding as usize] = sampler;
            self.bindings[binding as usize] = binding;
            self.num_resources += 1;
        }
        self
    }

    pub fn set_name(&mut self, name: &'a str) -> &mut Self {
        self.name = Some(name);
        self
    }
}

impl Default for DescriptorSetCreation<'_> {
    fn default() -> Self {
        DescriptorSetCreation::new()
    }
}
pub struct DescriptorSetUpdate {
    descriptor_set: DescriptorSetHandle,

    frame_issued: u32,
}

impl DescriptorSetUpdate {
    pub fn new(descriptor_set: DescriptorSetHandle) -> Self {
        DescriptorSetUpdate {
            descriptor_set,
            frame_issued: 0,
        }
    }
}



pub struct VertexAttribute {
    location: u16,
    binding: u16,
    offset: u32,
    format: vertex_component_format::Enum,
}

impl VertexAttribute {
    pub fn new() -> Self {
        VertexAttribute {
            location: 0,
            binding: 0,
            offset: 0,
            format: vertex_component_format::Enum::Count,
        }
    }
}



pub struct VertexStream {
    binding: u16,
    stride: u16,
    input_rate: vertex_input_rate::Enum,
}

impl VertexStream {
    pub fn new() -> Self {
        VertexStream {
            binding: 0,
            stride: 0,
            input_rate: vertex_input_rate::Enum::Count,
        }
    }
}

pub struct VertexInputCreation {
    num_vertex_streams: u32,
    num_vertex_attributes: u32,

    vertex_streams: [VertexStream; K_MAX_VERTEX_STREAMS],
    vertex_attributes: [VertexAttribute; K_MAX_VERTEX_ATTRIBUTES],
}

impl VertexInputCreation {
    pub fn new() -> Self {
        VertexInputCreation {
            num_vertex_streams: 0,
            num_vertex_attributes: 0,
            vertex_streams: std::array::from_fn(|_|VertexStream::new()),
            vertex_attributes: std::array::from_fn(|_|VertexAttribute::new()),
        }
    }

    pub fn reset(&mut self) -> &mut Self {
        self.num_vertex_streams = 0;
        self.num_vertex_attributes = 0;
        self.vertex_streams = std::array::from_fn(|_|VertexStream::new());
        self.vertex_attributes = std::array::from_fn(|_|VertexAttribute::new());
        self
    }

    pub fn add_vertex_stream(&mut self, stream: VertexStream) -> &mut Self {
        if self.num_vertex_streams < K_MAX_VERTEX_STREAMS as u32 {
            self.vertex_streams[self.num_vertex_streams as usize] = stream;
            self.num_vertex_streams += 1;
        }
        self
    }

    pub fn add_vertex_attribute(&mut self, attribute: VertexAttribute) -> &mut Self {
        if self.num_vertex_attributes < K_MAX_VERTEX_ATTRIBUTES as u32 {
            self.vertex_attributes[self.num_vertex_attributes as usize] = attribute;
            self.num_vertex_attributes += 1;
        }
        self
    }
}



pub struct RenderPassOutput {
    color_formats: [vk::Format; K_MAX_IMAGE_OUTPUTS], 
    depth_stencil_format: vk::Format,
    num_color_formats: u32,

    color_operation: render_pass_operation::Enum,
    depth_operation: render_pass_operation::Enum,
    stencil_operation: render_pass_operation::Enum,
}

impl RenderPassOutput {
    pub fn new() -> Self {
        RenderPassOutput {
            color_formats: [vk::Format::UNDEFINED; K_MAX_IMAGE_OUTPUTS],
            depth_stencil_format: vk::Format::UNDEFINED,
            num_color_formats: 0,
            color_operation: render_pass_operation::Enum::DontCare,
            depth_operation: render_pass_operation::Enum::DontCare,
            stencil_operation: render_pass_operation::Enum::DontCare,
        }
    }

    pub fn reset(&mut self) -> &mut Self {
        self.color_formats = [vk::Format::UNDEFINED; K_MAX_IMAGE_OUTPUTS];
        self.depth_stencil_format = vk::Format::UNDEFINED;
        self.num_color_formats = 0;
        self.color_operation = render_pass_operation::Enum::DontCare;
        self.depth_operation = render_pass_operation::Enum::DontCare;
        self.stencil_operation = render_pass_operation::Enum::DontCare;
        self
    }

    pub fn color(&mut self, format: vk::Format) -> &mut Self {
        if self.num_color_formats < K_MAX_IMAGE_OUTPUTS as u32 {
            self.color_formats[self.num_color_formats as usize] = format;
            self.num_color_formats += 1;
        }
        self
    }

    pub fn depth(&mut self, format: vk::Format) -> &mut Self {
        self.depth_stencil_format = format;
        self
    }

    pub fn set_operations(&mut self, color: render_pass_operation::Enum, depth: render_pass_operation::Enum, stencil: render_pass_operation::Enum) -> &mut Self {
        self.color_operation = color;
        self.depth_operation = depth;
        self.stencil_operation = stencil;
        self
    }
}

pub struct RenderPassCreation<'a> {
    num_render_targets: u16,
    ty: render_pass_type::Enum,

    output_textures: [TextureHandle; K_MAX_IMAGE_OUTPUTS], // Assuming TextureHandle is defined elsewhere
    depth_stencil_texture: TextureHandle,

    scale_x: f32,
    scale_y: f32,
    resize: u8,

    color_operation: render_pass_operation::Enum,
    depth_operation: render_pass_operation::Enum,
    stencil_operation: render_pass_operation::Enum,

    name: Option<&'a str>,
}

impl<'a> RenderPassCreation<'a> {
    pub fn new() -> Self {
        RenderPassCreation {
            num_render_targets: 0,
            ty: render_pass_type::Enum::Geometry,
            output_textures: [TextureHandle::default(); K_MAX_IMAGE_OUTPUTS],
            depth_stencil_texture: TextureHandle::default(),
            scale_x: 1.0,
            scale_y: 1.0,
            resize: 1,
            color_operation: render_pass_operation::Enum::DontCare,
            depth_operation: render_pass_operation::Enum::DontCare,
            stencil_operation: render_pass_operation::Enum::DontCare,
            name: None,
        }
    }

    pub fn reset(&mut self) -> &mut Self {
        self.num_render_targets = 0;
        self.ty = render_pass_type::Enum::Geometry;
        self.output_textures = [TextureHandle::default(); K_MAX_IMAGE_OUTPUTS];
        self.depth_stencil_texture = TextureHandle::default();
        self.scale_x = 1.0;
        self.scale_y = 1.0;
        self.resize = 1;
        self.color_operation = render_pass_operation::Enum::DontCare;
        self.depth_operation = render_pass_operation::Enum::DontCare;
        self.stencil_operation = render_pass_operation::Enum::DontCare;
        self.name = None;
        self
    }

    pub fn add_render_texture(&mut self, texture: TextureHandle) -> &mut Self {
        if self.num_render_targets < K_MAX_IMAGE_OUTPUTS as u16 {
            self.output_textures[self.num_render_targets as usize] = texture;
            self.num_render_targets += 1;
        }
        self
    }

    pub fn set_scaling(&mut self, scale_x: f32, scale_y: f32, resize: u8) -> &mut Self {
        self.scale_x = scale_x;
        self.scale_y = scale_y;
        self.resize = resize;
        self
    }

    pub fn set_depth_stencil_texture(&mut self, texture: TextureHandle) -> &mut Self {
        self.depth_stencil_texture = texture;
        self
    }

    pub fn set_name(&mut self, name: &'a str) -> &mut Self {
        self.name = Some(name);
        self
    }

    pub fn set_type(&mut self, ty: render_pass_type::Enum) -> &mut Self {
        self.ty = ty;
        self
    }

    pub fn set_operations(&mut self, color: render_pass_operation::Enum, depth: render_pass_operation::Enum, stencil: render_pass_operation::Enum) -> &mut Self {
        self.color_operation = color;
        self.depth_operation = depth;
        self.stencil_operation = stencil;
        self
    }
}

pub struct PipelineCreation<'a> {
    rasterization: RasterizationCreation,
    depth_stencil: DepthStencilCreation,
    blend_state: BlendStateCreation,
    vertex_input: VertexInputCreation,
    shaders: ShaderStateCreation,
    render_pass: RenderPassOutput,
    descriptor_set_layout: [DescriptorSetLayoutHandle; K_MAX_DESCRIPTOR_SET_LAYOUTS], // Assuming K_MAX_DESCRIPTOR_SET_LAYOUTS is defined
    viewport: Option<&'a ViewportState>, // Assuming ViewportState is defined

    num_active_layouts: u32,
    name: Option<&'a str>,
}

impl<'a> PipelineCreation<'a> {
    pub fn new() -> Self {
        PipelineCreation {
            rasterization: RasterizationCreation::default(),
            depth_stencil: DepthStencilCreation::default(),
            blend_state: BlendStateCreation::default(),
            vertex_input: VertexInputCreation::new(),
            shaders: ShaderStateCreation::new(),
            render_pass: RenderPassOutput::new(),
            descriptor_set_layout: [DescriptorSetLayoutHandle::default(); K_MAX_DESCRIPTOR_SET_LAYOUTS], // Assuming DescriptorSetLayoutHandle has a default implementation
            viewport: None,
            num_active_layouts: 0,
            name: None,
        }
    }

    pub fn add_descriptor_set_layout(&mut self, handle: DescriptorSetLayoutHandle) -> &mut Self {
        if self.num_active_layouts < K_MAX_DESCRIPTOR_SET_LAYOUTS as u32 {
            self.descriptor_set_layout[self.num_active_layouts as usize] = handle;
            self.num_active_layouts += 1;
        }
        self
    }

    pub fn render_pass_output(&mut self) -> &mut RenderPassOutput {
        &mut self.render_pass
    }
}


pub mod TextureFormat {
    use ash::vk;
    #[inline]
    pub fn is_depth_stencil(value: vk::Format) -> bool {
        matches!(
            value,
            vk::Format::D16_UNORM_S8_UINT
                | vk::Format::D24_UNORM_S8_UINT
                | vk::Format::D32_SFLOAT_S8_UINT
        )
    }
    #[inline]
    pub fn is_depth_only(value: vk::Format) -> bool {
        (vk::Format::D16_UNORM..vk::Format::D32_SFLOAT).contains(&value)
    }
    #[inline]
    pub fn is_stencil_only(value: vk::Format) -> bool {
        value == vk::Format::S8_UINT
    }
    #[inline]
    pub fn has_depth(value: vk::Format) -> bool {
        ((vk::Format::D16_UNORM..vk::Format::S8_UINT).contains(&value))
            || ((vk::Format::D16_UNORM_S8_UINT..=vk::Format::D32_SFLOAT_S8_UINT).contains(&value))
    }
    #[inline]
    pub fn has_stencil(value: vk::Format) -> bool {
        (vk::Format::S8_UINT..=vk::Format::D32_SFLOAT_S8_UINT).contains(&value)
    }
    #[inline]
    pub fn has_depth_or_stencil(value: vk::Format) -> bool {
        (vk::Format::D16_UNORM..=vk::Format::D32_SFLOAT_S8_UINT).contains(&value)
    }
}

pub struct ResourceData {
    pub data: Option<*mut std::ffi::c_void>, // Using Option for nullability
}
impl Default for ResourceData{
    #[inline]
    fn default() -> Self {
        ResourceData::new()
    }
}
impl ResourceData {
    pub fn new() -> Self {
        ResourceData { data: None }
    }
}


pub struct ResourceBinding {
    pub ty: u16,    // ResourceType
    pub start: u16,
    pub count: u16,
    pub set: u16,
    pub name: Option<&'static str>,
}

impl Default for ResourceBinding {
    fn default() -> Self {
        ResourceBinding::new()
    }
}

impl ResourceBinding {
    pub fn new() -> Self {
        ResourceBinding {
            ty: 0,
            start: 0,
            count: 0,
            set: 0,
            name: None,
        }
    }
}

pub struct ShaderStateDescription<'a> {
    pub native_handle: Option<*mut std::ffi::c_void>,
    pub name: Option<&'a str>,
}

impl<'a> ShaderStateDescription<'a> {
    pub fn new() -> Self {
        ShaderStateDescription {
            native_handle: None,
            name: None,
        }
    }
}

pub struct BufferDescription<'a> {
    pub native_handle: Option<*mut std::ffi::c_void>, 
    pub name: Option<&'a str>,

    pub type_flags: vk::BufferUsageFlags,
    pub usage: resource_usage_type::Enum,
    pub size: u32,
    pub parent_handle: BufferHandle, // Assuming BufferHandle is defined
}

impl<'a> BufferDescription<'a> {
    pub fn new() -> Self {
        BufferDescription {
            native_handle: None,
            name: None,
            type_flags: vk::BufferUsageFlags::empty(),
            usage: resource_usage_type::Enum::Immutable,
            size: 0,
            parent_handle: BufferHandle::default(), 
        }
    }
}pub struct TextureDescription<'a> {
    pub native_handle: Option<*mut std::ffi::c_void>, // Using Option for nullability
    pub name: Option<&'a str>,

    pub width: u16,
    pub height: u16,
    pub depth: u16,
    pub mipmaps: u8,
    pub render_target: u8,
    pub compute_access: u8,

    pub format: vk::Format,
    pub ty: texture_type::Enum, // Assuming TextureType::Enum is defined
}

impl<'a> TextureDescription<'a> {
    pub fn new() -> Self {
        TextureDescription {
            native_handle: None,
            name: None,
            width: 1,
            height: 1,
            depth: 1,
            mipmaps: 1,
            render_target: 0,
            compute_access: 0,
            format: vk::Format::UNDEFINED,
            ty: texture_type::Enum::Texture2D, 
        }
    }
}


pub struct SamplerDescription<'a> {
    pub name: Option<&'a str>,
    pub min_filter: vk::Filter,
    pub mag_filter: vk::Filter,
    pub mip_filter: vk::SamplerMipmapMode,
    pub address_mode_u: vk::SamplerAddressMode,
    pub address_mode_v: vk::SamplerAddressMode,
    pub address_mode_w: vk::SamplerAddressMode,
}

impl<'a> Default for SamplerDescription<'a> {
    fn default() -> Self {
        SamplerDescription {
            name: None,
            min_filter: vk::Filter::NEAREST,
            mag_filter: vk::Filter::NEAREST,
            mip_filter: vk::SamplerMipmapMode::NEAREST,
            address_mode_u: vk::SamplerAddressMode::REPEAT,
            address_mode_v: vk::SamplerAddressMode::REPEAT,
            address_mode_w: vk::SamplerAddressMode::REPEAT,
        }
    }
}

pub struct DescriptorSetLayoutDescription {
    pub bindings: [ResourceBinding; K_MAX_DESCRIPTORS_PER_SET], // Assuming K_MAX_DESCRIPTORS_PER_SET is defined
    pub num_active_bindings: u32,
}

impl Default for DescriptorSetLayoutDescription {
    fn default() -> Self {
        DescriptorSetLayoutDescription {
            bindings: std::array::from_fn(|_|ResourceBinding::default()), // Assuming ResourceBinding has a default implementation
            num_active_bindings: 0,
        }
    }
}


pub struct DescriptorSetDescription {
    pub resources: [ResourceData; K_MAX_DESCRIPTORS_PER_SET], // Assuming K_MAX_DESCRIPTORS_PER_SET is defined
    pub num_active_resources: u32,
}

impl Default for DescriptorSetDescription {
    fn default() -> Self {
        DescriptorSetDescription {
            resources: std::array::from_fn(|_|ResourceData::default()), // Assuming ResourceData has a default implementation
            num_active_resources: 0,
        }
    }
}

pub struct PipelineDescription {
    pub shader: ShaderStateHandle, // Assuming ShaderStateHandle is defined
}


pub struct MapBufferParameters {
    pub buffer: BufferHandle, // Assuming BufferHandle is defined
    pub offset: u32,
    pub size: u32,
}

impl Default for MapBufferParameters {
    fn default() -> Self {
        MapBufferParameters {
            buffer: BufferHandle::default(), // Assuming BufferHandle has a default implementation
            offset: 0,
            size: 0,
        }
    }
}

pub struct ImageBarrier {
    pub texture: TextureHandle, // Assuming TextureHandle is defined
}

impl Default for ImageBarrier {
    fn default() -> Self {
        ImageBarrier {
            texture: TextureHandle::default(), // Assuming TextureHandle has a default implementation
        }
    }
}


pub struct MemoryBarrier {
    pub buffer: BufferHandle, // Assuming BufferHandle is defined
}

impl Default for MemoryBarrier {
    fn default() -> Self {
        MemoryBarrier {
            buffer: BufferHandle::default(), // Assuming BufferHandle has a default implementation
        }
    }
}

pub struct ExecutionBarrier {
    pub source_pipeline_stage: pipeline_stage::Enum, // Assuming PipelineStage::Enum is defined
    pub destination_pipeline_stage: pipeline_stage::Enum,

    pub new_barrier_experimental: u32,
    pub load_operation: u32,

    pub num_image_barriers: u32,
    pub num_memory_barriers: u32,

    pub memory_barriers: [MemoryBarrier; 8], // Assuming the size is fixed to 8
    pub image_barriers: [ImageBarrier; 8], // Assuming the size is fixed to 8
}

impl Default for ExecutionBarrier {
    fn default() -> Self {
        ExecutionBarrier {
            source_pipeline_stage: pipeline_stage::Enum::default(), // Assuming PipelineStage::Enum has a default implementation
            destination_pipeline_stage: pipeline_stage::Enum::default(),
            new_barrier_experimental: u32::MAX,
            load_operation: 0,
            num_image_barriers: 0,
            num_memory_barriers: 0,
            image_barriers: std::array::from_fn(|_|ImageBarrier::default()),
            memory_barriers: std::array::from_fn(|_|MemoryBarrier::default()),
        }
    }
}

impl ExecutionBarrier {
    pub fn reset(&mut self) -> &mut Self {
        self.num_image_barriers = 0;
        self.num_memory_barriers = 0;
        self
    }

    pub fn set(&mut self, source: pipeline_stage::Enum, destination: pipeline_stage::Enum) -> &mut Self {
        self.source_pipeline_stage = source;
        self.destination_pipeline_stage = destination;
        self
    }

    pub fn add_image_barrier(&mut self, image_barrier: ImageBarrier) -> &mut Self {
        if self.num_image_barriers < 8 {
            self.image_barriers[self.num_image_barriers as usize] = image_barrier;
            self.num_image_barriers += 1;
        }
        self
    }

    pub fn add_memory_barrier(&mut self, memory_barrier: MemoryBarrier) -> &mut Self {
        if self.num_memory_barriers < 8 {
            self.memory_barriers[self.num_memory_barriers as usize] = memory_barrier;
            self.num_memory_barriers += 1;
        }
        self
    }
}

