## Deferred

The `Deferred` element allow you to defer the layout and paint of a element. GPUI follows the [Painter's algorithm](https://en.wikipedia.org/wiki/Painter%27s_algorithm) where elements that get painted after will be drawn on top of a element that got painted earlier. This is where the `Deferred` element allows you to delay the layout and paint of its child.

### Creating a Deferred

The `deferred` function takes a `impl IntoElement` which is the child element and returns the `Deferred` element.

```rust
{{ #include snippets/creating_a_deferred.rs }}
```

### Deferred with Priority

This sets the `priority` at which the element will be deferred at, controlling the order relative to other deferred elements. Higher `priority` values are drawn on top of lower `priority` values.

```rust
{{ #include snippets/deferred_with_priority.rs }}
```
