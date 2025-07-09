use gpui::Application;

fn main() {
    Application::new().run(|app| {
        app.spawn(async |app| {
            // Some asynchronous work
        })
        .detach();
    });
}
