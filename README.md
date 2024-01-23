# Vulkan examples built with Ash

This is a collection of some examples for [Vulkan®](https://www.khronos.org/vulkan/) that
uses [Ash](https://github.com/ash-rs/ash) as its wrapper.

These examples try to be suited for use in a normal application by being well structured and
following best practices.

Feel free to suggest new examples or improvements for old ones.

## Table of Contents

- [Instance creation](https://github.com/ZakStar17/ash-by-example/tree/main/src/bin/instance):
  Covers Instance creation and enabling validation layers.

## Running

You can run the examples by issuing the command `cargo run --bin <name_of_the_example>`.
You can find more information by going to each example folder and reading its README.

Be aware that the examples use multiple cargo features that enable specific functionality. Some
of theses features include `vl` to enable validation layers or `link` to link the Vulkan loader
with the resulting binary instead of loading it at runtime. However, just running with the
default features should be enough in most cases.

## Checking the logs

Every example uses the [log](https://github.com/rust-lang/log) crate with
[env_logger](https://docs.rs/env_logger/latest/env_logger/) as its facade implementation. This
means that, for example, the validation layers (if enabled) will only show errors by default.

Different levels of visibility can be changed with the environment variable `RUST_LOG`, so if
for example you want to see all error, warning, info and debug messages, just run the executable preceding
it with `RUST_LOG=debug`. You can find more information at [https://docs.rs/env_logger/0.11.0/env_logger/](https://docs.rs/env_logger/0.11.0/env_logger/).

`RUST_LOG=debug cargo run --bin instance`
