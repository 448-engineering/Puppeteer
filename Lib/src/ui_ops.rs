use crate::{PuppeteerError, PuppeteerResult, UiPaint};
use std::borrow::Cow;
use tao::{
    dpi::{PhysicalPosition, PhysicalSize},
    window::Window,
};

/// The callback used to modify a node based on the result of computation of it's text content
pub type JsCallback = fn(&str) -> ModifyView;

/// Event Handler for Puppeteer
#[derive(Debug)]
pub enum UiEvent<T: crate::Puppeteer + 'static + Send + Sync> {
    /// Initialize the default root page after splashcreen
    Init,
    /// Minimize the window
    Minimize,
    /// Maximize the window
    Maximize,
    /// Close the window
    Close,
    /// Drag the window
    Drag,
    /// User defined custom event
    Custom(T),
    /// Error occurred
    Error(PuppeteerError),
    /// Modify the webview with new contents
    Paint(ModifyView),
}

/// Used to modify the view which can be a WebView
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ModifyView {
    /// Replaces content in the app using the provided ID
    ReplaceApp(String),
    /// Replaces the node with the specified ID
    ReplaceNodeWithId {
        /// The `id` of the node being replaced
        id: String,
        /// The `content to replace the content in target node
        content: String,
    },
    /// This event closes the window and exits the EventLoop
    CloseWindow,
    /// Maximize the current window
    MaximizeWindow,
    /// Minimize current window
    MinimizeWindow,
    /// Drag the window by clicking and holding the title bar
    DragWindow,
    /// Perform No Action on the data, this is useful for reactive UI like taking data from input field in real time
    Skip,
    /// Fetch the inner text of the node with ID and pass it along to the closure defined as `func`
    ComputeWithIdData {
        /// The node id to fetch it's text content
        id: String,
        /// Callback function to use to send event to update node based on an operation on the text content
        func: JsCallback,
    },
}

impl ModifyView {
    /// Replace the nodes in the body of the app leaving the [crate::Shell] intact
    pub fn replace_app(content: &'static dyn UiPaint) -> Self {
        ModifyView::ReplaceApp(content.to_html().to_string())
    }

    /// Replace the node with the specified ID
    pub fn replace_with_id(id: &'static str, content: &'static dyn UiPaint) -> Self {
        ModifyView::ReplaceNodeWithId {
            id: id.to_string(),
            content: content.to_html().to_string(),
        }
    }
}

impl UiPaint for ModifyView {
    fn to_html(&self) -> Cow<str> {
        match self {
            Self::ReplaceApp(content) => {
                Cow::Borrowed(r#"document.getElementById("puppeteer_app").innerHTML=`"#)
                    + content.as_ref()
                    + "`;"
            }
            Self::ReplaceNodeWithId { id, content } => {
                Cow::Borrowed(r#"document.getElementById(""#)
                    + Cow::Owned(id.to_owned())
                    + r#"").innerHTML=`"#
                    + content.as_ref()
                    + "`;"
            }
            Self::CloseWindow => Cow::Borrowed("Close Window Requested"),
            Self::MaximizeWindow => Cow::Borrowed("Maximize Window Requested"),
            Self::MinimizeWindow => Cow::Borrowed("Minimize Window Requested"),
            Self::DragWindow => Cow::Borrowed("Dragging Window..."),
            Self::Skip => Cow::Borrowed("Skipped"),
            Self::ComputeWithIdData { id, func: _ } => {
                Cow::Borrowed("document.getElementById('")
                    + Cow::Owned(id.to_owned())
                    + "').textContent;"
            }
        }
    }
}

/// An window resize operation
#[derive(Debug, PartialEq)]
pub enum WindowResize {
    /// Resize to fullscreen
    FullScreen,
    /// Maximize the window if it is not maximized
    Maximize,
    /// Minimize the window if it is not already minimized
    Minimize,
    /// Center the window
    Center,
    /// Resize the window based on percentage of the current monitor
    ResizePercent(f32),
}

impl WindowResize {
    /// Match the operation to it's resize values and perform resizing
    pub fn get_op(&self, window: &Window) -> PuppeteerResult<()> {
        if !window.is_resizable() {
            return Err(PuppeteerError::WindowIsNotResizable);
        }

        match self {
            Self::FullScreen => window.set_fullscreen(Some(tao::window::Fullscreen::Borderless(
                window.current_monitor(),
            ))),
            Self::Maximize => {
                if !window.is_maximized() {
                    window.set_maximized(true)
                }
            }
            Self::Minimize => {
                if !window.is_minimizable() {
                    window.set_minimized(true)
                }
            }
            Self::Center => {
                if let Some(monitor) = window.current_monitor() {
                    let screen_size = monitor.size();
                    let window_size = window.outer_size();

                    window.set_outer_position(PhysicalPosition {
                        x: screen_size.width.saturating_sub(window_size.width) as f64 / 2.
                            + monitor.position().x as f64,
                        y: screen_size.height.saturating_sub(window_size.height) as f64 / 2.
                            + monitor.position().y as f64,
                    });
                }
            }
            Self::ResizePercent(inner_value) => {
                let value = if inner_value > &1f32 {
                    1f32
                } else {
                    *inner_value
                };

                let _scale_value = value / 100f32; //FIXME Use this scaling factor

                if let Some(monitor) = window.current_monitor() {
                    let screen_size = monitor.size();

                    window.set_inner_size(PhysicalSize {
                        width: screen_size.width as f32 * value,
                        height: screen_size.height as f32 * value,
                    });
                }
            }
        }

        Ok(())
    }
}
