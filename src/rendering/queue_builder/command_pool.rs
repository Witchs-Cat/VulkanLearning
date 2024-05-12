use vulkanalia::{Device, Entry, Instance, vk};
use vulkanalia::vk::{DeviceV1_0, HasBuilder, RenderPass};
use crate::rendering::queue_builder::command_buffer::CommandBufferBuildStage;
use crate::rendering::RenderingError::CreateCommandPoolError;
use crate::rendering::RqResult;

pub struct CommandPoolBuildStage {
    pub entry: Box<Entry>,
    pub instance: Box<Instance>,
    pub messenger: Option<vk::DebugUtilsMessengerEXT>,
    pub physical_device: vk::PhysicalDevice,
    pub logical_device: Box<Device>,
    pub queue_families: super::QueueFamilyIndices,
    pub surface: vk::SurfaceKHR,
    pub swap_chain: Box<super::SwapChainData>,
    pub render_pass: RenderPass,
    pub pipeline: vk::Pipeline,
    pub framebuffers: Vec<vk::Framebuffer>
}

impl CommandPoolBuildStage {
    pub fn create_command_pool(self) -> RqResult<CommandBufferBuildStage>{
        let command_pool_info = vk::CommandPoolCreateInfo::builder()
            .flags(vk::CommandPoolCreateFlags::empty())
            .queue_family_index(self.queue_families.graphics);

        let command_pool = unsafe {
            self.logical_device.create_command_pool(&command_pool_info, None)
                .map_err(|err|CreateCommandPoolError(err))?
        };

        Result::Ok(CommandBufferBuildStage{
            entry: self.entry,
            instance: self.instance,
            messenger: self.messenger,
            physical_device: self.physical_device,
            logical_device: self.logical_device,
            queue_families: self.queue_families,
            surface: self.surface,
            swap_chain: self.swap_chain,
            render_pass: self.render_pass,
            pipeline: self.pipeline,
            framebuffers: self.framebuffers,
            command_pool
        })
    }
}