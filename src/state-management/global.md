## Global

`Global`'s are used when you need to share some state with your whole application. A common example of a global state is the application's settings. Any type that implements the `Global` marker trait can be stored as a global, similar to [Entity](entity.md) they are owned by GPUI.

### Marking a type as a Global

Before you can set some type as a global it must implement the `Global` marker trait.

```rust
{{ #include snippets/marking_a_type_as_a_global.rs }}
```

### Setting a Global

This will set the global for the given type. The type must implement `Global`.

```rust
{{ #include snippets/setting_a_global.rs }}
```

### Accessing a Global

This will give you a reference to the global. Accessing a global that has not been set will cause a panic.

```rust
{{ #include snippets/accessing_a_global.rs }}
```

### Mutably Accessing a Global

This will give you a mutable reference to the global. Accessing a global that has not been set will cause a panic.

```rust
{{ #include snippets/mutably_accessing_a_global.rs }}
```

### Attempt to access a Global

This will give you a reference to the global wrapped in a `Option<T>`.

```rust
{{ #include snippets/attempt_to_access_a_global.rs }}
```

### Check whether a Global has been set

This will check if the global has been set for the given type.

```rust
{{ #include snippets/check_whether_a_global_has_been_set.rs }}
```

### Removing a Global

This will remove the global for the given type.

```rust
{{ #include snippets/removing_a_global.rs }}
```

### Mutably Accessing a Global with Default fallback

This will give you a mutable reference to the global. If the global has not already been set it wil set it to the default given by the type's `Default` trait implementation.

```rust
{{ #include snippets/mutably_accessing_a_global_with_default_fallback.rs }}
```

### Updating a Global

This will update the global for the given type.

```rust
{{ #include snippets/updating_a_global.rs }}
```

### Observing a Global

This will register a callback that will be called when the global is updated.

```rust
{{ #include snippets/observing_a_global.rs }}
```
