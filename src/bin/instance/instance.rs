use ash::vk;
use std::{
  ffi::CString,
  ptr::{self, addr_of},
};

#[cfg(feature = "vl")]
use std::os::raw::{c_char, c_void};

use crate::{
  utility, ADDITIONAL_VALIDATION_FEATURES, APPLICATION_NAME, APPLICATION_VERSION,
  TARGET_API_VERSION,
};

// Checks if all required extensions exist and are supported by the host system
// If found returns a list of required but not available extensions as an error
fn test_instance_extension_support<'a>(
  entry: &ash::Entry,
  extensions: &'a Vec<&'a str>,
) -> Result<(), Vec<&'a &'a str>> {
  let required = extensions;
  let mut available: Vec<String> = entry
    .enumerate_instance_extension_properties(None)
    .unwrap() // should only fail if out of memory
    .iter()
    .filter_map(
      |props| match utility::i8_array_to_string(&props.extension_name) {
        Ok(s) => Some(s),
        Err(_) => {
          log::warn!(
            "There exists an available extension with an invalid name that couldn't be decoded"
          );
          None
        }
      },
    )
    .collect();

  log::debug!("Available instance extensions: {:?}", available);

  let unavailable = utility::not_in_string_slice(available.as_mut_slice(), &mut required.iter());
  if unavailable.is_empty() {
    Ok(())
  } else {
    Err(unavailable)
  }
}

// the function expects any pointers to be valid
pub fn create_instance(
  entry: &ash::Entry,
  #[cfg(feature = "vl")] vl_pointers: &Vec<*const c_char>,
  #[cfg(feature = "vl")] debug_create_info: &vk::DebugUtilsMessengerCreateInfoEXT,
) -> ash::Instance {
  let max_supported_version = match entry.try_enumerate_instance_version() {
    // Vulkan 1.1+
    Ok(opt) => match opt {
      Some(version) => version,
      None => vk::API_VERSION_1_0,
    },
    // Vulkan 1.0
    Err(_) => vk::API_VERSION_1_0,
  };

  log::info!(
    "Vulkan library max supported version: {}",
    utility::parse_vulkan_api_version(max_supported_version)
  );

  if max_supported_version < TARGET_API_VERSION {
    panic!("Vulkan implementation API maximum supported version is less than the one targeted by the application.");
  }

  let app_info = vk::ApplicationInfo {
    s_type: vk::StructureType::APPLICATION_INFO,
    api_version: TARGET_API_VERSION,
    p_application_name: APPLICATION_NAME.as_ptr(),
    application_version: APPLICATION_VERSION,
    p_engine_name: ptr::null(),
    engine_version: vk::make_api_version(0, 1, 0, 0),
    p_next: ptr::null(),
  };

  #[allow(unused_mut)]
  let mut required_extensions = Vec::with_capacity(1);
  #[cfg(feature = "vl")]
  required_extensions.push(ash::extensions::ext::DebugUtils::name().to_str().unwrap());

  log::info!(
    "Required instance extensions by the application: {:?}",
    required_extensions
  );

  test_instance_extension_support(entry, &required_extensions).unwrap_or_else(|unavailable| {
    panic!(
      "Some unavailable instance extensions are strictly required: {:?}",
      unavailable
    )
  });

  // required to be alive until the end of instance creation
  let required_extensions_c: Vec<CString> = required_extensions
    .into_iter()
    .map(|v| CString::new(v).unwrap())
    .collect();
  let required_extensions_ptr: Vec<*const i8> = required_extensions_c
    .iter()
    .map(|v| v.as_ptr() as *const i8)
    .collect();

  // this is the create info without validation layers, they are added if the "vl" feature is enabled
  #[allow(unused_mut)]
  let mut create_info = vk::InstanceCreateInfo {
    s_type: vk::StructureType::INSTANCE_CREATE_INFO,
    p_next: ptr::null(),
    p_application_info: &app_info,
    pp_enabled_layer_names: ptr::null(),
    enabled_layer_count: 0,
    pp_enabled_extension_names: required_extensions_ptr.as_ptr(),
    enabled_extension_count: required_extensions_ptr.len() as u32,
    flags: vk::InstanceCreateFlags::empty(),
  };

  // Additional validation features can be enabled by adding this structure in the pNext chain
  #[cfg(feature = "vl")]
  let additional_features = vk::ValidationFeaturesEXT {
    s_type: vk::StructureType::VALIDATION_FEATURES_EXT,
    p_next: debug_create_info as *const vk::DebugUtilsMessengerCreateInfoEXT as *const c_void,
    enabled_validation_feature_count: ADDITIONAL_VALIDATION_FEATURES.len() as u32,
    p_enabled_validation_features: ADDITIONAL_VALIDATION_FEATURES.as_ptr(),
    disabled_validation_feature_count: 0,
    p_disabled_validation_features: ptr::null(),
  };
  // should be valid until the end of instance creation
  #[cfg(feature = "vl")]
  let additional_features_ptr = addr_of!(additional_features) as *const c_void;

  #[cfg(feature = "vl")]
  {
    create_info.p_next = additional_features_ptr;
    create_info.pp_enabled_layer_names = vl_pointers.as_ptr();
    create_info.enabled_layer_count = vl_pointers.len() as u32;
  }

  log::debug!("Creating instance");
  let instance: ash::Instance = unsafe {
    entry
      .create_instance(&create_info, None)
      .expect("Failed to create instance")
  };

  instance
}
