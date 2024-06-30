pub(crate) mod color_write_enabled {
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum Enum {
        Red,
        Green,
        Blue,
        Alpha,
        All,
        Count,
    }

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum Mask {
        RedMask = 1 << 0,
        GreenMask = 1 << 1,
        BlueMask = 1 << 2,
        AlphaMask = 1 << 3,
        AllMask = (1 << 0) | (1 << 1) | (1 << 2) | (1 << 3),
    }

    pub const S_VALUE_NAMES: [&str; (Enum::Count as usize) + 1] = [
        "Red", "Green", "Blue", "Alpha", "All", "Count"
    ];

    pub fn to_string(e: Enum) -> &'static str {
        if (e as usize) < Enum::Count as usize {
            S_VALUE_NAMES[e as usize]
        } else {
            "unsupported"
        }
    }
}

pub(crate) mod cull_mode {
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum Enum {
        None,
        Front,
        Back,
        Count,
    }

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum Mask {
        NoneMask = 1 << 0,
        FrontMask = 1 << 1,
        BackMask = 1 << 2,
        CountMask = 1 << 3,
    }

    pub const S_VALUE_NAMES: [&str; (Enum::Count as usize) + 1] = [
        "None", "Front", "Back", "Count"
    ];

    pub fn to_string(e: Enum) -> &'static str {
        if (e as usize) < Enum::Count as usize {
            S_VALUE_NAMES[e as usize]
        } else {
            "unsupported"
        }
    }
}

pub(crate) mod depth_write_mask {
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum Enum {
        Zero,
        All,
        Count,
    }

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum Mask {
        ZeroMask = 1 << 0,
        AllMask = 1 << 1,
        CountMask = 1 << 2,
    }

    pub const S_VALUE_NAMES: [&str; (Enum::Count as usize) + 1] = [
        "Zero", "All", "Count"
    ];

    pub fn to_string(e: Enum) -> &'static str {
        if (e as usize) < Enum::Count as usize {
            S_VALUE_NAMES[e as usize]
        } else {
            "unsupported"
        }
    }
}

pub(crate) mod fill_mode {
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum Enum {
        Wireframe,
        Solid,
        Point,
        Count,
    }

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum Mask {
        WireframeMask = 1 << 0,
        SolidMask = 1 << 1,
        PointMask = 1 << 2,
        CountMask = 1 << 3,
    }

    pub const S_VALUE_NAMES: [&str; (Enum::Count as usize) + 1] = [
        "Wireframe", "Solid", "Point", "Count"
    ];

    pub fn to_string(e: Enum) -> &'static str {
        if (e as usize) < Enum::Count as usize {
            S_VALUE_NAMES[e as usize]
        } else {
            "unsupported"
        }
    }
}

pub(crate) mod front_clockwise {
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum Enum {
        True,
        False,
        Count,
    }

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum Mask {
        TrueMask = 1 << 0,
        FalseMask = 1 << 1,
        CountMask = 1 << 2,
    }

    pub const S_VALUE_NAMES: [&str; (Enum::Count as usize) + 1] = [
        "True", "False", "Count"
    ];

    pub fn to_string(e: Enum) -> &'static str {
        if (e as usize) < Enum::Count as usize {
            S_VALUE_NAMES[e as usize]
        } else {
            "unsupported"
        }
    }
}

pub(crate) mod stencil_operation {
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum Enum {
        Keep,
        Zero,
        Replace,
        IncrSat,
        DecrSat,
        Invert,
        Incr,
        Decr,
        Count,
    }

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum Mask {
        KeepMask = 1 << 0,
        ZeroMask = 1 << 1,
        ReplaceMask = 1 << 2,
        IncrSatMask = 1 << 3,
        DecrSatMask = 1 << 4,
        InvertMask = 1 << 5,
        IncrMask = 1 << 6,
        DecrMask = 1 << 7,
        CountMask = 1 << 8,
    }

    pub const S_VALUE_NAMES: [&str; (Enum::Count as usize) + 1] = [
        "Keep", "Zero", "Replace", "IncrSat", "DecrSat", "Invert", "Incr", "Decr", "Count"
    ];

    pub fn to_string(e: Enum) -> &'static str {
        if (e as usize) < Enum::Count as usize {
            S_VALUE_NAMES[e as usize]
        } else {
            "unsupported"
        }
    }
}

pub(crate) mod topology_type {
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum Enum {
        Unknown,
        Point,
        Line,
        Triangle,
        Patch,
        Count,
    }

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum Mask {
        UnknownMask = 1 << 0,
        PointMask = 1 << 1,
        LineMask = 1 << 2,
        TriangleMask = 1 << 3,
        PatchMask = 1 << 4,
        CountMask = 1 << 5,
    }

