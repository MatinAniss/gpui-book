use gpui::{Application, WindowOptions};

fn main() {
    Application::new().run(|app| {
        app.open_window(WindowOptions::default(), |window, app| {
            // Return root view
        })
        .unwrap();
    });
}
