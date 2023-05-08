use std::borrow::Cow;

use crate::{
    Background, Coordinates, Direction, FlexAlignment, Float, Overflow, Position, StyleDeclaration,
    Visibility, WhiteSpace, WordBreak, Wrap,
};
use hex_color::HexColor;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Default)]
pub struct Styling<'p> {
    fill_max_width: bool,
    fill_max_height: bool,
    min_height: usize,
    min_width: usize,
    max_height: usize,
    max_width: usize,
    width: usize,  //Taken as percentage of width available from parent element
    height: usize, //Taken as percentage of width available from parent element
    width_as_pixels: bool,
    height_as_pixels: bool,
    direction: Direction,
    alignment: FlexAlignment,
    wrap: Wrap,
    background: Option<Background<'p>>,
    z_index: u8,
    grow: Option<u8>,
    shrink: Option<u8>,
    border: Border,
    border_radius: BorderRadius,
    padding: Padding,
    margin: Margin,
    rotate: u16,
    scale: Option<Coordinates>,
    animation: (),
    opacity: Option<u8>,
    overflow: Overflow,
    position: Position,
    visibility: Visibility,
    whitespace: WhiteSpace,
    word_break: WordBreak,
    word_spacing: Option<usize>,
    line_height: Option<usize>,
    filter: Option<Filter>,
    backdrop_filter: Option<Filter>,
    shadow: Option<BoxShadow>,
    // If is `None` use `normal` for css property
    //
    /*
    TODO constrain 0-1
        page-break-after 	Insert a page breaks after an element.
        page-break-before 	Insert a page breaks before an element.
        page-break-inside 	Insert a page breaks inside an element.
    TODO transform_origin
        transform-style
        transition

    */
}

impl<'p> Styling<'p> {
    pub fn fill_max_width(&mut self) -> &mut Self {
        self.fill_max_width = true;

        self
    }

    pub fn is_max_width(&self) -> bool {
        self.fill_max_width
    }

    pub fn fill_max_height(&mut self) -> &mut Self {
        self.fill_max_height = true;

        self
    }

    pub fn max_width_to_css(&self) -> Cow<str> {
        let max_width = Cow::Owned(self.max_width.to_string());

        if self.max_width == 0 {
            Cow::Borrowed("max-width: 100%;")
        } else if self.width_as_pixels {
            Cow::Borrowed("min-width: ") + max_width + "px;"
        } else {
            Cow::Borrowed("max-width: ") + max_width + "%;"
        }
    }

    pub fn is_max_height(&self) -> bool {
        self.fill_max_height
    }

    pub fn set_min_height(&mut self, value: usize) -> &mut Self {
        self.min_height = value;

        self
    }

    pub fn min_height(&self) -> usize {
        self.min_height
    }

    pub fn min_height_to_css(&self) -> Cow<str> {
        let min_height = Cow::Owned(self.min_height.to_string());

        if self.min_height == 0 {
            Cow::Borrowed("min-height: none;")
        } else if self.height_as_pixels {
            Cow::Borrowed("min-height: ") + min_height + "px;"
        } else {
            Cow::Borrowed("min-height: ") + min_height + "%;"
        }
    }

    pub fn set_min_width(&mut self, value: usize) -> &mut Self {
        self.min_width = value;

        self
    }

    pub fn min_width(&self) -> usize {
        self.min_width
    }

    pub fn min_width_to_css(&self) -> Cow<str> {
        let min_width = Cow::Owned(self.min_width.to_string());

        if self.min_width == 0 {
            Cow::Borrowed("min-width: none;")
        } else if self.height_as_pixels {
            Cow::Borrowed("min-width: ") + min_width + "px;"
        } else {
            Cow::Borrowed("min-width: ") + min_width + "%;"
        }
    }

    pub fn set_max_width(&mut self, value: usize) -> &mut Self {
        self.max_width = value;

        self
    }

    pub fn max_width(&self) -> usize {
        self.max_width
    }

    pub fn set_max_height(&mut self, value: usize) -> &mut Self {
        self.max_height = value;

        self
    }

    pub fn max_height(&self) -> usize {
        self.max_height
    }

    pub fn max_height_to_css(&self) -> Cow<str> {
        let max_height = Cow::Owned(self.max_height.to_string());

        if self.max_height == 0 {
            Cow::Borrowed("max-height: none;")
        } else if self.height_as_pixels {
            Cow::Borrowed("max-height: ") + max_height + "px;"
        } else {
            Cow::Borrowed("max-height: ") + max_height + "%;"
        }
    }

