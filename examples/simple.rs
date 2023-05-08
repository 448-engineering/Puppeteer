use puppeteer::{Puppeteer, PuppeteerUtils};
use wry::{
    application::{event_loop::ControlFlow, window::WindowBuilder},
    webview::WebView,
};

const INITIAL_UI: &str = include_str!("./custom_title_bar.html");

fn main() {
    let window = WindowBuilder::new()
        .with_decorations(false)
        .with_title("Puppeteer");
    Puppeteer::<CustomEvent>::new(INITIAL_UI)
        .unwrap()
        .set_window(window)
        .unwrap()
        .run(custom_event_handler)
        .unwrap();
}

#[derive(Debug)]
enum CustomEvent {
    CloseWindow,
    NextPage,
    AdjustWindow,
}

fn custom_event_handler(
    custom_event: CustomEvent,
    webview: &mut Option<WebView>,
    control_flow: &mut ControlFlow,
) {
    match custom_event {
        CustomEvent::CloseWindow => {
            let _ = webview.take();
            *control_flow = ControlFlow::Exit
        }
        CustomEvent::NextPage => webview
            .as_mut()
            .unwrap()
            .evaluate_script(r#"document.body.innerText = "NEW PAGE LOADED. YEAH!";"#)
            .unwrap(),
        CustomEvent::AdjustWindow => {
            PuppeteerUtils::new(webview.as_mut().unwrap().window()).set_maximized()
        }
    }
}

impl From<String> for CustomEvent {
    fn from(value: String) -> Self {
        match value.as_str() {
            "close" => CustomEvent::CloseWindow,
            "next_page" => CustomEvent::NextPage,
            "maximize" => CustomEvent::AdjustWindow,
            _ => CustomEvent::CloseWindow,
        }
    }
}

/*
impl UiPaint for CustomEvent {
    fn to_html(&self) -> String {

    }

    fn to_hml(&self) -> Self {
        match event_as_str {
            "close" => CustomEvent::CloseWindow,
            "next_page" => CustomEvent::NextPage,
            "maximize" => CustomEvent::AdjustWindow,
            _ => CustomEvent::CloseWindow,
        }
    }
}
*/
