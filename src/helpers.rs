use crate::UiPaint;
use std::borrow::Cow;
use wry::application::{
    dpi::{LogicalPosition, LogicalSize, Position, Size},
    window::{Fullscreen, Window},
};

pub const DEFAULT_SVG: &str = "<svg viewBox='0 0 100 100' xmlns='http://www.w3.org/2000/svg'><path d='m0 0h100v100h-100z' fill='#f00' stroke-width='.829695'/><path d='m13.778851 13.77885h72.442299v72.442299h-72.442299z' fill='#0f0' stroke-width='.60105'/><path d='m28.070158 28.07016h43.859684v43.859684h-43.859684z' fill='#00f' stroke-width='.363902'/><path d='m40.307922 40.30792h19.384157v19.384157h-19.384157z' fill='#fff' stroke-width='.16083'/></svg>";

pub const DEFAULT_SVG_AS_BYTES: &[u8] = DEFAULT_SVG.as_bytes();

pub type UiPaintBoxed = Box<dyn UiPaint>;

#[derive(Debug)]
pub struct PuppeteerUtils<'a> {
    window: &'a Window,
}

impl<'a> PuppeteerUtils<'a> {
    pub fn new(window: &'a Window) -> Self {
        PuppeteerUtils { window }
    }
    pub fn set_borderless_fullscreen(&self) {
        if self.window.fullscreen().is_none() {
            self.window
                .set_fullscreen(Some(Fullscreen::Borderless(None))); //TODO Select fullscreen on different monitor
        } else {
            self.window.set_fullscreen(None)
        }
    }

    pub fn set_maximized(&self) {
        if !self.window.is_maximized() && self.window.is_maximizable() {
            self.window.set_maximized(true)
        } else {
            self.window.set_maximized(false)
        }
    }

    pub fn set_minimized(&self) {
        if !self.window.is_minimized() && self.window.is_minimizable() {
            self.window.set_minimized(true)
        } else {
            self.window.set_minimized(false)
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Default)]
pub struct WindowSize {
    width: f64,
    height: f64,
}

impl From<&WindowSize> for Size {
    fn from(value: &WindowSize) -> Self {
        Size::Logical(LogicalSize {
            width: value.width,
            height: value.height,
        })
    }
}

impl From<WindowSize> for Size {
    fn from(value: WindowSize) -> Self {
        Size::Logical(LogicalSize {
            width: value.width,
            height: value.height,
        })
    }
}

#[derive(Debug, PartialEq, PartialOrd, Default)]
pub struct WindowPosition {
    width: f64,
    height: f64,
}

impl From<&WindowPosition> for Position {
    fn from(value: &WindowPosition) -> Self {
        Position::Logical(LogicalPosition {
            x: value.width,
            y: value.height,
        })
    }
}

impl From<WindowPosition> for Position {
    fn from(value: WindowPosition) -> Self {
        Position::Logical(LogicalPosition {
            x: value.width,
            y: value.height,
        })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
pub struct MenuCreator {} //TODO

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
pub struct IconCreator {} //TODO

#[derive(Debug)]
pub enum ModifyView {
    /// Replaces the node in the view.
    /// For webview, it will replace everything in the webpage
    ReplaceShell(UiPaintBoxed),
    /// Replaces node after the titlebar
    ReplaceApp(UiPaintBoxed),
    /// Replaces the node with the specified ID
    ReplaceNodeWithId { id: String, content: UiPaintBoxed },
    // Replaces the nodes with the specified class
    // ReplaceNodesWithClass{ class: String, content: UiPaintBoxed} //TODO
}

impl ModifyView {
    pub fn replace_shell(data: UiPaintBoxed) -> ModifyView {
        ModifyView::ReplaceShell(data)
    }

    pub fn replace_app(data: UiPaintBoxed) -> ModifyView {
        ModifyView::ReplaceApp(data)
    }

    pub fn replace_with_id(id: &str, data: UiPaintBoxed) -> ModifyView {
        ModifyView::ReplaceNodeWithId {
            id: id.to_owned(),
            content: data,
        }
    }

    /*pub fn replace_with_class(class: &str, data: UiPaintBoxed) -> ModifyView {
        ModifyView::ReplaceNodesWithClass {
            class: class.to_owned(),
            content:data,
        } //TODO
    }*/
}

impl UiPaint for ModifyView {
    fn to_html(&self) -> Cow<str> {
        match self {
            Self::ReplaceShell(content) => {
                Cow::Borrowed(r#"document.documentElement.innerHTML=`"#) + content.to_html() + "`;"
            }
            Self::ReplaceApp(content) => {
                Cow::Borrowed(r#"document.getElementById("puppeteer_app").innerHTML=`"#)
                    + content.to_html()
                    + "`;"
            }
            Self::ReplaceNodeWithId { id, content } => {
                Cow::Borrowed(r#"document.getElementById(""#)
                    + id.as_str()
                    + r#"").innerHTML=`"#
                    + content.to_html()
                    + "`;"
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct HtmlContent<'p> {
    pub content: &'p str,
}

impl<'p> UiPaint for HtmlContent<'p> {
    fn to_html(&self) -> Cow<str> {
        Cow::Borrowed(self.content)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct HtmlStaticContent {
    pub content: &'static str,
}

impl UiPaint for HtmlStaticContent {
    fn to_html(&self) -> Cow<str> {
        Cow::Borrowed(self.content)
    }
}