    pub const S_VALUE_NAMES: [&str; (Enum::Count as usize) + 1] = [
        "Unknown", "Point", "Line", "Triangle", "Patch", "Count"
    ];

    pub fn to_string(e: Enum) -> &'static str {
        if (e as usize) < Enum::Count as usize {
            S_VALUE_NAMES[e as usize]
        } else {
            "unsupported"
        }
    }
}

pub(crate) mod resource_usage_type {
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum Enum {
        Immutable,
        Dynamic,
        Stream,
        Count,
    }

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum Mask {
        ImmutableMask = 1 << 0,
        DynamicMask = 1 << 1,
        StreamMask = 1 << 2,
        CountMask = 1 << 3,
    }

    pub const S_VALUE_NAMES: [&str; (Enum::Count as usize) + 1] = [
        "Immutable", "Dynamic", "Stream", "Count"
    ];

    pub fn to_string(e: Enum) -> &'static str {
        if (e as usize) < Enum::Count as usize {
            S_VALUE_NAMES[e as usize]
        } else {
            "unsupported"
        }
    }
}

pub(crate) mod index_type {
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum Enum {
        Uint16,
        Uint32,
        Count,
    }

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum Mask {
        Uint16Mask = 1 << 0,
        Uint32Mask = 1 << 1,
        CountMask = 1 << 2,
    }

    pub const S_VALUE_NAMES: [&str; (Enum::Count as usize) + 1] = [
        "Uint16", "Uint32", "Count"
    ];

    pub fn to_string(e: Enum) -> &'static str {
        if (e as usize) < Enum::Count as usize {
            S_VALUE_NAMES[e as usize]
        } else {
            "unsupported"
        }
    }
}

pub(crate) mod texture_type {
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum Enum {
        Texture1D,
        Texture2D,
        Texture3D,
        Texture1DArray,
        Texture2DArray,
        TextureCubeArray,
        Count,
    }

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum Mask {
        Texture1DMask = 1 << 0,
        Texture2DMask = 1 << 1,
        Texture3DMask = 1 << 2,
        Texture1DArrayMask = 1 << 3,
        Texture2DArrayMask = 1 << 4,
        TextureCubeArrayMask = 1 << 5,
        CountMask = 1 << 6,
    }

    pub const S_VALUE_NAMES: [&str; (Enum::Count as usize) + 1] = [
        "Texture1D", "Texture2D", "Texture3D", "Texture1DArray", "Texture2DArray", "TextureCubeArray", "Count"
    ];

    pub fn to_string(e: Enum) -> &'static str {
        if (e as usize) < Enum::Count as usize {
            S_VALUE_NAMES[e as usize]
        } else {
            "unsupported"
        }
    }
}

pub(crate) mod vertex_component_format {
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum Enum {
        Float,
        Float2,
        Float3,
        Float4,
        Mat4,
        Byte,
        Byte4N,
        UByte,
        UByte4N,
        Short2,
        Short2N,
        Short4,
        Short4N,
        Uint,
        Uint2,
        Uint4,
        Count,
    }

    pub const S_VALUE_NAMES: [&str; (Enum::Count as usize) + 1] = [
        "Float", "Float2", "Float3", "Float4", "Mat4", "Byte", "Byte4N", "UByte", "UByte4N", "Short2", "Short2N", "Short4", "Short4N", "Uint", "Uint2", "Uint4", "Count"
    ];

    pub fn to_string(e: Enum) -> &'static str {
        if (e as usize) < Enum::Count as usize {
            S_VALUE_NAMES[e as usize]
        } else {
            "unsupported"
        }
    }
}

pub(crate) mod vertex_input_rate {
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum Enum {
        PerVertex,
        PerInstance,
        Count,
    }

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum Mask {
        PerVertexMask = 1 << 0,
        PerInstanceMask = 1 << 1,
        CountMask = 1 << 2,
    }

    pub const S_VALUE_NAMES: [&str; (Enum::Count as usize) + 1] = [
        "PerVertex", "PerInstance", "Count"
    ];

    pub fn to_string(e: Enum) -> &'static str {
        if (e as usize) < Enum::Count as usize {
            S_VALUE_NAMES[e as usize]
        } else {
            "unsupported"
        }
    }
}

