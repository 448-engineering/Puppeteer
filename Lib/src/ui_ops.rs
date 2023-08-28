use crate::{PuppeteerError, PuppeteerResult, UiPaint};
use std::borrow::Cow;
use wry::{
    application::dpi::{PhysicalPosition, PhysicalSize},
    webview::WebView,
};

/// An window resize operation
#[derive(Debug, PartialEq, Eq)]
pub enum WindowResize {
    /// Resize to fullscreen
    FullScreen,
    /// Maximize the window if it is not maximized
    Maximize,
    /// Minimize the window if it is not already minized
    Minimize,
    /// Resize to a certain width and height
    ResizeWidthHeight((u32, u32)),
    /// Resize width and height based on the `X` and `Y` position of top most monitor
    /// if the current desktop has multiple monitors
    ResizeXY((i32, i32)),
    /// Calculate the width and height to resize based on a certain percentage of the current monitor
    /// Anything past 100 is reset to 100
    ResizePercent(u8),
}

impl WindowResize {
    /// Match the operation to it's resize values and perform resizing
    pub fn get_op(&self, webview: Option<&WebView>) -> PuppeteerResult<()> {
        let (window, inner_size) = if let Some(webview_exists) = webview.as_ref() {
            (webview_exists.window(), webview_exists.inner_size())
        } else {
            return Err(PuppeteerError::WebViewDoesNotExist);
        };

        if !window.is_resizable() {
            return Err(PuppeteerError::WindowIsNotResizable);
        }

        match self {
            Self::FullScreen => window.set_fullscreen(Some(
                wry::application::window::Fullscreen::Borderless(window.current_monitor()),
            )),
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
            Self::ResizeWidthHeight((width, height)) => {
                window.set_inner_size::<PhysicalSize<u32>>(PhysicalSize {
                    width: *width,
                    height: *height,
                })
            }
            Self::ResizeXY((x, y)) => window
                .set_outer_position::<PhysicalPosition<i32>>(PhysicalPosition { x: *x, y: *y }),
            Self::ResizePercent(inner_value) => {
                let value = if inner_value > &100 {
                    100u8
                } else {
                    *inner_value
                };

                let scale_value = value as f32 / 100f32;

                let screen_size = if let Some(some_monitor) = window.current_monitor() {
                    PhysicalSize {
                        width: (some_monitor.size().width as f32 * scale_value) as u32,
                        height: (some_monitor.size().height as f32 * scale_value) as u32,
                    }
                } else {
                    return Err(PuppeteerError::UnableToDetectCurrentMonitor);
                };

                let x = (screen_size.width as i32 - inner_size.width as i32) / 2;
                let y = (screen_size.height as i32 - inner_size.height as i32) / 2;

                window.set_outer_position::<PhysicalPosition<i32>>(PhysicalPosition { x, y });

                window.set_inner_size::<PhysicalSize<u32>>(screen_size);
            }
        }

        Ok(())
    }
}

/// Used to modify the view which can be a WebView
#[derive(Debug)]
pub enum ModifyView {
    /// Replaces content in the app using the provided ID
    ReplaceApp(Cow<'static, str>),
    /// Replaces the node with the specified ID
    ReplaceNodeWithId {
        /// The `id` of the node being replaced
        id: &'static str,
        /// The `content to replace the content in target node
        content: Cow<'static, str>,
    },
}

impl ModifyView {
    /// Replace the nodes in the body of the app leaving the [Shell] intact
    pub fn replace_app(content: &'static dyn UiPaint) -> Self {
        ModifyView::ReplaceApp(content.to_html())
    }

    /// Replace the node with the specified ID
    pub fn replace_with_id(id: &'static str, content: &'static dyn UiPaint) -> Self {
        ModifyView::ReplaceNodeWithId {
            id,
            content: content.to_html(),
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
                    + id.as_ref()
                    + r#"").innerHTML=`"#
                    + content.as_ref()
                    + "`;"
            }
        }
    }
}
