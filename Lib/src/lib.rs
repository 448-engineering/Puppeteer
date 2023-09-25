#![forbid(unsafe_code)]
#![forbid(missing_docs)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/448-engineering/Puppeteer/master/Documentation/Puppeteer-Logo-Icon.svg"
)]
//! Puppeteer
//! ![crates.io](https://img.shields.io/crates/v/puppeteer.svg) [![Docs](https://docs.rs/puppeteer/badge.svg)](https://docs.rs/puppeteer) [![Rust](https://github.com/448-engineering/Puppeteer/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/448-engineering/Puppeteer/actions/workflows/rust.yml)
//!
//! <p align="center"><img src="https://raw.githubusercontent.com/448-engineering/Puppeteer/master/Documentation/Puppeteer-Logo-dark-bg.svg" width="40%" /></p>
//!
//! A Minimal Dependency and Easy to Use GUI Creator in Rust using Async Channels

mod app_env;
pub use app_env::*;

mod app;
pub use app::*;

mod traits;
pub use traits::*;

mod errors;
pub use errors::*;

mod logging;
pub use logging::*;

mod shell;
pub use shell::*;

mod titlebar;
pub use titlebar::*;

mod ui_ops;
pub use ui_ops::*;

mod html_utils;
pub use html_utils::*;

/// Reuse crates in the lib for better compatibility
//
pub use arrayvec;
pub use async_trait;
pub use futures_lite;
pub use smol;
pub use thiserror;
pub use tracing;
pub use wry;
