use puppeteer::{
    tracing::{self, Level},
    InvokeWebView, Puppeteer,
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

    let mut app = Puppeteer::<AppTest>::init("Puppeteer Test App").unwrap();

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

impl InvokeWebView for AppTest {
    fn shell(&self) -> puppeteer::Shell {
        puppeteer::Shell {}
    }

    fn parse(message: &str) -> Self {
        if message.starts_with("PuppeteerError»") {
            // Handle this error into `Self`
            panic!("Encountered error: {}", message)
        }

        AppTest::PhantomData
    }
}
