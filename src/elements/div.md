## Div

The `Div` element is the most commonly used element in GPUI. It functions as a container for other elements and can be styled and enhanced with interactivity to build a wide range of components. To learn how to style and make your element interactive go the the respective chapters [Styling](../styling/index.md) and [Interactivity](../interactivity/index.md).

### Creating a Div

```rust
div()
```

### Adding a child to a Div

```rust
div().child(div())
```

### Adding children to a Div

```rust
div().children([div(), div()])
```
