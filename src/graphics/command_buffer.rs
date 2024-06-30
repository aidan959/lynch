use crate::graphics::{GpuDevice, RenderPass, Pipeline, BufferHandle, DescriptorSetHandle, ExecutionBarrier};
use ash::vk;

use super::{queue_type, RenderPassHandle, ResourceHandle};
pub struct CommandBuffer<'a> {
    vk_command_buffer: vk::CommandBuffer, 
    device: &'a mut GpuDevice, 
    vk_descriptor_sets: [vk::DescriptorSet; 16], 

    current_render_pass: Option<&'a mut RenderPass>, 
    current_pipeline: Option<&'a mut Pipeline<'a>>, 
    clears: [vk::ClearValue; 2], 

    is_recording: bool,
    handle: u32,

    current_command: u32,
    resource_handle: ResourceHandle, // Replace with actual type if available

    type_: queue_type::Enum, // Replace with actual QueueType enum type
    buffer_size: u32,
    baked: bool,
}

impl<'a> CommandBuffer<'a> {
    pub fn new(device: &'a mut GpuDevice, handle: u32, buffer_size: u32, baked: bool) -> Self {
        // Initialize Vulkan command buffer, descriptor sets, and other members
        CommandBuffer {
            vk_command_buffer: unsafe { std::mem::zeroed() }, 
            device,
            vk_descriptor_sets: [unsafe { std::mem::zeroed() }; 16], 

            current_render_pass: None,
            current_pipeline: None,
            clears: [unsafe { std::mem::zeroed() }; 2], 

            is_recording: false,
            handle,

            current_command: 0,
            resource_handle: ResourceHandle::default(),

            type_: queue_type::Enum::Graphics, 
            buffer_size,
            baked,
        }
    }

    pub fn bind_pass(&mut self, handle: RenderPassHandle) {
        self.is_recording = true;
        self.current_render_pass = self.device.get_render_pass(handle);


    }

    pub fn bind_pipeline(&mut self, handle: PipelineHandle) {
        // Implement bind_pipeline functionality
    }
    
    pub fn terminate(&mut self) {
        self.is_recording = false;
    }

}