use gpui::{AppContext, Application, Context, IntoElement, Render, Window, WindowOptions, div};

struct RootView;

impl Render for RootView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}

fn main() {
    Application::new().run(|app| {
        app.open_window(WindowOptions::default(), |window, app| {
            println!("{:?}", window.mouse_position());

            app.new(|_cx| RootView)
        })
        .unwrap();
    });
}
