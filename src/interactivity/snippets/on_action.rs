use gpui::{
    AppContext, Application, Context, FocusHandle, InteractiveElement, IntoElement, Render, Window,
    WindowOptions, actions, div,
};

actions!(actions_namespace, [Enter]);

struct RootView {
    focus_handle: FocusHandle,
}

impl Render for RootView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .track_focus(&self.focus_handle)
            .on_action(|&Enter, _window, _app| {
                println!("Enter key hit!");
            })
    }
}

fn main() {
    Application::new().run(|app| {
        app.open_window(WindowOptions::default(), |window, app| {
            app.bind_keys([gpui::KeyBinding::new("enter", Enter, None)]);

            let focus_handle = app.focus_handle();
            focus_handle.focus(window);

            app.new(|_cx| RootView { focus_handle })
        })
        .unwrap();
    });
}
