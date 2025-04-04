## Context

`Context` dereferences to `App` so it has all the functionality available to [App](./app.md) and it also has additional functionality for controlling the `Entity` it belongs to.

### Notify

The `notify` function alerts GPUI that the `Entity` has been updated and that observers of it should be notified. If the `T` type of `Context<T>` implements `Render` then the view will be re-rendered.

```rust
{{ #include snippets/notify.rs }}
```
