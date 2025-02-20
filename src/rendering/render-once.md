## RenderOnce

The `RenderOnce` trait as opposed to the `Render` trait is used when you want to create a component instead of a view. As the name suggests they are only rendered once, they are constructed, rendered, and then dropped. This is the immediate mode of rendering which is done inside retained mode views.

Contrary to `Render` the the `render` function of `RenderOnce` takes ownership of self unlike the mutable reference of self that `Render` supplies. This is useful for components that do not need to store any state.

```rust
struct SomeStruct;

impl RenderOnce for SomeStruct {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
    }
}
```
