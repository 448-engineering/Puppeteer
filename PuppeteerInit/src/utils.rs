use crate::constants::SPACING;
use std::{borrow::Cow, env};
use yansi::Paint;

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
