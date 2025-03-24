use gpui::{
    AppContext, Application, Context, InteractiveElement, IntoElement, ParentElement, Render,
    StatefulInteractiveElement, Styled, Window, WindowOptions, div, red, white,
};

struct RootView;

impl Render for RootView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .child(
                div()
                    .id("some_id")
                    .bg(red())
                    .text_color(white())
                    .h_6()
                    .child("Hover Here")
                    .on_hover(|hovered, _window, _app| {
                        println!(
                            "{}",
                            if *hovered {
                                "Hover started"
                            } else {
                                "Hover ended"
                            }
                        );
                    }),
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
