use std::borrow::Cow;

const PUPPETEER_LOGO: &str = include_str!("./Puppeteer-Logo.svg");

const DEFAULT_SPLASH_STYLE: &str = r#"
 style="
        width: 100vw;
        height: 100vh;
        text-align: center;
        background-color: #1b1b1b;
        display: flex;
        justify-content: center;
        justify-self: center;
        justify-items: center;
        align-items: center;
        align-self: center;
        align-content: center;
        padding: 0%;
        margin: 0%;
        box-sizing: border-box;
    "
"#;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SplashScreen {
    svg_logo: &'static str,
    style: Option<&'static str>,
    //duration: u64,
}

impl SplashScreen {
    pub fn new() -> Self {
        SplashScreen::default()
    }

    pub fn set_logo(mut self, logo: &'static str) -> Self {
        self.svg_logo = logo;

        self
    }

    pub fn set_style(mut self, style: &'static str) -> Self {
        self.style = Some(style);

        self
    }

    pub fn svg_logo(&self) -> &str {
        self.svg_logo
    }

    pub fn style(&self) -> Option<&str> {
        self.style
    }

    pub fn build(self) -> Cow<'static, str> {
        let splash_open = Cow::Borrowed(r#"<div id="splashscreen""#);
        let close_div = "</div>";
        let logo_parent = r#"<div style="width: 50%;">"#;
        let style = if let Some(style) = self.style {
            style
        } else {
            DEFAULT_SPLASH_STYLE
        };

        splash_open + style + ">" + logo_parent + self.svg_logo + close_div + close_div
    }
}

impl Default for SplashScreen {
    fn default() -> Self {
        SplashScreen {
            svg_logo: PUPPETEER_LOGO,
            style: Some(DEFAULT_SPLASH_STYLE),
        }
    }
}
