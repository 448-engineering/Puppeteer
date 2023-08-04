use crate::{TitleBar, UiPaint, TITLE_BAR_SCRIPT};
use arrayvec::ArrayVec;
use std::{borrow::Cow, collections::HashMap};
use wry::application::window::Theme as WryTheme;

pub type Style<'p> = (&'p str, Cow<'p, str>);
pub type StylesMap<'p> = HashMap<u64, Style<'p>>; //(Style name, style)

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Shell {
    title: &'static str,
    head_links: ArrayVec<&'static str, 20>,
    styles: ArrayVec<&'static str, 10>,
    theme: Theme,
    theme_light: &'static str, //FIXME switch to `HexColor`
    theme_dark: &'static str,  //FIXME switch to `HexColor`
    title_bar: TitleBar,
    scripts: ArrayVec<&'static str, 10>,
}

impl Shell {
    pub fn new() -> Self {
        Shell::default()
    }

    pub fn set_title(mut self, title: &'static str) -> Self {
        self.title = title;

        self
    }

    pub fn add_head_links(mut self, element: &'static str) -> Self {
        self.head_links.push(element);

        self
    }

    pub fn add_style(mut self, style: &'static str) -> Self {
        self.styles.push(style);

        self
    }

    pub fn set_scripts(mut self, script: &'static str) -> Self {
        self.scripts.push(script);

        self
    }

    pub fn set_title_bar(mut self, title_bar: TitleBar) -> Self {
        self.title_bar = title_bar;

        self
    }

    pub fn toggle_theme(&mut self) -> &mut Self {
        let toggled = self.theme.toggle_theme();

        self.theme = toggled;

        self
    }

    pub fn set_theme_light(mut self, color: &'static str) -> Self {
        self.theme_light = color;

        self
    }

    pub fn set_theme_dark(mut self, color: &'static str) -> Self {
        self.theme_dark = color;

        self
    }

    pub fn title(&self) -> &str {
        self.title
    }

    pub fn head_links(&self) -> &[&'static str] {
        self.head_links.as_slice()
    }

    pub fn styles(&self) -> &[&'static str] {
        self.styles.as_slice()
    }

    pub fn scripts(&self) -> &[&'static str] {
        self.scripts.as_slice()
    }

    pub fn title_bar(&self) -> TitleBar {
        self.title_bar
    }

    pub fn theme(&self) -> Theme {
        self.theme
    }

    pub fn theme_light(&self) -> &'static str {
        self.theme_light
    }

    pub fn theme_dark(&self) -> &'static str {
        self.theme_dark
    }
}

impl UiPaint for Shell {
    fn to_html(&self) -> Cow<str> {
        let background_color = if self.theme == Theme::Dark {
            self.theme_dark
        } else if self.theme == Theme::Light {
            self.theme_light
        } else {
            self.theme_dark
        };

        let text_color = if self.theme == Theme::Dark {
            self.theme_light
        } else if self.theme == Theme::Light {
            self.theme_dark
        } else {
            self.theme_light
        };

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
            + "<title>"
            + self.title
            + "</title>"
            + "<style>"
            + Cow::Owned(styles)
            + self.title_bar.style()
            + "body { "
            + "background-color: "
            + background_color
            + ";"
            + "color: "
            + text_color
            + ";"
            + " }"
            + "</style>"
            + "</head>"
            + "<body>"
            + self.title_bar.to_html()
            + r#"<div id="puppeteer_app"></div>"#
            + TITLE_BAR_SCRIPT
            + Cow::Owned(scripts)
            + "</body>"
            + "</html>"
    }
}

impl Default for Shell {
    fn default() -> Self {
        Shell {
            title: "Puppeteer App",
            styles: ArrayVec::new(),
            head_links: ArrayVec::new(),
            theme: Theme::System,
            theme_dark: "#1b1b1b",
            theme_light: "#fafafa",
            title_bar: TitleBar::default(),
            scripts: ArrayVec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Theme {
    Dark,
    Light,
    System,
}

impl Theme {
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
