use gpui::Application;

fn main() {
    Application::new().run(|app| {
        app.background_executor()
            .spawn(async {
                // Some asynchronous work
            })
            .detach();
    });
}
