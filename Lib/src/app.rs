use crate::{
    AppEnvironment, Logging, ModifyView, Puppeteer, PuppeteerError, PuppeteerResult, StaticAsset,
    UiEvent, UiPaint,
};
use tao::{
    dpi::PhysicalSize,
    event::{Event, StartCause, WindowEvent},
    event_loop::{
        ControlFlow, EventLoop, EventLoopBuilder, EventLoopClosed, EventLoopProxy,
        EventLoopWindowTarget,
    },
    monitor::MonitorHandle,
    window::{Window, WindowBuilder},
};
use tracing::Level;
use wry::{WebView, WebViewBuilder};

/// Environment variables set when the app is initialized
#[derive(Debug, Clone)]
pub struct ActiveAppEnv {
    /// The name of the app.
    /// This name appears in the Window Title if decorations are enabled
    /// and in the logs.
    pub app_name: &'static str, //FIXME AD ORG NAME
    /// The [AppEnvironment]
    pub env: AppEnvironment,
    /// The primary monitor as detected by the window
    pub primary_monitor: Option<MonitorHandle>,
    /// The current monitor the app is running on as detected by the window
    pub current_monitor: Option<MonitorHandle>,
    /// All the monitors that have been detected.
    /// This is mostly useful for desktops where there could be multiple monitors connected
    pub available_monitors: Vec<MonitorHandle>,
    /// List all the fonts that were loaded by the app
    pub fonts: &'static [StaticAsset],
}

/// This struct us used to build your app
pub struct PuppeteerApp<T: Puppeteer + 'static + Send + Sync> {
    /// The app environment
    pub env: ActiveAppEnv,
    event_loop: EventLoop<UiEvent<T>>,
    proxy: EventLoopProxy<UiEvent<T>>,
}

