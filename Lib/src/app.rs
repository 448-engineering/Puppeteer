use crate::{AppEnvironment, PuppeteerResult};
use async_executor::Executor;
use tracing::Level;
use wry::{
    application::{
        dpi::{PhysicalPosition, PhysicalSize},
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop, EventLoopBuilder, EventLoopClosed, EventLoopProxy},
        monitor::MonitorHandle,
        window::Window,
    },
    webview::{WebView, WebViewBuilder},
};

/// This struct us used to build your app
pub struct Puppeteer<T: 'static> {
    title: &'static str,
    /// The app environment
    pub env: AppEnvironment,
    event_loop: EventLoop<T>,
    proxy: EventLoopProxy<T>,
    window: Window,
    primary_monitor: Option<MonitorHandle>,
    log_filter_name: &'static str,
}

impl<T> Puppeteer<T>
where
    T: 'static + crate::InvokeWebView + AsRef<str>,
{
    /// Initializes the Puppeteer app
    pub fn init(title: &'static str) -> PuppeteerResult<Self, T> {
        let event_loop = EventLoopBuilder::<T>::with_user_event().build();
        Logging::new(title).log("INITIALIZED EVENT_LOOP");

        let proxy = event_loop.create_proxy();
        Logging::new(title).log("INITIALIZED EVENT_LOOP PROXY");

        let window = Window::new(&event_loop)?;
        Logging::new(title).log("INITIALIZED WINDOW");
        window.set_decorations(false);

        let primary_monitor = window.primary_monitor();

        if let Some(monitor_found) = primary_monitor.as_ref() {
            Logging::new(title).log(&format!("{:?}", monitor_found));
        } else {
            Logging::new(title).log("COULD NOT IDENTIFY PRIMARY MONITOR");
        }

        Ok(Puppeteer {
            title,
            event_loop,
            proxy,
            env: AppEnvironment::init(),
            window,
            primary_monitor,
            log_filter_name: title,
        })
    }

    /// Change the identifier that can be used to filter this apps log in the logs file or stdout
    pub fn change_log_filter_name(mut self, change_to: &'static str) -> Self {
        self.log_filter_name = change_to;

        self
    }

    /// Start the event loop
    pub async fn start(self) -> PuppeteerResult<(), T> {
        let handler = Puppeteer::handler(self.proxy.clone(), &self.log_filter_name);

        let mut webview = Some(
            WebViewBuilder::new(self.window)?
                .with_html("FOO")?
                .with_ipc_handler(handler)
                .build()?,
        );

        let primary_monitor = self.primary_monitor.as_ref().cloned();
        let init_proxy = self.proxy.clone();

        let executor = Executor::new();

        executor.spawn(async {}).await;

        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::NewEvents(StartCause::Init) => {
                    Logging::new(&self.log_filter_name).log("Puppeteer Application Started");
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
                    let req_parse = T::parse(&format!("{}{}", ERROR_PREFIX, error));
                    Puppeteer::proxy_error_handler(proxy.send_event(req_parse), log_filter_name);
                }
            },
            _ => {
                let req_parse = T::parse(&req);
                Puppeteer::proxy_error_handler(proxy.send_event(req_parse), log_filter_name);
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

/// Name that can be used to quickly identify all filters of the running app in logs.
pub const LOGGING_SYMBOL: &str = "» ";
/// This is the String used to indicate an error has been sent via the `EventLoopProxy`
pub const ERROR_PREFIX: &str = "PuppeteerError» ";

/// Custom logging handler for Puppeteer apps.
/// Text filtering can be done by searching for `[app_title]»`
#[derive(Debug, PartialEq, Eq)]
pub struct Logging {
    level: Level,
    app_name: &'static str,
}

impl Logging {
    /// Add the app name to be used in logging so that the logs can be filtered using the app name.
    /// The default tracing level is info
    pub fn new(app_name: &'static str) -> Self {
        Logging {
            level: Level::INFO,
            app_name,
        }
    }

    /// Change the name used to identify logging by this app in the logs.
    /// Default is `Level::INFO `.
    pub fn with_level(mut self, level: Level) -> Self {
        self.level = level;

        self
    }

    /// Log the message. This is a simple logging mechanism
    pub fn log(self, message: &str) {
        match self.level {
            Level::DEBUG => tracing::debug!("{}{}{}", self.app_name, LOGGING_SYMBOL, message),
            Level::ERROR => tracing::error!("{}{}{}", self.app_name, LOGGING_SYMBOL, message),
            Level::INFO => tracing::info!("{}{}{}", self.app_name, LOGGING_SYMBOL, message),
            Level::TRACE => tracing::trace!("{}{}{}", self.app_name, LOGGING_SYMBOL, message),
            Level::WARN => tracing::warn!("{}{}{}", self.app_name, LOGGING_SYMBOL, message),
        }
    }
}

/// Used to add styles, scripts and fonts.
/// It can also change the webview title in HTML.
pub struct Shell {}
