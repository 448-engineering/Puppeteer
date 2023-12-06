use crate::{
    AppEnvironment, Logging, ModifyView, PuppeteerError, PuppeteerResult, StaticAsset, UiPaint,
    WindowResize,
};
use std::{
    marker::PhantomData,
    sync::{Arc, Mutex, RwLock},
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
pub struct PuppeteerApp<T> {
    /// The app environment
    pub env: ActiveAppEnv,
    event_loop: EventLoop<ModifyView>,
    proxy: EventLoopProxy<ModifyView>,
    phantom: PhantomData<T>,
}

impl<T> PuppeteerApp<T>
where
    T: crate::Puppeteer + 'static + Send + Sync,
{
    /// Initializes the Puppeteer app
    pub fn init(app_name: &'static str) -> Self {
        let event_loop = EventLoopBuilder::<ModifyView>::with_user_event().build();
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
            phantom: PhantomData,
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
        let splash_html = T::splashscreen();

        Self::eval_script_exit_on_error(self.env.app_name, &webview, &splash_html.to_html());

        let mut webview = Some(webview);

        let mut app_webview = false;

        let init_proxy = self.proxy.clone();

        let app_env = self.env.clone();

        smol::spawn(async move {
            let init = T::init(&app_env.clone()).await;

            PuppeteerApp::<T>::proxy_error_handler(init_proxy.send_event(init), self.env.app_name)
        })
        .detach();

        self.event_loop
            .run(move |event, _event_loop, control_flow| {
                *control_flow = ControlFlow::Wait;

                match event {
                    Event::NewEvents(StartCause::Init) => {
                        Logging::new(self.env.app_name).log("LOADED SPLASHSCREEN");

                        match WindowResize::ResizePercent(T::splash_window_size()).get_op(&window) {
                            Ok(_) => (),
                            Err(error) => {
                                Logging::new(self.env.app_name)
                                    .with_level(Level::ERROR)
                                    .log(error.to_string().as_str());

                                std::process::exit(1);
                            }
                        }

                        let view_data = T::splashscreen();

                        let webview =
                            Self::get_webview_log_error(self.env.app_name, webview.as_ref());

                        Self::eval_script_exit_on_error(
                            self.env.app_name,
                            webview,
                            &view_data.to_html(),
                        );
                    }
                    Event::WindowEvent {
                        event: WindowEvent::CloseRequested,
                        ..
                    } => {
                        webview.take();
                        *control_flow = ControlFlow::Exit;
                    }
                    Event::UserEvent(update_view) => {
                        match update_view {
                            ModifyView::CloseWindow => {
                                webview.take();
                                Logging::new(self.env.app_name).log("REQUESTED TO CLOSE WINDOW");

                                *control_flow = ControlFlow::Exit;

                                std::process::exit(0)
                            }
                            ModifyView::MaximizeWindow => window.set_minimized(true),
                            ModifyView::MinimizeWindow => {
                                window.set_maximized(!window.is_maximized())
                            }
                            ModifyView::DragWindow => match window.drag_window() {
                                Ok(_) => (),
                                Err(error) => {
                                    let error = T::error_handler(error);

                                    let local_proxy = self.proxy.clone();

                                    smol::spawn(async move {
                                        let outcome = error.await;
                                        PuppeteerApp::<T>::proxy_error_handler(
                                            local_proxy.send_event(outcome),
                                            self.env.app_name,
                                        );
                                    })
                                    .detach();
                                }
                            },

                            _ => (),
                        }

                        if !app_webview {
                            app_webview = true;
                            Logging::new(self.env.app_name).log("INITIALIZED ROOT PAGE");

                            match WindowResize::ResizePercent(T::window_size()).get_op(&window) {
                                Ok(_) => (),
                                Err(error) => {
                                    Logging::new(self.env.app_name)
                                        .with_level(Level::ERROR)
                                        .log(error.to_string().as_str());

                                    std::process::exit(1);
                                }
                            }

                            match WindowResize::Center.get_op(&window) {
                                Ok(_) => (),
                                Err(error) => {
                                    Logging::new(self.env.app_name)
                                        .with_level(Level::ERROR)
                                        .log(error.to_string().as_str());

                                    std::process::exit(1);
                                }
                            }
                        }

                        match &update_view {
                            ModifyView::ReplaceApp(_) => {
                                let view_data = update_view.to_html();

                                let webview = Self::get_webview_log_error(
                                    self.env.app_name,
                                    webview.as_ref(),
                                );

                                Self::eval_script_exit_on_error(
                                    self.env.app_name,
                                    webview,
                                    &view_data.to_html(),
                                );
                            }
                            ModifyView::ComputeWithIdData { id, func } => {
                                let webview = Self::get_webview_log_error(
                                    self.env.app_name,
                                    webview.as_ref(),
                                );

                                Self::eval_script_callback_exit_on_error(
                                    self.env.app_name,
                                    webview,
                                    id,
                                    *func,
                                );
                            }
                            _ => (),
                        }
                    }
                    _ => (),
                }
            });
    }
    fn create_webview(
        event_loop: &EventLoopWindowTarget<ModifyView>,
        proxy: EventLoopProxy<ModifyView>,
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
        proxy: EventLoopProxy<ModifyView>,
        app_env: ActiveAppEnv,
    ) -> Box<dyn Fn(String) + 'static> {
        let outcome = move |req: String| match req.as_str() {
            "minimize" => PuppeteerApp::<T>::proxy_error_handler(
                proxy.send_event(ModifyView::MinimizeWindow),
                app_env.app_name,
            ),

            "maximize" => PuppeteerApp::<T>::proxy_error_handler(
                proxy.send_event(ModifyView::MaximizeWindow),
                app_env.app_name,
            ),
            "drag_window" => PuppeteerApp::<T>::proxy_error_handler(
                proxy.send_event(ModifyView::DragWindow),
                app_env.app_name,
            ),
            "close_window" => PuppeteerApp::<T>::proxy_error_handler(
                proxy.send_event(ModifyView::CloseWindow),
                app_env.app_name,
            ),
            _ => {
                let mut req_parse = T::parse(&req);

                let local_app_env = app_env.clone();
                let local_proxy = proxy.clone();

                smol::spawn(async move {
                    let outcome = req_parse.event_handler(local_app_env).await;
                    PuppeteerApp::<T>::proxy_error_handler(
                        local_proxy.send_event(outcome),
                        app_env.app_name,
                    );
                })
                .detach();
            }
        };

        Box::new(outcome)
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

    fn eval_script_callback_exit_on_error(
        app_name: &'static str,
        webview: &WebView,
        id: &str,
        compute_fn: fn(&str) -> Box<dyn UiPaint>,
    ) {
        let script =
            String::new() + "var fetchData = document.getElementById('" + id + "').textContent;";
        let mut outcome = String::new();

        use std::sync::mpsc::channel;

        let callback = move |value: String| {
            let eval_outcome = compute_fn(&value).to_html().to_string();
            Box::new(webview.evaluate_script("js"));
        };

        match webview.evaluate_script_with_callback(&script, callback) {
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
