use crate::Shell;
use async_trait::async_trait;
use std::borrow::Cow;

/// This trait is used to perform UI operations like parsing the IPC messages
/// and generating content to be displayed
pub trait InvokeWebView {
    /// Method is run to generate a [Shell].
    fn shell() -> Shell;
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
