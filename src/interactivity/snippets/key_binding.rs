use gpui::{Application, actions};

actions!(actions_namespace, [Enter]);

fn main() {
    Application::new().run(|app| {
        app.bind_keys([gpui::KeyBinding::new("enter", Enter, None)]);
    });
}
