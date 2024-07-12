use std::{alloc::Allocator, cell::RefCell, os::raw::c_char, sync::{Arc, Mutex}};

use ash::vk;

use crate::fundamental::{ResourcePool, ResourcePoolTyped, StackAllocator, StringBuffer};

use super::{present_mode, queue_type, render_pass_operation, Buffer, BufferCreation, BufferDescription, BufferHandle, CommandBuffer, DescriptorSet, DescriptorSetCreation, DescriptorSetDescription, DescriptorSetHandle, DescriptorSetLayout, DescriptorSetLayoutCreation, DescriptorSetLayoutDescription, DescriptorSetLayoutHandle, DescriptorSetUpdate, ExecutionBarrier, MapBufferParameters, Pipeline, PipelineCreation, PipelineDescription, PipelineHandle, RenderPass, RenderPassCreation, RenderPassHandle, RenderPassOutput, ResourceHandle, ResourceUpdate, Sampler, SamplerCreation, SamplerDescription, SamplerHandle, ShaderState, ShaderStateCreation, ShaderStateDescription, ShaderStateHandle, Texture, TextureCreation, TextureDescription, TextureHandle, K_MAX_SWAPCHAIN_IMAGES};


#[repr(C)]
struct GPUTimestamp {
    start: u32,
    end: u32,
    elapsed_ms: f64,
    parent_index: u16,
    depth: u16,
    color: u32,
    frame_index: u32,
    name: Option<String>,
}

struct GPUTimestampManager {
    allocator: &'static dyn Allocator,
    timestamps: *mut GPUTimestamp,
    timestamps_data: *mut u64,
    queries_per_frame: u32,
    current_query: u32,
    parent_index: u32,
    depth: u32,
    current_frame_resolved: bool,
}

struct DeviceCreation {
    allocator: &'static dyn Allocator,
    temporary_allocator: &'static StackAllocator,
    window: *mut std::ffi::c_void,
    width: u16,
    height: u16,
    gpu_time_queries_per_frame: u16,
    enable_gpu_time_queries: bool,
    debug: bool,
}

pub struct GpuDevice{
    fullscreen_vertex_buffer: BufferHandle,
    swapchain_pass: RenderPassHandle,
    default_sampler: SamplerHandle,
    // Dummy resources
    dummy_texture: TextureHandle,
    dummy_constant_buffer: BufferHandle,
    swapchain_output: RenderPassOutput,
    string_buffer: StringBuffer,
    allocator: &'static dyn Allocator, // Placeholder for Allocator type
    temporary_allocator: &'static  StackAllocator, 
    dynamic_max_per_frame_size: u32,
    dynamic_buffer: BufferHandle,
    dynamic_mapped_memory: *mut u8,
    dynamic_allocated_size: u32,
    dynamic_per_frame_size: u32,
    queued_command_buffers: RefCell<Vec<Arc<CommandBuffer>>>,
    num_allocated_command_buffers: u32,
    num_queued_command_buffers: u32,
    present_mode: present_mode::Enum,
    current_frame: u32,
    previous_frame: u32,
    absolute_frame: u32,
    depth_texture: TextureHandle,
    buffers: ResourcePool,
    textures: ResourcePool,
    pipelines: ResourcePool,
    samplers: ResourcePool,
    descriptor_set_layouts: ResourcePool,
    descriptor_sets: ResourcePool,
	render_passes: Vec<Mutex<RenderPass>>,
    command_buffers: Vec<Mutex<CommandBuffer>>,
    shaders: ResourcePool,
    //vulkan_allocation_callbacks: vk::AllocationCallbacks,
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
    vulkan_timestamp_query_pool: vk::QueryPool,
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
    timestamps_enabled: bool,
    debug_utils_extension_present: bool,
}

impl GpuDevice {
    pub fn instance() -> *mut GpuDevice{
        todo!("Implement")
    }

    // Init/Terminate methods
    pub fn init(&mut self, creation: &DeviceCreation) {
        self.allocator  = creation.allocator;
        self.temporary_allocator = creation.temporary_allocator;

    }

    pub fn shutdown(&mut self) {
        todo!("Implement")
    }

    // Creation/Destruction of resources
    pub fn create_buffer(&mut self, creation: &BufferCreation) -> BufferHandle {
        todo!("Implement")
    }

    pub fn create_texture(&mut self, creation: &TextureCreation) -> TextureHandle {
        todo!("Implement")
    }

    pub fn create_pipeline(&mut self, creation: &PipelineCreation) -> PipelineHandle {
        todo!("Implement")
    }

    pub fn create_sampler(&mut self, creation: &SamplerCreation) -> SamplerHandle {
        todo!("Implement")
    }

    pub fn create_descriptor_set_layout(&mut self, creation: &DescriptorSetLayoutCreation) -> DescriptorSetLayoutHandle {
        todo!("Implement")
    }

    pub fn create_descriptor_set(&mut self, creation: &DescriptorSetCreation) -> DescriptorSetHandle {
        todo!("Implement")
    }

    pub fn create_render_pass(&mut self, creation: &RenderPassCreation) -> RenderPassHandle {
        todo!("Implement")
    }

