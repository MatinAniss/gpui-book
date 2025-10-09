# Getting Started

In this chapter, you will learn how to get started with a basic GPUI application either by using [Create GPUI App](https://github.com/zed-industries/create-gpui-app) or manually getting started with your own Rust project.

Versions of GPUI are periodically published to the [crates.io registered crate](https://crates.io/crates/gpui).

Using GPUI can be done with either the crates.io crate or with a git dependency in your `Cargo.toml`, the GPUI crate is kept in the [Zed repository](https://github.com/zed-industries/zed) as it is constantly evolving with the needs of Zed.

```toml
gpui = { version = "*" } # Specify a specific version
# OR
gpui = { git = "https://github.com/zed-industries/zed" }
```
