use crate::{
    PuppeteerResult, Shell, SplashScreen, TitleBar, TitleBarType, UiPaint,
    PUPPETEER_INITIALIZED_APP,
};
use std::collections::HashMap;
use wry::{
    application::{
        dpi::{PhysicalPosition, PhysicalSize, Position, Size},
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop, EventLoopProxy},
        monitor::MonitorHandle,
        window::{self, Window},
    },
    webview::WebViewBuilder,
};

pub type UiEvent = u64;
pub type UiPaintBoxed = Box<(dyn UiPaint + 'static)>;
pub type EventsMap = HashMap<u64, (&'static str, fn() -> UiPaintBoxed)>;

#[derive(Debug)]
pub struct Puppeteer<'p> {
    app_name: &'static str,
    event_loop: EventLoop<UiEvent>,
    proxy: EventLoopProxy<UiEvent>,
    window: Window,
    title_bar: TitleBar<'p>,
    pub shell: Shell<'p>,
    active: UiPaintBoxed,
    events: EventsMap,
    primary_monitor: Option<MonitorHandle>,
}

impl<'p> Puppeteer<'p> {
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
            active: Box::new(splash_screen),
            events: HashMap::default(),
            primary_monitor,
        })
    }

    pub fn set_splash(&mut self, splash_screen: SplashScreen) -> &mut Self {
        self.active = Box::new(splash_screen);

        self
    }

    pub fn set_title_bar(mut self, title_bar: TitleBar<'p>) -> Self {
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

    pub fn run(mut self, init_func: fn() -> bool) -> PuppeteerResult<()> {
        self.set_window();

        let proxy = self.proxy.clone();

        let shell = self.shell.set_content(self.active);
        let handler = Puppeteer::handler(self.proxy);
        let webview = WebViewBuilder::new(self.window)?
            .with_html(shell.to_html())?
            .with_ipc_handler(handler)
            .build()?;

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
                    //println!("Puppeteer Application Started"); //TODO Use logging to give more useful info about the program and window like rocket does
                }
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    //let _ = webview.take();
                    *control_flow = ControlFlow::Exit
                }
                Event::UserEvent(ui_event) => {
                    if ui_event == seahash::hash(PUPPETEER_INITIALIZED_APP.as_bytes()) {
                        let inner_size = webview.inner_size();
                        let window = webview.window();

                        let size_params =
                            Puppeteer::window_position_calc(primary_monitor.clone(), inner_size);

                        Puppeteer::set_outer_position(size_params.0, window);
                        Puppeteer::set_inner_size(size_params.1, window);

                        webview
                        .evaluate_script(
                            r#"document.body.innerHTML = "<html><body>AFTER SPLASH</body></html>""#,
                        )
                        .unwrap();
                    }
                }
                _ => (),
            }
        });
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
