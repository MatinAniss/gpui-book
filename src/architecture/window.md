## Window

`Window`'s contain the views and components that will be rendered in your application, they each correspond to a platform window.

### Opening a Window

Using `App` you can access the `open_window` function which takes a `WindowOptions` and a callback which supplies mutable references to a `Window` and `App` that is used to build the root view. To learn more on how views are created you can read the [Render section](../rendering/render.md).

```rust
{{ #include snippets/opening_a_window.rs }}
```
