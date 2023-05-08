use crate::{Image, StyleDeclaration};
use hex_color::HexColor;
use std::borrow::Cow;

/// This is used to style the alignment of items inside a flex container
///
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct FlexAlignment {
    /// The CSS align-content property sets the distribution of space between and
    /// around content items along a flexbox's cross-axis or a grid's block axis.
    /// [CSS align-content](https://developer.mozilla.org/en-US/docs/Web/CSS/align-content)
    pub align_content: AlignContent,
    /// The CSS align-items property sets the align-self value on all direct children as a group.
    /// In Flexbox, it controls the alignment of items on the Cross Axis.
    /// In Grid Layout, it controls the alignment of items on the Block Axis within their grid area.
    /// [CSS align-items](https://developer.mozilla.org/en-US/docs/Web/CSS/align-items)
    pub align_items: AlignItems,
    /// The align-self CSS property overrides a grid or flex item's align-items value.
    /// In Grid, it aligns the item inside the grid area. In Flexbox, it aligns the item on the cross axis.
    /// [CSS align-self](https://developer.mozilla.org/en-US/docs/Web/CSS/align-self)
    pub align_self: AlignItems,
    /// The CSS justify-content property defines how the browser distributes space between and around content items
    /// along the main-axis of a flex container, and the inline axis of a grid container.
    /// [CSS justify-content](https://developer.mozilla.org/en-US/docs/Web/CSS/justify-content)
    pub justify_content: AlignContent,
    /// The CSS justify-items property defines the default justify-self for all items of the box, giving them all a default
    /// way of justifying each box along the appropriate axis.
    /// [CSS justify-items](https://developer.mozilla.org/en-US/docs/Web/CSS/justify-items)
    pub justify_items: AlignItems,
    /// The CSS justify-self property sets the way a box is justified inside its alignment container along the appropriate axis.
    /// [CSS justify-self](https://developer.mozilla.org/en-US/docs/Web/CSS/justify-self)
    pub justify_self: AlignItems,
}

impl StyleDeclaration for FlexAlignment {
    fn to_css(&self) -> Cow<str> {
        let align_content = Cow::Borrowed("align-content: ") + self.align_content.to_css() + ";";
        let align_items = Cow::Borrowed("align-items: ") + self.align_items.to_css() + ";";
        let align_self = Cow::Borrowed("align-self: ") + self.align_self.to_css() + ";";
        let justify_content =
            Cow::Borrowed("justify-content: ") + self.justify_content.to_css() + ";";
        let justify_items = Cow::Borrowed("justify-items: ") + self.justify_items.to_css() + ";";
        let justify_self = Cow::Borrowed("justify-self: ") + self.justify_self.to_css() + ";";

        align_content
            + " "
            + align_items
            + " "
            + align_self
            + " "
            + justify_content
            + " "
            + justify_items
            + " "
            + justify_self
    }
}

