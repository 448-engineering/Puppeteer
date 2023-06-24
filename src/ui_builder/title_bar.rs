use std::borrow::Cow;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
pub enum TitleBarType {
    Native,
    #[default]
    Puppeteer,
    None,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct TitleBar<'p> {
    text_content: Cow<'p, str>,
    minimize: Cow<'p, str>,
    maximize: Cow<'p, str>,
    close: Cow<'p, str>,
    state: TitleBarType,
    style: &'p str,
}

impl<'p> TitleBar<'p> {
    pub fn new() -> Self {
        TitleBar::default()
    }

    pub fn set_text_content(mut self, text_content: &'p str) -> Self {
        self.text_content = text_content.into();

        self
    }

    pub fn set_minimize_icon(mut self, minimize_icon: &'p str) -> Self {
        self.minimize = minimize_icon.into();

        self
    }

    pub fn set_maximize_icon(mut self, maximize_icon: &'p str) -> Self {
        self.maximize = maximize_icon.into();

        self
    }

    pub fn set_close_icon(mut self, close_icon: &'p str) -> Self {
        self.close = close_icon.into();

        self
    }

    pub fn set_style(mut self, style: &'p str) -> Self {
        self.style = style.into();

        self
    }

    pub fn set_title_bar_type(mut self, title_bar_type: TitleBarType) -> Self {
        self.state = title_bar_type;

        self
    }

    pub fn set_title_bar_type_borrowed(&mut self, title_bar_type: TitleBarType) -> &mut Self {
        self.state = title_bar_type;

        self
    }

    pub fn text_content(&self) -> &Cow<'_, str> {
        &self.text_content
    }

    pub fn minimize(&self) -> &Cow<'_, str> {
        &self.minimize
    }

    pub fn maximize(&self) -> &Cow<'_, str> {
        &self.maximize
    }

    pub fn title_bar_type(&self) -> TitleBarType {
        self.state
    }

    pub fn style(&self) -> &str {
        self.style
    }

    pub fn build(self) -> Cow<'p, str> {
        let title_bar_open = Cow::Borrowed("<div id = titlebar>");
        let div_close = "</div>";
        let drag_region = r#"<div class="drag-region">"#;
        let minimize_button =
            r#"<div class="titlebar-button" onclick="window.ipc.postMessage('minimize')">"#;
        let maximize_button =
            r#"<div class="titlebar-button" onclick="window.ipc.postMessage('maximize')">"#;
        let close_button =
            r#"<div class="titlebar-button" onclick="window.ipc.postMessage('close')">"#;

        title_bar_open
            + drag_region
            + self.text_content
            + div_close
            + minimize_button
            + self.minimize
            + div_close
            + maximize_button
            + self.maximize
            + div_close
            + close_button
            + self.close
            + div_close
            + div_close
    }
}

impl<'p> Default for TitleBar<'p> {
    fn default() -> Self {
        TitleBar {
            text_content: "Puppeteer App".into(),
            minimize: MINIMIZE.into(),
            maximize: MAXIMIZE.into(),
            close: CLOSE.into(),
            state: TitleBarType::Puppeteer,
            style: STYLE,
        }
    }
}

const MINIMIZE: &str = r#"
<svg enable-background="new 0 0 32 32" height="32" viewBox="0 0 16 16" width="32"
    xmlns="http://www.w3.org/2000/svg">
    <path
        d="m7.8193475 11.317932c.049921.04992.1152784.07486.1806711.07486.065393 0 .130786-.02494.1806715-.07486l4.0891049-4.0891039c.09984-.099843.09984-.2615359 0-.3613787-.09984-.099842-.261536-.099842-.361379 0l-3.9083974 3.9084336-3.9084331-3.9084336c-.099843-.099842-.2615359-.099842-.3613782 0-.099843.099843-.099843.2615358 0 .3613787z"
        fill="\#f4f6f9" stroke-width=".036149" />
</svg>
"#;

const MAXIMIZE: &str = r#"
<svg enable-background="new 0 0 32 32" height="32" viewBox="0 0 16 16" width="32"
    xmlns="http://www.w3.org/2000/svg">
    <path
        d="m1403.7995 252h3.3818c.4503 0 .8162.36847.8187.8188v3.3817zm2.4074 6.00692h-3.395c-.4504 0-.8188-.36842-.8188-.81875v-3.39509z"
        fill="\#fff" fill-rule="evenodd" transform="translate(-1397 -247)" />
</svg>
"#;

const CLOSE: &str = r#"
<svg enable-background="new 0 0 32 32" height="32" viewBox="0 0 16 16" width="32"
    xmlns="http://www.w3.org/2000/svg">
    <g fill="\#fff" transform="matrix(2.4549179 0 0 2.4549179 -11.838519 -11.666773)">
        <rect height="2.395204" ry=".322258"
            transform="matrix(-.71029879 -.7039003 -.7039003 .71029879 0 0)" width=".644516"
            x="-11.74636" y="-1.229385" />
        <rect height="2.395204" ry=".322258"
            transform="matrix(.71029879 -.7039003 .7039003 .71029879 0 0)" width=".644516" x="-.187102"
            y="10.225745" />
        <path
            d="m8.1368979 5.9943268a2.0244772 2.0244772 0 0 0 -2.0245266 2.0245273 2.0244772 2.0244772 0 0 0 2.0245266 2.0245259 2.0244772 2.0244772 0 0 0 2.0245271-2.0245259 2.0244772 2.0244772 0 0 0 -2.0245271-2.0245273zm0 .4247501a1.6002238 1.6002238 0 0 1 1.5997774 1.5997772 1.6002238 1.6002238 0 0 1 -1.5997774 1.6008925 1.6002238 1.6002238 0 0 1 -1.6008922-1.6008925 1.6002238 1.6002238 0 0 1 1.6008922-1.5997772z" />
    </g>
</svg>
"#;

const STYLE: &str = r#"
<style>
        html {
            font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
        }

        * {
            padding: 0;
            margin: 0;
            box-sizing: border-box;
        }

        main {
            display: grid;
            place-items: center;
            height: calc(100vh - 30px);
        }

        .titlebar {
            height: 30px;
            padding-left: 5px;
            display: grid;
            grid-auto-flow: column;
            grid-template-columns: 1fr max-content max-content max-content;
            align-items: center;
            background: #1F1F1F;
            color: white;
            user-select: none;
        }

        .titlebar-button {
            display: inline-flex;
            justify-content: center;
            align-items: center;
            width: 30px;
            height: 30px;
        }

        .titlebar-button:hover {
            color: #FFFFFF;
            background: #3b3b3b;
        }

        .titlebar-button#close:hover {
            background: #da3d3d;
        }

        .titlebar-button img {
            filter: invert(100%);
        }
    </style>
"#;
