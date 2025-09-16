use std::time::Duration;

use gpui::{
    Animation, AnimationExt, AppContext, Application, Context, IntoElement, ParentElement, Render,
    Styled, Window, WindowOptions, blue, div, ease_in_out, rems, white,
};

struct RootView;

impl Render for RootView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .bg(white())
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .child(
                div().bg(blue()).with_animation(
                    "animation",
                    Animation::new(Duration::from_secs(1))
                        .repeat()
                        .with_easing(ease_in_out),
                    |this, delta| this.size(rems(delta * 5.)),
                ),
            )
    }
}

fn main() {
    Application::new().run(|app| {
        app.open_window(WindowOptions::default(), |_window, app| {
            app.new(|_| RootView)
        })
        .unwrap();
    });
}
