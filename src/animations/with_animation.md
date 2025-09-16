## With Animation

This section explains how `with_animation` allows you to apply an [`Animation`](animation.md) to an element.

### with_animation

The `with_animation` function takes three parameters including an `ElementId`, an `Animation`, and a `impl Fn(Self, f32) -> Self`. This function returns `AnimationElement<Self>` where `Self` is any type that can be converted into an element.

The closure supplies `Self` and a `f32`, inside this function is where you can style this element using the `f32` delta. This delta starts from `0.0` and ends at `1.0`, depending on the [`Animation`](animation.md) it can repeat.

```rust
{{ #include snippets/with_animation.rs }}
```
