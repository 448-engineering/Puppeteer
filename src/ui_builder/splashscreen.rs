use crate::UiPaint;
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
        flex-direction: column;
    "
"#;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SplashScreen {
    content: &'static str,
    style: &'static str,
    loader: Option<&'static str>,
}

impl SplashScreen {
    pub fn new() -> Self {
        SplashScreen::default()
    }

    pub fn set_content(mut self, content: &'static str) -> Self {
        self.content = content;

        self
    }

    pub fn set_style(mut self, style: &'static str) -> Self {
        self.style = style;

        self
    }

    pub fn content(&self) -> &str {
        self.content
    }

    pub fn style(&self) -> &str {
        self.style
    }
}

impl UiPaint for SplashScreen {
    fn to_html(&self) -> Cow<str> {
        let splash_open = Cow::Borrowed(r#"<div id="splashscreen""#);
        let close_div = "</div>";
        let logo_parent = r#"<div style="width: 40%;">"#;

        splash_open + self.style + ">" + logo_parent + self.content + close_div + close_div
    }
}

impl Default for SplashScreen {
    fn default() -> Self {
        SplashScreen {
            content: PUPPETEER_LOGO,
            style: DEFAULT_SPLASH_STYLE,
            loader: None,
        }
    }
}

pub const PUPPETEER_LOADER: &str = r#"
<div class="lds-ellipsis">
    <div></div>
    <div></div>
    <div></div>
    <div></div>
</div>
"#;

pub const SPLASH_ANIMATION_CSS: &str = r#"
#splashscreen {
    display: flex;
    flex-direction: column;
}
.lds-ellipsis {
    display: inline-block;
    position: relative;
    width: 80px;
    height: 80px;
  }
  .lds-ellipsis div {
    position: absolute;
    top: 33px;
    width: 13px;
    height: 13px;
    border-radius: 50%;
    background: #fff;
    animation-timing-function: cubic-bezier(0, 1, 1, 0);
  }
  .lds-ellipsis div:nth-child(1) {
    left: 8px;
    animation: lds-ellipsis1 0.6s infinite;
  }
  .lds-ellipsis div:nth-child(2) {
    left: 8px;
    animation: lds-ellipsis2 0.6s infinite;
  }
  .lds-ellipsis div:nth-child(3) {
    left: 32px;
    animation: lds-ellipsis2 0.6s infinite;
  }
  .lds-ellipsis div:nth-child(4) {
    left: 56px;
    animation: lds-ellipsis3 0.6s infinite;
  }
  @keyframes lds-ellipsis1 {
    0% {
      transform: scale(0);
    }
    100% {
      transform: scale(1);
    }
  }
  @keyframes lds-ellipsis3 {
    0% {
      transform: scale(1);
    }
    100% {
      transform: scale(0);
    }
  }
  @keyframes lds-ellipsis2 {
    0% {
      transform: translate(0, 0);
    }
    100% {
      transform: translate(24px, 0);
    }
  }
  
"#;
