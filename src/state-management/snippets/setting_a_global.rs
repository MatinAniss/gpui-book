use gpui::{Application, Global, UpdateGlobal};

pub struct SomeState {
    some_value: bool,
}

impl Global for SomeState {}

fn main() {
    Application::new().run(|app| {
        app.set_global::<SomeState>(SomeState { some_value: true });

        // OR

        SomeState::set_global(app, SomeState { some_value: true });
    });
}