    pub fn set_width(&mut self, value: usize) -> &mut Self {
        self.width = value;

        self
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn width_to_css(&self) -> Cow<str> {
        let width = Cow::Owned(self.width.to_string());

        if self.width == 0 {
            Cow::Borrowed("width: auto;")
        } else if self.width_as_pixels {
            Cow::Borrowed("width: ") + width + "px;"
        } else {
            Cow::Borrowed("width: ") + width + "%;"
        }
    }

    pub fn set_height(&mut self, value: usize) -> &mut Self {
        self.height = value;

        self
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn height_to_css(&self) -> Cow<str> {
        let height = Cow::Owned(self.height.to_string());

        if self.height == 0 {
            Cow::Borrowed("height: auto;")
        } else if self.height_as_pixels {
            Cow::Borrowed("height: ") + height + "px;"
        } else {
            Cow::Borrowed("height: ") + height + "%;"
        }
    }

    pub fn set_width_as_pixels(&mut self) -> &mut Self {
        self.width_as_pixels = true;

        self
    }

    pub fn set_width_as_percentage(&mut self) -> &mut Self {
        self.width_as_pixels = false;

        self
    }

    pub fn width_as_pixels(&self) -> bool {
        self.width_as_pixels
    }

    pub fn set_height_as_pixels(&mut self) -> &mut Self {
        self.height_as_pixels = true;

        self
    }

    pub fn set_height_as_percentage(&mut self) -> &mut Self {
        self.height_as_pixels = false;

        self
    }

    pub fn height_as_pixels(&self) -> bool {
        self.height_as_pixels
    }

    pub fn set_direction(&mut self, value: Direction) -> &mut Self {
        self.direction = value;

        self
    }

    pub fn direction(&self) -> Direction {
        self.direction
    }

    pub fn set_alignment(&mut self, value: FlexAlignment) -> &mut Self {
        self.alignment = value;

        self
    }

    pub fn alignment(&self) -> FlexAlignment {
        self.alignment
    }

    pub fn set_wrap(&mut self, value: Wrap) -> &mut Self {
        self.wrap = value;

        self
    }

    pub fn wrap(&self) -> Wrap {
        self.wrap
    }

    pub fn set_background(&mut self, value: Background<'p>) -> &mut Self {
        self.background = Some(value);

        self
    }

    pub fn background(&self) -> Option<Background> {
        self.background
    }

    pub fn set_z_index(&mut self, value: u8) -> &mut Self {
        self.z_index = value;

        self
    }

    pub fn z_index_to_css(&self) -> Cow<str> {
        Cow::Borrowed("z-index: ") + Cow::Owned(self.z_index.to_string()) + ";"
    }

    pub fn z_index(&self) -> u8 {
        self.z_index
    }

    pub fn set_grow(&mut self, value: u8) -> &mut Self {
        if value == 0 {
            self.grow = None;
        } else {
            self.grow = Some(value);
        }

        self
    }

    pub fn grow(&self) -> Option<u8> {
        self.grow
    }

    pub fn grow_to_css(&self) -> Cow<str> {
        if let Some(grow) = self.grow {
            Cow::Borrowed("flex-grow: ") + Cow::Owned(grow.to_string()) + ";"
        } else {
            Cow::Borrowed("flex-grow: 1;")
        }
    }

    pub fn set_shrink(&mut self, value: u8) -> &mut Self {
        if value == 0 {
            self.shrink = None;
        } else {
            self.shrink = Some(value);
        }

        self
    }

    pub fn shrink(&self) -> Option<u8> {
        self.shrink
    }

    pub fn shrink_to_css(&self) -> Cow<str> {
        if let Some(shrink) = self.shrink {
            Cow::Borrowed("flex-shrink: ") + Cow::Owned(shrink.to_string()) + ";"
        } else {
            Cow::Borrowed("flex-shrink: 1;")
        }
    }

    pub fn set_border(&mut self, value: Border) -> &mut Self {
        self.border = value;

        self
    }

    pub fn border(&self) -> Border {
        self.border
    }

    pub fn set_border_radius(&mut self, value: BorderRadius) -> &mut Self {
        self.border_radius = value;

        self
    }

    pub fn border_radius(&self) -> BorderRadius {
        self.border_radius
    }

    pub fn set_padding(&mut self, value: Padding) -> &mut Self {
        self.padding = value;

        self
    }

    pub fn padding(&self) -> Padding {
        self.padding
    }

    pub fn set_margin(&mut self, value: Margin) -> &mut Self {
        self.margin = value;

        self
    }

    pub fn margin(&self) -> Margin {
        self.margin
    }

    pub fn set_rotate(&mut self, angle: u16) -> &mut Self {
        //TODO write a helper to convert degrees to radians
        self.rotate = angle;

        self
    }

    pub fn rotate(&self) -> u16 {
        self.rotate
    }

    pub fn rotate_as_css(&self) -> Cow<'p, str> {
        let rotation = Cow::from("transform: rotate(");

        rotation + Cow::Owned(self.rotate().to_string()) + "deg);"
    }

    /// Scaling as a percentage value of the type maximum of 255 %
    pub fn set_scale(&mut self, scale_value: Coordinates) -> &mut Self {
        self.scale = Some(scale_value);

        self
    }

    pub fn scale(&self) -> Option<Coordinates> {
        self.scale
    }

    pub fn scale_as_css(&self) -> Option<Cow<str>> {
        if let Some(scale) = self.scale {
            let scale_to = Cow::Borrowed("transform: scale(");
            let scale_x = scale.x.to_string();
            let scale_y = scale.y.to_string();

            Some(
                scale_to
                    + Cow::Owned(scale_x.to_string())
                    + ", "
                    + Cow::Owned(scale_y.to_string())
                    + ");",
            )
        } else {
            None
        }
    }

    pub fn set_opacity(&mut self, opacity: u8) -> &mut Self {
        if opacity > 100 {
            self.opacity = Some(100);
        } else {
            self.opacity = Some(opacity);
        }

        self
    }

    pub fn opacity(&self) -> Option<u8> {
        self.opacity
    }

    pub fn opacity_as_css(&self) -> Cow<'p, str> {
        if let Some(value) = self.opacity {
            let opacity = Cow::from("opacity: ");

            opacity + Cow::Owned(format!("{:.1}", value)) + ");"
        } else {
            Cow::Borrowed("opacity: 1;")
        }
    }

