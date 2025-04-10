use gpui::{
    AppContext, Application, ClickEvent, Context, InteractiveElement, IntoElement, ParentElement,
    Render, StatefulInteractiveElement, Styled, Window, WindowOptions, black, div, green, red,
    white,
};

struct RootView {
    count: isize,
}

impl Render for RootView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .gap_5()
            .bg(white())
            .child(
                div()
                    .id("decrement_button")
                    .cursor_pointer()
                    .flex()
                    .items_center()
                    .justify_center()
                    .size_8()
                    .rounded_md()
                    .border_1()
                    .border_color(black())
                    .child("-")
                    .hover(|style| style.bg(red()))
                    .on_click(cx.listener(Self::decrement)),
            )
            .child(
                div()
                    .min_w_16()
                    .text_3xl()
                    .text_center()
                    .child(self.count.to_string()),
            )
            .child(
                div()
                    .id("increment_button")
                    .cursor_pointer()
                    .flex()
                    .items_center()
                    .justify_center()
                    .size_8()
                    .rounded_md()
                    .border_1()
                    .border_color(black())
                    .child("+")
                    .hover(|style| style.bg(green()))
                    .on_click(cx.listener(Self::increment)),
            )
    }
}

impl RootView {
    fn increment(&mut self, _event: &ClickEvent, _window: &mut Window, cx: &mut Context<Self>) {
        self.count += 1;
        cx.notify();
    }

    fn decrement(&mut self, _event: &ClickEvent, _window: &mut Window, cx: &mut Context<Self>) {
        self.count -= 1;
        cx.notify();
    }
}

fn main() {
    Application::new().run(|app| {
        app.open_window(WindowOptions::default(), |_window, app| {
            app.new(|_cx| RootView { count: 0 })
        })
        .unwrap();
    });
}
