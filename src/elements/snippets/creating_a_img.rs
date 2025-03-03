use std::path::Path;

use gpui::{
    AppContext, Application, Context, IntoElement, ParentElement, Render, Styled, Window,
    WindowOptions, div, img,
};

struct RootView;

impl Render for RootView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .child(img(Path::new(env!("CARGO_MANIFEST_DIR")).join("image.png")).size_full())
    }
}

fn main() {
    Application::new().run(|app| {
        app.open_window(WindowOptions::default(), |_window, app| {
            app.new(|_cx| RootView)
        })
        .unwrap();
    });
}
