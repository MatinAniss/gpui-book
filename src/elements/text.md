## Text

There are multiple different types in GPUI that allow you to render text, to learn how to style text read the [styling text](../styling/text.md) chapter.

### &'static str

```rust
{{ #include snippets/static_str.rs }}
```

### String

Using a `String` should be used sparingly as it will cause a heap allocation on every re-render, read on about [SharedString](#sharedstring) to learn how to avoid this.

```rust
{{ #include snippets/string.rs }}
```

### SharedString

`SharedString` is an immutable string that can be cheaply cloned, this is especially useful in the `render` function as it avoids a heap allocation on every re-render which occurs when using `String`.

The `SharedString` is typically stored in the `Entity` state where it can be accessed by the `render` function.

```rust
{{ #include snippets/shared_string.rs }}
```

### StyledText

The `StyledText` component allows you style specific ranges of the text differently, the text ranges are called `TextRun`'s. It is unnecessary to use `StyleText` if the whole range of your text uses the same style.

```rust
{{ #include snippets/styled_text.rs }}
```

### InteractiveText

The `InteractiveText` component allows you to make specific ranges of the text interactive, it allows you to add click and hover listeners for specific text ranges. The component takes a `ElementId` that must be identical on every frame and a [StyledText](#styledtext).

```rust
{{ #include snippets/interactive_text.rs }}
```
