## Foreground Executor

The foreground executor allows you to spawn asynchronous tasks on the main thread.

When spawning tasks using the foreground executor it is vital to not run operations that may block for any non-trivial duration, doing so will block the main thread which will block the entire application.

### Accessing the Foreground Executor

The `foreground_executor` returns a reference to the platform `ForegroundExecutor`.

```rust
{{ #include snippets/foreground_executor.rs }}
```

### Spawn

The `spawn` function takes a `Future` and enqueues it to run on the main thread, a `Task<T>` is returned. When this task is dropped it will be cancelled immediately. Using `detach` on a `Task<T>` allows it to run to completion.

The [`App::spawn`](../architecture/app.md#spawn) function may alternatively be used as a shorthand that also supplies `AsyncApp` as a paramater of the async closure.

```rust
{{ #include snippets/foreground_executor_spawn.rs }}
```
