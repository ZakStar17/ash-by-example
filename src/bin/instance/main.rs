mod entry;
mod instance;
mod utility;

// validation layers module will only exist if validation layers are enabled
#[cfg(feature = "vl")]
mod validation_layers;

use ash::vk;
use std::ffi::CStr;
#[cfg(feature = "vl")]
use validation_layers::DebugUtils;

// simple macro to transmute literals to static CStr
macro_rules! cstr {
  ( $s:literal ) => {{
    unsafe { std::mem::transmute::<_, &CStr>(concat!($s, "\0")) }
  }};
}

// array of validation layers that should be loaded
// validation layers names should be valid cstrings (not contain null bytes nor invalid characters)
pub const VALIDATION_LAYERS: [&'static CStr; 1] = [cstr!("VK_LAYER_KHRONOS_validation")];

// Vulkan API version required to run the program
// In your case you may request a optimal version of the API in order to use specific features
// but fallback to an older version if the target is not supported by the driver or any physical
// device
pub const TARGET_API_VERSION: u32 = vk::API_VERSION_1_3;

// somewhat arbitrary
pub const APPLICATION_NAME: &'static CStr = cstr!("Vulkan Instance creation");
pub const APPLICATION_VERSION: u32 = vk::make_api_version(0, 1, 0, 0);

fn main() {
  env_logger::init();

  let entry: ash::Entry = unsafe { entry::get_entry() };

  #[cfg(feature = "vl")]
  let (instance, mut debug_utils) = {
    let validation_layers = validation_layers::get_supported_validation_layers(&entry);
    // valid for as long as "validation_layers"
    let vl_pointers: Vec<*const std::ffi::c_char> =
      validation_layers.iter().map(|name| name.as_ptr()).collect();

    let debug_create_info = DebugUtils::get_debug_messenger_create_info();
    let instance = instance::create_instance(&entry, &vl_pointers, &debug_create_info);
    let debug_utils = DebugUtils::setup(&entry, &instance, debug_create_info);

    (instance, debug_utils)
  };

  #[cfg(not(feature = "vl"))]
  let instance = instance::create_instance(&entry);

  println!("Created instance successfully!");

  // Cleanup
  unsafe {
    #[cfg(feature = "vl")]
    {
      log::debug!("Destroying debug utils messenger");
      debug_utils.destroy_self();
    }

    log::debug!("Destroying instance");
    instance.destroy_instance(None);
  }
}
