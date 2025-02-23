use gpui::{AppContext, Application};

pub struct SomeState {
    some_value: bool,
}

fn main() {
    Application::new().run(|app| {
        let entity = app.new(|_cx| SomeState { some_value: true });

        let some_state = entity.read(app);
    });
}
