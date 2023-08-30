use std::borrow::Cow;

use html_to_string_macro::html;
use puppeteer::{
    async_trait::{self},
    tracing::{self, Level},
    ActiveAppEnv, ModifyView, Puppeteer, PuppeteerApp, Shell, DEFAULT_WINDOW_ACTIONS,
    DEFAULT_WINDOW_ACTIONS_SCRIPT, DEFAULT_WINDOW_ACTIONS_STYLE,
};
use tracing_subscriber::FmtSubscriber;

fn main() {
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::TRACE)
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    smol::block_on(async {
        PuppeteerApp::<AppTest>::init("Puppeteer Test App")
            .unwrap()
            .start()
            .await
            .unwrap()
    })
}

#[derive(Debug)]
pub enum AppTest {
    Root,
    CloseWindow,
}

impl AsRef<str> for AppTest {
    fn as_ref(&self) -> &str {
        match self {
            Self::Root => "root",
            Self::CloseWindow => "close_window",
        }
    }
}

const PUPPETEER_LOGO: &str = include_str!("../../../Documentation/Puppeteer-Logo.svg");
const PUPPETEER_ICON: &str = include_str!("../../../Documentation/Puppeteer-Logo-Icon.svg");

#[async_trait::async_trait]
impl Puppeteer for AppTest {
    fn shell() -> Shell {
        Shell::new()
            .add_style("body {background-color: #1a1a1a; color: #FFFFFF;}")
            .add_style(".splash-icon>svg{width: 50vw}")
            .add_style(DEFAULT_WINDOW_ACTIONS_STYLE)
            .add_style(include_str!("../assets/frow.min.css"))
            .add_script(DEFAULT_WINDOW_ACTIONS_SCRIPT)
    }

    fn splashscreen() -> ModifyView {
        let splash_html = html!(
            <div class="frow row-center ">
                <div class="splash-icon frow row-center p-20">{PUPPETEER_LOGO}</div>
            </div>
        );

        ModifyView::ReplaceApp(Cow::Owned(splash_html))
    }

    async fn init() -> ModifyView {
        smol::Timer::after(std::time::Duration::from_secs(3)).await;

        let title_bar = html!({ DEFAULT_WINDOW_ACTIONS });

        ModifyView::ReplaceApp(Cow::Owned(title_bar))
    }

    fn parse(message: &str) -> Self {
        if message.starts_with("PuppeteerError»") {
            // Handle this error into `Self`
            panic!("Encountered error: {}", message)
        }

        todo!()
    }

    async fn event_handler(&mut self, app_env: ActiveAppEnv) -> ModifyView {
        println!("ACTIVE_ENV: {:?}", app_env);

        ModifyView::ReplaceApp("EVENT RECV".into())
    }

    async fn error_handler(_error: impl std::error::Error + Send) -> ModifyView {
        ModifyView::ReplaceApp("ERROR RECV".into())
    }
}
