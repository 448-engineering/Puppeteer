use crate::{AppEnvironment, Logging, ModifyView, PuppeteerError, PuppeteerResult, UiPaint};
use async_executor::Executor;
use std::{future::Future, marker::PhantomData};
use tracing::Level;
use wry::{
    application::{
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop, EventLoopBuilder, EventLoopClosed, EventLoopProxy},
        monitor::MonitorHandle,
        window::{Window, WindowBuilder},
    },
    webview::{WebView, WebViewBuilder},
};

/// Environment variables set when the app is initialized
#[derive(Debug, Clone)]
pub struct ActiveAppEnv {
    /// The name of the app.
    /// This name appears in the Window Title if decorations are enabled
    /// and in the logs.
    pub app_name: &'static str,
    /// The [AppEnvironment]
    pub env: AppEnvironment,
    /// The primary monitor as detected by the window
    pub primary_monitor: Option<MonitorHandle>,
    /// The current monitor the app is running on as detected by the window
    pub current_monitor: Option<MonitorHandle>,
    /// All the monitors that have been detected.
    /// This is mostly useful for desktops where there could be multiple monitors connected
    pub available_monitors: Vec<MonitorHandle>,
}

/// This struct us used to build your app
pub struct PuppeteerApp<T> {
    /// The app environment
    pub env: ActiveAppEnv,
    event_loop: EventLoop<ModifyView>,
    proxy: EventLoopProxy<ModifyView>,
    window: Window,
    phantom: PhantomData<T>,
}

impl<T> PuppeteerApp<T>
where
    T: crate::Puppeteer + 'static,
{
    /// Initializes the Puppeteer app
    pub fn init(app_name: &'static str) -> PuppeteerResult<Self> {
        let event_loop = EventLoopBuilder::<ModifyView>::with_user_event().build();
        Logging::new(app_name).log("INITIALIZED EVENT_LOOP");

        let proxy = event_loop.create_proxy();
        Logging::new(app_name).log("INITIALIZED EVENT_LOOP PROXY");

        let window = WindowBuilder::new()
            .with_title(app_name)
            .with_decorations(false)
            .build(&event_loop)?;
        Logging::new(app_name).log("INITIALIZED WINDOW");

        let primary_monitor = window.primary_monitor();
        let current_monitor = window.current_monitor();

        if let Some(monitor_found) = primary_monitor.as_ref() {
            Logging::new(app_name).log(&format!("{:?}", monitor_found));
        } else {
            Logging::new(app_name).log("COULD NOT IDENTIFY PRIMARY MONITOR");
        }

        let mut available_monitors = Vec::<MonitorHandle>::new();
        window
            .available_monitors()
            .for_each(|monitor| available_monitors.push(monitor));
        if !available_monitors.is_empty() {
            Logging::new(app_name)
                .log(format!("LIST OF AVAILABLE MONITORS {:#?}", &available_monitors).as_str());
        }

        let env = ActiveAppEnv {
            app_name,
            env: AppEnvironment::init(),
            primary_monitor,
            current_monitor,
            available_monitors,
        };

        Ok(PuppeteerApp {
            event_loop,
            proxy,
            env,
            window,
            phantom: PhantomData::default(),
        })
    }

    /// Start the event loop.
    /// This method is async runtime agnostic and can be used with any
    /// Rust async runtime that respects `std::future::Future`
    pub async fn start(self) -> PuppeteerResult<()> {
        let handler = PuppeteerApp::<T>::handler(self.proxy.clone(), self.env.clone());

        let devtools_enabled = if cfg!(debug_assertions) { true } else { false };

        let mut webview = Some(
            WebViewBuilder::new(self.window)?
                .with_html(T::shell().to_html())?
                .with_devtools(devtools_enabled)
                .with_ipc_handler(handler)
                .build()?,
        );

        let app_name = self.env.app_name.clone();

        let init_proxy = self.proxy.clone();

        let executor = Executor::new();

        executor
            .spawn(async move {
                let init = T::init().await;

                PuppeteerApp::<T>::proxy_error_handler(init_proxy.send_event(init), app_name)
            })
            .detach();

        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::NewEvents(StartCause::Init) => {
                    Logging::new(&app_name).log("INITIALIZED SUCCESSFULLY");

                    match T::window_size().get_op(webview.as_ref()) {
                        Ok(_) => (),
                        Err(error) => {
                            Logging::new(&app_name)
                                .with_level(Level::ERROR)
                                .log(error.to_string().as_str());

                            std::process::exit(1);
                        }
                    }

                    let html = T::splashscreen();

                    let webview = Self::get_webview_log_error(&app_name, webview.as_ref());

                    Self::eval_script_exit_on_error(&app_name, webview, html);
                }
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    let _ = webview.take();
                    *control_flow = ControlFlow::Exit
                }
                Event::UserEvent(ui_event) => {
                    let webview = Self::get_webview_log_error(&app_name, webview.as_ref());

                    Self::eval_script_exit_on_error(&app_name, webview, &ui_event);
                }
                _ => (),
            }
        });
    }

    fn handler(
        proxy: EventLoopProxy<ModifyView>,
        app_env: ActiveAppEnv,
    ) -> impl Fn(&Window, String) {
        move |window: &Window, req: String| match req.as_str() {
            "minimize" => window.set_minimized(true),
            "maximize" => window.set_maximized(!window.is_maximized()),
            "drag_window" => match window.drag_window() {
                Ok(_) => (),
                Err(error) => {
                    let error = T::error_handler(error);

                    async_executor::Executor::new()
                        .spawn(async {
                            let outcome = error.await;
                            PuppeteerApp::<T>::proxy_error_handler(
                                proxy.send_event(outcome),
                                app_env.app_name,
                            );
                        })
                        .detach();
                }
            },
            _ => {
                let mut req_parse = T::parse(&req);
                let prepare_outcome: std::pin::Pin<Box<dyn Future<Output = ModifyView> + Send>> =
                    req_parse.event_handler(&app_env, window);

                async_executor::Executor::new()
                    .spawn(async {
                        let outcome = prepare_outcome.await;
                        PuppeteerApp::<T>::proxy_error_handler(
                            proxy.send_event(outcome),
                            app_env.app_name,
                        );
                    })
                    .detach();
            }
        }
    }

    fn proxy_error_handler(
        value: Result<(), EventLoopClosed<ModifyView>>,
        log_filter_name: &'static str,
    ) {
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

    fn get_webview_log_error<'p>(
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

    fn eval_script_exit_on_error(app_name: &'static str, webview: &WebView, content: &dyn UiPaint) {
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
}
