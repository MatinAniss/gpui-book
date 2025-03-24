## Mouse

Any element that implements the `InteractiveElement` trait allows it to access the interactive event handlers that don't require state, some interactive event handlers such as `on_click` and `on_hover` require the `StatefulInteractiveElement` trait which allow them to have some element state between renders.

To make a element stateful you must use the `id` function available from the `InteractiveElement` trait which takes a `Into<ElementId>`, this ID must not change between renders to allow its state to be tracked.

### On Click

The `on_click` function allows you to bind a callback when the user completes a mouse left click on the element, the callback is fired when the user releases the left click. The function takes a closure that supplies a `ClickEvent`, `Window`, and `App`.

```rust
{{ #include snippets/on_click.rs }}
```

### On Hover

The `on_hover` function allows you to bind a callback when the user hovers on and off the element, the callback is fired when the user hovers enters a hover and leaves the hover. The function takes a closure that supplies a `bool` which represents true if the hover has started and false if the hover has ended, it also supplies the `Window` and `App`.

```rust
{{ #include snippets/on_hover.rs }}
```