    pub fn set_overflow(&mut self, overflow: Overflow) -> &mut Self {
        self.overflow = overflow;

        self
    }

    pub fn overflow(&self) -> Overflow {
        self.overflow
    }

    pub fn set_position(&mut self, position: Position) -> &mut Self {
        self.position = position;

        self
    }

    pub fn position(&self) -> Position {
        self.position
    }

    pub fn set_visibility(&mut self, visibility: Visibility) -> &mut Self {
        self.visibility = visibility;

        self
    }

    pub fn visibility(&self) -> Visibility {
        self.visibility
    }

    pub fn set_whitespace(&mut self, whitespace: WhiteSpace) -> &mut Self {
        self.whitespace = whitespace;

        self
    }

    pub fn whitespace(&self) -> WhiteSpace {
        self.whitespace
    }

    pub fn set_word_break(&mut self, word_break: WordBreak) -> &mut Self {
        self.word_break = word_break;

        self
    }

    pub fn word_break(&self) -> WordBreak {
        self.word_break
    }

    pub fn set_word_spacing(&mut self, word_spacing: usize) -> &mut Self {
        if word_spacing == 0 {
            self.word_spacing = None;
        } else {
            self.word_spacing = Some(word_spacing);
        }

        self
    }

    pub fn word_spacing(&self) -> Option<usize> {
        self.word_spacing
    }

