use std::collections::HashSet;

use vulkanalia::{
    Entry,
    Instance,
    prelude::v1_0::InstanceV1_0
};

use vulkanalia::vk;
use vulkanalia::vk::KhrSurfaceExtension;

use crate::rendering::RenderingQueueBuildError::{
    ErrorCode,
    ErrorMessage
};

use super::{
    SwapСhainSupport,
    REQUIRED_EXTENSIONS,
    RenderingQueueBuildError,
    LogicalDeviceBuilder
};

#[derive(Debug)]
pub struct QueueFamilyIndices{
    pub graphics: u32,
    pub present: u32
}
impl QueueFamilyIndices{
    pub fn create(
        instance: &Instance,
        device: &vk::PhysicalDevice,
        surface: &vk::SurfaceKHR
    ) -> Result<QueueFamilyIndices, RenderingQueueBuildError> {

        let queue_properties = unsafe {
            instance
                .get_physical_device_queue_family_properties(device.clone())
        };

        let graphics = find_present_queue_index(&queue_properties, instance, device, surface)
            .ok_or(ErrorMessage("Present queue family is not supported"))?;
        let present = find_queue_index(&queue_properties, vk::QueueFlags::GRAPHICS)
            .ok_or(ErrorMessage("Graphics queue family is not supported"))?;


        Result::Ok(Self {
            graphics,
            present
        })
    }

    pub fn get_unique_indices(&self) -> Vec<u32>{
        if self.graphics == self.present{
            vec![self.graphics]
        } else {
            vec![self.graphics, self.present]
        }
    }
}

pub struct PhysicalDeviceBuilder{
    pub entry: Box<Entry>,
    pub instance: Box<Instance>,
    pub messenger: Option<Box<vk::DebugUtilsMessengerEXT>>,
    pub surface: Box<vk::SurfaceKHR>,
}

impl PhysicalDeviceBuilder {
    pub fn choose_physical_device(self)
        -> Result<LogicalDeviceBuilder, RenderingQueueBuildError>
    {
        let devices =  unsafe {
            self.instance
                .enumerate_physical_devices()
                .map_err(|err| ErrorCode(err))?
        };

        for device in devices{
            let queue_families = QueueFamilyIndices::create(&self.instance, &device, &self.surface)?;
            let swap_chain_support = SwapСhainSupport::create(
                &self.instance,
                &self.surface,
                &device,
            )?;

            if check_device_suitable(&self.instance, &device, &swap_chain_support).is_ok() {
                return Result::Ok(LogicalDeviceBuilder {
                    entry: self.entry,
                    messenger: self.messenger,
                    instance: self.instance,
                    physical_device: Box::new(device),
                    surface: self.surface,
                    queue_families: queue_families,
                    swap_chain_support: Box::new(swap_chain_support)
                })
            }
        }
        Result::Err(ErrorMessage("Supported device not found"))
    }
}

fn check_device_suitable(
    instance: &Instance,
    device: &vk::PhysicalDevice,
    swap_chain_support: &SwapСhainSupport
) ->  Result<(), RenderingQueueBuildError>
{
    unsafe {
        check_physical_device(instance, device)?;
        check_extensions_support(instance, device)?;
        check_swap_chain_support(swap_chain_support)?;
    }

    Result::Ok(())
}

unsafe fn check_swap_chain_support(
    swap_chain_support: &SwapСhainSupport
) ->  Result<(), RenderingQueueBuildError>
{

    if swap_chain_support.formats.is_empty() || swap_chain_support.present_modes.is_empty(){
        return Result::Err(ErrorMessage("swap chain is not supported"))
    }

    Result::Ok(())
}

unsafe fn check_physical_device(
    instance: &Instance,
    device: &vk::PhysicalDevice
)->  Result<(), RenderingQueueBuildError>
{
    //Имя, тип, поддерживаемая версия вулкан
    let device_properties = instance
        .get_physical_device_properties(device.clone());
    //Поддержка сжатия текстур,  64- битные переоды,
    //Ренедринг с несколькими видовыми экранами
    let device_features = instance
        .get_physical_device_features(device.clone());

    if device_properties.device_type != vk::PhysicalDeviceType::DISCRETE_GPU {
        return Result::Err(ErrorMessage("device is not GPU."));
    }
    if device_features.geometry_shader != vk::TRUE{
        return Result::Err(ErrorMessage("missing geometry shaders support."));
    }

    Result::Ok(())
}

unsafe fn check_extensions_support(
    instance: &Instance,
    device: &vk::PhysicalDevice
) -> Result<(), RenderingQueueBuildError>
{
    let extensions = instance
        .enumerate_device_extension_properties(device.clone(), None)
        .map_err(|error|ErrorMessage("сouldn't get extensions"))?;

    let extensions = extensions
        .iter()
        .map(|extension| extension.extension_name)
        .collect::<HashSet<_>>();

    if REQUIRED_EXTENSIONS.iter().all(|name|extensions.contains(name)) {
        Result::Ok(())
    }
    else {
        Result::Err(ErrorMessage("missing required device extensions"))
    }
}

fn find_queue_index(
    queue_family_properties: &Vec<vk::QueueFamilyProperties>,
    flags: vk::QueueFlags
) -> Option<u32>
{
    queue_family_properties
        .iter()
        .position(|propery|
            propery.queue_flags
                .contains(
                    flags
                )
        ).map(|index| index as u32)
}

fn find_present_queue_index(
    queue_family_properties: &Vec<vk::QueueFamilyProperties>,
    instance: &Instance,
    device: &vk::PhysicalDevice,
    surface: &vk::SurfaceKHR
) -> Option<u32>
{
    let properties_enum = queue_family_properties
        .iter()
        .enumerate();

    for (index, properties) in properties_enum {
        let surface_support = unsafe {
            instance.get_physical_device_surface_support_khr(
                device.clone(),
                index as u32,
                surface.clone()
            )
        };

        if surface_support.is_err(){
            break
        }

        let surface_support = surface_support.unwrap();
        if surface_support {
            return Some(index as u32);
        }
    }

    return None;
}
