use crate::{
    root_ui_not_found, EventHandler, ModifyView, Page, PuppeteerResult, Shell, SplashScreen, Theme,
    TitleBar, TitleBarType, UiPaint, COMMAND_ROOT_UI, COMMAND_UI_UPDATE, INIT_ERROR_PAGE_NOT_FOUND,
    PUPPETEER_INITIALIZED_APP, PUPPETEER_INIT_ERROR_PAGE, PUPPETEER_ROOT_PAGE,
};
use async_dup::Arc;
use async_lock::RwLock;
use smol::{
    channel::{bounded, Receiver, Sender},
    future::zip,
};
use std::{borrow::Cow, collections::HashMap, future::Future};
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
pub type EventsAsyncFunction = dyn Future<Output = String> + 'static;
pub type UiPaintBoxed = Box<dyn UiPaint>;
pub type EventsMap = HashMap<u64, (&'static str, fn(String) -> EventsAsyncFunction)>;
pub type PagesMap = HashMap<u64, (&'static str, ModifyView)>;

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
pub struct Puppeteer {
    app_name: &'static str,
    event_loop: EventLoop<PageOp>,
    proxy: EventLoopProxy<PageOp>,
    window: Window,
    title_bar: TitleBar,
    splash_screen: SplashScreen,
    shell: Shell,
    events: EventsMap,
    pages: PagesMap,
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
            events: HashMap::default(),
            pages: HashMap::default(),
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

    /// Register a page
    pub fn register_page(&mut self, page_name: &'static str, page_op: ModifyView) -> &mut Self {
        self.pages
            .insert(seahash::hash(page_name.as_bytes()), (page_name, page_op));

        self
    }

    /// Register a page
    pub fn list_pages(&self) -> &PagesMap {
        &self.pages
    }

    /// Register an event. This event will be called from the UI code.
    /// This function arguments take an `event_name` which is used to lookup the event and
    /// a `callback` function which is called to execute the function.
    /// The function signature is `fn() -> ModifyView` . This in a function that does not take
    /// any arguments and  returns a [ModifyView] which is executed by the `EventLoop` to
    /// either change the whole content of the `WebView` or the `app` or an element by it's `id` or `class` name.
    pub fn register_event(
        &mut self,
        event_name: &'static str,
        callback: fn(String) -> (dyn Future<Output = String> + 'static),
    ) -> &mut Self {
        self.events
            .insert(seahash::hash(event_name.as_bytes()), (event_name, callback));

        self
    }

    /// List all the registered events
    pub fn list_events(&self) -> &EventsMap {
        &self.events
    }

    /// Add the UI for the root page. The root page is the page immediately loaded after
    /// the app has initialized and it replaces splash screen
    pub fn with_root_page(&mut self, page: ModifyView) -> &mut Self {
        self.pages.insert(
            seahash::hash(PUPPETEER_ROOT_PAGE.as_bytes()),
            (PUPPETEER_ROOT_PAGE, page),
        );

        self
    }

    pub fn initialization_error_page(&mut self, page: ModifyView) -> &mut Self {
        self.pages.insert(
            seahash::hash(PUPPETEER_INIT_ERROR_PAGE.as_bytes()),
            (PUPPETEER_INIT_ERROR_PAGE, page),
        );

        self
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
    pub fn run<T: ToString + EventHandler + core::fmt::Debug>(
        mut self,
        init_func: fn() -> bool,
    ) -> PuppeteerResult<()> {
        let (sender, receiver) = bounded::<String>(20);

        /*smol::block_on(async {
            zip(
                self.event_loop_runner(init_func, sender),
                Puppeteer::page_ops_listener(receiver),
            )
            .await;

            Ok(())
        })*/
        let proxy = self.proxy.clone();

        Ok(())
    }

    async fn page_ops_listener(receiver: Receiver<u64>) {
        while let Ok(page) = receiver.recv().await {}
    }

    async fn event_loop_runner<T: From<String> + EventHandler + core::fmt::Debug + Send + Sync>(
        mut self,
        init_func: fn() -> bool,
        parser_func: Sender<String>,
        pages_map: PagesMap,
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
        let proxy = self.proxy.clone();

        smol::spawn(async move {
            /*if init_func() {
                proxy
                    .send_event(PUPPETEER_INITIALIZED_APP.to_owned())
                    .unwrap();
            } else {
                proxy
                    .send_event(PUPPETEER_INIT_ERROR_PAGE.to_string())
                    .unwrap();
            }*/
        })
        .detach();

        let (sender, receiver) = bounded::<String>(20);

        smol::spawn(async move {
            while let Ok(received) = receiver.recv().await {
                let parsed = T::event_logic(&received);

                let executed = parsed.view_model().await;
                let prepared = Cow::Borrowed(COMMAND_UI_UPDATE) + executed.to_html();

                proxy.send_event(prepared.to_string()).unwrap();
            }
        })
        .detach();

        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::NewEvents(StartCause::Init) => {
                    Puppeteer::view_ops(
                        &mut webview,
                        &ModifyView::ReplaceView(Box::new(self.splash_screen.clone())),
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
                    if ui_event.as_str().starts_with(COMMAND_ROOT_UI) {
                        let inner_size = webview.as_ref().unwrap().inner_size();
                        let window = webview.as_ref().unwrap().window();

                        let size_params =
                            Puppeteer::window_position_calc(primary_monitor.as_ref(), inner_size);
                        Puppeteer::set_outer_position(size_params.0, window);
                        Puppeteer::set_inner_size(size_params.1, window);

                        Puppeteer::view_ops(
                            &mut webview,
                            &ModifyView::ReplaceView(Box::new(self.shell.clone())),
                        );

                        if let Some(root_ui) =
                            pages_map.get(&seahash::hash(PUPPETEER_ROOT_PAGE.as_bytes()))
                        {
                            Puppeteer::view_ops(&mut webview, &root_ui.1);
                        } else {
                            Puppeteer::view_ops(&mut webview, &root_ui_not_found());
                        };
                    } else if ui_event.as_bytes() == "close_window".as_bytes() {
                        let _ = webview.take();
                        *control_flow = ControlFlow::Exit
                    } else if ui_event.as_str().starts_with(COMMAND_UI_UPDATE) {
                    } else {
                        smol::block_on(async { sender.send(ui_event).await.unwrap() })
                    }

                    //smol::spawn(async { proxy.send_event(String::new()).unwrap() }).detach();

                    /*if ui_event.as_bytes() == PUPPETEER_INITIALIZED_APP.as_bytes() {
                        let inner_size = webview.as_ref().unwrap().inner_size();
                        let window = webview.as_ref().unwrap().window();

                        let size_params =
                            Puppeteer::window_position_calc(primary_monitor.clone(), inner_size);
                        Puppeteer::set_outer_position(size_params.0, window);
                        Puppeteer::set_inner_size(size_params.1, window);

                        Puppeteer::view_ops(
                            &mut webview,
                            ModifyView::ReplaceView(Box::new(self.shell.clone())),
                        );

                        let root_ui = if let Some(root_ui) =
                            pages_map.get(&seahash::hash(PUPPETEER_ROOT_PAGE.as_bytes()))
                        {
                            root_ui.1
                        } else {
                            root_ui_not_found()
                        };
                        Puppeteer::view_ops(&mut webview, root_ui);
                    } else if ui_event.as_bytes() == "close_window".as_bytes() {
                        let _ = webview.take();
                        *control_flow = ControlFlow::Exit
                    } else if ui_event == error_page_hash {
                        if let Some(page) = self.events.get(&error_page_hash) {
                            Puppeteer::view_ops(&mut webview, page.1());
                        } else {
                            Puppeteer::view_ops(
                                &mut webview,
                                ModifyView::ReplaceApp(INIT_ERROR_PAGE_NOT_FOUND.into()),
                            );
                        }
                    } else if let Some(registered_event) = &self.events.get(&ui_event) {
                        let outcome = registered_event.1();

                        Puppeteer::view_ops(&mut webview, outcome);
                    } else {
                        Puppeteer::view_ops(&mut webview, event_not_found())
                    }*/
                }
                _ => (),
            }
        });
    }

    fn view_ops(webview: &mut Option<WebView>, data: &ModifyView) {
        let html = data.to_html();

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
