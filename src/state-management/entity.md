## Entity

`Entity<T>`'s are used when you need to store application state that must communicate between different parts of your application. They are owned by GPUI and can be managed with any type that implements the `AppContext` trait, `App` and `Context<T>` are two common types seen in GPUI which implement `AppContext`. All entities are owned inside of `App` and all other types that implement `AppContext` dereference to `App` to access the entities.

There are some other types that also implement `AppContext` that are used in more advanced cases relating to [async GPUI](/async/index.md).

If the `T` type of a `Entity<T>` implements the `Render` trait it is commonly referred to as a view.

### Creating a Entity

This will create a `Entity` with the given state.

```rust
pub struct SomeState {
    some_value: bool
}

let entity = app.new(|cx| {
    SomeState { some_value: true }
});
```

### Reading a Entity

This will give you a reference to the state.

```rust
let some_state = entity.read(app);
```

### Updating a Entity

This will update the state.

```rust
entity.update(app, |some_state, cx| {
    some_state.some_value = false;
});
```

### Downgrading a Entity

This will turn a `Entity` into a `WeakEntity` which is a weak pointer.

```rust
let weak_entity = entity.downgrade();
```
