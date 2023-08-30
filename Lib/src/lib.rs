#![forbid(unsafe_code)]
#![forbid(missing_docs)]
#![doc = include_str!("../../README.md")]

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
///
//
pub use arrayvec;
pub use async_trait;
pub use futures_lite;
pub use smol;
pub use thiserror;
pub use tracing;
pub use wry;
