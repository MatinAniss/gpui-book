## Manual Project

This section will walkthrough how to manually setup a monolithic project structure for your GPUI application without the use of the `create-gpui-app` CLI tool.

### Setup

```properties
cargo new my-app
cd my-app
```

You may either add `gpui` using the crates.io crate or git repository.

Adding `gpui` using a crates.io dependency to the `Cargo.toml`.

```properties
cargo add gpui
```

Adding `gpui` using a git dependency to the `Cargo.toml`.

```properties
cargo add gpui --git https://github.com/zed-industries/zed
```

### Hello World Example

Add the basic Hello World example code to the `main.rs`.

```rust
{{ #include snippets/hello_world.rs }}
```
