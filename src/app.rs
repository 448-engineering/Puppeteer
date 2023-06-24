use crate::{Image, Page, PuppeteerResult, Shell, SplashScreen, TitleBar, TitleBarType};
use wry::{
    application::{
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop, EventLoopProxy},
        window::{Window, WindowBuilder},
    },
    webview::{WebView, WebViewBuilder},
};

#[derive(Debug)]
pub struct Puppeteer<'p, T: 'static> {
    app_name: &'static str,
    event_loop: EventLoop<T>,
    proxy: EventLoopProxy<T>,
    window: Window,
    splash_screen: SplashScreen,
    title_bar: TitleBar<'p>,
    init: Option<fn() -> Page>,
    storage: Option<fn()>,
    //shell // HERE load the shell after splashscreen,
    // reset all styles by default
}

impl<'p, T> Puppeteer<'p, T>
where
    T: core::fmt::Debug + From<String>,
{
    pub fn new(app_name: &'static str) -> PuppeteerResult<Self> {
        let splash_screen = SplashScreen::default();
        let event_loop = EventLoop::<T>::with_user_event();
        let proxy = event_loop.create_proxy();
        let window = Window::new(&event_loop).unwrap();

        let title_bar = TitleBar::default().set_text_content(app_name);

        Ok(Puppeteer {
            app_name,
            event_loop,
            proxy,
            window,
            splash_screen,
            title_bar,
            init: Option::default(),
            storage: Option::default(),
        })
    }

    pub fn set_splash(&mut self, splash_screen: SplashScreen) -> &mut Self {
        self.splash_screen = splash_screen;

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

    pub fn run(
        mut self,
        custom_event_handler: fn(
            ui_event: T,
            webview: &mut Option<WebView>,
            control_flow: &mut ControlFlow,
        ),
    ) -> PuppeteerResult<()> {
        self.set_window();

        let shell = Shell::new().set_content(self.splash_screen.build()).build();

        let handler = Puppeteer::handler(self.proxy);
        let mut webview = Some(
            WebViewBuilder::new(self.window)?
                .with_html(shell)?
                .with_ipc_handler(handler)
                .with_accept_first_mouse(true)
                .build()?,
        );

        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::NewEvents(StartCause::Init) => {
                    //println!("Puppeteer Application Started"), //TODO Use logging to give more useful info about the program and window like rocket does
                }
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    let _ = webview.take();
                    *control_flow = ControlFlow::Exit
                }
                Event::UserEvent(ui_event) => {
                    custom_event_handler(ui_event, &mut webview, control_flow)
                }
                _ => (),
            }
        });
    }

    fn handler(proxy: EventLoopProxy<T>) -> impl Fn(&Window, String) {
        move |window: &Window, req: String| match req.as_str() {
            "minimize" => window.set_minimized(true),
            "maximize" => window.set_maximized(!window.is_maximized()),
            "drag_window" => window.drag_window().unwrap(), //FIXME Handle me
            _ => {
                let custom_event: T = req.into();
                proxy.send_event(custom_event).unwrap(); //FIXME
            }
        }
    }
}
