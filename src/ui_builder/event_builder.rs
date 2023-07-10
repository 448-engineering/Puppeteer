use crate::UiPaint;
use std::borrow::Cow;

pub fn to_html_event<'p>(event: impl UiPaint) -> Cow<'p, str> {
    Cow::Borrowed("window.ipc.postMessage('") + Cow::Owned(event.to_html().to_string()) + "')"
}
