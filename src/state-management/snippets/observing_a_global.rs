use gpui::{Application, Global};

pub struct SomeState {
    some_value: bool,
}

impl Global for SomeState {}

fn main() {
    Application::new().run(|app| {
        app.set_global::<SomeState>(SomeState { some_value: true });

        let subscription = app.observe_global::<SomeState>(|_app| {
            // Global update callback
        });

        // OR

        app.observe_global::<SomeState>(|_app| {
            // Global update callback
        })
        .detach();
    });
}
