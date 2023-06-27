use crate::{TitleBar, UiPaint};
use std::{borrow::Cow, collections::HashMap};
use wry::application::window::Theme as WryTheme;

pub type Style<'p> = (&'p str, Cow<'p, str>);
pub type StylesMap<'p> = HashMap<u64, Style<'p>>; //(Style name, style)

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Shell {
    title: &'static str,
    style: &'static str,
    theme: Theme,
    theme_light: &'static str, //FIXME switch to `HexColor`
    theme_dark: &'static str,  //FIXME switch to `HexColor`
    title_bar: TitleBar,
}

impl Shell {
    pub fn new() -> Self {
        Shell::default()
    }

    pub fn set_title(mut self, title: &'static str) -> Self {
        self.title = title;

        self
    }

    pub fn set_style(mut self, style: &'static str) -> Self {
        self.style = style;

        self
    }

    pub fn set_title_bar(mut self, title_bar: TitleBar) -> Self {
        self.title_bar = title_bar;

        self
    }

    pub fn set_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;

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

    pub fn style(&self) -> &'static str {
        self.style
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

        Cow::Borrowed("<!DOCTYPE html>")
            + "<head>"
            + r#"<meta charset="UTF-8">"#
            + r#"<meta name="viewport" content="width=device-width, initial-scale=1.0">"#
            + "<title>"
            + self.title
            + "</title>"
            + "<style>"
            + self.style
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
            + "</body>"
            + "</html>"
    }
}

impl Default for Shell {
    fn default() -> Self {
        Shell {
            title: "Puppeteer App",
            style: CSS_RESET_STYLE,
            theme: Theme::System,
            theme_dark: "#1b1b1b",
            theme_light: "#fafafa",
            title_bar: TitleBar::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Theme {
    Dark,
    Light,
    System,
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

pub const TITLE_BAR_SCRIPT: &str = r#"
<script>
document.addEventListener('mousedown', (e) => {
    if (e.target.classList.contains('drag-region') && e.buttons === 1) {
        e.detail === 2
            ? window.ipc.postMessage('maximize')
            : window.ipc.postMessage('drag_window');
    }
})
document.addEventListener('touchstart', (e) => {
    if (e.target.classList.contains('drag-region')) {
        window.ipc.postMessage('drag_window');
    }
})
</script>
"#;

pub const CSS_RESET_STYLE: &str = r#"
/* http://meyerweb.com/eric/tools/css/reset/ 
   v2.0 | 20110126
   License: none (public domain)
*/

html, body, div, span, applet, object, iframe,
h1, h2, h3, h4, h5, h6, p, blockquote, pre,
a, abbr, acronym, address, big, cite, code,
del, dfn, em, img, ins, kbd, q, s, samp,
small, strike, strong, sub, sup, tt, var,
b, u, i, center,
dl, dt, dd, ol, ul, li,
fieldset, form, label, legend,
table, caption, tbody, tfoot, thead, tr, th, td,
article, aside, canvas, details, embed, 
figure, figcaption, footer, header, hgroup, 
menu, nav, output, ruby, section, summary,
time, mark, audio, video {
	margin: 0;
	padding: 0;
	border: 0;
	font-size: 100%;
	font: inherit;
	vertical-align: baseline;
}
/* HTML5 display-role reset for older browsers */
article, aside, details, figcaption, figure, 
footer, header, hgroup, menu, nav, section {
	display: block;
}
body {
	line-height: 1;
}
ol, ul {
	list-style: none;
}
blockquote, q {
	quotes: none;
}
blockquote:before, blockquote:after,
q:before, q:after {
	content: '';
	content: none;
}
table {
	border-collapse: collapse;
	border-spacing: 0;
}
"#;
