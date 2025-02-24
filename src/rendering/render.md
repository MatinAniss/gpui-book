## Render

The `Render` trait is what allows a type to draw elements to the screen. This is what turns the `T` type of a `Entity<T>` into what is commonly referred to as a view. A view is the retained mode rendering of GPUI, it will not re-render unless it is notified. Views can contain components which implement `RenderOnce`, these components are the immediate mode rendering of GPUI, you can learn more in the [RenderOnce](render-once.md) section.

### Implementation

```rust
{{ #include snippets/render.rs }}
```