pub(crate) mod logic_operation {
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum Enum {
        Clear,
        Set,
        Copy,
        CopyInverted,
        Noop,
        Invert,
        And,
        Nand,
        Or,
        Nor,
        Xor,
        Equiv,
        AndReverse,
        AndInverted,
        OrReverse,
        OrInverted,
        Count,
    }

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum Mask {
        ClearMask = 1 << 0,
        SetMask = 1 << 1,
        CopyMask = 1 << 2,
        CopyInvertedMask = 1 << 3,
        NoopMask = 1 << 4,
        InvertMask = 1 << 5,
        AndMask = 1 << 6,
        NandMask = 1 << 7,
        OrMask = 1 << 8,
        NorMask = 1 << 9,
        XorMask = 1 << 10,
        EquivMask = 1 << 11,
        AndReverseMask = 1 << 12,
        AndInvertedMask = 1 << 13,
        OrReverseMask = 1 << 14,
        OrInvertedMask = 1 << 15,
        CountMask = 1 << 16,
    }

    pub const S_VALUE_NAMES: [&str; (Enum::Count as usize) + 1] = [
        "Clear", "Set", "Copy", "CopyInverted", "Noop", "Invert", "And", "Nand", "Or", "Nor", "Xor", "Equiv", "AndReverse", "AndInverted", "OrReverse", "OrInverted", "Count"
    ];

    pub fn to_string(e: Enum) -> &'static str {
        if (e as usize) < Enum::Count as usize {
            S_VALUE_NAMES[e as usize]
        } else {
            "unsupported"
        }
    }
}

pub(crate) mod queue_type {
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum Enum {
        Graphics,
        Compute,
        CopyTransfer,
        Count,
    }

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum Mask {
        GraphicsMask = 1 << 0,
        ComputeMask = 1 << 1,
        CopyTransferMask = 1 << 2,
        CountMask = 1 << 3,
    }

    pub const S_VALUE_NAMES: [&str; (Enum::Count as usize) + 1] = [
        "Graphics", "Compute", "CopyTransfer", "Count"
    ];

    pub fn to_string(e: Enum) -> &'static str {
        if (e as usize) < Enum::Count as usize {
            S_VALUE_NAMES[e as usize]
        } else {
            "unsupported"
        }
    }
}

pub(crate) mod command_type {
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum Enum {
        BindPipeline,
        BindResourceTable,
        BindVertexBuffer,
        BindIndexBuffer,
        BindResourceSet,
        Draw,
        DrawIndexed,
        DrawInstanced,
        DrawIndexedInstanced,
        Dispatch,
        CopyResource,
        SetScissor,
        SetViewport,
        Clear,
        ClearDepth,
        ClearStencil,
        BeginPass,
        EndPass,
        Count,
    }

    pub const S_VALUE_NAMES: [&str; (Enum::Count as usize) + 1] = [
        "BindPipeline", "BindResourceTable", "BindVertexBuffer", "BindIndexBuffer", "BindResourceSet",
        "Draw", "DrawIndexed", "DrawInstanced", "DrawIndexedInstanced", "Dispatch", "CopyResource",
        "SetScissor", "SetViewport", "Clear", "ClearDepth", "ClearStencil", "BeginPass", "EndPass", "Count"
    ];

