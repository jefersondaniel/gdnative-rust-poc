use gdnative::{api::VisualServer, core_types::{Rid, Rect2, Point2, Vector2, Color, Size2}};

use crate::systems::visual_server::text::common::HorizontalAlign;

use super::common::Text;

struct CharDrawing {
    rect: Rect2,
    current: char,
}

#[derive(Default)]
struct LineDrawing {
    items: Vec<CharDrawing>,
}

impl LineDrawing {
    fn compute_bounds(&self) -> Rect2 {
        let mut min_x: f32 = f32::MAX;
        let mut min_y: f32 = f32::MAX;
        let mut max_x: f32 = f32::MIN;
        let mut max_y: f32 = f32::MIN;

        for item in self.items.iter() {
            min_x = min_x.min(item.rect.origin.x);
            min_y = min_y.min(item.rect.origin.y);
            max_x = max_x.max(item.rect.origin.x + item.rect.size.width);
            max_y = max_y.min(item.rect.origin.y + item.rect.size.height);
        }

        Rect2::new(Point2::new(min_x, min_y), Size2::new(max_x - min_x, max_y - min_y))
    }
}


pub fn render_text(visual_server: &VisualServer, canvas_item: Rid, text: &mut Text) {
    gdnative::godot_print!("font draw on rid: {}", canvas_item.get_id());

    let characters: Vec<char> = text.value.chars().collect();
    let mut drawing: Vec<LineDrawing> = Vec::new();
    let mut current_line = LineDrawing::default();
    let mut cursor: Point2 = Point2::new(0.0, 0.0);
    let mut dimension: Vector2 = Vector2::new(0.0, 0.0);
    let font = &text.style.font;
    let font_size = text.style.font_size;
    let font_spacing = font.get_font_spacing(font_size);
    let mut previous_char: Option<char> = None;

    for current_char in characters.iter() {
        if current_char.eq(&'\n') {
            cursor = Point2::new(0.0, cursor.y + font_spacing.height + font_spacing.line_gap);
            drawing.push(current_line);
            current_line = LineDrawing::default();
            continue
        }

        let char_rect = font.get_char_rect(*current_char, font_size);
        let glyph_spacing = font.get_glyph_spacing(previous_char, *current_char, font_size);

        let position = Point2::new(
            cursor.x + char_rect.origin.x,
            cursor.y + char_rect.origin.y
        );
        current_line.items.push(CharDrawing {
            rect: Rect2::new(
                position,
                char_rect.size
            ),
            current: *current_char,
        });
        cursor.x += glyph_spacing.h_advance;
        dimension.y = f32::max(char_rect.size.height, dimension.y);
        dimension.x = cursor.x;
        previous_char = Some(*current_char);
    }

    drawing.push(current_line);

    for line in drawing.iter() {
        let offset: Vector2 = match text.alignment.horizontal {
            HorizontalAlign::Center => {
                let bounds = line.compute_bounds();
                Vector2::new(bounds.size.width / -2.0, 0.0)
            }
            HorizontalAlign::Right => {
                let bounds = line.compute_bounds();
                Vector2::new(-bounds.size.width, 0.0)
            }
            HorizontalAlign::Left => {
                Vector2::new(0.0, 0.0)
            }
        };

        for character in line.items.iter() {
            let texture = text.style.font.get_texture(character.current, text.style.font_size);

            if let Some(texture) = texture {
                visual_server.canvas_item_add_texture_rect(
                    canvas_item,
                    Rect2::new(character.rect.origin + offset, character.rect.size),
                    texture.rid,
                    false,
                    Color::rgba(1.0, 1.0, 1.0, 1.0),
                    false,
                    Rid::new(),
                );
            }
        }
    }
}
