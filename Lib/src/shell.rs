use crate::{ActiveAppEnv, PuppeteerError, PuppeteerResult, StaticCowStr, StaticStr, UiPaint};
use base64ct::{Base64, Encoding};
use file_format::FileFormat;
use futures_lite::{AsyncReadExt, StreamExt};
use smol::fs::{read_dir, File};
use std::{borrow::Cow, io::ErrorKind, path::Path};
use wry::application::window::Theme as WryTheme;

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

    /// Add all the styles in a certain directory `path` with the file names provided.
    /// NOTE: CSS style sheets are cascading so the order in which the file names for
    /// the styles are added is important.
    pub async fn add_styles_dir(
        mut self,
        path: &str,
        files: impl AsRef<[&str]>,
    ) -> PuppeteerResult<Self> {
        let paths = files
            .as_ref()
            .iter()
            .map(|file| path.to_owned() + file + ".css")
            .collect::<Vec<String>>();

        while let Some(file) = futures_lite::stream::iter(&paths).next().await {
            let mut file = smol::fs::File::open(file).await?;
            let mut contents = String::new();

            file.read_to_string(&mut contents).await?;

            self.styles.push(contents.into());
        }

        Ok(self)
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

    /// Add font to the shell. The font is required to be Base64
    pub fn add_fonts(mut self, font_bytes: &'static str) -> Self {
        self.fonts.push(Cow::Borrowed(font_bytes));

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

    /// Load fonts in a particular directory
    pub async fn load_fonts_dir(
        mut self,
        path_to_fonts: impl AsRef<Path>,
        app_env: &mut ActiveAppEnv,
    ) -> PuppeteerResult<Self> {
        let mut entries = match read_dir(path_to_fonts).await {
            Ok(dir) => dir,
            Err(error) => {
                dbg!(&error.to_string());
                if error.kind() == ErrorKind::NotFound {
                    return Err(PuppeteerError::FontsDirNotFound);
                } else if error.kind() == ErrorKind::PermissionDenied {
                    return Err(PuppeteerError::FontsDirPermissionDenied);
                } else {
                    return Err(error.into());
                }
            }
        };

        while let Some(entry) = entries.try_next().await? {
            let mut file = File::open(entry.path()).await?;
            let mut buffer = Vec::<u8>::new();

            file.read_to_end(&mut buffer).await?;

            let file_format_detected = FileFormat::from_bytes(&buffer);

            if file_format_detected != FileFormat::WebOpenFontFormat2 {
                return Err(PuppeteerError::InvalidFontExpectedWoff2);
            }

            let font_name = entry.path().clone();
            let font_stem = match font_name.file_stem() {
                Some(file_stem) => file_stem,
                None => return Err(PuppeteerError::InvalidFileStemName),
            };

            tracing::trace!("LOADED FONT: {:?}", &font_stem);
            app_env
                .fonts
                .push(StaticCowStr::Owned(font_stem.to_string_lossy().to_string()));

            let font = Cow::Borrowed("data:application/font-woff2;base64,")
                + Cow::Owned(Base64::encode_string(&buffer));
            let injector = Cow::Borrowed("var dataUri = \"")
                + font
                + "\";"
                + Cow::Borrowed(
                    r#"
                    var fontFace = new FontFace(""#,
                )
                + Cow::Owned(font_stem.to_string_lossy().to_string())
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
        }

        Ok(self)
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
