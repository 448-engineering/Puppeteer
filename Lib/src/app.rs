use std::borrow::Cow;

use crate::{
    AppEnvironment, Logging, PuppeteerError, PuppeteerResult, Shell, UiPaint, WindowResize,
};
use async_executor::Executor;
use tracing::Level;
use wry::{
    application::{
        dpi::{PhysicalPosition, PhysicalSize},
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop, EventLoopBuilder, EventLoopClosed, EventLoopProxy},
        monitor::MonitorHandle,
        window::{Window, WindowBuilder},
    },
    webview::{WebView, WebViewBuilder},
};

/// This struct us used to build your app
pub struct PuppeteerApp<T: 'static> {
    title: &'static str,
    /// The app environment
    pub env: AppEnvironment,
    event_loop: EventLoop<T>,
    proxy: EventLoopProxy<T>,
    window: Window,
    primary_monitor: Option<MonitorHandle>,
    current_monitor: Option<MonitorHandle>,
    available_monitors: Vec<MonitorHandle>,
    log_filter_name: &'static str,
}

impl<T> PuppeteerApp<T>
where
    T: 'static + crate::Puppeteer + AsRef<str> + UiPaint,
{
    /// Initializes the Puppeteer app
    pub fn init(title: &'static str) -> PuppeteerResult<Self> {
        let event_loop = EventLoopBuilder::<T>::with_user_event().build();
        Logging::new(title).log("INITIALIZED EVENT_LOOP");

        let proxy = event_loop.create_proxy();
        Logging::new(title).log("INITIALIZED EVENT_LOOP PROXY");

        let window = WindowBuilder::new()
            .with_title(title)
            .with_decorations(false)
            .build(&event_loop)?;
        Logging::new(title).log("INITIALIZED WINDOW");

        let primary_monitor = window.primary_monitor();
        let current_monitor = window.current_monitor();

        if let Some(monitor_found) = primary_monitor.as_ref() {
            Logging::new(title).log(&format!("{:?}", monitor_found));
        } else {
            Logging::new(title).log("COULD NOT IDENTIFY PRIMARY MONITOR");
        }

        let mut available_monitors = Vec::<MonitorHandle>::new();
        window
            .available_monitors()
            .for_each(|monitor| available_monitors.push(monitor));
        if !available_monitors.is_empty() {
            Logging::new(title)
                .log(format!("LIST OF AVAILABLE MONITORS {:#?}", &available_monitors).as_str());
        }

        Ok(PuppeteerApp {
            title,
            event_loop,
            proxy,
            env: AppEnvironment::init(),
            window,
            primary_monitor,
            current_monitor,
            available_monitors,
            log_filter_name: title,
        })
    }

    /// Change the identifier that can be used to filter this apps log in the logs file or stdout
    pub fn change_log_filter_name(mut self, change_to: &'static str) -> Self {
        self.log_filter_name = change_to;

        self
    }

    /// Start the event loop
    pub async fn start(self) -> PuppeteerResult<()> {
        let handler = PuppeteerApp::handler(self.proxy.clone(), &self.log_filter_name);

        let devtools_enabled = if cfg!(debug_assertions) { true } else { false };

        let mut webview = Some(
            WebViewBuilder::new(self.window)?
                .with_html(T::shell().to_html())?
                .with_devtools(devtools_enabled)
                .with_ipc_handler(handler)
                .build()?,
        );

        let init_proxy = self.proxy.clone();

        let executor = Executor::new();

        executor
            .spawn(async {
                let init = T::init().await;
            })
            .detach();

        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::NewEvents(StartCause::Init) => {
                    Logging::new(&self.log_filter_name).log("INITIALIZED SUCCESSFULLY");

                    match T::window_size().get_op(webview.as_ref()) {
                        Ok(_) => (),
                        Err(error) => {
                            Logging::new(&self.log_filter_name)
                                .with_level(Level::ERROR)
                                .log(error.to_string().as_str());

                            std::process::exit(1);
                        }
                    }

                    let html = T::splashscreen();

                    let webview = get_webview_log_error(&self.log_filter_name, webview.as_ref());

                    eval_script_exit_on_error(&self.log_filter_name, webview, html);
                }
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    let _ = webview.take();
                    *control_flow = ControlFlow::Exit
                }
                Event::UserEvent(ui_event) => {}
                _ => (),
            }
        });
    }

    fn handler(
        proxy: EventLoopProxy<T>,
        log_filter_name: &'static str,
    ) -> impl Fn(&Window, String) {
        move |window: &Window, req: String| match req.as_str() {
            "minimize" => window.set_minimized(true),
            "maximize" => window.set_maximized(!window.is_maximized()),
            "drag_window" => match window.drag_window() {
                Ok(_) => (),
                Err(error) => {
                    let req_parse = T::parse(&format!("{}{}", crate::ERROR_PREFIX, error));
                    PuppeteerApp::proxy_error_handler(proxy.send_event(req_parse), log_filter_name);
                }
            },
            _ => {
                let req_parse = T::parse(&req);
                PuppeteerApp::proxy_error_handler(proxy.send_event(req_parse), log_filter_name);
            }
        }
    }

    fn proxy_error_handler(value: Result<(), EventLoopClosed<T>>, log_filter_name: &'static str) {
        match value {
            Ok(_) => (),
            Err(error) => {
                Logging::new(log_filter_name)
                    .with_level(Level::ERROR)
                    .log(error.to_string().as_str());
                std::process::exit(1);
            }
        }
    }
}

pub(crate) fn get_webview_log_error<'p>(
    app_name: &'static str,
    webview: Option<&'p WebView>,
) -> &'p WebView {
    if let Some(webview_exists) = webview {
        webview_exists
    } else {
        Logging::new(app_name)
            .with_level(Level::ERROR)
            .log(PuppeteerError::WebViewDoesNotExist.to_string().as_str());

        std::process::exit(1);
    }
}

pub(crate) fn eval_script_exit_on_error(
    app_name: &'static str,
    webview: &WebView,
    content: &dyn UiPaint,
) {
    match webview.evaluate_script(&content.to_html()) {
        Ok(_) => (),
        Err(error) => {
            Logging::new(app_name)
                .with_level(Level::ERROR)
                .log(error.to_string().as_str());

            std::process::exit(1);
        }
    }
}
