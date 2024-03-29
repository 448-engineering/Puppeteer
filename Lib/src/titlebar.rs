use crate::{ColorPalette, StaticCowStr, StaticStr, UiPaint};

/// The default script to use when a drag event is done on the title bar
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

/// This is helper function to create a title bar for the app
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct TitleBar {
    id: &'static str,
    text_content: &'static str,
    minimize: &'static str,
    maximize: &'static str,
    close: &'static str,
}

impl TitleBar {
    /// Create a new title bar
    pub fn new() -> Self {
        TitleBar::default()
    }

    /// Add an id for the title bar. If not added it with default to `id="title-bar"`
    pub fn with_id(mut self, id: &'static str) -> Self {
        self.id = id;

        self
    }

    /// Set the text to be displayed in the title bar
    pub fn set_text_content(mut self, text_content: &'static str) -> Self {
        self.text_content = text_content;

        self
    }

    /// Sets the SVG minimize icon
    pub fn set_minimize_icon(mut self, minimize_icon: &'static str) -> Self {
        self.minimize = minimize_icon;

        self
    }

    /// Sets the SVG maximize icon
    pub fn set_maximize_icon(mut self, maximize_icon: &'static str) -> Self {
        self.maximize = maximize_icon;

        self
    }

    /// Sets the SVG close icon
    pub fn set_close_icon(mut self, close_icon: &'static str) -> Self {
        self.close = close_icon;

        self
    }

    /// Get te text displayed in the title bar
    pub fn text_content(&self) -> &str {
        self.text_content
    }

    /// Get the minimize icon
    pub fn minimize(&self) -> &str {
        self.minimize
    }

    /// Get the maximize svg icon
    pub fn maximize(&self) -> &str {
        self.maximize
    }

    /// Get the close svg icon
    pub fn close(&self) -> &str {
        self.close
    }
}

impl UiPaint for TitleBar {
    fn to_html(&self) -> StaticCowStr {
        let title_bar_open = StaticCowStr::Borrowed(r#"<div id=""#) + self.id + r#"">"#;
        let div_close = "</div>";
        let drag_region = r#"<div class="drag-region">"#;
        let minimize_button =
            r#"<div class="titlebar-button" onclick="window.ipc.postMessage('minimize')">"#;
        let maximize_button =
            r#"<div class="titlebar-button" onclick="window.ipc.postMessage('maximize')">"#;
        let close_button =
            r#"<div class="titlebar-button" onclick="window.ipc.postMessage('close_window')">"#;

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

impl Default for TitleBar {
    fn default() -> Self {
        TitleBar {
            id: "title-bar",
            text_content: "Puppeteer App",
            minimize: MINIMIZE,
            maximize: MAXIMIZE,
            close: CLOSE,
        }
    }
}

/// Create CSS for the title bar based on the color palette
#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub struct TitleBarCSS {
    /// The background color of the window icon
    pub background_color: StaticStr,
    /// The background color of the window icon on hover
    pub background_color_hover: StaticStr,
    /// The padding surrounding the window icon
    pub padding: StaticStr,
    /// The padding surrounding the window icon on hover
    pub padding_hover: StaticStr,
    /// The width of the SVG icon
    pub svg_icon_width: StaticStr,
    /// The width of the SVG icon on hover
    pub svg_icon_width_hover: StaticStr,
    /// The color of the SVG icon
    pub svg_icon_color: StaticStr,
    /// The width of the SVG icon on hover
    pub svg_icon_color_hover: StaticStr,
    /// Extra CSS styles to add
    pub extra_css: StaticStr,
}

impl TitleBarCSS {
    /// Initialize using the [ColorPalette]
    pub fn new(palette: &ColorPalette) -> Self {
        TitleBarCSS {
            background_color: palette.primary,
            background_color_hover: palette.secondary,
            padding: "10px",
            padding_hover: "10px",
            svg_icon_width: "20px",
            svg_icon_width_hover: "20px",
            svg_icon_color: palette.secondary,
            svg_icon_color_hover: palette.primary,
            extra_css: "",
        }
    }

    /// Build CSS to use to insert to the stylesheet
    pub fn build(self) -> StaticCowStr {
        let window_icon = StaticCowStr::Borrowed(
            r#"
        .window-icon {
            padding: "#,
        ) + self.padding
            + ";"
            + "background-color: "
            + self.background_color
            + ";"
            + r#"            
        }"#;

        let window_icon_hover = StaticCowStr::Borrowed(
            r#"
        .window-icon:hover {
            padding: "#,
        ) + self.padding_hover
            + ";"
            + "background-color: "
            + self.background_color_hover
            + ";"
            + r#"            
        }"#;

        let svg_icon_width = StaticCowStr::Borrowed(
            r#"        
        .window-icon svg {
            width: "#,
        ) + self.svg_icon_width
            + ";
        }";

        let svg_icon_width_hover = StaticCowStr::Borrowed(
            r#"        
        .window-icon svg {
            width: "#,
        ) + self.svg_icon_width_hover
            + ";
        }";

        let svg_icon_color = StaticCowStr::Borrowed(
            r#"
        .window-icon .window-icon-svg path {
            fill: "#,
        ) + self.svg_icon_color
            + r#";
        }"#;

        let svg_icon_hover = StaticCowStr::Borrowed(
            r#"
        .window-icon:hover .window-icon-svg path {
            fill: "#,
        ) + self.svg_icon_color_hover
            + r#";
        }"#;

        StaticCowStr::Borrowed(
            r#"
        #window-actions {
            display: flex;
            justify-content: end;
            -webkit-box-orient: horizontal;
            -webkit-box-direction: normal;
            flex-direction: row;
            -webkit-box-align: center;
            align-items: center;
        }
        "#,
        ) + window_icon
            + window_icon_hover
            + svg_icon_width
            + svg_icon_width_hover
            + svg_icon_color
            + svg_icon_hover
            + self.extra_css
    }
}

const MINIMIZE: &str = r#"
<svg enable-background="new 0 0 32 32" height="32" viewBox="0 0 16 16" width="32"
    xmlns="http://www.w3.org/2000/svg">
    <path
        d="m7.8193475 11.317932c.049921.04992.1152784.07486.1806711.07486.065393 0 .130786-.02494.1806715-.07486l4.0891049-4.0891039c.09984-.099843.09984-.2615359 0-.3613787-.09984-.099842-.261536-.099842-.361379 0l-3.9083974 3.9084336-3.9084331-3.9084336c-.099843-.099842-.2615359-.099842-.3613782 0-.099843.099843-.099843.2615358 0 .3613787z"
        fill="" stroke-width=".036149" />
</svg>
"#;

const MAXIMIZE: &str = r#"
<svg enable-background="new 0 0 32 32" height="32" viewBox="0 0 16 16" width="32"
    xmlns="http://www.w3.org/2000/svg">
    <path
        d="m1403.7995 252h3.3818c.4503 0 .8162.36847.8187.8188v3.3817zm2.4074 6.00692h-3.395c-.4504 0-.8188-.36842-.8188-.81875v-3.39509z"
        fill="" fill-rule="evenodd" transform="translate(-1397 -247)" />
</svg>
"#;

const CLOSE: &str = r#"
<svg enable-background="new 0 0 32 32" height="32" viewBox="0 0 16 16" width="32"
    xmlns="http://www.w3.org/2000/svg">
    <g fill="" transform="matrix(2.4549179 0 0 2.4549179 -11.838519 -11.666773)">
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
