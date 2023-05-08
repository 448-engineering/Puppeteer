## [Unreleased]

## [1.1.1] - 2023-03-05

### Added

- Create an event loop
- Add ability to use the webview and window within the argument function
- Control the UI by just passing a function to the event loop 
```rust
custom_event_handler: fn(
    ui_event: T,
    webview: &mut Option<WebView>,
    control_flow: &mut ControlFlow,
)
```
- Expose `wry::window::Window` for reuse by other functions and methods
- Add helper functions to reduce logic needed for `Window` methods like `set_minimized` , `set_maximized` ...
- Add default events for `minimize` , `maximize` and `drag_window` and handle them in the event loop
- Constrain licenses to permissive ones using `deny.toml` file
- Add a CHANGELOG.md file
- Add a README File
- Add an example in `examples` folder called `simple.rs` that shows a minimal example