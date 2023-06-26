use crate::{PuppeteerResult, Shell, SplashScreen, TitleBar, TitleBarType, UiPaint};
use std::collections::HashMap;
use wry::{
    application::{
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop, EventLoopProxy},
        window::Window,
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
    shell: Shell<'p>,
    active: UiPaintBoxed,
    events: EventsMap,
    init_func: Option<fn() -> bool>,
    storage: Option<fn()>,
    // reset all styles by default
}

impl<'p> Puppeteer<'p> {
    pub fn new(app_name: &'static str) -> PuppeteerResult<Self> {
        let splash_screen = SplashScreen::default();
        let event_loop = EventLoop::<UiEvent>::with_user_event();
        let proxy = event_loop.create_proxy();
        let window = Window::new(&event_loop).unwrap();

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
            init_func: Option::default(),
            storage: Option::default(),
        })
    }

    pub fn set_init(mut self, init_func: fn() -> bool) -> Self {
        self.init_func = Some(init_func);

        self
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

    pub fn run(mut self) -> PuppeteerResult<()> {
        self.set_window();

        let proxy = self.proxy.clone();

        let shell = self.shell.set_content(self.active);
        let handler = Puppeteer::handler(self.proxy);
        let webview = WebViewBuilder::new(self.window)?
            .with_html(shell.to_html())?
            .with_ipc_handler(handler)
            .build()?;

        let init_func = self.init_func.unwrap();

        smol::spawn(async move {
            init_func();
            proxy.send_event(seahash::hash(b"initializedApp")).unwrap();
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
                    if ui_event == seahash::hash(b"initializedApp") {

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
