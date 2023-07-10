use crate::ModifyView;
use async_trait::async_trait;
use std::borrow::Cow;

#[async_trait]
pub trait EventHandler {
    async fn view_model(&self) -> ModifyView;

    async fn init_func() -> ModifyView;
}

pub trait UiPaint {
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

pub trait StyleDeclaration {
    fn to_css(&self) -> Cow<str>;

    fn to_native(&self) -> () {
        ()
    }
}

impl From<&'static str> for Box<dyn UiPaint> {
    fn from(value: &'static str) -> Self {
        Box::new(crate::HtmlStaticContent { content: value })
    }
}
