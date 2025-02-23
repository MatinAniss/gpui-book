use gpui::{Application, BorrowAppContext, Global, UpdateGlobal};

pub struct SomeState {
    some_value: bool,
}

impl Global for SomeState {}

fn main() {
    Application::new().run(|app| {
        app.set_global::<SomeState>(SomeState { some_value: true });

        app.update_global::<SomeState, _>(|some_state, _app| {
            some_state.some_value = false;
        });

        // OR

        SomeState::update_global(app, |some_state, _app| {
            some_state.some_value = false;
        });
    });
}
