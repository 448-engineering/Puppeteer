//!
//!
//!
//!
//!
//!
//! The CSS documentation is from the Mozilla Developer Network Web Docs by the Mozilla foundation
//! and can be found at [https://developer.mozilla.org/en-US/docs/Web/CSS](https://developer.mozilla.org/en-US/docs/Web/CSS)
//!
//!
mod errors;
pub use errors::*;

mod app;
pub use app::*;

mod traits;
pub use traits::*;

mod helpers;
pub use helpers::*;

mod ui_builder;
pub use ui_builder::*;

#[cfg(test)]
mod sanity_tests {
    use crate::{
        Background, Border, BoxShadow, Container, Filter, FlexAlignment, Heading, StyleDeclaration,
        Styling, UiPaint,
    };
    use hex_color::HexColor;

    #[test]
    fn alignment() {
        let alignment = FlexAlignment::default();

        assert_eq!(
        "align-content: center; align-items: center; align-self: center; justify-content: center; justify-items: center; justify-self: center;",
        alignment.to_css()
         );
    }

    #[test]
    fn background() {
        let background = Background::default();

        assert_eq!(
            "background-image: [IMAGE TYPE => svg - Blake3 Hash => 8ab40f26abfe2ac3f41c147c2c8a675bfd786ac2381afdebbfa84ede96c34ac7]; background-color: #000000; background-clip: border-box; background-origin: border-box; background-position: center; background-repeat: background-repeat: no-repeat; background-attachment: background-attachment: scroll;",
        background.debug_css()
         );

        assert_eq!("background-image: data:image/svg+xml;base64, PHN2ZyB2aWV3Qm94PScwIDAgMTAwIDEwMCcgeG1sbnM9J2h0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnJz48cGF0aCBkPSdtMCAwaDEwMHYxMDBoLTEwMHonIGZpbGw9JyNmMDAnIHN0cm9rZS13aWR0aD0nLjgyOTY5NScvPjxwYXRoIGQ9J20xMy43Nzg4NTEgMTMuNzc4ODVoNzIuNDQyMjk5djcyLjQ0MjI5OWgtNzIuNDQyMjk5eicgZmlsbD0nIzBmMCcgc3Ryb2tlLXdpZHRoPScuNjAxMDUnLz48cGF0aCBkPSdtMjguMDcwMTU4IDI4LjA3MDE2aDQzLjg1OTY4NHY0My44NTk2ODRoLTQzLjg1OTY4NHonIGZpbGw9JyMwMGYnIHN0cm9rZS13aWR0aD0nLjM2MzkwMicvPjxwYXRoIGQ9J200MC4zMDc5MjIgNDAuMzA3OTJoMTkuMzg0MTU3djE5LjM4NDE1N2gtMTkuMzg0MTU3eicgZmlsbD0nI2ZmZicgc3Ryb2tlLXdpZHRoPScuMTYwODMnLz48L3N2Zz4=; background-color: #000000; background-clip: border-box; background-origin: border-box; background-position: center; background-repeat: background-repeat: no-repeat; background-attachment: background-attachment: scroll;",
        background.to_css());
    }

    #[test]
    fn border() {
        let border = Border::new();

        let outcome = "border-top: 0px solid #000000; border-bottom: 0px solid #000000; border-left: 0px solid #000000; border-right: 0px solid #000000;";

        assert_eq!(outcome, border.to_css());
    }

    #[test]
    fn filters() {
        let brightness = Filter::default();
        let outcome = "filter: brightness(100%);";
        assert_eq!(outcome, brightness.to_css());

        let blur = Filter::Blur(120);
        let outcome = "filter: blur(120px);";
        assert_eq!(outcome, blur.to_css());

        let contrast = Filter::Contrast(255);
        let outcome = "filter: contrast(255%);";
        assert_eq!(outcome, contrast.to_css());

        let drop_shadow = Filter::DropShadow(255, 255, 0, HexColor::default());
        let outcome = "filter: drop-shadow(255px 255px 0px #000000);";
        assert_eq!(outcome, drop_shadow.to_css());

        let grayscale = Filter::GrayScale(255);
        let outcome = "filter: grayscale(100%);";
        assert_eq!(outcome, grayscale.to_css());

        let hue_rotation = Filter::HueRotation(400);
        let outcome = "filter: hue-rotate(360deg);";
        assert_eq!(outcome, hue_rotation.to_css());

        let invert = Filter::Invert(255);
        let outcome = "filter: invert(100%);";
        assert_eq!(outcome, invert.to_css());

        let opacity = Filter::Opacity(255);
        let outcome = "filter: opacity(100%);";
        assert_eq!(outcome, opacity.to_css());

        let sephia = Filter::Sephia(255);
        let outcome = "filter: sephia(100%);";
        assert_eq!(outcome, sephia.to_css());

        let saturate = Filter::Saturate(300);
        let outcome = "filter: saturate(300%);";
        assert_eq!(outcome, saturate.to_css());
    }

    #[test]
    fn box_shadow() {
        let box_shadow = BoxShadow::default();
        let outcome = "box-shadow:  0px 0px 0px 0px #000000";
        assert_eq!(outcome, box_shadow.to_css());

        let mut box_shadow = BoxShadow::default();
        box_shadow.inset = true;
        let outcome = "box-shadow: inset 0px 0px 0px 0px #000000";
        assert_eq!(outcome, box_shadow.to_css());
    }

    #[test]
    fn styling() {
        let style = Styling::default();
        let outcome = "max-width: 100%; max-height: none; min-width: none; min-height: none; width: auto; height: auto; flex-direction: row; align-content: center; align-items: center; align-self: center; justify-content: center; justify-items: center; justify-self: center; flex-wrap: wrap;  z-index: 0; flex-grow: 1; flex-shrink: 1; border-top: 0px solid #000000; border-bottom: 0px solid #000000; border-left: 0px solid #000000; border-right: 0px solid #000000; border-top-left-radius: 0px; border-top-right-radius: 0px; border-bottom-left-radius: 0px; border-bottom-right-radius: 0px; padding-top: 0px; padding-bottom: 0px; padding-left: 0px; padding-right: 0px; margin-top: 0px; margin-bottom: 0px; margin-left: 0px; margin-right: 0px; transform: rotate(0deg); opacity: 1; overflow: visible; position: relative; visibility: visible; white-space: normal; word-break: normal; word-spacing: normal; line-height: normal; ";
        assert_eq!(outcome, style.to_css());
    }

    #[test]
    fn container() {
        let h1 = Heading::new("This is my first heading");

        let container = Container::new()
            .add_class("foo_class_1")
            .add_id("foo_id")
            .add_class("foo_class_2")
            .add_class("foo_class_3")
            .add_classes(&["foo_class_4", "foo_class_5"])
            .add_child(h1);

        assert_eq!("<div id=\"foo_id\" class=\" foo_class_1 foo_class_2 foo_class_3 foo_class_4 foo_class_5 \"><h1>This is my first heading</h1></div>", &container.to_html());
    }
}
