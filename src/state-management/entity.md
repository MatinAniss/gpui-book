## Entity

`Entity<T>`'s are used when you need to store application state that must communicate between different parts of your application. They are owned by GPUI and can be managed with any type that implements the `AppContext` trait, `App` and `Context<T>` are two common types seen in GPUI which implement `AppContext`. All entities are owned inside of `App` and all other types that implement `AppContext` dereference to `App` to access the entities.

There are some other types that also implement `AppContext` that are used in more advanced cases relating to [async GPUI](../async/index.md).

If the `T` type of a `Entity<T>` implements the `Render` trait it is commonly referred to as a view.

### Creating a Entity

This will create a `Entity` with the given state.

```rust
{{ #include snippets/creating_a_entity.rs }}
```

### Reading a Entity

This will give you a reference to the state.

```rust
{{ #include snippets/reading_a_entity.rs }}
```

### Updating a Entity

This will update the state.

```rust
{{ #include snippets/updating_a_entity.rs }}
```

### Downgrading a Entity

This will turn a `Entity` into a `WeakEntity` which is a weak pointer.

```rust
{{ #include snippets/downgrading_a_entity.rs }}
```
