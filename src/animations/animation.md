## Animation

`Animation` allows you to configure a specific animation with the following properties duration, easing, and whether it should repeat or not. This animation can then be applied to element with [`with_animation`](with_animation.md#with_animation)

### Creating an animation

The `new` function takes a `Duration` this and returns the `Animation`.

```rust
{{ #include snippets/creating_an_animation.rs }}
```

### Repeating animation

The `repeat` function will make the animation repeat indefinitely.

```rust
{{ #include snippets/repeating_animation.rs }}
```

### Easing animation

The `with_easing` function takes a `impl Fn(f32) -> f32` this closure takes a delta and allows you to modify the delta during the animation which allows easing.

GPUI supplies various easing functions that you may want to use such as `linear`, `quadratic`, `ease_in_out`, and `ease_out_quint`.

```rust
{{ #include snippets/easing_animation.rs }}
```
