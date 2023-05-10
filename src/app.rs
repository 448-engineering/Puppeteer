use crate::{PuppeteerResult, UiPaint};
use wry::{
    application::{
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop, EventLoopProxy},
        window::{Window, WindowBuilder},
    },
    webview::{WebView, WebViewBuilder},
};

#[derive(Debug)]
pub struct Puppeteer<T: 'static> {
    event_loop: EventLoop<T>,
    proxy: EventLoopProxy<T>,
    window: Window,
    root_ui: &'static str,
}

impl<T> Puppeteer<T>
where
    T: core::fmt::Debug + From<String>,
{
    pub fn new(root_ui: &'static str) -> PuppeteerResult<Self> {
        let event_loop = EventLoop::<T>::with_user_event();
        let proxy = event_loop.create_proxy();
        let window = Window::new(&event_loop).unwrap();
        window.set_focus(); //TODO add optional

        Ok(Puppeteer {
            event_loop,
            proxy,
            window,
            root_ui,
        })
    }

    pub fn set_window(mut self, window_builder: WindowBuilder) -> PuppeteerResult<Self> {
        let window = window_builder.build(&self.event_loop)?;

        self.window = window;

        Ok(self)
    }

    pub fn expose_window(&mut self) -> &mut Window {
        &mut self.window
    }

    pub fn run(
        self,
        custom_event_handler: fn(
            ui_event: T,
            webview: &mut Option<WebView>,
            control_flow: &mut ControlFlow,
        ),
    ) -> PuppeteerResult<()> {
        let handler = Puppeteer::handler(self.proxy);
        let mut webview = Some(
            WebViewBuilder::new(self.window)?
                .with_html(self.root_ui)?
                .with_ipc_handler(handler)
                .with_accept_first_mouse(true)
                .build()?,
        );

        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::NewEvents(StartCause::Init) => println!("Puppeteer Application Started"), //TODO Use logging to give more useful info about the program and window like rocket does
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    let _ = webview.take();
                    *control_flow = ControlFlow::Exit
                }
                Event::UserEvent(ui_event) => {
                    custom_event_handler(ui_event, &mut webview, control_flow)
                }
                _ => (),
            }
        });
    }

    fn handler(proxy: EventLoopProxy<T>) -> impl Fn(&Window, String) {
        move |window: &Window, req: String| match req.as_str() {
            "minimize" => window.set_minimized(true),
            "maximize" => window.set_maximized(!window.is_maximized()),
            "drag_window" => window.drag_window().unwrap(), //FIXME Handle me
            _ => {
                let custom_event: T = req.into();
                proxy.send_event(custom_event).unwrap(); //FIXME
            }
        }
    }
}
