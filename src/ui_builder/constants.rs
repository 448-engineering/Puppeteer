pub const PUPPETEER_INITIALIZED_APP: &str = "PuppeteerInitializedApp";

pub const PUPPETEER_ADD_STYLE: &str = "PuppeteerAddStyle";

pub const PUPPETEER_REMOVE_STYLE: &str = "PuppeteerRemoveStyle";

pub const PUPPETEER_CSS_RESET_STYLES: &str = "PuppeteerCssResetStyles";

pub const PUPPETEER_ROOT_PAGE: &str = "PuppeteerRootPage";

pub const PUPPETEER_INIT_ERROR_PAGE: &str = "PuppeteerInitErrorPage";

pub const COMMAND_UI_UPDATE: &str = "PuppUi»";

pub const COMMAND_ROOT_UI: &str = "RootUi»";

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
