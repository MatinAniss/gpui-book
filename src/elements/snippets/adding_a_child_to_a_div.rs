use gpui::{
    AppContext, Application, Context, IntoElement, ParentElement, Render, Window, WindowOptions,
    div,
};

struct RootView;

impl Render for RootView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().child(div())
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
