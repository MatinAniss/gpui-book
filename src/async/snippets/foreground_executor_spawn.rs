use gpui::Application;

fn main() {
    Application::new().run(|app| {
        app.foreground_executor()
            .spawn(async {
                // Some asynchronous work
            })
            .detach();
    });
}
