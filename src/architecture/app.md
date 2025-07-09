## App

`App` contains the state of your whole application, it allows you to control many aspects of the application's functionality such as managing the state of `Entity`'s.

### Opening a Window

Using `App` you can access the `open_window` function which takes a `WindowOptions` and a callback which supplies mutable references to a `Window` and `App` that is used to build the root view. To learn more on how views are created, you can read the [Render section](../rendering/render.md).

```rust
{{ #include snippets/opening_a_window.rs }}
```

### On Action

Using `App` you can use `on_action` function to bind a callback to the firing of a action globally throughout your application.

```rust
{{ #include snippets/on_action.rs }}
```

### Spawn

Using `App` you can use the `spawn` function to enqueue a future on the main thread, it takes a `AsyncFnOnce` which will provide `AsyncApp` when the async closure is invoked. The `AsyncApp` allows you to access application state.

```rust
{{ #include snippets/spawn.rs }}
```
