#[cfg(feature = "vl")]
use validation_layers::{get_validation_layers, DebugUtils};

mod instance;
mod utility;

// validation layers module will only exist if validation layers are enabled
#[cfg(feature = "vl")]
mod validation_layers;

#[cfg(all(feature = "link", feature = "load"))]
compile_error!(
  "\
    Features \"link\" and \"load\" \
    were included at the same time. \
    Choose between \"load\" to load the Vulkan library \
    at runtime or \"link\" to link it while building the binary."
);

#[allow(unreachable_code)]
unsafe fn get_entry() -> ash::Entry {
  #[cfg(feature = "link")]
  return ash::Entry::linked();

  #[cfg(feature = "load")]
  return match ash::Entry::load() {
    Ok(entry) => entry,
    Err(err) => match err {
      ash::LoadingError::MissingEntryPoint(missing_entry_error) => {
        panic!(
          "Missing entry point when loading Vulkan library: {}",
          missing_entry_error
        )
      }
      ash::LoadingError::LibraryLoadFailure(load_error) => {
        panic!("Failed to load Vulkan Library: {:?}", load_error)
      }
    },
  };

  // panic will only happen if neither of the features are enabled
  panic!(
    "No compile feature was included for accessing the Vulkan library.\n\
    Choose between \"load\" to load the Vulkan library \
    at runtime or \"link\" to link it while building the binary."
  );
}

fn main() {
  env_logger::init();

  let entry: ash::Entry = unsafe { get_entry() };

  #[cfg(feature = "vl")]
  let (instance, mut debug_utils) = {
    let validation_layers = get_validation_layers(&entry);
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
