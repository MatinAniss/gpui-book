# Styling

In this chapter, you will learn how to style elements in your GPUI application.

GPUI uses [Taffy](https://github.com/DioxusLabs/taffy) as the underlying layout engine, it powers how elements are laid out.

The styling of elements can be done using the utility CSS-like styling API or by modifying the underlying `Style` struct of the element, this can be done with `.style()` function which is available to any element that implements the `Styled` trait. The utility CSS-like styling API are just shorthand functions that allow you to apply the same underlying style adjustments in a more concise manner.

## Comparison

This compares the concise nature of the utility CSS-like styling functions against the verbose method of modifying the underlying `Style` struct, while both still achieve the same resulting style. It is your choice if you would like to use the shorthand functions or you can even create your own custom shorthand functions.

### Utility CSS-like styling API

```rust
impl Render for SomeView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().flex().flex_col().child("Hello").child("World")
    }
}
```

### Modifying underlying element Style struct

```rust
impl Render for SomeView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let mut element = div().child("Hello").child("World");

        let style = element.style();
        style.display = Some(Display::Flex);
        style.flex_direction = Some(FlexDirection::Column);

        element
    }
}
```
