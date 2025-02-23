use gpui::{AppContext, Application, ParentElement, Render, Styled, WindowOptions, div, rgb};

struct RootView;

impl Render for RootView {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<'_, Self>,
    ) -> impl gpui::IntoElement {
        div()
            .size_full()
            .bg(rgb(0xFFFFFF))
            .flex()
            .justify_center()
            .items_center()
            .text_3xl()
            .child("Hello")
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
