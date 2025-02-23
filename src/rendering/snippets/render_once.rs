use gpui::{
    App, AppContext, Application, Context, IntoElement, ParentElement, Render, RenderOnce, Window,
    WindowOptions, div,
};

#[derive(IntoElement)]
struct SomeComponent;

impl RenderOnce for SomeComponent {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
    }
}

struct RootView;

impl Render for RootView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<'_, Self>) -> impl IntoElement {
        div().child(SomeComponent)
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
