use puppeteer_types::{
    wry::{application::event_loop::ControlFlow, webview::WebView},
    Container, Heading, PuppeteerUtils, UiPaint,
};
use std::borrow::Cow;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub(crate) enum UserEvent {
    CloseWindow,
    MaximizeWindow,
    NextPage,
    Data,
}

pub(crate) fn custom_event_handler(
    custom_event: UserEvent,
    webview: &mut Option<WebView>,
    control_flow: &mut ControlFlow,
) {
    let h1 = Heading::new("ROOT UI");

    let mut root_ui = Container::new();
    root_ui.add_child(Box::new(h1));

    let eval = {
        let element = Cow::Borrowed(r#"document.body.innerHTML=""#);

        element + root_ui.to_html() + r#"";"#
    };

    match custom_event {
        UserEvent::CloseWindow => {
            let _ = webview.take();
            *control_flow = ControlFlow::Exit
        }
        UserEvent::NextPage => webview.as_mut().unwrap().evaluate_script(&eval).unwrap(),
        UserEvent::MaximizeWindow => {
            PuppeteerUtils::new(webview.as_mut().unwrap().window()).set_maximized()
        }
    }
}

impl From<String> for UserEvent {
    fn from(value: String) -> Self {
        match value.as_str() {
            "close" => UserEvent::CloseWindow,
            "next_page" => UserEvent::NextPage,
            "maximize" => UserEvent::MaximizeWindow,
            _ => UserEvent::CloseWindow,
        }
    }
}
