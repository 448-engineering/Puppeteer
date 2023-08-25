use crate::{AppEnvironment, Logging, PuppeteerResult, Shell, UiPaint};
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
    log_filter_name: &'static str,
}

impl<T> PuppeteerApp<T>
where
    T: 'static + crate::Puppeteer + AsRef<str> + UiPaint,
{
    /// Initializes the Puppeteer app
    pub fn init(title: &'static str) -> PuppeteerResult<Self, T> {
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

        if let Some(monitor_found) = primary_monitor.as_ref() {
            Logging::new(title).log(&format!("{:?}", monitor_found));
        } else {
            Logging::new(title).log("COULD NOT IDENTIFY PRIMARY MONITOR");
        }

        Ok(PuppeteerApp {
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
        let handler = PuppeteerApp::handler(self.proxy.clone(), &self.log_filter_name);

        let devtools_enabled = if cfg!(debug_assertions) { true } else { false };

        let mut webview = Some(
            WebViewBuilder::new(self.window)?
                .with_html(T::shell().to_html())?
                .with_devtools(devtools_enabled)
                .with_ipc_handler(handler)
                .build()?,
        );

        let primary_monitor = self.primary_monitor.as_ref().cloned();
        let init_proxy = self.proxy.clone();

        let executor = Executor::new();

        executor.spawn(async {}).detach();

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
                Event::UserEvent(ui_event) => {
                    let inner_size = webview.as_ref().unwrap().inner_size();
                    let window = webview.as_ref().unwrap().window();

                    let size_params =
                        Self::window_position_calc(primary_monitor.as_ref(), inner_size);
                    Self::set_outer_position(size_params.0, window);
                    Self::set_inner_size(size_params.1, window);
                }
                _ => (),
            }
        });
    }

    fn window_position_calc(
        primary_monitor: Option<&MonitorHandle>,
        inner_size: PhysicalSize<u32>,
    ) -> (PhysicalPosition<i32>, PhysicalSize<u32>) {
        let screen_size = if let Some(some_monitor) = primary_monitor {
            PhysicalSize {
                width: (some_monitor.size().width as f32 * 0.9) as u32,
                height: (some_monitor.size().height as f32 * 0.9) as u32,
            }
        } else {
            PhysicalSize {
                width: (1270f32 * 0.95) as u32, //FIXME Set window to maximized if outer position cannot be detected
                height: (720f32 * 0.95) as u32, //FIXME Set window to maximized if outer position cannot be detected
            }
        };

        let x = (screen_size.width as i32 - inner_size.width as i32) / 2;
        let y = (screen_size.height as i32 - inner_size.height as i32) / 2;

        (PhysicalPosition { x, y }, screen_size)
    }

    fn set_outer_position(outer_size: PhysicalPosition<i32>, window: &Window) {
        window.set_outer_position(outer_size);
    }

    fn set_inner_size(inner_size: PhysicalSize<u32>, window: &Window) {
        window.set_inner_size(inner_size);
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
