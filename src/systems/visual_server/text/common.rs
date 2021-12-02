use crate::drawing::color::Color;

use super::font::Font;

#[derive(Copy, Clone, Default)]
pub struct FontSpacing {
    pub h_advance: f32,
    pub descent: f32,
    pub ascent: f32,
}

#[derive(Clone, Default)]
pub struct TextStyle {
    pub font: Font,
    pub font_size: i32,
    pub color: Color
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HorizontalAlign {
    /// Leftmost character is immediately to the right of the render position.<br/>
    /// Bounds start from the render position and advance rightwards.
    Left,
    /// Leftmost & rightmost characters are equidistant to the render position.<br/>
    /// Bounds start from the render position and advance equally left & right.
    Center,
    /// Rightmost character is immetiately to the left of the render position.<br/>
    /// Bounds start from the render position and advance leftwards.
    Right,
}

#[derive(Clone)]
pub struct TextAlignment {
    pub horizontal: HorizontalAlign,
}

impl Default for TextAlignment {
    fn default() -> Self {
        TextAlignment {
            horizontal: HorizontalAlign::Left,
        }
    }
}

#[derive(Clone, Default)]
pub struct Text {
    pub value: String,
    pub style: TextStyle,
    pub alignment: TextAlignment,
}

impl Text {
    pub fn new(text: &str, style: TextStyle, alignment: TextAlignment) -> Self {
        Text {
            value: text.to_string(),
            style: style,
            alignment,
        }
    }
}
