use std::borrow::Cow;

pub trait UiPaint {
    fn to_native(&self) -> () {
        ()
    }

    /// For an event is handled using HTML format `window.ipc.postMessage('EVENT_STRING_NAME')` .
    /// Alternatively use the `to_html_event()` function to simplify this
    fn to_html(&self) -> String;
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
