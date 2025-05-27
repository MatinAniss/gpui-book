use gpui::{
    AppContext, Application, Context, IntoElement, Render, Window, WindowOptions, actions, div,
};

actions!(actions_namespace, [Enter]);

struct RootView;

impl Render for RootView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}

fn main() {
    Application::new().run(|app| {
        app.open_window(WindowOptions::default(), |_window, app| {
            app.bind_keys([gpui::KeyBinding::new("enter", Enter, None)]);
            app.on_action(|&Enter, _app| println!("Enter key hit!"));

            app.new(|_cx| RootView)
        })
        .unwrap();
    });
}