    pub fn create_shader_state(&mut self, creation: &ShaderStateCreation) -> ShaderStateHandle {
        todo!("Implement")
    }

    pub fn destroy_buffer(&mut self, buffer: BufferHandle) {
        todo!("Implement")
    }

    pub fn destroy_texture(&mut self, texture: TextureHandle) {
        todo!("Implement")
    }

    pub fn destroy_pipeline(&mut self, pipeline: PipelineHandle) {
        todo!("Implement")
    }

    pub fn destroy_sampler(&mut self, sampler: SamplerHandle) {
        todo!("Implement")
    }

    pub fn destroy_descriptor_set_layout(&mut self, layout: DescriptorSetLayoutHandle) {
        todo!("Implement")
    }

    pub fn destroy_descriptor_set(&mut self, set: DescriptorSetHandle) {
        todo!("Implement")
    }

    pub fn destroy_render_pass(&mut self, render_pass: RenderPassHandle) {
        todo!("Implement")
    }

    pub fn destroy_shader_state(&mut self, shader: ShaderStateHandle) {
        todo!("Implement")
    }

    // Query Description
    pub fn query_buffer(&self, buffer: BufferHandle, out_description: &mut BufferDescription) {
        todo!("Implement")
    }

    pub fn query_texture(&self, texture: TextureHandle, out_description: &mut TextureDescription) {
        todo!("Implement")
    }

    pub fn query_pipeline(&self, pipeline: PipelineHandle, out_description: &mut PipelineDescription) {
        todo!("Implement")
    }

    pub fn query_sampler(&self, sampler: SamplerHandle, out_description: &mut SamplerDescription) {
        todo!("Implement")
    }

    pub fn query_descriptor_set_layout(&self, layout: DescriptorSetLayoutHandle, out_description: &mut DescriptorSetLayoutDescription) {
        todo!("Implement")
    }

    pub fn query_descriptor_set(&self, set: DescriptorSetHandle, out_description: &mut DescriptorSetDescription) {
        todo!("Implement")
    }

    pub fn query_shader_state(&self, shader: ShaderStateHandle, out_description: &mut ShaderStateDescription) {
        todo!("Implement")
    }

    pub fn get_render_pass_output(&self, render_pass: RenderPassHandle) -> Option<RenderPassOutput> {
        match self.access_render_pass(render_pass) {
            Some(render_pass) => Some(render_pass.lock().unwrap().output.clone()),
            None => None,
        }
    }

    // Update/Reload resources
    pub fn resize_output_textures(&mut self, render_pass: RenderPassHandle, width: u32, height: u32) {
        todo!("Implement")
    }

    pub fn update_descriptor_set(&mut self, set: DescriptorSetHandle) {
        todo!("Implement")
    }

    // Misc
    pub fn link_texture_sampler(&mut self, texture: TextureHandle, sampler: SamplerHandle) {
        todo!("Implement")
    }

    pub fn set_present_mode(&mut self, mode: present_mode::Enum) {
        todo!("Implement")
    }

    pub fn frame_counters_advance(&mut self) {
        todo!("Implement")
    }

    pub fn get_family_queue(&self, physical_device: vk::PhysicalDevice) -> bool {
        todo!("Implement")
    }

    pub fn compile_shader(&self, code: &str, code_size: u32, stage: vk::ShaderStageFlags, name: &str) -> vk::ShaderModuleCreateInfo {
        todo!("Implement")
    }

    // Swapchain
    pub fn create_swapchain(&mut self) {
        todo!("Implement")
    }

    pub fn destroy_swapchain(&mut self) {
        todo!("Implement")
    }

    pub fn resize_swapchain(&mut self) {
        todo!("Implement")
    }

    // Map/Unmap
    pub fn map_buffer(&mut self, parameters: &MapBufferParameters) -> *mut std::ffi::c_void {
        todo!("Implement")
    }

    pub fn unmap_buffer(&mut self, parameters: &MapBufferParameters) {
        todo!("Implement")
    }

    pub fn dynamic_allocate(&mut self, size: u32) -> *mut std::ffi::c_void {
        todo!("Implement")
    }

    pub fn set_buffer_global_offset(&mut self, buffer: BufferHandle, offset: u32) {
        todo!("Implement")
    }

    // Command Buffers
    pub fn get_command_buffer(&mut self, r#type: queue_type::Enum, begin: bool) -> *mut CommandBuffer {
        todo!("Implement")
    }

    pub fn get_instant_command_buffer(&mut self) -> *mut CommandBuffer {
        todo!("Implement")
    }

    pub fn queue_command_buffer(&mut self, command_buffer: CommandBuffer) {
        self.command_buffers.push(Mutex::new(command_buffer));
    }

    // Rendering
    pub fn new_frame(&mut self) {
        todo!("Implement")
    }

    pub fn present(&mut self) {
        todo!("Implement")
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        todo!("Implement")
    }

    pub fn set_presentation_mode(&mut self, mode: present_mode::Enum) {
        todo!("Implement")
    }

