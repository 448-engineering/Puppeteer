use crate::{
    root_ui_not_found, PuppeteerResult, Shell, SplashScreen, Theme, TitleBar, TitleBarType,
    UiPaint, PUPPETEER_INITIALIZED_APP, PUPPETEER_ROOT_PAGE,
};
use std::{borrow::Cow, collections::HashMap};
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

pub type UiEvent = u64;
pub type UiPaintBoxed = Box<dyn UiPaint>;
pub type EventsMap = HashMap<u64, (&'static str, fn() -> UiPaintBoxed)>;

#[derive(Debug)]
pub struct Puppeteer {
    app_name: &'static str,
    event_loop: EventLoop<UiEvent>,
    proxy: EventLoopProxy<UiEvent>,
    window: Window,
    title_bar: TitleBar,
    splash_screen: SplashScreen,
    shell: Shell,
    events: EventsMap,
    primary_monitor: Option<MonitorHandle>,
}

impl Puppeteer {
    pub fn new(app_name: &'static str) -> PuppeteerResult<Self> {
        let splash_screen = SplashScreen::default();
        let event_loop = EventLoop::<UiEvent>::with_user_event();
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
            primary_monitor,
        })
    }

    pub fn set_splash(&mut self, splash_screen: SplashScreen) -> &mut Self {
        self.splash_screen = splash_screen;

        self
    }

    pub fn set_shell(&mut self, shell: Shell) -> &mut Self {
        self.shell = shell;

        self
    }

    pub fn set_default_theme(&mut self, theme: Theme) -> &mut Self {
        self.shell.set_theme(theme);

        self
    }

    pub fn set_title_bar(mut self, title_bar: TitleBar) -> Self {
        self.title_bar = title_bar;

        self
    }

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

    pub fn expose_window(&mut self) -> &mut Window {
        &mut self.window
    }

    pub fn register_event(&mut self, event: (&'static str, fn() -> UiPaintBoxed)) -> &mut Self {
        self.events.insert(seahash::hash(event.0.as_bytes()), event);

        self
    }

    pub fn list_events(&self) -> &EventsMap {
        &self.events
    }

    pub fn with_root_page(&mut self, page: fn() -> Box<(dyn UiPaint)>) -> &mut Self {
        self.events.insert(
            seahash::hash(PUPPETEER_ROOT_PAGE.as_bytes()),
            (PUPPETEER_ROOT_PAGE, page),
        );

        self
    }

    pub fn run(mut self, init_func: fn() -> bool) -> PuppeteerResult<()> {
        self.set_window();

        let theme = self.window.theme();
        let theme: Theme = theme.into();
        self.set_default_theme(theme);

        let proxy = self.proxy.clone();

        let handler = Puppeteer::handler(self.proxy);
        let mut webview = Some(
            WebViewBuilder::new(self.window)?
                .with_html(self.shell.to_html())?
                .with_ipc_handler(handler)
                .build()?,
        );

        let primary_monitor = self.primary_monitor;

        smol::spawn(async move {
            if init_func() {
                proxy
                    .send_event(seahash::hash(PUPPETEER_INITIALIZED_APP.as_bytes()))
                    .unwrap();
            }
        })
        .detach();

        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::NewEvents(StartCause::Init) => {
                    Puppeteer::update_webview(&mut webview, &self.splash_screen.to_html());

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
                    if ui_event == seahash::hash(PUPPETEER_INITIALIZED_APP.as_bytes()) {
                        let inner_size = webview.as_ref().unwrap().inner_size();
                        let window = webview.as_ref().unwrap().window();

                        let size_params =
                            Puppeteer::window_position_calc(primary_monitor.clone(), inner_size);
                        Puppeteer::set_outer_position(size_params.0, window);
                        Puppeteer::set_inner_size(size_params.1, window);

                        Puppeteer::update_webview(&mut webview, &self.shell.to_html());

                        let root_ui = load_root(&self.events);
                        Puppeteer::update_app(&mut webview, &root_ui.to_html());
                    }

                    if ui_event == seahash::hash(b"close_window") {
                        let _ = webview.take();
                        *control_flow = ControlFlow::Exit
                    }
                }
                _ => (),
            }
        });
    }

    fn update_webview(webview: &mut Option<WebView>, data: &str) {
        let html = Cow::Borrowed(r#"document.documentElement.innerHTML=`"#) + data + "`;";
        webview.as_ref().unwrap().evaluate_script(&html).unwrap();
    }

    fn update_app(webview: &mut Option<WebView>, data: &str) {
        let html =
            Cow::Borrowed(r#"document.getElementById("puppeteer_app").innerHTML=`"#) + data + "`;";
        webview.as_ref().unwrap().evaluate_script(&html).unwrap();
    }

    fn window_position_calc(
        primary_monitor: Option<MonitorHandle>,
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
                width: (1270f32 * 0.95) as u32,
                height: (720f32 * 0.95) as u32,
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

    fn handler(proxy: EventLoopProxy<UiEvent>) -> impl Fn(&Window, String) {
        move |window: &Window, req: String| match req.as_str() {
            "minimize" => window.set_minimized(true),
            "maximize" => window.set_maximized(!window.is_maximized()),
            "drag_window" => window.drag_window().unwrap(), //FIXME Handle me
            _ => {
                proxy.send_event(seahash::hash(req.as_bytes())).unwrap(); //FIXME
            }
        }
    }
}

fn load_root(events_map: &EventsMap) -> UiPaintBoxed {
    if let Some(root_ui) = events_map.get(&seahash::hash(PUPPETEER_ROOT_PAGE.as_bytes())) {
        root_ui.1()
    } else {
        root_ui_not_found()
    }
}
