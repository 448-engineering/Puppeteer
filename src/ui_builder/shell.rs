use crate::SplashScreen;
use std::borrow::Cow;

pub struct Shell<'p> {
    title: &'p str,
    content: Cow<'p, str>,
    style: Cow<'p, str>, // The styling for the shell for example CSS reset styles.
}

impl<'p> Shell<'p> {
    pub fn new() -> Self {
        Shell::default()
    }

    pub fn set_title(mut self, title: &'p str) -> Self {
        self.title = title;

        self
    }

    pub fn set_content(mut self, content: Cow<'p, str>) -> Self {
        self.content = content;

        self
    }

    pub fn set_style(mut self, style: &'p str) -> Self {
        self.style = style.into();

        self
    }

    pub fn title(&self) -> &str {
        self.title
    }

    pub fn content(&self) -> &Cow<'p, str> {
        &self.content
    }

    pub fn style(&self) -> &Cow<'p, str> {
        &self.style
    }

    pub fn build(self) -> Cow<'p, str> {
        Cow::Borrowed("<!DOCTYPE html>")
            + "<head>"
            + r#"<meta charset="UTF-8">"#
            + r#"<meta name="viewport" content="width=device-width, initial-scale=1.0">"#
            + "<title>"
            + self.title
            + "</title>"
            + "<style>\n"
            + self.style
            + "\n</style>"
            + "</head>"
            + "<body>"
            + self.content
            + "</body>"
            + "</html>"
    }
}

impl<'p> Default for Shell<'p> {
    fn default() -> Self {
        Shell {
            title: "Puppeteer App",
            content: SplashScreen::default().build(),
            style: CSS_RESET_STYLE.into(),
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
