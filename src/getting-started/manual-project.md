## Manual Project

This section will walkthrough how to manually setup a monolithic project structure for your GPUI application without the use of the `create-gpui-app` CLI tool.

### Setup

```
cargo new my-app
cd my-app
```

Add the `gpui` git depedency to the `Cargo.toml`

```toml
[package]
name = "my-app"
version = "0.1.0"
edition = "2021"

[dependencies]
gpui = { git = "https://github.com/zed-industries/zed" }
```

Add the basic Hello World example code to the `main.rs`

```rust
use gpui::{
    div, prelude::*, px, rgb, size, App, Application, Bounds, Context, SharedString, Window,
    WindowBounds, WindowOptions,
};

struct HelloWorld {
    text: SharedString,
}

impl Render for HelloWorld  {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .size_full()
            .bg(rgb(0x505050))
            .justify_center()
            .items_center()
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(format!("Hello, {}!", &self.text))
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.open_window(WindowOptions::default(), |_, cx| {
            cx.new(|_cx| HelloWorld {
                text: "World".into(),
            })
        })
        .unwrap();
    });
}
```
