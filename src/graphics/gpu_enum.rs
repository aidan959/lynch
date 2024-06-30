mod color_write_enabled {
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

mod cull_mode {
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

mod depth_write_mask {
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

mod fill_mode {
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