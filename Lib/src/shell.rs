use crate::UiPaint;
use std::borrow::Cow;
use wry::application::window::Theme as WryTheme;

/// The HTML element where all the app body will be injected
pub const PUPPETEER_APP_ELEMENT: &str = r#"<div id="puppeteer_app"></div>"#;

/// The [Shell] of the app contains all the imports
/// like fonts, styles and scripts
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Shell {
    head_links: Vec<&'static str>,
    styles: Vec<&'static str>,
    scripts: Vec<&'static str>,
}

impl Shell {
    /// Initialize a new [Shell]
    pub fn new() -> Self {
        Shell::default()
    }

    /// Add the links for `head` tag like fonts, CSS etc. These are where the resources
    /// should be loaded from.
    pub fn add_head_links(mut self, element: &'static str) -> Self {
        self.head_links.push(element);

        self
    }

    /// Add styles into the `<style></style>` element.
    pub fn add_style(mut self, style: &'static str) -> Self {
        self.styles.push(style);

        self
    }

    /// Add the scripts in the `<body></body>` field
    pub fn add_script(mut self, script: &'static str) -> Self {
        self.scripts.push(script);

        self
    }

    /// Get the head_links
    pub fn head_links(&self) -> &[&'static str] {
        self.head_links.as_slice()
    }

    /// Get the styles
    pub fn styles(&self) -> &[&'static str] {
        self.styles.as_slice()
    }

    /// Get the scripts
    pub fn scripts(&self) -> &[&'static str] {
        self.scripts.as_slice()
    }
}

impl UiPaint for Shell {
    fn to_html(&self) -> Cow<str> {
        let head_links = self
            .head_links
            .iter()
            .map(|head_link| head_link.to_owned())
            .collect::<String>();

        let styles = self
            .styles
            .iter()
            .map(|style| style.to_owned())
            .collect::<String>();

        let scripts = self
            .scripts
            .iter()
            .map(|script| script.to_owned())
            .collect::<String>();

        Cow::Borrowed("<!DOCTYPE html>")
            + "<head>"
            + r#"<meta charset="UTF-8">"#
            + r#"<meta name="viewport" content="width=device-width, initial-scale=1.0">"#
            + Cow::Owned(head_links)
            + "<style>"
            + Cow::Owned(styles)
            + "</style>"
            + "</head>"
            + "<body>"
            + PUPPETEER_APP_ELEMENT
            + Cow::Owned(scripts)
            + "</body>"
            + "</html>"
    }
}

impl Default for Shell {
    fn default() -> Self {
        Shell {
            styles: Vec::default(),
            head_links: Vec::default(),
            scripts: Vec::default(),
        }
    }
}

/// Whether it is a dark or light theme or whether it will respect system settings
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Theme {
    /// Dark Theme
    Dark,
    /// Light Theme
    Light,
    /// The system theme
    System,
}

impl Theme {
    /// Toggle the theme from light to dark and vise versa
    pub fn toggle_theme(&self) -> Self {
        match self {
            Self::Dark => Self::Light,
            Self::Light => Self::Dark,
            Self::System => Self::Dark,
        }
    }
}

impl From<WryTheme> for Theme {
    fn from(value: WryTheme) -> Self {
        match value {
            WryTheme::Dark => Theme::Dark,
            WryTheme::Light => Theme::Light,
            _ => Theme::System,
        }
    }
}
