use crate::UiPaint;

pub fn to_html_event<T: UiPaint>(event: T) -> String {
    String::from("window.ipc.postMessage('") + event.to_html().as_str() + "')"
}
