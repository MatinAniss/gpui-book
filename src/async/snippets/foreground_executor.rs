use gpui::{AppContext, Application};

fn main() {
    Application::new().run(|app| {
        let foreground_executor = app.foreground_executor();
    });
}