    /// Uses `em`
    pub fn word_spacing_as_css(&self) -> Cow<'p, str> {
        if let Some(word_spacing) = self.word_spacing {
            Cow::Borrowed("word-spacing: ") + Cow::Owned(word_spacing.to_string()) + "em);"
        } else {
            Cow::Borrowed("word-spacing: normal;")
        }
    }

    pub fn set_line_height(&mut self, line_height: usize) -> &mut Self {
        if line_height == 0 {
            self.line_height = None;
        } else {
            self.line_height = Some(line_height);
        }

        self
    }

    pub fn line_height(&self) -> Option<usize> {
        self.line_height
    }

    /// Uses `%`
    pub fn line_height_as_css(&self) -> Cow<'p, str> {
        if let Some(line_height) = self.line_height {
            Cow::Borrowed("line-height: ") + Cow::Owned(line_height.to_string()) + "%);"
        } else {
            Cow::Borrowed("line-height: normal;")
        }
    }

    pub fn set_filter(&mut self, filter: Filter) -> &mut Self {
        self.filter = Some(filter);

        self
    }

    pub fn filter(&self) -> Option<Filter> {
        self.filter
    }

    pub fn set_backdrop_filter(&mut self, filter: Filter) -> &mut Self {
        self.backdrop_filter = Some(filter);

        self
    }

    pub fn backdrop_filter(&self) -> Option<Filter> {
        self.backdrop_filter
    }

    pub fn set_shadow(&mut self, shadow: BoxShadow) -> &mut Self {
        self.shadow = Some(shadow);

        self
    }

    pub fn shadow(&self) -> Option<BoxShadow> {
        self.shadow
    }

    pub fn compute_css_width(&self) -> String {
        if self.width_as_pixels {
            self.width.to_string() + "px"
        } else {
            self.width.to_string() + "%"
        }
    }

    pub fn compute_css_height(&self) -> String {
        if self.height_as_pixels {
            self.height.to_string() + "px"
        } else {
            self.height.to_string() + "%"
        }
    }
}

