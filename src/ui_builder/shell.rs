use crate::{SplashScreen, UiPaint, PUPPETEER_CSS_RESET_STYLES};
use std::{borrow::Cow, collections::HashMap};
pub type Style<'p> = (&'p str, Cow<'p, str>);
pub type StylesMap<'p> = HashMap<u64, Style<'p>>; //(Style name, style)

#[derive(Debug)]
pub struct Shell<'p> {
    title: &'p str,
    content: Box<dyn UiPaint>,
    styles: StylesMap<'p>,
}

impl<'p> Shell<'p> {
    pub fn new() -> Self {
        Shell::default()
    }

    pub fn set_title(&mut self, title: &'p str) -> &mut Self {
        self.title = title;

        self
    }

    pub fn set_content(&mut self, content: Box<dyn UiPaint>) -> &mut Self {
        self.content = content;

        self
    }

    pub fn add_style(&mut self, style_name: &'p str, style: &'p str) -> &mut Self {
        self.styles.insert(
            seahash::hash(style_name.as_bytes()),
            (style_name, style.into()),
        );

        self
    }

    pub fn title(&self) -> &str {
        self.title
    }

    pub fn content(&self) -> &Box<dyn UiPaint> {
        &self.content
    }

    pub fn get_style(&self, style_name: &'p str) -> Option<&Style<'p>> {
        self.styles.get(&seahash::hash(style_name.as_bytes()))
    }

    pub fn remove_style(&mut self, style_name: &'p str) -> Option<Style<'p>> {
        self.styles.remove(&seahash::hash(style_name.as_bytes()))
    }

    pub fn list_style(&self) -> &StylesMap {
        &self.styles
    }

    fn build_styles(&self) -> Cow<'p, str> {
        let inner_styles = self
            .styles
            .values()
            .map(|value| value.1.to_string() + " ")
            .collect::<String>();

        Cow::Borrowed("<style>") + Cow::Owned(inner_styles) + "\n</style>"
    }
}

impl<'p> UiPaint for Shell<'p> {
    fn to_html(&self) -> Cow<str> {
        Cow::Borrowed("<!DOCTYPE html>")
            + "<head>"
            + r#"<meta charset="UTF-8">"#
            + r#"<meta name="viewport" content="width=device-width, initial-scale=1.0">"#
            + "<title>"
            + self.title
            + "</title>"
            + self.build_styles()
            + "</head>"
            + self.content.to_html()
            + "</html>"
    }
}

impl<'p> Default for Shell<'p> {
    fn default() -> Self {
        let mut styles = StylesMap::default();
        styles.insert(
            seahash::hash(PUPPETEER_CSS_RESET_STYLES.as_bytes()),
            (PUPPETEER_CSS_RESET_STYLES, CSS_RESET_STYLE.into()),
        );

        Shell {
            title: "Puppeteer App",
            content: Box::new(SplashScreen::default()),
            styles,
        }
    }
}

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
