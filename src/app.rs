#![forbid(unsafe_code)]

use crate::{
    EventHandler, ModifyView, PuppeteerResult, Shell, SplashScreen, Theme, TitleBar, TitleBarType,
    UiPaint, COMMAND_APP, COMMAND_INIT, COMMAND_SHELL,
};
use std::borrow::Cow;
use wry::{
    application::{
        dpi::{PhysicalPosition, PhysicalSize},
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop, EventLoopProxy},
        monitor::MonitorHandle,
        window::Window,
    },
    webview::{WebView, WebViewBuilder},
};

pub type PageOp = String;

/// The entrypoint for a Puppeteer based app.
/// The structure of this struct is:
/// ```rust
/// #[derive(Debug)]
/// pub struct Puppeteer {
///     app_name: &'static str,
///     event_loop: EventLoop<PageOp>,
///     proxy: EventLoopProxy<PageOp>,
///     window: Window,
///     title_bar: TitleBar,
///     splash_screen: SplashScreen,
///     shell: Shell,
///     events: EventsMap,
///     primary_monitor: Option<MonitorHandle>,
/// }
/// ```
#[derive(Debug)]
pub struct Puppeteer {
    app_name: &'static str,
    event_loop: EventLoop<PageOp>,
    proxy: EventLoopProxy<PageOp>,
    window: Window,
    title_bar: TitleBar,
    splash_screen: SplashScreen,
    shell: Shell,
    primary_monitor: Option<MonitorHandle>,
}

impl Puppeteer {
    /// Create a new app. This function takes the app name as function arguments.
    /// This app name appears on the status bar and the window bar.
    /// It also initializes the Puppeteer struct with [SplashScreen::default()] for the `splashscreen` field.
    /// The `EventLoop` and `EventLoopProxy` is also initialized and passed to the window.
    pub fn new(app_name: &'static str) -> PuppeteerResult<Self> {
        let splash_screen = SplashScreen::default();
        let event_loop = EventLoop::<PageOp>::with_user_event();
        let proxy = event_loop.create_proxy();
        let window = Window::new(&event_loop).unwrap();
        let primary_monitor = window.primary_monitor();

        let title_bar = TitleBar::default().set_text_content(app_name);

        Ok(Puppeteer {
            app_name,
            event_loop,
            proxy,
            window,
            title_bar,
            shell: Shell::default(),
            splash_screen,
            primary_monitor,
        })
    }

    /// Replaces the default splash screen. It is initialized using [SplashScreen]
    pub fn set_splash(&mut self, splash_screen: SplashScreen) -> &mut Self {
        self.splash_screen = splash_screen;

        self
    }

    /// Replace the default [Shell] with a new custom [Shell]
    pub fn set_shell(&mut self, shell: Shell) -> &mut Self {
        self.shell = shell;

        self
    }

    /// Change the default theme to a new [Theme]. The default theme is dark mode
    pub fn set_default_theme(&mut self, theme: Theme) -> &mut Self {
        self.shell.set_theme(theme);

        self
    }

    /// Change the default title bar to a custom [TitleBar]
    pub fn set_title_bar(mut self, title_bar: TitleBar) -> Self {
        self.title_bar = title_bar;

        self
    }

    /// Change the type of title bar using the enum [TitleBarType].
    /// The [TitleBarType] is used to enable or disable the OS window decorations or
    /// create a custom one.
    pub fn set_title_bar_type(mut self, title_bar: TitleBarType) -> Self {
        self.title_bar.set_title_bar_type_borrowed(title_bar);

        self
    }

    fn set_window(&mut self) -> &mut Self {
        let window = match self.title_bar.title_bar_type() {
            TitleBarType::Native => {
                let window = Window::new(&self.event_loop).unwrap();
                window.set_decorations(true);
                window.set_focus();
                window.set_title(self.app_name);

                window
            }
            _ => {
                let window = Window::new(&self.event_loop).unwrap();
                window.set_decorations(false);
                window.set_focus();
                window.set_title(self.app_name);

                window
            }
        };

        self.window = window;

        self
    }

    /// Get the initialized `Window`
    pub fn expose_window(&mut self) -> &mut Window {
        &mut self.window
    }

    /// Expose EventLoop
    pub fn event_loop(&self) -> &EventLoop<PageOp> {
        &self.event_loop
    }