    pub fn fill_barrier(&mut self, render_pass: RenderPassHandle, out_barrier: &mut ExecutionBarrier) {
        todo!("Implement")
    }

    pub fn get_fullscreen_vertex_buffer(&self) -> BufferHandle {
        todo!("Implement")
    }

    pub fn get_swapchain_pass(&self) -> RenderPassHandle {
        todo!("Implement")
    }

    pub fn get_dummy_texture(&self) -> TextureHandle {
        todo!("Implement")
    }

    pub fn get_dummy_constant_buffer(&self) -> BufferHandle {
        todo!("Implement")
    }

    pub fn get_swapchain_output(&self) -> &RenderPassOutput {
        &self.swapchain_output
    }

    pub fn get_vulkan_render_pass(&self, output: &RenderPassOutput, name: &str) -> vk::RenderPass {
        todo!("Implement")
    }

    // Names and markers
    pub fn set_resource_name(&mut self, object_type: vk::ObjectType, handle: u64, name: &str) {
        todo!("Implement")
    }

    pub fn push_marker(&mut self, command_buffer: vk::CommandBuffer, name: &str) {
        todo!("Implement")
    }

    pub fn pop_marker(&mut self, command_buffer: vk::CommandBuffer) {
        todo!("Implement")
    }

    // GPU Timings
    pub fn set_gpu_timestamps_enable(&mut self, value: bool) {
        self.timestamps_enabled = value;
    }

    pub fn get_gpu_timestamps(&self, out_timestamps: &mut [GPUTimestamp]) -> u32 {
        todo!("Implement")
    }

    pub fn push_gpu_timestamp(&mut self, command_buffer: *mut CommandBuffer, name: &str) {
        todo!("Implement")
    }

    pub fn pop_gpu_timestamp(&mut self, command_buffer: *mut CommandBuffer) {
        todo!("Implement")
    }

    // Instant methods
    pub fn destroy_buffer_instant(&mut self, buffer: ResourceHandle) {
        todo!("Implement")
    }

    pub fn destroy_texture_instant(&mut self, texture: ResourceHandle) {
        todo!("Implement")
    }

    pub fn destroy_pipeline_instant(&mut self, pipeline: ResourceHandle) {
        todo!("Implement")
    }

    pub fn destroy_sampler_instant(&mut self, sampler: ResourceHandle) {
        todo!("Implement")
    }

    pub fn destroy_descriptor_set_layout_instant(&mut self, layout: ResourceHandle) {
        todo!("Implement")
    }

    pub fn destroy_descriptor_set_instant(&mut self, set: ResourceHandle) {
        todo!("Implement")
    }

    pub fn destroy_render_pass_instant(&mut self, render_pass: ResourceHandle) {
        todo!("Implement")
    }

    pub fn destroy_shader_state_instant(&mut self, shader: ResourceHandle) {
        todo!("Implement")
    }

    pub fn update_descriptor_set_instant(&mut self, update: &DescriptorSetUpdate) {
        todo!("Implement")
    }

    // Access methods
    pub fn access_shader_state(&self, shader: ShaderStateHandle) -> &ShaderState {
        todo!("Implement")
    }

    pub fn access_shader_state_mut(&mut self, shader: ShaderStateHandle) -> &mut ShaderState {
        todo!("Implement")
    }

    pub fn access_texture(&self, texture: TextureHandle) -> &Texture {
        todo!("Implement")
    }

    pub fn access_texture_mut(&mut self, texture: TextureHandle) -> &mut Texture {
        todo!("Implement")
    }

    pub fn access_buffer(&self, buffer: BufferHandle) -> &Buffer {
        todo!("Implement")
    }

    pub fn access_buffer_mut(&mut self, buffer: BufferHandle) -> &mut Buffer {
        todo!("Implement")
    }

    pub fn access_pipeline(&self, pipeline: PipelineHandle) -> &Pipeline {
        todo!("Implement")
    }

    pub fn access_pipeline_mut(&mut self, pipeline: PipelineHandle) -> &mut Pipeline {
        todo!("Implement")
    }

    pub fn access_sampler(&self, sampler: SamplerHandle) -> &Sampler {
        todo!("Implement")
    }

    pub fn access_sampler_mut(&mut self, sampler: SamplerHandle) -> &mut Sampler {
        todo!("Implement")
    }

    pub fn access_descriptor_set_layout(&self, layout: DescriptorSetLayoutHandle) -> &DescriptorSetLayout {
        todo!("Implement")
    }

    pub fn access_descriptor_set_layout_mut(&mut self, layout: DescriptorSetLayoutHandle) -> &mut DescriptorSetLayout {
        todo!("Implement")
    }

    pub fn access_descriptor_set(&self, set: DescriptorSetHandle) -> &DescriptorSet {
        todo!("Implement")
    }

    pub fn access_descriptor_set_mut(&mut self, set: DescriptorSetHandle) -> &mut DescriptorSet {
        todo!("Implement")
    }

    pub fn access_render_pass(&self, render_pass: RenderPassHandle) -> Option<&Mutex<RenderPass>> {
        return self.render_passes.get(render_pass.index as usize);
    }

}