impl Default for FlexAlignment {
    fn default() -> Self {
        FlexAlignment {
            align_content: AlignContent::Center,
            align_items: AlignItems::Center,
            align_self: AlignItems::Center,
            justify_content: AlignContent::Center,
            justify_items: AlignItems::Center,
            justify_self: AlignItems::Center,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
pub enum AlignContent {
    /// The item is packed flush to each other toward the start edge of the alignment container in the appropriate axis.
    Start,
    /// The item is packed flush to each other toward the end edge of the alignment container in the appropriate axis.
    End,
    /// The items are packed flush to each other toward the center of the alignment container.
    #[default]
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

impl StyleDeclaration for AlignContent {
    fn to_css(&self) -> Cow<str> {
        let value = match self {
            Self::Start => "start",
            Self::End => "end",
            Self::Center => "center",
            Self::SpaceBetween => "space-between",
            Self::SpaceAround => "space-between",
            Self::SpaceEvenly => "space-evenly",
        };

        Cow::Borrowed(value)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
pub enum AlignItems {
    /// The item is packed flush to each other toward the start edge of the alignment container in the appropriate axis.
    Start,
    /// The item is packed flush to each other toward the end edge of the alignment container in the appropriate axis.
    End,
    /// The items are packed flush to each other toward the center of the alignment container.
    #[default]
    Center,
    /// If the combined size of the items is less than the size of the alignment container,
    /// any auto-sized items have their size increased equally (not proportionally),
    /// while still respecting the constraints imposed by max-height/max-width (or equivalent functionality),
    /// so that the combined size exactly fills the alignment container.
    Stretch,
    ///Specifies participation in first- or last-baseline alignment: aligns the alignment baseline
    /// of the box's first or last baseline set with the corresponding baseline in the shared first
    /// or last baseline set of all the boxes in its baseline-sharing group.
    Baseline,
}

impl StyleDeclaration for AlignItems {
    fn to_css(&self) -> Cow<str> {
        let value = match self {
            Self::Start => "start",
            Self::End => "end",
            Self::Center => "center",
            Self::Stretch => "stretch",
            Self::Baseline => "baseline",
        };

        Cow::Borrowed(value)
    }
}

/// The flex-wrap CSS property sets whether flex items are forced onto one line or can wrap onto multiple lines.
/// If wrapping is allowed, it sets the direction that lines are stacked.
/// [CSS flex-wrap](https://developer.mozilla.org/en-US/docs/Web/CSS/flex-wrap)
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
pub enum Wrap {
    /// The flex items are laid out in a single line which may cause the flex container to overflow. CSS equivalent to `nowrap`
    None,
    /// The flex items break into multiple lines.
    /// The cross-start is either equivalent to start or before depending flex-direction value and the cross-end is the opposite
    /// of the specified cross-start. CSS equivalent to `wrap`
    #[default]
    Normal,
    /// Behaves the same as wrap but cross-start and cross-end are permuted. Css equivalent is `wrap-reverse`
    Reverse,
}

impl StyleDeclaration for Wrap {
    fn to_css(&self) -> Cow<str> {
        let value = match self {
            Self::None => "flex-wrap: nowrap;",
            Self::Normal => "flex-wrap: wrap;",
            Self::Reverse => "flex-wrap: wrap-reverse;",
        };

        Cow::Borrowed(value)
    }
}

/// The flex-direction CSS property sets how flex items are placed in the flex container defining
/// the main axis and the direction (normal or reversed). [CSS flex-direction](https://developer.mozilla.org/en-US/docs/Web/CSS/flex-direction)
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
pub enum Direction {
    /// The flex container's main-axis is defined to be the same as the text direction.
    /// The main-start and main-end points are the same as the content direction.
    #[default]
    Row,
    /// Behaves the same as row but the main-start and main-end points are opposite to the content direction.
    RowReverse,
    /// The flex container's main-axis is the same as the block-axis.
    /// The main-start and main-end points are the same as the before and after points of the writing-mode.
    Column,
    /// Behaves the same as column but the main-start and main-end are opposite to the content direction.
    ColumnReverse,
}

impl StyleDeclaration for Direction {
    fn to_css(&self) -> Cow<str> {
        let value = match self {
            Direction::Row => "flex-direction: row;",
            Direction::RowReverse => "flex-direction: row-reverse;",
            Direction::Column => "flex-direction: column;",
            Direction::ColumnReverse => "flex-direction: column-reverse;",
        };

        Cow::Borrowed(value)
    }
}

/// Defines the background of an item.
/// [CSS background](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Backgrounds_and_Borders)
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
pub struct Background<'p> {
    /// The background image
    pub image: Image<'p>,
    /// The color of the background
    pub color: HexColor,
    ///  The background-clip CSS property sets whether an element's background extends underneath its border box, padding box, or content box.
    /// [CSS background-clip](https://developer.mozilla.org/en-US/docs/Web/CSS/background-clip)
    pub clip: BoxClip,
    /// The background-origin CSS property sets the background's origin: from the border start, inside the border, or inside the padding.
    /// [CSS background-origin](https://developer.mozilla.org/en-US/docs/Web/CSS/background-origin)
    pub origin: BoxClip,
    /// The background-position CSS property sets the initial position for each background image.
    /// The position is relative to the position layer set by background-origin
    pub position: AlignPosition,
    /// How images with be repeated
    pub repeat: BackgroundRepeat,
    /// The background-attachment CSS property sets whether a background image's position is fixed within the viewport, or scrolls with its containing block.
    pub attachment: BackgroundAttachment,
}

impl<'p> Background<'p> {
    pub fn debug_css(&self) -> Cow<str> {
        let background_image =
            Cow::Borrowed("background-image: ") + Cow::Owned(format!("{}", self.image)) + ";";

        let background_color =
            Cow::Borrowed("background-color: ") + Cow::Owned(self.color.to_string()) + ";";
        let background_clip = Cow::Borrowed("background-clip: ") + self.clip.to_css() + ";";
        let background_origin = Cow::Borrowed("background-origin: ") + self.origin.to_css() + ";";
        let background_position =
            Cow::Borrowed("background-position: ") + self.position.to_css() + ";";
        let background_repeat = Cow::Borrowed("background-repeat: ") + self.repeat.to_css();
        let background_attachment =
            Cow::Borrowed("background-attachment: ") + self.attachment.to_css();

        background_image
            + " "
            + background_color
            + " "
            + background_clip
            + " "
            + background_origin
            + " "
            + background_position
            + " "
            + background_repeat
            + " "
            + background_attachment
    }
}

impl<'p> StyleDeclaration for Background<'p> {
    fn to_css(&self) -> Cow<str> {
        let background_image = Cow::Borrowed("background-image: ") + self.image.to_css() + ";";
        let background_color =
            Cow::Borrowed("background-color: ") + Cow::Owned(self.color.to_string()) + ";";
        let background_clip = Cow::Borrowed("background-clip: ") + self.clip.to_css() + ";";
        let background_origin = Cow::Borrowed("background-origin: ") + self.origin.to_css() + ";";
        let background_position =
            Cow::Borrowed("background-position: ") + self.position.to_css() + ";";
        let background_repeat = Cow::Borrowed("background-repeat: ") + self.repeat.to_css();
        let background_attachment =
            Cow::Borrowed("background-attachment: ") + self.attachment.to_css();

        background_image
            + " "
            + background_color
            + " "
            + background_clip
            + " "
            + background_origin
            + " "
            + background_position
            + " "
            + background_repeat
            + " "
            + background_attachment
    }
}

/// Alignment of an item to `top`, `bottom`, `left` or `right`
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
pub enum AlignPosition {
    /// Align to the `top`
    Top,
    /// Align to the `bottom`
    Bottom,
    /// Align to the `center`
    #[default]
    Center,
    /// Align to the `left`
    Left,
    /// ALign to the `right`
    Right,
}

impl StyleDeclaration for AlignPosition {
    fn to_css(&self) -> Cow<str> {
        let value = match self {
            Self::Top => "top",
            Self::Bottom => "bottom",
            Self::Left => "left",
            Self::Right => "right",
            Self::Center => "center",
        };

        value.into()
    }
}

/// How the background images are repeated
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
pub enum BackgroundRepeat {
    /// Repeat to the horizontal axis
    X,
    /// Repeat to the vertical axis
    Y,
    /// Do not repeat the background
    #[default]
    NoRepeat,
}

impl StyleDeclaration for BackgroundRepeat {
    fn to_css(&self) -> Cow<str> {
        let value = match self {
            Self::X => "x",
            Self::Y => "y",
            Self::NoRepeat => "no-repeat",
        };

        Cow::Borrowed("background-repeat: ") + value + ";"
    }
}

///  The background-size CSS property sets the size of the element's background image.
/// The image can be left to its natural size, stretched, or constrained to fit the available space.
/// [CSS backgroud-size](https://developer.mozilla.org/en-US/docs/Web/CSS/background-size)
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
pub enum BackgroundSize {
    /// Scales the background image in the corresponding direction such that its intrinsic proportions are maintained.
    Auto,
    ///  Scales the image (while preserving its ratio) to the smallest possible size to fill the container
    /// (that is: both its height and width completely cover the container), leaving no empty space.
    /// If the proportions of the background differ from the element, the image is cropped either vertically or horizontally.
    #[default]
    Cover,
    ///  Scales the image as large as possible within its container without cropping or stretching the image.
    /// If the container is larger than the image, this will result in image tiling,
    /// unless the background-repeat property is set to no-repeat.
    Contain,
}

impl StyleDeclaration for BackgroundSize {
    fn to_css(&self) -> Cow<str> {
        let value = match self {
            Self::Auto => "auto",
            Self::Contain => "contain",
            Self::Cover => "cover",
        };

        Cow::Borrowed("background-size: ") + value + ";"
    }
}

/// The background-attachment CSS property sets whether a background image's position is fixed within the viewport, or scrolls with its containing block.
/// [CSS background-attachment](https://developer.mozilla.org/en-US/docs/Web/CSS/background-attachment)
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
pub enum BackgroundAttachment {
    /// The background is fixed relative to the viewport. Even if an element has a scrolling mechanism, the background doesn't move with the element.
    /// (This is not compatible with background-clip: text.)
    Fixed,
    /// The background is fixed relative to the element itself and does not scroll with its contents.
    /// (It is effectively attached to the element's border.)
    #[default]
    Scroll,
    /// The background is fixed relative to the element's contents.
    /// If the element has a scrolling mechanism, the background scrolls with the element's contents,
    /// and the background painting area and background positioning area are relative to the scrollable area
    /// of the element rather than to the border framing them.
    Local,
}

impl StyleDeclaration for BackgroundAttachment {
    fn to_css(&self) -> Cow<str> {
        let value = match self {
            Self::Fixed => "fixed",
            Self::Scroll => "scroll",
            Self::Local => "local",
        };

        Cow::Borrowed("background-attachment: ") + value + ";"
    }
}

/// Defines the border box, padding box, or content box.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
pub enum BoxClip {
    /// 'border-box` is a value for the box-sizing property in CSS that tells the browser to include the padding and border in an element's total width and height
    #[default]
    BorderBox,
    /// In CSS, the padding box is the area between the content box and the border of an element
    PaddingBox,
    /// In CSS, the content box is the area inside an element's padding and border where the actual content, such as text and images, is displayed
    ContentBox,
}

impl StyleDeclaration for BoxClip {
    fn to_css(&self) -> Cow<str> {
        let value = match self {
            Self::BorderBox => "border-box",
            Self::PaddingBox => "padding-box",
            Self::ContentBox => "content-box",
        };

        value.into()
    }
}

/// The visibility CSS property shows or hides an element without changing the layout of a document.
/// The property can also hide rows or columns in a <table>.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
pub enum Visibility {
    /// The element box is visible.
    #[default]
    Visible,
    /// The element box is invisible (not drawn), but still affects layout as normal.
    /// Descendants of the element will be visible if they have visibility set to visible.
    /// The element cannot receive focus (such as when navigating through tab indexes).
    Hidden,
    /// The collapse keyword has different effects for different elements:
    ///
    /// - For <table> rows, columns, column groups, and row groups, the row(s) or column(s) are hidden and the space
    /// they would have occupied is removed (as if display: none were applied to the column/row of the table).
    /// However, the size of other rows and columns is still calculated as though the cells in the collapsed row(s) or column(s) are present.
    /// This value allows for the fast removal of a row or column from a table without forcing the recalculation of widths and heights for the entire table.
    /// - Collapsed flex items and ruby annotations are hidden, and the space they would have occupied is removed.
    /// - For other elements, collapse is treated the same as hidden.
    Collapse,
}

impl StyleDeclaration for Visibility {
    fn to_css(&self) -> Cow<str> {
        let value = match self {
            Self::Visible => "visible",
            Self::Hidden => "hidden",
            Self::Collapse => "collapse",
        };

        Cow::Borrowed("visibility: ") + value + ";"
    }
}

/// The position CSS property sets how an element is positioned in a document.
/// The top, right, bottom, and left properties determine the final location of positioned elements.
/// [CSS position](https://developer.mozilla.org/en-US/docs/Web/CSS/position)
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
pub enum Position {
    /// The element is positioned according to the normal flow of the document.
    /// The top, right, bottom, left, and z-index properties have no effect. This is the default value.
    Static,
    /// The element is positioned according to the normal flow of the document,
    /// and then offset relative to itself based on the values of top, right, bottom, and left.
    /// The offset does not affect the position of any other elements; thus, the space given for the element
    /// in the page layout is the same as if position were static. This value creates a new stacking context
    /// when the value of z-index is not auto. Its effect on table-*-group, table-row, table-column, table-cell, and table-caption elements is undefined.
    #[default]
    Relative,
    /// The element is removed from the normal document flow, and no space is created for the element in the page layout.
    /// It is positioned relative to its closest positioned ancestor, if any; otherwise, it is placed relative to the initial containing block.
    /// Its final position is determined by the values of top, right, bottom, and left.
    /// This value creates a new stacking context when the value of z-index is not auto.
    /// The margins of absolutely positioned boxes do not collapse with other margins.
    Absolute,
    /// The element is removed from the normal document flow, and no space is created for the element in the page layout.
    /// It is positioned relative to the initial containing block established by the viewport,
    /// except when one of its ancestors has a transform, perspective, or filter property set to something other than none
    /// (see the CSS Transforms Spec), or the will-change property is set to transform,
    ///  in which case that ancestor behaves as the containing block. (Note that there are browser
    /// inconsistencies with perspective and filter contributing to containing block formation.)
    /// Its final position is determined by the values of top, right, bottom, and left.
    /// This value always creates a new stacking context. In printed documents, the element is placed in the same position on every page.
    Fixed,
    /// The element is positioned according to the normal flow of the document, and then offset relative to its nearest scrolling ancestor
    /// and containing block (nearest block-level ancestor), including table-related elements, based on the values of top, right, bottom, and left.
    /// The offset does not affect the position of any other elements.
    /// This value always creates a new stacking context.
    /// Note that a sticky element "sticks" to its nearest ancestor that
    /// has a "scrolling mechanism" (created when overflow is hidden, scroll, auto, or overlay),
    /// even if that ancestor isn't the nearest actually scrolling ancestor.
    Sticky,
}

impl StyleDeclaration for Position {
    fn to_css(&self) -> Cow<str> {
        let value = match self {
            Self::Static => "static",
            Self::Relative => "relative",
            Self::Absolute => "absolute",
            Self::Sticky => "sticky",
            Self::Fixed => "fixed",
        };

        Cow::Borrowed("position: ") + value + ";"
    }
}

/// The overflow CSS shorthand property sets the desired behavior when content does not fit
/// in the parent element box (overflows) in the horizontal and/or vertical direction.
/// [CSS overflow](https://developer.mozilla.org/en-US/docs/Web/CSS/overflow)
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
pub enum Overflow {
    /// Overflow content is clipped at the element's padding box, and overflow content can be scrolled into view.
    /// Unlike scroll, user agents display scroll bars only if the content is overflowing and hide scroll bars by default.
    /// If content fits inside the element's padding box, it looks the same as with visible but still establishes a new formatting context.
    /// The element box is a scroll container.
    Auto,
    /// Overflow content is clipped at the element's padding box.
    /// There are no scroll bars, and the clipped content is not visible (i.e., clipped content is hidden),
    /// but the content still exists. User agents do not add scroll bars and also do not allow users to view
    /// the content outside the clipped region by actions such as dragging on a touch screen or using the scroll wheel on a mouse.
    /// The content can be scrolled programmatically (for example, by setting the value of the scrollLeft property or the scrollTo() method),
    /// in which case. The element box is a scroll container.
    Hidden,
    /// Overflow content is clipped at the element's padding box, and overflow content can be scrolled into view using scroll bars.
    /// User agents display scroll bars in both horizontal and vertical directions if only one value is set, whether or not any content is overflowing or clipped.
    /// The use of this keyword, therefore, can prevent scroll bars from appearing and disappearing as content changes.
    /// Printers may still print overflowing content. The element box is a scroll container.
    Scroll,
    /// Overflow content is not clipped and may be visible outside the element's padding box.
    /// The element box is not a scroll container. This is the default value of the overflow property.
    #[default]
    Visible,
    /// Overflow content is clipped at the element's overflow clip edge that is defined using the overflow-clip-margin property.
    /// As a result, content overflows the element's padding box by the <length> value of overflow-clip-margin or by 0px if not set.
    /// Overflow content outside the clipped region is not visible, user agents do not add a scroll bar, and programmatic scrolling is also not supported.
    /// No new formatting context is created.
    /// To establish a formatting context, use overflow: clip along with display: flow-root. The element box is not a scroll container.
    Clip,
}

impl StyleDeclaration for Overflow {
    fn to_css(&self) -> Cow<str> {
        let value = match self {
            Self::Auto => "auto",
            Self::Hidden => "hidden",
            Self::Scroll => "scroll",
            Self::Visible => "visible",
            Self::Clip => "clip",
        };

        Cow::Borrowed("overflow: ") + value + ";"
    }
}

/// The word-break CSS property sets whether line breaks appear wherever the text would otherwise overflow its content box.
/// [CSS word-break](https://developer.mozilla.org/en-US/docs/Web/CSS/word-break)
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
pub enum WordBreak {
    /// Use the default line break rule.
    #[default]
    Normal,
    /// To prevent overflow, word breaks should be inserted between any two characters (excluding Chinese/Japanese/Korean text).
    BreakAll,
    /// Word breaks should not be used for Chinese/Japanese/Korean (CJK) text. Non-CJK text behavior is the same as for normal.
    KeepAll,
    /// Has the same effect as word-break: normal and overflow-wrap: anywhere, regardless of the actual value of the overflow-wrap property.
    BreakWord,
}

impl StyleDeclaration for WordBreak {
    fn to_css(&self) -> Cow<str> {
        let value = match self {
            Self::Normal => "normal",
            Self::BreakAll => "break-all",
            Self::KeepAll => "keep-all",
            Self::BreakWord => "break-word",
        };

        Cow::Borrowed("word-break: ") + value + ";"
    }
}

/// The white-space CSS property sets how white space inside an element is handled.
/// [CSS white-space property](https://developer.mozilla.org/en-US/docs/Web/CSS/white-space)
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
pub enum WhiteSpace {
    /// Sequences of white space are collapsed. Newline characters in the source are handled the same as other white space.
    /// Lines are broken as necessary to fill line boxes.
    #[default]
    Normal,
    /// Sequences of white space are preserved. Lines are only broken at newline characters in the source and at <br> elements.
    Pre,
    /// Collapses white space as for normal, but suppresses line breaks (text wrapping) within the source.
    NoWrap,
    /// Sequences of white space are collapsed. Lines are broken at newline characters, at <br>, and as necessary to fill line boxes.
    PreLine,
    /// Sequences of white space are preserved. Lines are broken at newline characters, at <br>, and as necessary to fill line boxes.
    PreWrap,
    /// The behavior is identical to that of pre-wrap, except that:
    /// - Any sequence of preserved white space always takes up space, including at the end of the line.
    /// - A line breaking opportunity exists after every preserved white space character, including between white space characters.
    /// - Such preserved spaces take up space and do not hang, and thus affect the box's intrinsic sizes (min-content size and max-content size).
    BreakSpaces,
}

impl StyleDeclaration for WhiteSpace {
    fn to_css(&self) -> Cow<str> {
        let value = match self {
            Self::Normal => "normal",
            Self::Pre => "pre",
            Self::NoWrap => "no-wrap",
            Self::PreLine => "pre-line",
            Self::PreWrap => "pre-wrap",
            Self::BreakSpaces => "break-spaces",
        };

        Cow::Borrowed("white-space: ") + value + ";"
    }
}

/// Contains the `x` and `y` coordinates on a screen
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Coordinates {
    /// The `x` coordinate
    pub x: Float,
    /// The `y` coordinate
    pub y: Float,
}

/// A float type that implements `Eq` and `Ord` so that it can be compared to or sorted inside a `Vec`
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Default, Clone, Copy)]
pub struct Float {
    /// Is it a signed (`true`) or unsigned (`false`) float
    pub signed: bool,
    /// The integral part of the float
    pub integral: usize,
    /// The fractional part of the float
    pub fractional: usize,
}

impl Float {
    /// Parses an f64 value into  a `Float` and if parsing fails it returns `Float::default()`
    pub fn parse(value: &str) -> Float {
        match value.parse::<f64>() {
            Ok(value) => {
                let int_part = value.floor() as usize;

                let frac_part = ((value - int_part as f64) * 10.0).round() / 10.0;

                Float {
                    signed: value.is_sign_negative(),
                    integral: value.floor() as usize,
                    fractional: (frac_part * 10f64) as usize,
                }
            }
            Err(_) => Float::default(), // TODO find if handling this error is necessary
        }
    }

    /// Convert `Float` into a string using clone on write
    pub fn to_string(&self) -> Cow<str> {
        if self.signed {
            Cow::Borrowed("-")
                + Cow::Owned(self.integral.to_string())
                + "."
                + Cow::Owned(self.fractional.to_string())
        } else {
            Cow::Owned(self.integral.to_string()) + "." + Cow::Owned(self.fractional.to_string())
        }
    }

    /// Convert `Float` into a string using clone on write without the fractional part
    pub fn to_string_integral(&self) -> Cow<str> {
        if self.signed {
            Cow::Borrowed("-") + Cow::Owned(self.integral.to_string())
        } else {
            Cow::Owned(self.integral.to_string())
        }
    }

    /// Convert `Float` into a string using clone on write without the integral part without the sign
    pub fn to_string_fractional(&self) -> Cow<str> {
        Cow::Owned(self.fractional.to_string())
    }

    /// The sign value `Float` into a string
    pub fn to_gloat_sign(&self) -> Cow<str> {
        if self.signed {
            Cow::Borrowed("-")
        } else {
            Cow::Borrowed("+")
        }
    }
}
