use gpui::{Application, Global};

#[derive(Default)]
pub struct SomeState {
    some_value: bool,
}

impl Global for SomeState {}

fn main() {
    Application::new().run(|app| {
        app.set_global::<SomeState>(SomeState { some_value: true });

        app.default_global::<SomeState>();
    });
}
