## Global

`Global`'s are used when you need to share some state with your whole application. A common example of a global state is the application's settings. Any type that implements the `Global` marker trait can be stored as a global, similar to [Entity](entity.md) they are owned by GPUI.

### Marking a type as a Global

Before you can set some type as a global it must implement the `Global` marker trait.

```rust
pub struct SomeState {
    some_value: bool
}

// Global marker trait
impl Global for SomeState {}
```

### Setting a Global

This will set the global for the given type. The type must implement `Global`.

```rust
app.set_global::<SomeState>(SomeState {
    some_value: true
});
```

### Accessing a Global

This will give you a reference to the global. Accessing a global that has not been set will cause a panic.

```rust
let some_value = app.global::<SomeState>().some_value;
```

### Mutably Accessing a Global

This will give you a mutable reference to the global. Accessing a global that has not been set will cause a panic.

```rust
let some_value = app.global::<SomeState>().some_value;
some_value = false;
```

### Attempt to access a Global

This will give you a reference to the global wrapped in a `Option<T>`.

```rust
let maybe_some_value = app.try_global::<SomeState>().some_value;
```

### Check whether a Global has been set

This will check if the global has been set for the given type.

```rust
let is_set_bool = app.has_global::<SomeState>();
```

### Removing a Global

This will remobe the global for the given type.

```rust
app.remove_global::<SomeState>();
```

### Mutably Accessing a Global with Default fallback

This will give you a mutable reference to the global. If the global has not already been set it wil set it to the default given by the type's `Default` trait implementation.

```rust
app.default_global::<SomeState>();
```

### Updating a Global

This will update the global for the given type.

```rust
app.update_global::<SomeState, _>(|some_state, app| {
    some_state.some_value = false;
});
```

### Observing a Global

This will register a callback that will be called when the global is updated.

```rust
app.observe_global::<SomeState>(|app| {
    // Global update callback
});
```
