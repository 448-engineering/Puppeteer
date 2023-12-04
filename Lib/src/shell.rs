use crate::{ActiveAppEnv, StaticAssetProperties, StaticCowStr, StaticStr, UiPaint};
use file_format::FileFormat;
use std::borrow::Cow;
use tao::window::Theme as WryTheme;

/// The HTML element where all the app body will be injected
pub const PUPPETEER_APP_ELEMENT: &str = r#"<div id="puppeteer_app"></div>"#;

/// This is the color palette of the app
#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub struct ColorPalette {
    /// The dominant color
    pub primary: StaticStr, //FIXME switch to HexColor
    /// The color to contrast the dominant color
    pub secondary: StaticStr, //FIXME switch to HexColor
    /// The accent color
    pub tertiary: StaticStr, //FIXME switch to HexColor
}

impl Default for ColorPalette {
    fn default() -> Self {
        ColorPalette {
            primary: "#FFFFFF",
            secondary: "#000000",
            tertiary: "#E6E6E6",
        }
    }
}

/// The [Shell] of the app contains all the imports
/// like fonts, styles and scripts
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Shell {
    head_links: Vec<StaticCowStr>,
    styles: Vec<StaticCowStr>,
    scripts: Vec<StaticCowStr>,
    fonts: Vec<StaticCowStr>,
    palette: ColorPalette,
}

impl Shell {
    /// Initialize a new [Shell]
    pub fn new() -> Self {
        Shell::default()
    }

    /// Add the links for `head` tag like fonts, CSS etc. These are where the resources
    /// should be loaded from.
    pub fn add_head_links(mut self, element: StaticStr) -> Self {
        self.head_links.push(Cow::Borrowed(element));

        self
    }

    /// Add styles into the `<style></style>` element.
    pub fn add_style(mut self, style: StaticStr) -> Self {
        self.styles.push(Cow::Borrowed(style));

        self
    }

    /// Add multiple stypes in the `<style></style>` field
    pub fn add_styles(mut self, styles: impl AsRef<[StaticCowStr]>) -> Self {
        self.styles.extend_from_slice(styles.as_ref());

        self
    }

    /// Add multiple stypes in the `<style></style>` field
    pub fn add_const_styles(mut self, styles: impl AsRef<[&'static str]>) -> Self {
        styles
            .as_ref()
            .iter()
            .for_each(|style| self.styles.push(Cow::Borrowed(style)));

        self
    }

    /// Add the scripts in the `<body></body>` field
    pub fn add_script(mut self, script: StaticCowStr) -> Self {
        self.scripts.push(script);

        self
    }

    /// Add the scripts in the `<body></body>` field
    pub fn add_scripts(mut self, scripts: impl AsRef<[StaticCowStr]>) -> Self {
        self.scripts.extend_from_slice(scripts.as_ref());

        self
    }

    /// Get the head_links
    pub fn head_links(&self) -> &[StaticCowStr] {
        self.head_links.as_slice()
    }

    /// Get the styles
    pub fn styles(&self) -> &[StaticCowStr] {
        self.styles.as_slice()
    }

    /// Get the scripts
    pub fn scripts(&self) -> &[StaticCowStr] {
        self.scripts.as_slice()
    }

    /// Add user specified fonts
    pub fn add_fonts(mut self, app_env: &ActiveAppEnv) -> Self {
        app_env.fonts.iter().for_each(|font| {
            let file_format_detected = font.format();

            if file_format_detected != FileFormat::WebOpenFontFormat2 {
                panic!(
                    "Invalid font type `{:?}` for file `{}`",
                    font.format(),
                    font.name
                );
            }

            tracing::info!("LOADED FONT: {:?}", &font.name());

            let injector = Cow::Borrowed("var dataUri = \"")
                + font.base64()
                + "\";"
                + Cow::Borrowed(
                    r#"
                    var fontFace = new FontFace(""#,
                )
                + font.name()
                + Cow::Borrowed(
                    r#"", `url(${dataUri})`, {
                        style: "normal",
                        weight: "normal",
                        stretch: "condensed",
                      });
                      document.fonts.add(fontFace);
                    "#,
                ); //FIXME Add styles for fonts here*/
            self.fonts.push(Cow::Owned(injector.to_string()));
        });

        self
    }
}

impl UiPaint for Shell {
    fn to_html(&self) -> Cow<str> {
        let head_links = self.head_links.iter().cloned().collect::<String>();

        let styles = self.styles.iter().cloned().collect::<String>();

        let scripts = self.scripts.iter().cloned().collect::<String>();

        let fonts = self
            .fonts
            .iter()
            .map(|font| font.to_string())
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
            + "<script>"
            + Cow::Owned(fonts)
            + "</script>"
            + PUPPETEER_APP_ELEMENT
            + Cow::Owned(scripts)
            + "</body>"
            + "</html>"
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
