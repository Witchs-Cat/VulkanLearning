use std::collections::LinkedList;
use std::env;
use std::ops::Range;
use log::{debug, info};
use winit::raw_window_handle::{
    HasDisplayHandle,
    HasWindowHandle
};
use vulkanalia::Entry;
use vulkanalia::loader::{
    LibloadingLoader,
    LIBRARY
};

use vulkanalia::{
    Instance,
    Device
};
use vulkanalia::bytecode::Bytecode;
use vulkanalia::window::create_surface;
use vulkanalia::vk;
use vulkanalia::vk::{
    DeviceV1_0,
    InstanceV1_0,
    ExtDebugUtilsExtension,
    Handle,
    HasBuilder,
    KhrSurfaceExtension,
    KhrSwapchainExtension
};
use winit::dpi::PhysicalSize;
use crate::rendering::RenderingError::CreatePipeLineError;

use super::shaders::Shader;
use super::{
    RenderingPipelineConfig,
    RenderingQueueBuildError,
    QueueFamilyIndices,
    RenderingError,
    SwapChainData,
    get_debug_info
};

#[derive(Debug)]
pub struct RenderingQueue {
    entry: Box<Entry>,
    instance: Box<Instance>,
    messenger: Option<Box<vk::DebugUtilsMessengerEXT>>,
    physical_device: Box<vk::PhysicalDevice>,
    logical_device: Box<Device>,
    queue_families: QueueFamilyIndices,
    surface: Box<vk::SurfaceKHR>,
    swap_chain: Box<SwapChainData>
}

impl RenderingQueue {

    pub fn new (
        entry: Box<Entry>,
        instance: Box<Instance>,
        messenger: Option<Box<vk::DebugUtilsMessengerEXT>>,
        physical_device: Box<vk::PhysicalDevice>,
        logical_device: Box<Device>,
        queue_families:QueueFamilyIndices,
        surface: Box<vk::SurfaceKHR>,
        swap_chain: Box<SwapChainData>
    ) -> RenderingQueue
    {
        return RenderingQueue {
            entry,
            instance,
            messenger,
            physical_device,
            logical_device,
            queue_families,
            surface,
            swap_chain
        }
    }

    pub fn create<TWindow>(
        config: &RenderingPipelineConfig<&TWindow>
    ) -> Result<RenderingQueue, RenderingQueueBuildError>
    where TWindow: HasWindowHandle+HasDisplayHandle
    {
        let now = std::time::Instant::now();

        let pipeline = Self::builder()
            .create_entry()?;

        let elapsed = now.elapsed();
        info!("Entry creation duration: {:?}", elapsed);

        let now = std::time::Instant::now();

        let pipeline= pipeline.create_instance(
            &config.window,
            config.use_validation_layer
        )?;

        let elapsed = now.elapsed();
        info!("Instance creation duration: {:?}", elapsed);

        let now = std::time::Instant::now();
        let pipeline = pipeline.choose_physical_device()?;

        let elapsed = now.elapsed();
        info!("Physical device creation duration: {:?}", elapsed);

        let now = std::time::Instant::now();
        let pipeline = pipeline.create_logical_device(
            config.use_validation_layer
        )?;

        let elapsed = now.elapsed();
        info!("Logical device creation duration: {:?}", elapsed);


        let now = std::time::Instant::now();
        let pipeline = pipeline.create_swap_chain(
            &config.rendering_resolution,
            vk::SwapchainKHR::null()
        )?;

        let elapsed = now.elapsed();
        info!("Swap chain creation duration: {:?}", elapsed);

        let now = std::time::Instant::now();
        let pipeline = pipeline.build();

        Result::Ok(pipeline)
    }
}

impl Drop for RenderingQueue {
    fn drop(&mut self){
        unsafe {
            if let Some(messenger) = &self.messenger {
                self.instance.destroy_debug_utils_messenger_ext(**messenger, None);
            }

            // self.logical_device.destroy_pipeline();

            for image_view in &self.swap_chain.image_views{
                self.logical_device.destroy_image_view(*image_view, None);
            }

            self.logical_device.destroy_swapchain_khr(self.swap_chain.swap_chain, None);

            self.instance.destroy_surface_khr(*self.surface, None);
            self.logical_device.destroy_device(None);
            self.instance.destroy_instance(None);
        }
        debug!("instance destroyed");
    }
}