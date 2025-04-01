## Application

The `Application` is the entry point into your GPUI application.

### Creating an Application

The `new` function creates the `Application`.

```rust
{{ #include snippets/creating_an_application.rs }}
```

### Running an Application

The `run` function consumes the `Application` and takes a callback which will be fired once the application has finished loading. This callback is the entry point into your application, a mutable reference of `App` is supplied where you can start controlling aspects of your application like opening a [window](./window.md).

```rust
{{ #include snippets/running_an_application.rs }}
```
