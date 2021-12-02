use gdnative::{api::VisualServer, core_types::{Rid, Rect2, Point2, Vector2, Color}};

use super::common::Text;

struct CharDrawing {
    rect: Rect2,
    current: char,
}

pub fn render_text(visual_server: &VisualServer, canvas_item: Rid, text: &mut Text) {
    gdnative::godot_print!("font draw on rid: {}", canvas_item.get_id());

    let characters: Vec<char> = text.value.chars().collect();
    let mut drawing: Vec<CharDrawing> = Vec::new();
    let mut cursor: Point2 = Point2::new(0.0, 0.0);
    let mut dimension: Vector2 = Vector2::new(0.0, 0.0);

    for current_char in characters.iter() {
        let char_rect = text.style.font.get_char_rect(*current_char, text.style.font_size);
        let spacing = text.style.font.get_spacing(*current_char, text.style.font_size);
        let position = Point2::new(
            cursor.x + char_rect.origin.x,
            cursor.y + char_rect.origin.y,
        );
        drawing.push(CharDrawing {
            rect: Rect2::new(
                position,
                char_rect.size
            ),
            current: *current_char,
        });
        cursor.x += char_rect.size.width + spacing.h_advance;
        dimension.y = f32::max(char_rect.size.height, dimension.y);
        dimension.x = cursor.x;
    }

    let offset = Vector2::new(100.0, 100.0); // TODO: Get property

    for character in drawing.iter() {
        let texture = text.style.font.get_texture(character.current, text.style.font_size);

        if let Some(texture) = texture {
            visual_server.canvas_item_add_texture_rect(
                canvas_item,
                character.rect,
                texture.rid,
                false,
                Color::rgba(1.0, 1.0, 1.0, 1.0),
                false,
                Rid::new(),
            );
        }
    }
}
