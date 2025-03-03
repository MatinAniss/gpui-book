use gpui::{
    AppContext, Application, Context, InteractiveText, IntoElement, ParentElement, Render, Styled,
    StyledText, Window, WindowOptions, div, rgb,
};

struct RootView;

impl Render for RootView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(rgb(0xFFFFFF))
            .flex()
            .justify_center()
            .items_center()
            .text_3xl()
            .child(
                InteractiveText::new("interactive_text_id", StyledText::new("Text")).on_click(
                    vec![1..3],
                    |_range_index, _window, _app| {
                        println!("Clicked");
                    },
                ),
            )
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
