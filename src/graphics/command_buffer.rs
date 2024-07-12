use std::{sync::Mutex, sync::Arc};

use crate::graphics::{GpuDevice, RenderPass, Pipeline, BufferHandle, DescriptorSetHandle, ExecutionBarrier};
use ash::vk;

use super::{queue_type, PipelineHandle, RenderPassHandle, RenderPassOutput, ResourceHandle};
pub struct CommandBuffer {
    vk_command_buffer: vk::CommandBuffer, 
    device: Arc<Mutex<GpuDevice>>, 
    vk_descriptor_sets: [vk::DescriptorSet; 16], 

    current_render_pass: Option<RenderPassHandle>, 
    current_pipeline: Option<RenderPassHandle>, 
    clears: [vk::ClearValue; 2], 

    is_recording: bool,
    handle: u32,

    current_command: u32,
    resource_handle: ResourceHandle, // Replace with actual type if available

    type_: queue_type::Enum, // Replace with actual QueueType enum type
    buffer_size: u32,
    baked: bool,
}

impl CommandBuffer {
    pub fn new(device: Arc<Mutex<GpuDevice>>, handle: u32, buffer_size: u32, baked: bool) -> Self {
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
        {
            let device = self.device.lock().unwrap();
            if device.access_render_pass(handle).is_some() {
                self.current_render_pass = Some(handle);
            }
        }

    }
    pub fn get_current_render_pass_output(&self) -> Option<&RenderPassOutput> {
        if let Some(index) = self.current_render_pass {
            let device = self.device.lock().unwrap();
            device.get_render_pass_output(RenderPassHandle { index })
        } else {
            None
        }
    }
    pub fn bind_pipeline(&mut self, handle: PipelineHandle) {
        // Implement bind_pipeline functionality
    }
    
    pub fn terminate(&mut self) {
        self.is_recording = false;
    }

}