use std::collections::HashMap;

use bevy_ecs::prelude::{Entity, Commands, Mut};
use gdnative::core_types::{Vector2, Transform2D, Color as GodotColor};

use crate::{systems::visual_server::{text::{text_plugin::TextBundle, common::{Text, TextStyle, TextAlignment, HorizontalAlign}}, canvas_item::ClipRect}, core::constants::TEXT_Z_INDEX};

use super::{mugen_font::MugenFont, print_data::PrintData};

#[derive(Clone)]
pub struct FontMap {
    map: HashMap<usize, MugenFont>,
}

impl FontMap {
    pub fn new(map: HashMap<usize, MugenFont>) -> Self {
        FontMap { map }
    }

    pub fn insert_font(
        &self,
        commands: &mut Commands,
        print_data: PrintData,
        location: Vector2,
        clip_rect: ClipRect,
        text: &str
    ) -> Option<Entity> {
        let font_option = self.map.get(&(print_data.index as usize));
        if font_option.is_none() {
            return None;
        }
        let font = font_option.unwrap().get_color_bank(print_data.colorindex as usize);
        let color = match print_data.color {
            Some(color) => GodotColor::from(color),
            None => GodotColor::rgba(1.0, 1.0, 1.0, 1.0),
        };
        let entity = commands.spawn_bundle(TextBundle {
            text: Text::new(
                text,
                TextStyle {
                    font,
                    font_size: 1,
                    color,
                    ..Default::default()
                },
                TextAlignment {
                    horizontal: print_data.justification.into(),
                    ..Default::default()
                },
            ),
            z_index: TEXT_Z_INDEX.into(),
            clip_rect,
            transform: Transform2D::translation(location.x, location.y),
            ..Default::default()
        }).id();

        Some(entity)
    }

    pub fn update_font(
        &self,
        print_data: PrintData,
        mut text: Mut<Text>
    ) {
        let font_option = self.map.get(&(print_data.index as usize));
        if font_option.is_none() {
            return;
        }
        let font = font_option.unwrap().get_color_bank(print_data.colorindex as usize);

        text.style.font = font;
        text.alignment = TextAlignment {
            horizontal: print_data.justification.into(),
            ..Default::default()
        };
        text.style.color = match print_data.color {
            Some(color) => GodotColor::from(color),
            None => GodotColor::rgba(1.0, 1.0, 1.0, 1.0),
        };
    }
}
