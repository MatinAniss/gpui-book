use gpui::Global;

pub struct SomeState {
    some_value: bool,
}

// Global marker trait
impl Global for SomeState {}
