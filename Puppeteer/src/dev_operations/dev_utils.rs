use std::{borrow::Cow, env};
use yansi::Paint;

/// Specifies the default path to look for the `wasm32-unknown-unknown` binary
pub const WASM32_DIR: &str = "./target/wasm32-unknown-unknown/debug/";
/// Specifies the default buffer capacity
pub const BUFFER_CAPACITY: usize = 64 * 1024;
/// The spacing fot logging information
pub const SPACING: &str = "     ";
/// The default cargo command executed after watched file(s) have been updated
pub const DEFAULT_BUILD_COMMAND: [&str; 4] =
    ["cargo", "build", "--target", "wasm32-unknown-unknown"];

#[derive(Debug, Default)]
pub struct Logger<'a> {
    pub symbol: Paint<Cow<'a, str>>,
    pub header: Paint<Cow<'a, str>>,
    pub label: Option<Paint<&'a str>>,
    pub body: Paint<Cow<'a, str>>,
}

impl<'a> Logger<'a> {
    pub fn new(body: &'a str) -> Self {
        Logger {
            symbol: Paint::cyan(Cow::Borrowed(SPACING) + "=>"),
            header: Paint::yellow(
                Cow::Borrowed("-------- PUPPETEER ") + "v" + pkg_version() + "--------\n",
            ),
            label: Option::None,
            body: Paint::green(Cow::Borrowed(body)),
        }
    }

    pub fn symbol(mut self, symbol: &'a str) -> Self {
        self.symbol = Paint::cyan(Cow::Borrowed(SPACING) + symbol + " ");

        self
    }

    pub fn add_header(mut self, header: &'a str) -> Self {
        let header = Cow::Borrowed("--------")
            + " "
            + header
            + "v"
            + pkg_version()
            + " "
            + "--------"
            + "\n";
        self.header = Paint::yellow(header);

        self
    }

    pub fn with_label(mut self, label: &'a str) -> Self {
        self.label = Some(Paint::cyan(label));

        self
    }
}

pub fn pkg_version<'a>() -> Cow<'a, str> {
    Cow::Borrowed(env!("CARGO_PKG_VERSION"))
}
