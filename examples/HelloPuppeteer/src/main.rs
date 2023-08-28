use puppeteer::{
    async_trait::{self},
    tracing::{self, Level},
    wry::application::window::Window,
    ActiveAppEnv, ModifyView, Puppeteer, PuppeteerApp, Shell,
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
    PhantomData,
}

impl AsRef<str> for AppTest {
    fn as_ref(&self) -> &str {
        match self {
            Self::Root => "root",
            Self::PhantomData => "todo",
        }
    }
}

#[async_trait::async_trait]
impl Puppeteer for AppTest {
    fn shell() -> Shell {
        Shell::new().add_style("body {background-color: #1a1a1a; color: #FFFFFF}")
    }

    fn splashscreen() -> ModifyView {
        ModifyView::ReplaceApp("SPLASHSCREEN".into())
    }

    async fn init() -> ModifyView {
        smol::Timer::after(std::time::Duration::from_secs(3)).await;

        ModifyView::ReplaceApp("INITIALIZED".into())
    }

    fn parse(message: &str) -> Self {
        if message.starts_with("PuppeteerError»") {
            // Handle this error into `Self`
            panic!("Encountered error: {}", message)
        }

        AppTest::PhantomData
    }

    async fn event_handler(&mut self, app_env: ActiveAppEnv) -> ModifyView {
        println!("ACTIVE_ENV: {:?}", app_env);

        ModifyView::ReplaceApp("EVENT RECV".into())
    }

    async fn error_handler(error: impl std::error::Error + Send) -> ModifyView {
        ModifyView::ReplaceApp("ERROR RECV".into())
    }
}
