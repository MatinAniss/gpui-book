use gpui::{
    AppContext, Application, Context, IntoElement, ParentElement, Render, SharedString, Styled,
    Window, WindowOptions, div, white,
};

struct HelloWorld {
    text: SharedString,
}

impl Render for HelloWorld {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(white())
            .flex()
            .justify_center()
            .items_center()
            .text_3xl()
            .child(format!("Hello, {}!", &self.text))
    }
}

fn main() {
    Application::new().run(|app| {
        app.open_window(WindowOptions::default(), |_window, app| {
            app.new(|_cx| HelloWorld {
                text: SharedString::new_static("World"),
            })
        })
        .unwrap();
    });
}
