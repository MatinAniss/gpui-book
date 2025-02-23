## Manual Project

This section will walkthrough how to manually setup a monolithic project structure for your GPUI application without the use of the `create-gpui-app` CLI tool.

### Setup

```
cargo new my-app
cd my-app
```

Add the `gpui` git depedency to the `Cargo.toml`.

```toml
{{ #include snippets/hello_world_cargo.toml }}
```

### Hello World Example

Add the basic Hello World example code to the `main.rs`.

```rust
{{ #include snippets/hello_world.rs }}
```
