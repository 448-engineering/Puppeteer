use crate::{ActiveAppEnv, ModifyView, Shell, WindowResize};
use async_trait::async_trait;
use std::borrow::Cow;
use wry::application::window::Window;

/// This trait is used to perform UI operations like parsing the IPC messages
/// and generating content to be displayed
#[async_trait::async_trait]
pub trait Puppeteer {
    /// The app default size window. This defaults to `WindowResize::ResizePercent(90)`
    fn window_size() -> WindowResize {
        WindowResize::ResizePercent(90)
    }

    /// Method is run to generate a [Shell].
    fn shell() -> Shell;

    /// Initialize function which loads data necessary for
    /// the app to function. This data can be use to load resources
    /// like fonts or load user data like username from a database, etc.
    /// Load the root page after initialization has completed

    async fn init() -> ModifyView;

    /// The splash screen loaded when an app is being initialized
    fn splashscreen() -> &'static dyn UiPaint;

    /// Parse the IPC message.
    /// Make sure you handle the errors received from the proxy.
    /// These errors start with the following keywords
    /// #### Example Error Handling
    /// ```rust
    /// // This code handles the error message abbreviated using `PuppeteerError»`
    ///
    /// if message.starts_with("PuppeteerError»") {
    /// // Handle this error into `Self`
    ///     panic!("Encountered error: {}", message)
    /// }
    ///```
    fn parse(message: &str) -> Self;

    /// After parsing the IPC message using the above `Puppeteer::parse()` method
    /// this method is called to perform updates to the UI
    async fn event_handler(&mut self, app_env: &ActiveAppEnv, window: &Window) -> ModifyView;

    /// This is used to handle errors. It is async so that I/O can be used like to log to a file.
    /// It returns a [ModifyView] which can display an error message to the user
    async fn error_handler(error: impl std::error::Error) -> ModifyView;
}

/// Trait that ensures a type can be converted to code that can be rendered into current view
#[async_trait]
pub trait UiPaint {
    /// Can be rendered by native renderer
    fn to_native(&self) -> () {
        ()
    }

    /// For an event is handled using HTML format `window.ipc.postMessage('EVENT_STRING_NAME')` .
    /// Alternatively use the `to_html_event()` function to simplify this
    fn to_html(&self) -> Cow<str>;
}

impl core::fmt::Debug for dyn UiPaint {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "UiPaint")
    }
}
