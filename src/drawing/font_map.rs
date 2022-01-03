use std::collections::HashMap;

use bevy_ecs::prelude::Entity;
use bevy_transform::hierarchy::ChildBuilder;
use gdnative::core_types::{Vector2, Transform2D};

use crate::{systems::visual_server::text::{text_plugin::TextBundle, common::{Text, TextStyle, TextAlignment, HorizontalAlign}}, core::constants::TEXT_Z_INDEX};

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
        commands: &mut ChildBuilder,
        print_data: PrintData,
        location: Vector2,
        text: &str
    ) -> Option<Entity> {
        let font_option = self.map.get(&(print_data.index as usize));
        if font_option.is_none() {
            return None;
        }
        let font = font_option.unwrap().get_color_bank(print_data.colorindex as usize);
        let entity = commands.spawn_bundle(TextBundle {
            text: Text::new(
                text,
                TextStyle {
                    font,
                    font_size: 1,
                    ..Default::default()
                },
                TextAlignment {
                    // horizontal: print_data.justification.into(),
                    horizontal: HorizontalAlign::Right,
                    ..Default::default()
                },
            ),
            z_index: TEXT_Z_INDEX.into(),
            transform: Transform2D::translation(location.x, location.y),
            ..Default::default()
        }).id();

        gdnative::godot_print!("text position: {:?}", location);

        Some(entity)
    }
}
