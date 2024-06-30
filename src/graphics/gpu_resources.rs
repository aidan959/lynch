use ash::vk;

const K_INVALID_INDEX: u32 = 0xffffffff;

type ResourceHandle = u32;

#[derive(Debug, Copy, Clone)]
struct BufferHandle {
    index: ResourceHandle,
}

#[derive(Debug, Copy, Clone)]
struct TextureHandle {
    index: ResourceHandle,
}

#[derive(Debug, Copy, Clone)]
struct ShaderStateHandle {
    index: ResourceHandle,
}

#[derive(Debug, Copy, Clone)]
struct SamplerHandle {
    index: ResourceHandle,
}

#[derive(Debug, Copy, Clone)]
struct DescriptorSetLayoutHandle {
    index: ResourceHandle,
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


const K_MAX_IMAGE_OUTPUTS: u8 = 8;               // Maximum number of images/render_targets/fbo attachments usable.
const K_MAX_DESCRIPTOR_SET_LAYOUTS: u8 = 8;      // Maximum number of layouts in the pipeline.
const K_MAX_SHADER_STAGES: u8 = 5;               // Maximum simultaneous shader stages. Applicable to all different types of pipelines.
const K_MAX_DESCRIPTORS_PER_SET: u8 = 16;        // Maximum list elements for both descriptor set layout and descriptor sets.
const K_MAX_VERTEX_STREAMS: u8 = 16;
const K_MAX_VERTEX_ATTRIBUTES: u8 = 16;

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
    color_write_mask: ColorWriteEnabledMask,
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
            color_write_mask: ColorWriteEnabledMask::ALL,
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

    fn set_color_write_mask(&mut self, value: ColorWriteEnabledMask) -> &mut Self {
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
    cull_mode: vk::CullModeFlagBits,
    front: vk::FrontFace,
    fill: FillMode,
}

impl Default for RasterizationCreation {
    fn default() -> Self {
        RasterizationCreation {
            cull_mode: vk::CullModeFlagBits::NONE,
            front: vk::FrontFace::COUNTER_CLOCKWISE,
            fill: FillMode::SOLID,
        }
    }
}

#[derive(Debug, Clone)]
struct BufferCreation {
    type_flags: vk::BufferUsageFlags,
    usage: ResourceUsageType,
    size: u32,
    initial_data: Option<*mut std::ffi::c_void>,
    name: Option<String>,
}

impl Default for BufferCreation {
    fn default() -> Self {
        BufferCreation {
            type_flags: 0,
            usage: ResourceUsageType::IMMUTABLE,
            size: 0,
            initial_data: None,
            name: None,
        }
    }
}

impl BufferCreation {
    fn reset(&mut self) -> &mut Self {
        self.type_flags = 0;
        self.usage = ResourceUsageType::IMMUTABLE;
        self.size = 0;
        self.initial_data = None;
        self.name = None;
        self
    }

    fn set(&mut self, flags: vk::BufferUsageFlags, usage: ResourceUsageType, size: u32) -> &mut Self {
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
    texture_type: TextureType,
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
            texture_type: TextureType::TEXTURE_2D,
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

    fn set_format_type(&mut self, format: vk::Format, texture_type: TextureType) -> &mut Self {
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