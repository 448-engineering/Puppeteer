use crate::WASM32_DIR;
use clap::Parser;
use puppeteer_types::camino::{Utf8Path, Utf8PathBuf};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    #[arg(short, long, default_value_t = WASM32_DIR.into())]
    path: String,
}

impl Args {
    pub(crate) fn parse() -> Self {
        Args::parse()
    }

    pub fn path(&self) -> &Utf8Path {
        Utf8Path::new(&self.path)
    }

    pub fn pathbuf(&self) -> Utf8PathBuf {
        self.path().to_path_buf()
    }
}
