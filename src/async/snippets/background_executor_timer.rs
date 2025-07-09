use std::time::Duration;

use gpui::Application;

fn main() {
    Application::new().run(|app| {
        let timer = app.background_executor().timer(Duration::from_secs(10));

        app.background_executor()
            .spawn(async {
                timer.await;
                println!("Timer Finished!");
            })
            .detach();
    });
}
