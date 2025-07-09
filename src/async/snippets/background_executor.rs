use gpui::{AppContext, Application};

fn main() {
    Application::new().run(|app| {
        let background_executor = app.background_executor();
    });
}
