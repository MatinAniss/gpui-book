## Background Executor

The background executor allows you to spawn asynchronous tasks onto background threads.

### Accessing the Background Executor

The `background_executor` returns a reference to the platform `BackgroundExecutor`.

```rust
{{ #include snippets/background_executor.rs }}
```

### Spawn

The `spawn` function takes a `Future` and enqueues it to run on a background thread, a `Task<T>` is returned. When this task is dropped it will be cancelled immediatley. Using `detach` on a `Task<T>` allows it to run to completion.

```rust
{{ #include snippets/background_executor_spawn.rs }}
```

### Timer

The `timer` function takes a `Duration` and returns a `Task<()>` that will complete after the elapsed duration. This task can then be awaited in a future.

```rust
{{ #include snippets/background_executor_timer.rs }}
```