    pub fn to_string(e: Enum) -> &'static str {
        if (e as usize) < Enum::Count as usize {
            S_VALUE_NAMES[e as usize]
        } else {
            "unsupported"
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DeviceExtensions {
    DeviceExtensionsDebugCallback = 1 << 0,
}

pub(crate) mod texture_flags {
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum Enum {
        Default,
        RenderTarget,
        Compute,
        Count,
    }

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum Mask {
        DefaultMask = 1 << 0,
        RenderTargetMask = 1 << 1,
        ComputeMask = 1 << 2,
    }

    pub const S_VALUE_NAMES: [&str; (Enum::Count as usize) + 1] = [
        "Default", "RenderTarget", "Compute", "Count"
    ];

    pub fn to_string(e: Enum) -> &'static str {
        if (e as usize) < Enum::Count as usize {
            S_VALUE_NAMES[e as usize]
        } else {
            "unsupported"
        }
    }
}

pub(crate) mod pipeline_stage {
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum Enum {
        DrawIndirect = 0,
        VertexInput,
        VertexShader,
        FragmentShader,
        RenderTarget,
        ComputeShader,
        Transfer,
    }

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum Mask {
        DrawIndirectMask = 1 << 0,
        VertexInputMask = 1 << 1,
        VertexShaderMask = 1 << 2,
        FragmentShaderMask = 1 << 3,
        RenderTargetMask = 1 << 4,
        ComputeShaderMask = 1 << 5,
        TransferMask = 1 << 6,
    }
}

pub(crate) mod render_pass_type {
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum Enum {
        Geometry,
        Swapchain,
        Compute,
    }
}

pub(crate) mod resource_deletion_type {
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum Enum {
        Buffer,
        Texture,
        Pipeline,
        Sampler,
        DescriptorSetLayout,
        DescriptorSet,
        RenderPass,
        ShaderState,
        Count,
    }
}

pub(crate) mod present_mode {
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum Enum {
        Immediate,
        VSync,
        VSyncFast,
        VSyncRelaxed,
        Count,
    }
}

pub(crate) mod render_pass_operation {
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum Enum {
        DontCare,
        Load,
        Clear,
        Count,
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub(crate) enum ResourceState {
    RESOURCE_STATE_UNDEFINED = 0,
    RESOURCE_STATE_VERTEX_AND_CONSTANT_BUFFER = 0x1,
    RESOURCE_STATE_INDEX_BUFFER = 0x2,
    RESOURCE_STATE_RENDER_TARGET = 0x4,
    RESOURCE_STATE_UNORDERED_ACCESS = 0x8,
    RESOURCE_STATE_DEPTH_WRITE = 0x10,
    RESOURCE_STATE_DEPTH_READ = 0x20,
    RESOURCE_STATE_NON_PIXEL_SHADER_RESOURCE = 0x40,
    RESOURCE_STATE_PIXEL_SHADER_RESOURCE = 0x80,
    RESOURCE_STATE_SHADER_RESOURCE = 0x40 | 0x80,
    RESOURCE_STATE_STREAM_OUT = 0x100,
    RESOURCE_STATE_INDIRECT_ARGUMENT = 0x200,
    RESOURCE_STATE_COPY_DEST = 0x400,
    RESOURCE_STATE_COPY_SOURCE = 0x800,
    RESOURCE_STATE_GENERIC_READ = 0x1 | 0x2 | 0x40 | 0x80 | 0x200 | 0x800,
    RESOURCE_STATE_PRESENT = 0x1000,
    RESOURCE_STATE_COMMON = 0x2000,
    RESOURCE_STATE_RAYTRACING_ACCELERATION_STRUCTURE = 0x4000,
    RESOURCE_STATE_SHADING_RATE_SOURCE = 0x8000,
}
#[deny(non_camel_case_types)]

impl std::ops::BitOr for ResourceState {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        unsafe {
            std::mem::transmute(self as u16 | rhs as u16)
        }
    }
}


impl std::ops::BitAnd for ResourceState {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        unsafe {
            std::mem::transmute(self as u16 & rhs as u16)
        }
    }
}

impl ResourceState {
    pub fn from_bits(bits: u32) -> ResourceState {
        match bits {
            0 => ResourceState::RESOURCE_STATE_UNDEFINED,
            0x1 => ResourceState::RESOURCE_STATE_VERTEX_AND_CONSTANT_BUFFER,
            0x2 => ResourceState::RESOURCE_STATE_INDEX_BUFFER,
            0x4 => ResourceState::RESOURCE_STATE_RENDER_TARGET,
            0x8 => ResourceState::RESOURCE_STATE_UNORDERED_ACCESS,
            0x10 => ResourceState::RESOURCE_STATE_DEPTH_WRITE,
            0x20 => ResourceState::RESOURCE_STATE_DEPTH_READ,
            0x40 => ResourceState::RESOURCE_STATE_NON_PIXEL_SHADER_RESOURCE,
            0x80 => ResourceState::RESOURCE_STATE_PIXEL_SHADER_RESOURCE,
            0xC0 => ResourceState::RESOURCE_STATE_SHADER_RESOURCE,
            0x100 => ResourceState::RESOURCE_STATE_STREAM_OUT,
            0x200 => ResourceState::RESOURCE_STATE_INDIRECT_ARGUMENT,
            0x400 => ResourceState::RESOURCE_STATE_COPY_DEST,
            0x800 => ResourceState::RESOURCE_STATE_COPY_SOURCE,
            0xAC3 => ResourceState::RESOURCE_STATE_GENERIC_READ,
            0x1000 => ResourceState::RESOURCE_STATE_PRESENT,
            0x2000 => ResourceState::RESOURCE_STATE_COMMON,
            0x4000 => ResourceState::RESOURCE_STATE_RAYTRACING_ACCELERATION_STRUCTURE,
            0x8000 => ResourceState::RESOURCE_STATE_SHADING_RATE_SOURCE,
            _ => ResourceState::RESOURCE_STATE_UNDEFINED, // Handle unknown bits
        }
    }
    pub fn contains(&self, other: ResourceState) -> bool {
        (*self & other) == other
    }   
}