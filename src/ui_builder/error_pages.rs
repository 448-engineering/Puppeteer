use crate::{HtmlStaticContent, ModifyView};

pub fn root_ui_not_found() -> ModifyView {
    let html_data = Box::new(HtmlStaticContent {
        content: ROOT_UI_NOT_FOUND,
    });

    ModifyView::replace_app(html_data)
}

pub const ROOT_UI_NOT_FOUND: &str = r#"
<div style="display: flex; justify-content: space-around; align-items: center; flex-direction: column; min-height: 50vh; ">
    <h1>ROOT Page NOT FOUND</h1>
    <p>The Root Page is called immediately after the splash screen when the <b>init_func: fn() -> bool</b> 
        argument in the <b>run(mut self, init_func: fn() -> bool)</b> is called.</p>
    <p>Add the Root Page by calling the <b>with_root_page()</b> method on the variable you defined as <b>Puppeteer::new("App Name")</b></p>
</div>
"#;

pub fn event_not_found() -> ModifyView {
    let html_data = Box::new(HtmlStaticContent {
        content: EVENT_NOT_FOUND,
    });

    ModifyView::replace_app(html_data)
}

pub const EVENT_NOT_FOUND: &str = r#"
<div style="display: flex; justify-content: space-around; align-items: center; flex-direction: column; min-height: 50vh; ">
    <h1>EVENT NOT REGISTERED</h1>
    <p>Event was called Successfully but does not appear to be registered in the <b>EventsMap</b></p>
    <p>Register the event by calling the <b>register_event()</b> method on the variable you defined as <b>Puppeteer::new("App Name")</b></p>
</div>
"#;