impl<'p> StyleDeclaration for Styling<'p> {
    fn to_css(&self) -> Cow<str> {
        let prefix = Cow::Borrowed("{ ");
        let suffix = Cow::Borrowed(" }");

        let background = if let Some(background) = self.background {
            background.to_css().to_string() + " "
        } else {
            String::default()
        };

        let filter = if let Some(filter) = self.filter {
            filter.to_css().to_string() + " "
        } else {
            String::default()
        };

        let backdrop_filter = if let Some(backdrop_filter) = self.backdrop_filter {
            backdrop_filter.to_css().to_string() + " "
        } else {
            String::default()
        };

        let scale = if let Some(scale) = self.scale_as_css() {
            scale + " "
        } else {
            Cow::default()
        };

        let shadow = if let Some(shadow) = self.shadow {
            shadow.to_css().to_string() + " "
        } else {
            String::default()
        };

        prefix
            + self.max_width_to_css()
            + " "
            + self.max_height_to_css()
            + " "
            + self.min_width_to_css()
            + " "
            + self.min_height_to_css()
            + " "
            + self.width_to_css()
            + " "
            + self.height_to_css()
            + " "
            + self.direction.to_css()
            + " "
            + self.alignment.to_css()
            + " "
            + self.wrap.to_css()
            + " "
            + Cow::Owned(background)
            + " "
            + self.z_index_to_css()
            + " "
            + self.grow_to_css()
            + " "
            + self.shrink_to_css()
            + " "
            + self.border.to_css()
            + " "
            + self.border_radius.to_css()
            + " "
            + self.padding.to_css()
            + " "
            + self.margin.to_css()
            + " "
            + self.rotate_as_css()
            + " "
            + scale
            + self.opacity_as_css()
            + " "
            + self.overflow.to_css()
            + " "
            + self.position.to_css()
            + " "
            + self.visibility.to_css()
            + " "
            + self.whitespace.to_css()
            + " "
            + self.word_break.to_css()
            + " "
            + self.word_spacing_as_css()
            + " "
            + self.line_height_as_css()
            + " "
            + Cow::Owned(filter)
            + Cow::Owned(backdrop_filter)
            + Cow::Owned(shadow)
            + suffix
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
pub struct Border {
    pub width_top: usize,
    pub width_top_color: HexColor,
    pub width_bottom: usize,
    pub width_bottom_color: HexColor,
    pub width_left: usize,
    pub width_left_color: HexColor,
    pub width_right: usize,
    pub width_right_color: HexColor,
    //TODO add border-style
}

impl Border {
    pub fn new() -> Self {
        let color = HexColor {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        };

        Border {
            width_top: 0,
            width_top_color: color,
            width_bottom: 0,
            width_bottom_color: color,
            width_left: 0,
            width_left_color: color,
            width_right: 0,
            width_right_color: color,
        }
    }

    /// Change the color of all border sides
    pub fn set_color(mut self, color: HexColor) -> Self {
        self.width_top_color = color;
        self.width_bottom_color = color;
        self.width_left_color = color;
        self.width_right_color = color;

        self
    }

    /// Change the width of all border sides
    pub fn set_width(mut self, value: usize) -> Self {
        self.width_top = value;
        self.width_bottom = value;
        self.width_left = value;
        self.width_right = value;

        self
    }

    pub fn set_width_top(mut self, top: usize) -> Self {
        self.width_top = top;

        self
    }

    pub fn set_width_bottom(mut self, bottom: usize) -> Self {
        self.width_bottom = bottom;

        self
    }

    pub fn set_width_right(mut self, right: usize) -> Self {
        self.width_right = right;

        self
    }

    pub fn set_width_left(mut self, left: usize) -> Self {
        self.width_left = left;

        self
    }

    pub fn set_width_top_color(mut self, color_top: HexColor) -> Self {
        self.width_top_color = color_top;

        self
    }

    pub fn set_width_bottom_color(mut self, color_bottom: HexColor) -> Self {
        self.width_bottom_color = color_bottom;

        self
    }

    pub fn set_width_left_color(mut self, color_left: HexColor) -> Self {
        self.width_left_color = color_left;

        self
    }

    pub fn set_width_right_color(mut self, color_right: HexColor) -> Self {
        self.width_right_color = color_right;

        self
    }
}

impl StyleDeclaration for Border {
    fn to_css(&self) -> Cow<str> {
        let px_and_color = "px solid ";

        let border_top = Cow::Borrowed("border-top: ")
            + Cow::Owned(self.width_top.to_string())
            + px_and_color
            + Cow::Owned(format!("{}", self.width_top_color))
            + ";";

        let border_bottom = Cow::Borrowed("border-bottom: ")
            + Cow::Owned(self.width_bottom.to_string())
            + px_and_color
            + Cow::Owned(format!("{}", self.width_bottom_color))
            + ";";

        let border_left = Cow::Borrowed("border-left: ")
            + Cow::Owned(self.width_left.to_string())
            + px_and_color
            + Cow::Owned(format!("{}", self.width_left_color))
            + ";";

        let border_right = Cow::Borrowed("border-right: ")
            + Cow::Owned(self.width_right.to_string())
            + px_and_color
            + Cow::Owned(format!("{}", self.width_right_color))
            + ";";

        border_top + " " + border_bottom + " " + border_left + " " + border_right
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Ltrb {
    pub left: usize,
    pub top: usize,
    pub right: usize,
    pub bottom: usize,
}

/// CSS Filters can be found at [MDN filter](https://developer.mozilla.org/en-US/docs/Web/CSS/filter)
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Filter {
    Blur(usize),
    Brightness(Float),
    Contrast(u8),
    DropShadow(u8, u8, u8, HexColor), // (Horizontal, Vertical, Blur_Radius, Color)
    GrayScale(u8),
    HueRotation(u16), //Measured in degrees
    Invert(u8),
    Opacity(u8),
    Sephia(u8),
    Saturate(u16),
    //TODO url()
}

impl StyleDeclaration for Filter {
    fn to_css(&self) -> Cow<str> {
        let value = match self {
            Self::Blur(value) => Cow::Borrowed("blur(") + Cow::Owned(value.to_string()) + "px)",
            Self::Brightness(value) => {
                Cow::Borrowed("brightness(") + value.to_string_integral() + "%)"
            }
            Self::Contrast(value) => {
                Cow::Borrowed("contrast(") + Cow::Owned(value.to_string()) + "%)"
            }
            Self::DropShadow(horizontal, vertical, blur_radius, color) => {
                let offset_x = Cow::Owned(horizontal.to_string()) + Cow::Borrowed("px");
                let offset_y = Cow::Owned(vertical.to_string()) + Cow::Borrowed("px");
                let blur_radius = Cow::Owned(blur_radius.to_string()) + Cow::Borrowed("px");
                let color = Cow::Owned(format!("{}", color));
                Cow::Borrowed("drop-shadow(")
                    + offset_x
                    + " "
                    + offset_y
                    + " "
                    + blur_radius
                    + " "
                    + color
                    + ")"
            }
            Self::GrayScale(value) => {
                Cow::Borrowed("grayscale(") + u8_to_percentage_max(value) + "%)"
            }
            Self::HueRotation(value) => {
                Cow::Borrowed("hue-rotate(") + { to_degree_str(value) } + "deg)"
            }
            Self::Invert(value) => Cow::Borrowed("invert(") + u8_to_percentage_max(value) + "%)",
            Self::Opacity(value) => Cow::Borrowed("opacity(") + u8_to_percentage_max(value) + "%)",
            Self::Sephia(value) => Cow::Borrowed("sephia(") + u8_to_percentage_max(value) + "%)",
            Self::Saturate(value) => {
                Cow::Borrowed("saturate(") + Cow::Owned(value.to_string()) + "%)"
            }
        };

        Cow::Borrowed("filter: ") + value + ";"
    }
}

pub fn u8_to_percentage_max(value: &u8) -> Cow<str> {
    if value > &100 {
        Cow::Borrowed("100")
    } else {
        Cow::Owned(value.to_string())
    }
}

pub fn to_degree_str<'p>(value: &u16) -> Cow<'p, str> {
    if value > &360 {
        Cow::Borrowed("360")
    } else {
        Cow::Owned(value.to_string())
    }
}

impl Default for Filter {
    fn default() -> Self {
        Filter::Brightness(Float {
            signed: false,
            integral: 100,
            fractional: 0,
        })
    }
}

/// offset-x | offset-y | blur-radius | spread-radius | color
/// [CSS box-shadow](https://developer.mozilla.org/en-US/docs/Web/CSS/box-shadow)
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
pub struct BoxShadow {
    pub offset_x: usize,
    pub offset_y: usize,
    pub blur_radius: usize,
    pub spread_radius: usize,
    pub inset: bool,
    pub color: HexColor,
}

impl StyleDeclaration for BoxShadow {
    fn to_css(&self) -> Cow<str> {
        let offset_x = Cow::Owned(self.offset_x.to_string()) + "px";
        let offset_y = Cow::Owned(self.offset_y.to_string()) + "px";
        let blur_radius = Cow::Owned(self.blur_radius.to_string()) + "px";
        let spread_radius = Cow::Owned(self.spread_radius.to_string()) + "px";
        let color = Cow::Owned(format!("{}", self.color));

        let inset = if self.inset == true { "inset" } else { "" };

        Cow::Borrowed("box-shadow: ")
            + inset
            + " "
            + offset_x
            + " "
            + offset_y
            + " "
            + blur_radius
            + " "
            + spread_radius
            + " "
            + color
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
pub struct BorderRadius {
    pub top_left_radius: usize,
    pub top_right_radius: usize,
    pub bottom_left_radius: usize,
    pub bottom_right_radius: usize,
}

impl StyleDeclaration for BorderRadius {
    fn to_css(&self) -> Cow<str> {
        let border_top_left_radius = Cow::Borrowed("border-top-left-radius: ")
            + Cow::Owned(self.top_left_radius.to_string())
            + "px;";
        let border_top_right_radius = Cow::Borrowed("border-top-right-radius: ")
            + Cow::Owned(self.top_right_radius.to_string())
            + "px;";
        let border_bottom_left_radius = Cow::Borrowed("border-bottom-left-radius: ")
            + Cow::Owned(self.bottom_left_radius.to_string())
            + "px;";
        let border_bottom_right_radius = Cow::Borrowed("border-bottom-right-radius: ")
            + Cow::Owned(self.bottom_right_radius.to_string())
            + "px;";

        border_top_left_radius
            + " "
            + border_top_right_radius
            + " "
            + border_bottom_left_radius
            + " "
            + border_bottom_right_radius
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
pub struct Padding {
    pub top: usize,
    pub bottom: usize,
    pub left: usize,
    pub right: usize,
}

impl StyleDeclaration for Padding {
    fn to_css(&self) -> Cow<str> {
        let top = Cow::Borrowed("padding-top: ") + Cow::Owned(self.top.to_string()) + "px;";
        let bottom =
            Cow::Borrowed("padding-bottom: ") + Cow::Owned(self.bottom.to_string()) + "px;";
        let left = Cow::Borrowed("padding-left: ") + Cow::Owned(self.left.to_string()) + "px;";
        let right = Cow::Borrowed("padding-right: ") + Cow::Owned(self.right.to_string()) + "px;";

        top + " " + bottom + " " + left + " " + right
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
pub struct Margin {
    pub top: usize,
    pub bottom: usize,
    pub left: usize,
    pub right: usize,
}

impl StyleDeclaration for Margin {
    fn to_css(&self) -> Cow<str> {
        let top = Cow::Borrowed("margin-top: ") + Cow::Owned(self.top.to_string()) + "px;";
        let bottom = Cow::Borrowed("margin-bottom: ") + Cow::Owned(self.bottom.to_string()) + "px;";
        let left = Cow::Borrowed("margin-left: ") + Cow::Owned(self.left.to_string()) + "px;";
        let right = Cow::Borrowed("margin-right: ") + Cow::Owned(self.right.to_string()) + "px;";

        top + " " + bottom + " " + left + " " + right
    }
}
