use puppeteer::{
    tracing::{self, Level},
    InvokeWebView, Puppeteer, PuppeteerApp, Shell, UiPaint,
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

    let mut app = PuppeteerApp::<AppTest>::init("Puppeteer Test App").unwrap();

    smol::block_on(async { app.start().await.unwrap() })
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

impl Puppeteer for AppTest {
    fn shell() -> Shell {
        Shell::new().add_style("body {background-color: #1a1a1a}")
    }

    fn parse(message: &str) -> Self {
        if message.starts_with("PuppeteerError»") {
            // Handle this error into `Self`
            panic!("Encountered error: {}", message)
        }

        AppTest::PhantomData
    }
}

impl UiPaint for AppTest {
    fn to_html(&self) -> std::borrow::Cow<str> {}
}
