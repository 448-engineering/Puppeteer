use std::io::ErrorKind;

use thiserror::Error;
use wry::{
    application::{
        error::{ExternalError, OsError},
        event_loop::EventLoopClosed,
    },
    Error as WryError,
};

/// A wrapper to `Result<T, PuppeteerError>`
pub type PuppeteerResult<T> = Result<T, PuppeteerError>;

/// The error type for Puppeteer
#[derive(Debug, Error)]
pub enum PuppeteerError {
    /// An error occured while initializing the app
    #[error("An error occurred while initializing the app")]
    AppInit(String),
    /// The error that is returned when an EventLoopProxy attempts to wake up an EventLoop that no longer exists. Contains the original event given to send_event.
    #[error("The error that is returned when an EventLoopProxy attempts to wake up an EventLoop that no longer exists. Contains the original event given to send_event.
    ")]
    WryEventLoopClosed,
    /// An error whose cause it outside the control of the running app
    #[error("An error whose cause it outside the control of the running app")]
    WryExternal(String),
    /// The error type for when the OS cannot perform the requested operation
    #[error("The error type for when the OS cannot perform the requested operation")]
    WryOsError(String),
    /// Errors returned by wry
    #[error("Errors returned by wry")]
    Wry(WryError),
    /// The app is unable to determine the primary monitor
    #[error("Unable to detect current monitor")]
    UnableToDetectCurrentMonitor,
    /// The app is unable to detect primary monitor
    #[error("Unable to detect primary monitor")]
    UnableToDetectPrimaryMonitor,
    /// The webview does not exist. It might have been terminated
    #[error("The `WebView` was not found")]
    WebViewDoesNotExist,
    /// The window is not resizable therefore the resize operation is impossible
    #[error("The `window is not resizable")]
    WindowIsNotResizable,
    /// An [std::io::ErrorKind]
    #[error("Represents std::io::ErrorKind")]
    Io(ErrorKind),
    /// The path provided for fonts in the `Shell::load_fonts()` is not a valid path
    #[error("Could not found the fonts directory specified. Check your font path")]
    FontsDirNotFound,
    /// The path provided for fonts is valid but the permission to read the directory was denied
    #[error(
        "The path provided for fonts is valid but the permission to read the directory was denied"
    )]
    FontsDirPermissionDenied,
    /// For webview only WOFF2 font format is supported.
    #[error("The font detected ({0:?}) is not a valid `WOFF2` format for the web.")]
    InvalidFontExpectedWoff2(String),
    /// Tried to get a file name without the extension part using `Path::file_stem()` but the file stem does not exist
    #[error("Tried to get a file name without the extension part using `Path::file_stem()` but the file stem does not exist")]
    InvalidFileStemName,
    /// The file being read has exceeded the maximum file size set
    #[error("The maximum size set for the resource has been exceeded")]
    MaxResourceLengthExceeded,
}

impl From<std::io::Error> for PuppeteerError {
    fn from(error: std::io::Error) -> Self {
        PuppeteerError::Io(error.kind())
    }
}

impl<T> From<EventLoopClosed<T>> for PuppeteerError {
    fn from(_: EventLoopClosed<T>) -> Self {
        PuppeteerError::WryEventLoopClosed
    }
}

impl From<ExternalError> for PuppeteerError {
    fn from(value: ExternalError) -> Self {
        PuppeteerError::WryExternal(value.to_string())
    }
}

impl From<WryError> for PuppeteerError {
    fn from(value: WryError) -> Self {
        PuppeteerError::Wry(value)
    }
}

impl From<OsError> for PuppeteerError {
    fn from(value: OsError) -> Self {
        PuppeteerError::WryOsError(value.to_string())
    }
}
