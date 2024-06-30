use std::os::raw::c_char;

use vk_mem::Allocator;

use ash::vk;

use crate::fundamental::{StackAllocator, StringBuffer};

use super::{present_mode, BufferHandle, CommandBuffer, DescriptorSetUpdate, RenderPassHandle, RenderPassOutput, ResourceUpdate, SamplerHandle, TextureHandle, K_MAX_SWAPCHAIN_IMAGES};


#[repr(C)]
struct GPUTimestamp {
    start: u32,
    end: u32,
    elapsed_ms: f64,
    parent_index: u16,
    depth: u16,
    color: u32,
    frame_index: u32,
    name: *const c_char,
}

struct GPUTimestampManager {
    allocator: *mut Allocator,
    timestamps: *mut GPUTimestamp,
    timestamps_data: *mut u64,
    queries_per_frame: u32,
    current_query: u32,
    parent_index: u32,
    depth: u32,
    current_frame_resolved: bool,
}

struct DeviceCreation {
    allocator: *mut Allocator,
    temporary_allocator: *mut StackAllocator,
    window: *mut std::ffi::c_void,
    width: u16,
    height: u16,
    gpu_time_queries_per_frame: u16,
    enable_gpu_time_queries: bool,
    debug: bool,
}

pub struct GpuDevice<'a> {
    fullscreen_vertex_buffer: BufferHandle,
    swapchain_pass: RenderPassHandle,
    default_sampler: SamplerHandle,
    // Dummy resources
    dummy_texture: TextureHandle,
    dummy_constant_buffer: BufferHandle,
    swapchain_output: RenderPassOutput,
    string_buffer: StringBuffer,
    allocator: *mut Allocator, // Placeholder for Allocator type
    temporary_allocator: *mut StackAllocator, // Placeholder for StackAllocator type
    dynamic_max_per_frame_size: u32,
    dynamic_buffer: BufferHandle,
    dynamic_mapped_memory: *mut u8,
    dynamic_allocated_size: u32,
    dynamic_per_frame_size: u32,
    queued_command_buffers: RefCell<Vec<Arc<CommandBuffer<'a>>,
    num_allocated_command_buffers: u32,
    num_queued_command_buffers: u32,
    present_mode: present_mode::Enum,
    current_frame: u32,
    previous_frame: u32,
    absolute_frame: u32,
    depth_texture: TextureHandle,
    vulkan_allocation_callbacks: vk::AllocationCallbacks<'a>,
    vulkan_instance: vk::Instance,
    vulkan_physical_device: vk::PhysicalDevice,
    vulkan_physical_properties: vk::PhysicalDeviceProperties,
    vulkan_device: vk::Device,
    vulkan_queue: vk::Queue,
    vulkan_queue_family: u32,
    vulkan_descriptor_pool: vk::DescriptorPool,
    vulkan_swapchain_images: [vk::Image; K_MAX_SWAPCHAIN_IMAGES],
    vulkan_swapchain_image_views: [vk::ImageView; K_MAX_SWAPCHAIN_IMAGES],
    vulkan_swapchain_framebuffers: [vk::Framebuffer; K_MAX_SWAPCHAIN_IMAGES],
    vulkan_timestamp_query_pool: *mut vk::QueryPool,
    vulkan_render_complete_semaphore: [vk::Semaphore; K_MAX_SWAPCHAIN_IMAGES],
    vulkan_image_acquired_semaphore: vk::Semaphore,
    vulkan_command_buffer_executed_fence: [vk::Fence; K_MAX_SWAPCHAIN_IMAGES],
    vulkan_window_surface: vk::SurfaceKHR,
    vulkan_surface_format: vk::SurfaceFormatKHR,
    vulkan_present_mode: vk::PresentModeKHR,
    vulkan_swapchain: vk::SwapchainKHR,
    vulkan_swapchain_image_count: u32,
    vulkan_debug_callback: vk::DebugReportCallbackEXT,
    vulkan_debug_utils_messenger: vk::DebugUtilsMessengerEXT,
    vulkan_image_index: u32,
    vma_allocator: vk_mem::Allocator,
    resource_deletion_queue: Vec<ResourceUpdate>,
    descriptor_set_updates: Vec<DescriptorSetUpdate>,
    gpu_timestamp_frequency: f32,
    gpu_timestamp_reset: bool,
    debug_utils_extension_present: bool,
    vulkan_binaries_path: [c_char; 512], // Adjust size as needed
}