impl<T> PuppeteerApp<T>
where
    T: Puppeteer + 'static + Send + Sync,
{
    /// Initializes the Puppeteer app
    pub fn init(app_name: &'static str) -> Self {
        let event_loop = EventLoopBuilder::<UiEvent<T>>::with_user_event().build();
        Logging::new(app_name).log("INITIALIZED EVENT_LOOP");

        let proxy = event_loop.create_proxy();
        Logging::new(app_name).log("INITIALIZED EVENT_LOOP PROXY");

        PuppeteerApp {
            event_loop,
            proxy,
            env: ActiveAppEnv {
                app_name,
                env: AppEnvironment::init(),
                primary_monitor: Option::default(),
                current_monitor: Option::default(),
                available_monitors: Vec::default(),
                fonts: &[StaticAsset {
                    name: "",
                    bytes: &[0u8],
                }],
            },
        }
    }

    /// Load fonts directory
    pub fn with_fonts(mut self, fonts: &'static [StaticAsset]) -> Self {
        self.env.fonts = fonts;
        T::shell().add_fonts(&self.env);

        self
    }

    /// Start the event loop.
    /// This method is async runtime agnostic and can be used with any
    /// Rust async runtime that respects `std::future::Future`
    pub async fn start(mut self) -> PuppeteerResult<()> {
        let (webview, window) =
            PuppeteerApp::<T>::create_webview(&self.event_loop, self.proxy.clone(), &mut self.env)?;

        self.event_loop
            .run(move |event, _event_loop, control_flow| {
                *control_flow = ControlFlow::Wait;

                match event {
                    Event::NewEvents(StartCause::Init) => {
                        let view_data = T::splashscreen();

                        Self::eval_script_exit_on_error(self.env.app_name, &webview, &view_data);

                        PuppeteerApp::<T>::send_event(
                            self.proxy.clone(),
                            self.env.clone(),
                            UiEvent::Init,
                        );
                    }
                    Event::WindowEvent {
                        event: WindowEvent::CloseRequested,
                        ..
                    } => {
                        *control_flow = ControlFlow::Exit;
                    }
                    Event::UserEvent(update_view) => match update_view {
                        UiEvent::Init => {
                            PuppeteerApp::<T>::send_init_event(
                                self.proxy.clone(),
                                self.env.clone(),
                            );
                        }
                        UiEvent::Close => {
                            Logging::new(self.env.app_name).log("REQUESTED TO CLOSE WINDOW");

                            *control_flow = ControlFlow::Exit;

                            std::process::exit(0)
                        }
                        UiEvent::Minimize => window.set_minimized(true),
                        UiEvent::Maximize => window.set_maximized(!window.is_maximized()),
                        UiEvent::Drag => {
                            if window.drag_window().is_err() {
                                PuppeteerApp::<T>::send_event(
                                    self.proxy.clone(),
                                    self.env.clone(),
                                    UiEvent::Error(PuppeteerError::TaoEventLoopClosed),
                                );
                            }
                        }
                        UiEvent::Error(error) => {
                            let app_env = self.env.clone();
                            let local_proxy = self.proxy.clone();

                            smol::spawn(async move {
                                let outcome = T::error_handler(error).await;

                                PuppeteerApp::<T>::send_event(
                                    local_proxy,
                                    app_env,
                                    UiEvent::Paint(outcome),
                                );
                            })
                            .detach();
                        }
                        UiEvent::Custom(custom_event) => {
                            PuppeteerApp::<T>::send_event_from_future(
                                self.proxy.clone(),
                                self.env.clone(),
                                custom_event,
                            );
                        }
                        UiEvent::Paint(paint_data) => match paint_data {
                            ModifyView::ComputeWithIdData { func, .. } => {
                                PuppeteerApp::<T>::callback_script_by_id(
                                    self.env.app_name,
                                    &webview,
                                    self.proxy.clone(),
                                    paint_data,
                                    func,
                                )
                            }
                            ModifyView::ComputeInputWithIdData { func, .. } => {
                                PuppeteerApp::<T>::callback_script_by_id(
                                    self.env.app_name,
                                    &webview,
                                    self.proxy.clone(),
                                    paint_data,
                                    func,
                                )
                            }
                            ModifyView::Skip => (),
                            _ => PuppeteerApp::<T>::eval_script_exit_on_error(
                                self.env.app_name,
                                &webview,
                                &paint_data,
                            ),
                        },
                    },
                    _ => (),
                }
            });
    }

    fn send_init_event(proxy: EventLoopProxy<UiEvent<T>>, app_env: ActiveAppEnv) {
        smol::spawn(async move {
            let outcome = T::init(&app_env).await;
            PuppeteerApp::<T>::proxy_error_handler(
                proxy.send_event(UiEvent::Paint(outcome)),
                app_env.app_name,
            );
        })
        .detach()
    }

    fn send_event_from_future(
        proxy: EventLoopProxy<UiEvent<T>>,
        app_env: ActiveAppEnv,
        mut event: T,
    ) {
        smol::spawn(async move {
            let outcome = event.event_handler(&app_env).await;
            PuppeteerApp::<T>::proxy_error_handler(
                proxy.send_event(UiEvent::Paint(outcome)),
                app_env.app_name,
            );
        })
        .detach()
    }

    fn send_event(proxy: EventLoopProxy<UiEvent<T>>, app_env: ActiveAppEnv, event: UiEvent<T>) {
        smol::spawn(async move {
            PuppeteerApp::<T>::proxy_error_handler(proxy.send_event(event), app_env.app_name);
        })
        .detach();
    }

    fn create_webview(
        event_loop: &EventLoopWindowTarget<UiEvent<T>>,
        proxy: EventLoopProxy<UiEvent<T>>,
        app_env: &mut ActiveAppEnv,
    ) -> PuppeteerResult<(WebView, Window)> {
        let window = WindowBuilder::new()
            .with_title(app_env.app_name)
            .with_decorations(false)
            .build(event_loop)?;
        Logging::new(app_env.app_name).log("INITIALIZED WINDOW");

        let primary_monitor = event_loop.primary_monitor();
        let current_monitor = window.current_monitor();

        if let Some(monitor_found) = primary_monitor.as_ref() {
            window.set_inner_size(PhysicalSize::new(
                monitor_found.size().width as f32 * T::window_size(),
                monitor_found.size().height as f32 * T::window_size(),
            ));

            Logging::new(app_env.app_name).log(&format!("{:?}", monitor_found));
        } else {
            Logging::new(app_env.app_name).log("COULD NOT IDENTIFY PRIMARY MONITOR");
        }

        window.available_monitors().for_each(|monitor| {
            Logging::new(app_env.app_name)
                .log(format!("FOUND MONITOR -  {:#?}", &monitor).as_str());
            app_env.available_monitors.push(monitor);
        });

        app_env.primary_monitor = primary_monitor;
        app_env.current_monitor = current_monitor;

        let handler = PuppeteerApp::<T>::handler(proxy, app_env.clone());

        let devtools_enabled = cfg!(debug_assertions);

        let shell = T::shell();

        #[cfg(any(
            target_os = "windows",
            target_os = "macos",
            target_os = "ios",
            target_os = "android"
        ))]
        let webview_builder = WebViewBuilder::new(&window);
        #[cfg(not(any(
            target_os = "windows",
            target_os = "macos",
            target_os = "ios",
            target_os = "android"
        )))]
        let webview_builder = {
            use tao::platform::unix::WindowExtUnix;
            use wry::WebViewBuilderExtUnix;
            let vbox = match window.default_vbox() {
                Some(vbox) => vbox,
                None => return Err(PuppeteerError::GtkError),
            };
            WebViewBuilder::new_gtk(vbox)
        };

        let webview = webview_builder
            .with_html(shell.to_html())?
            .with_devtools(devtools_enabled)
            .with_ipc_handler(handler)
            .build()?;

        Ok((webview, window))
    }

    fn handler(
        proxy: EventLoopProxy<UiEvent<T>>,
        app_env: ActiveAppEnv,
    ) -> Box<dyn Fn(String) + 'static> {
        let outcome = move |req: String| match req.as_str() {
            "minimize" => PuppeteerApp::<T>::proxy_error_handler(
                proxy.send_event(UiEvent::Minimize),
                app_env.app_name,
            ),

            "maximize" => PuppeteerApp::<T>::proxy_error_handler(
                proxy.send_event(UiEvent::Maximize),
                app_env.app_name,
            ),
            "drag_window" => PuppeteerApp::<T>::proxy_error_handler(
                proxy.send_event(UiEvent::Drag),
                app_env.app_name,
            ),
            "close_window" => PuppeteerApp::<T>::proxy_error_handler(
                proxy.send_event(UiEvent::Close),
                app_env.app_name,
            ),
            _ => {
                let req_parse = T::parse(&req);

                PuppeteerApp::<T>::proxy_error_handler(
                    proxy.send_event(UiEvent::Custom(req_parse)),
                    app_env.app_name,
                )
            }
        };

        Box::new(outcome)
    }

    fn proxy_error_handler(
        value: Result<(), EventLoopClosed<UiEvent<T>>>,
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

    fn eval_script_exit_on_error(app_name: &'static str, webview: &WebView, content: &ModifyView) {
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

    fn callback_script_by_id(
        app_name: &'static str,
        webview: &WebView,
        proxy: EventLoopProxy<UiEvent<T>>,
        script: impl UiPaint,
        callback_fn: crate::JsCallback,
    ) {
        let callback = move |value: String| {
            if proxy
                .send_event(UiEvent::Paint(callback_fn(&value)))
                .is_err()
            {
                Logging::new(app_name)
                    .with_level(Level::ERROR)
                    .log(PuppeteerError::TaoEventLoopClosed.to_string().as_str());
            }
        };

        match webview.evaluate_script_with_callback(&script.to_html(), callback) {
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
