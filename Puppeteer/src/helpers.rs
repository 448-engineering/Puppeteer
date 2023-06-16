use wry::application::{
    dpi::{LogicalPosition, LogicalSize, Position, Size},
    window::{Fullscreen, Window},
};

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
