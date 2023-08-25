use crate::{ModifyView, Shell, WindowResize};
use async_trait::async_trait;
use std::borrow::Cow;

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

    /// Load the root page after initialization has completed
    fn root<'p>() -> &'p dyn UiPaint;

    /// Initialize function which loads data necessary for
    /// the app to function. This data can be use to load resources
    /// like fonts or load user data like username from a database, etc,
    async fn init<'p>() -> ModifyView<'p>;

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
