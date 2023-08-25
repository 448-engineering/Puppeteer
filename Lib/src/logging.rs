use tracing::Level;

/// Name that can be used to quickly identify all filters of the running app in logs.
pub const LOGGING_SYMBOL: &str = "» ";
/// This is the String used to indicate an error has been sent via the `EventLoopProxy`
pub const ERROR_PREFIX: &str = "PuppeteerError» ";

/// Custom logging handler for Puppeteer apps.
/// Text filtering can be done by searching for `[app_title]»`
#[derive(Debug, PartialEq, Eq)]
pub struct Logging {
    level: Level,
    app_name: &'static str,
}

impl Logging {
    /// Add the app name to be used in logging so that the logs can be filtered using the app name.
    /// The default tracing level is info
    pub fn new(app_name: &'static str) -> Self {
        Logging {
            level: Level::INFO,
            app_name,
        }
    }

    /// Change the name used to identify logging by this app in the logs.
    /// Default is `Level::INFO `.
    pub fn with_level(mut self, level: Level) -> Self {
        self.level = level;

        self
    }

    /// Log the message. This is a simple logging mechanism
    pub fn log(self, message: &str) {
        match self.level {
            Level::DEBUG => tracing::debug!("{}{}{}", self.app_name, LOGGING_SYMBOL, message),
            Level::ERROR => tracing::error!("{}{}{}", self.app_name, LOGGING_SYMBOL, message),
            Level::INFO => tracing::info!("{}{}{}", self.app_name, LOGGING_SYMBOL, message),
            Level::TRACE => tracing::trace!("{}{}{}", self.app_name, LOGGING_SYMBOL, message),
            Level::WARN => tracing::warn!("{}{}{}", self.app_name, LOGGING_SYMBOL, message),
        }
    }
}