    /// This function is used to run the app in the `EventLoop`.
    /// It takes an initialization function `init_func` as argument.
    /// This function that is used to initialize all the functionality needed to
    /// run the app, like initializing the app database or app cache.
    /// This initialization function returns a `bool` value where if the value
    /// returned is `true` it will load the `root page` and if the value returned
    /// is false it will load the initialization error page which you can
    /// create a custom one using the `initialization_error_page()` method on the
    /// [Puppeteer] struct.
    pub fn run<T: From<String> + EventHandler + core::fmt::Debug + Send + Sync>(
        mut self,
    ) -> PuppeteerResult<()> {
        self.set_window();

        let theme = self.window.theme();
        let theme: Theme = theme.into();
        self.set_default_theme(theme);

        let proxy = self.proxy.clone();

        let handler = Puppeteer::handler(proxy);
        let mut webview = Some(
            WebViewBuilder::new(self.window)?
                .with_html(self.shell.to_html())?
                .with_ipc_handler(handler)
                .build()?,
        );

        let primary_monitor = self.primary_monitor.as_ref().cloned();
        let init_proxy = self.proxy.clone();

        smol::spawn(async move {
            let initialized = T::init_func().await;

            init_proxy.send_event(COMMAND_INIT.to_string()).unwrap();

            let app_html = Cow::Borrowed(COMMAND_APP) + initialized.to_html();

            init_proxy.send_event(app_html.to_string()).unwrap();
        })
        .detach();

        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::NewEvents(StartCause::Init) => {
                    Puppeteer::view_ops(
                        &mut webview,
                        &ModifyView::ReplaceShell(Box::new(self.splash_screen.clone())),
                    );

                    println!("Puppeteer Application Started"); //TODO Use logging to give more useful info about the program and window like rocket does
                }
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    let _ = webview.take();
                    *control_flow = ControlFlow::Exit
                }
                Event::UserEvent(ui_event) => {
                    if ui_event.as_str().starts_with(COMMAND_INIT) {
                        let inner_size = webview.as_ref().unwrap().inner_size();
                        let window = webview.as_ref().unwrap().window();

                        let size_params =
                            Puppeteer::window_position_calc(primary_monitor.as_ref(), inner_size);
                        Puppeteer::set_outer_position(size_params.0, window);
                        Puppeteer::set_inner_size(size_params.1, window);

                        let html = Cow::Borrowed(r#"document.documentElement.innerHTML=`"#)
                            + self.shell.to_html()
                            + "`;";

                        Puppeteer::view_ops_prepared(&mut webview, &html);
                    } else if ui_event.as_str().starts_with(COMMAND_SHELL) {
                        let html = Cow::Borrowed(r#"document.documentElement.innerHTML=`"#)
                            + self.shell.to_html()
                            + "`;";

                        Puppeteer::view_ops_prepared(&mut webview, &html);
                    } else if ui_event.as_bytes() == "close_window".as_bytes() {
                        let _ = webview.take();
                        *control_flow = ControlFlow::Exit
                    } else if ui_event.as_str().starts_with(COMMAND_APP) {
                        let html = ui_event.replace(COMMAND_APP, "");

                        Puppeteer::view_ops_prepared(&mut webview, &html);
                    } else {
                        let proxy = self.proxy.clone();

                        smol::spawn(async move {
                            let parsed: T = ui_event.into();

                            let executed = parsed.view_model().await;
                            let prepared = Cow::Borrowed(COMMAND_APP) + executed.to_html();

                            proxy.send_event(prepared.to_string()).unwrap();
                        })
                        .detach();
                    }
                }
                _ => (),
            }
        });
    }

    fn view_ops(webview: &mut Option<WebView>, data: &ModifyView) {
        let html = data.to_html();

        webview.as_ref().unwrap().evaluate_script(&html).unwrap();
    }

    fn view_ops_prepared(webview: &mut Option<WebView>, html: &str) {
        webview.as_ref().unwrap().evaluate_script(&html).unwrap();
    }

    fn window_position_calc(
        primary_monitor: Option<&MonitorHandle>,
        inner_size: PhysicalSize<u32>,
    ) -> (PhysicalPosition<i32>, PhysicalSize<u32>) {
        dbg!(&primary_monitor); //TODO Log this

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

    fn handler(proxy: EventLoopProxy<String>) -> impl Fn(&Window, String) {
        move |window: &Window, req: String| match req.as_str() {
            "minimize" => window.set_minimized(true),
            "maximize" => window.set_maximized(!window.is_maximized()),
            "drag_window" => window.drag_window().unwrap(), //FIXME Handle me
            _ => proxy.send_event(req).unwrap(),
        }
    }
}
