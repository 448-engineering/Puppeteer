use crate::{ActiveAppEnv, ModifyView, Shell};
use async_trait::async_trait;
use bytes::BytesMut;
use file_format::FileFormat;
use std::borrow::Cow;

/// A type that defines [Cow<'static, str>] for easier reuse
pub type StaticCowStr = Cow<'static, str>;
/// A type that defines [Cow<'p, str>] for easier reuse
pub type CowStr<'p> = Cow<'p, str>;
/// A type of &'static str
pub type StaticStr = &'static str;

/// This trait is used to perform UI operations like parsing the IPC messages
/// and generating content to be displayed
#[async_trait::async_trait]
pub trait Puppeteer {
    /// The app default size window. This defaults to `WindowResize::ResizePercent(90)`
    fn window_size() -> f32 {
        90f32 / 100f32
    }
    /// The app default size window. This defaults to `WindowResize::ResizePercent(90)`
    fn splash_window_size() -> f32 {
        50f32 / 100f32
    }

    /// Method is run to generate a [Shell].
    async fn shell() -> Shell;

    /// Initialize function which loads data necessary for
    /// the app to function. This data can be use to load resources
    /// like fonts or load user data like username from a database, etc.
    /// Load the root page after initialization has completed

    async fn init(app_env: &ActiveAppEnv) -> ModifyView;

    /// The splash screen loaded when an app is being initialized
    fn splashscreen() -> ModifyView;

    /// Parse the IPC message.
    fn parse(message: &str) -> Self;

    /// After parsing the IPC message using the above `Puppeteer::parse()` method
    /// this method is called to perform updates to the UI
    async fn event_handler(&mut self, app_env: ActiveAppEnv) -> ModifyView;

    /// This is used to handle errors. It is async so that I/O can be used like to log to a file.
    /// It returns a [ModifyView] which can display an error message to the user
    async fn error_handler(error: impl std::error::Error + Send) -> ModifyView;
}

/// Trait that ensures a type can be converted to code that can be rendered into current view
#[async_trait]
pub trait UiPaint {
    /// Can be rendered by native renderer
    fn to_native(&self) {}

    /// Convert to HTML format for use in HTML Web based renderer
    fn to_html(&self) -> Cow<str>;
}

impl core::fmt::Debug for dyn UiPaint {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "UiPaint")
    }
}

impl UiPaint for &String {
    fn to_html(&self) -> Cow<str> {
        Cow::Borrowed("") + Cow::Owned(self.to_string())
    }
}

impl UiPaint for Cow<'_, str> {
    fn to_html(&self) -> Cow<str> {
        self.clone()
    }
}

impl UiPaint for &str {
    fn to_html(&self) -> Cow<str> {
        Cow::Borrowed(self)
    }
}

/// Methods to detect file type and convert to encoding formats like base64
pub trait AssetProperties {
    /// The name of the resource
    fn name(&self) -> Cow<str>;

    /// The [FileFormat] of the resource
    fn format(&self) -> FileFormat;

    /// The content bytes
    fn bytes(&self) -> &BytesMut;

    /// Base64 encoding for html
    fn base64(&self) -> Cow<str>;

    /// Get the blake3 hash of the bytes
    fn hash(&self) -> blake3::Hash;
}

/// Methods to detect file type and convert to encoding formats like base64
pub trait StaticAssetProperties {
    /// The name of the resource
    fn name(&self) -> &'static str;

    /// The [FileFormat] of the resource
    fn format(&self) -> FileFormat;

    /// The content bytes
    fn bytes(&self) -> &'static [u8];

    /// Base64 encoding for html
    fn base64(&self) -> Cow<str>;

    /// Get the blake3 hash of the bytes
    fn hash(&self) -> blake3::Hash;
}
