use gpui::{AppContext, Application};

pub struct SomeState {
    some_value: bool,
}

fn main() {
    Application::new().run(|app| {
        let entity = app.new(|_cx| SomeState { some_value: true });

        entity.update(app, |some_state, _cx| {
            some_state.some_value = false;
        });
    });
}
