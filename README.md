# Vulkan examples built with Ash

This is a collection of [Vulkan®](https://www.khronos.org/vulkan/) examples with [Ash](https://github.com/ash-rs/ash) as its wrapper API.

Each example is separated in a different folder and uses only the necessary crates to function. This has some drawbacks, but makes the functionality more apparent. All the examples follow a general module structure so you can just look at the functionality that you need.

Each example resides separately in a different folder and has its own `README.md` that explains the general code flow, used Vulkan functionality and some differences/similarities to other examples.

Feel free to suggest new examples or improvements for old ones.

## Table of Contents

- [Instance creation](https://github.com/ZakStar17/ash-by-example/tree/main/instance): Covers Instance creation and enabling validation layers.
- [Device creation](https://github.com/ZakStar17/ash-by-example/tree/main/device): Covers physical device selection, logical device creation and queue retrieval.
- [Compute image clear](https://github.com/ZakStar17/ash-by-example/tree/main/compute_image_clear): Clears an image, copies it from device memory to host accessible (CPU) memory and saves it to a file. This example covers command buffer and image creation, image layout transitions with image barriers, queue family ownership transfer and queue submission.
- [Storage image compute shader](https://github.com/ZakStar17/ash-by-example/tree/main/storage_image_compute_shader): Generates the Mandelbrot Set offline by using a compute shader on a storage image and saves it to a file. This example covers compute pipeline creation, pipeline caches, descriptor sets and compute shaders. It also demonstrates the use of specialization constants in order to assign constant values in the shader during pipeline creation.
- [Triangle image](https://github.com/ZakStar17/ash-by-example/tree/main/triangle_image): Draws a triangle and saves it to a file. Covers executing a simple graphics pipeline with a render pass, vertex and index buffers.
- [Bouncing texture](https://github.com/ZakStar17/ash-by-example/tree/main/bouncy_ferris): Have Ferris the crab bouncing around the screen. In order words, renders a texture multiple times per second to a window with different positions. Covers creating a combined image sampler to be rendered and presenting to a window surface with the help of a swapchain.

This list is mostly ordered in terms of difficulty.

## Running

Running the examples requires the nightly Rust Toolchain as well as the [Vulkan SDK](https://www.lunarg.com/vulkan-sdk/).

To run a example with all validations enabled, navigate to the respective folder and run `RUST_LOG=debug cargo run <name_of_the_example>`. More information can be found in the respective README.

The examples use cargo features that enable specific functionality. These include `vl` to enable validation layers and `link` to link the Vulkan loader at compile time instead of loading it at runtime. Using default crate features is enough to have it working.

## Checking the logs

Every example uses the [log](https://github.com/rust-lang/log) crate with [env_logger](https://docs.rs/env_logger/latest/env_logger/) as its facade implementation. This means that, for example, the validation layers (if enabled) will only show errors by default.

Different levels of visibility can be changed with the environment variable `RUST_LOG`, so if
for example you want to see all error, warning, info and debug messages, just run the executable preceding
it with `RUST_LOG=debug`. You can find more information at [https://docs.rs/env_logger/0.11.0/env_logger/](https://docs.rs/env_logger/0.11.0/env_logger/).
