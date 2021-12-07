use gdnative::core_types::Color;

use super::font::Font;

#[derive(Debug, Copy, Clone, Default)]
pub struct GlyphSpacing {
    pub h_advance: f32,
    pub h_side_bearing: f32,
    pub kern: f32,
}

#[derive(Debug, Copy, Clone, Default)]
pub struct FontSpacing {
    pub descent: f32,
    pub ascent: f32,
    pub line_gap: f32,
    pub height: f32
}

pub struct TextStyle {
    pub font: Font,
    pub font_size: i32,
    pub color: Color
}

impl Default for TextStyle {
    fn default() -> Self {
        Self {
            font: Font::default(),
            font_size: 14,
            color: Color::rgba(1.0, 1.0, 1.0, 1.0)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HorizontalAlign {
    Left,
    Center,
    Right,
}

impl Default for HorizontalAlign {
    fn default() -> Self {
        Self::Left
    }
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

#[derive(Default)]
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
