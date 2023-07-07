use crate::{ModifyView, UiPaint};
use async_trait::async_trait;
use std::borrow::Cow;

pub fn to_html_event<'p>(event: impl UiPaint) -> Cow<'p, str> {
    Cow::Borrowed("window.ipc.postMessage('") + Cow::Owned(event.to_html().to_string()) + "')"
}

#[async_trait]
pub trait EventHandler {
    fn event_logic(value: &str) -> Self;

    async fn view_model(&self) -> ModifyView;
}
