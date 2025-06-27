use gpui::{AppContext, Application, Context, Empty, IntoElement, Render, Window, WindowOptions};

struct RootView;

impl Render for RootView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        Empty
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
