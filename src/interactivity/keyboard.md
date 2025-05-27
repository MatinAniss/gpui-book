## Keyboard

### Actions Macro

To create actions you can use the `actions` macro, it takes two parameters. The name of the namespace that will be designated to the group of actions and a slice of names that will be used as the identifier of the created unit structs.

This snippet below creates one action named `Enter` under the namespace of `actions_namespace` with the use of the `actions` macro.

```rust
{{ #include snippets/actions_macro.rs }}
```

### Key Binding

To bind and create a key binding for a specific action you must first use the `KeyBinding::new` function to create the actual `KeyBinding` struct then use the `bind_keys` function from `App` to bind the key binding to the application.

This snippet below binds a `KeyBinding` to the application, the `KeyBinding` has assigned the enter key as the keystroke with the action of `Enter` with the key context specified as `None`.

```rust
{{ #include snippets/key_binding.rs }}
```

#### Keystroke Modifiers

To create a keystroke with modifiers simple add the modifier name or multiple modifier names and the specific key separated with the character `-`, so for example a keystroke of the key `F` with the `control` and `shift` modifiers would look like this `ctrl-shift-f`.

- The `control` modifier is denoted by the name `ctrl`.
- The `alt` modifier is denoted by the name `alt`.
- The `shift` modifier is denoted by the name `shift`.
- The `function` modifier is denoted by the name `fn`.
- The `platform` modifier is denoted `cmd` or `super` or `win`.

The name `secondary` can also be used but it has platform specific behavior, on MacOS it is translated to the `platform` modifier, on all other platforms it is translated to the `control` modifier.

#### Key Context

When using `bind_keys` from `App` you may also specify an optional context which is taken as a `&str`, this allows you to limit your key binding of an action to a specific key context. This key context can be specified using the `key_context` function available from the `InteractiveElement` trait which will take a `&str`, if the context matches with a bounded key binding then the action will be dispatched to any of the relevant bounded callbacks from `on_action`.

### On Action

The `on_action` function available from the `InteractiveElement` trait allows you to bind a callback to the firing of a specific action on a element. The element where `on_action` is used must be focused for actions to be dispatched, this is done with `track_focus` function which takes a `FocusHandle`, this `FocusHandle` must be focused.

If you want to bind a callback to the firing of a action globally throughout your application use [on_action](../architecture/app.md#on-action) from `App`.

The snippet below uses `track_focus` to track the given `FocusHandle` which allows for the dispatching of the actions when it is focussed and `on_action` to bind a callback that prints a message when the `Enter` action is dispatched by pressing the enter key.

```rust
{{ #include snippets/on_action.rs }}
```
