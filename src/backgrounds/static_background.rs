use bevy_ecs::{prelude::*};
use bevy_transform::hierarchy::ChildBuilder;
use std::{sync::Arc};
use gdnative::{api::{visual_server::{TextureFlags, PrimitiveType}, SurfaceTool}, core_types::{Point2, Vector2, Vector3, Transform2D}};

use crate::{core::{configuration::Configuration, error::DataError, sprite_id::SpriteId, enumerations::BackgroundLayer, constants::{BG_LAYER_BACK_Z_INDEX_MAX, BG_LAYER_FRONT_Z_INDEX_MAX}}, drawing::{sprite_file::SpriteFile}, io::text_section::TextSection, systems::{visual_server::{sprite::Sprite, texture::Texture, mesh_2d::{Mesh2dBundle, Mesh2d}, canvas_item::{ClipRect, ZIndex, BackBufferCopy}, material::Material}}};

use super::base_background::BaseBackground;

#[derive(Clone)]
pub struct StaticBackground {
    pub base_background: BaseBackground,
    pub spriteid: SpriteId,
    pub texture: Arc<Texture>,
    pub sprite: Sprite,
}

impl StaticBackground {
    pub fn build(
        configuration: &Configuration,
        textsection: &TextSection,
        sprite_file: &mut SpriteFile
    ) -> Result<Self, DataError> {
        let spriteid = textsection.get_attribute_or("spriteno", SpriteId::invalid());
        let sff_data = sprite_file.get_sprite(&spriteid)?;
        let texture = sff_data.create_texture(None, TextureFlags(0))?;
        let sprite = Sprite {
            offset: Point2::new(sff_data.x as f32, sff_data.y as f32),
            size: texture.size,
            ..Default::default()
        };

        Ok(StaticBackground {
            base_background: BaseBackground::build(configuration, textsection)?,
            spriteid,
            texture,
            sprite,
        })
    }

    pub fn render(&self, commands: &mut ChildBuilder, configuration: &Res<Configuration>, z_index: i32) -> Entity {
        let st = SurfaceTool::new();
        let size = self.sprite.size;
        let (tilestart, tileend) = self.base_background.get_tile_length(self.sprite.size, &configuration);
        let tilingspacing = self.base_background.tilingspacing;

        st.begin(PrimitiveType::TRIANGLES.0);

        for y in (tilestart.y as i32)..(tileend.y as i32) {
            for x in (tilestart.x as i32)..(tileend.x as i32) {
                let adjustment = (Vector2::new(size.width, size.height) + tilingspacing).component_mul(Vector2::new(x as f32, y as f32));
                let location = self.base_background.startlocation
                    + adjustment
                    - Vector2::new(self.sprite.offset.x, self.sprite.offset.y);

                // First Triangle
                st.add_uv(Vector2::new(0.0, 0.0)); // Top Left
                st.add_vertex(Vector3::new(location.x, location.y, 0.0));
                st.add_uv(Vector2::new(1.0, 0.0)); // Top Right
                st.add_vertex(Vector3::new(location.x + size.width, location.y, 0.0));
                st.add_uv(Vector2::new(1.0, 1.0)); // Bottom Right
                st.add_vertex(Vector3::new(location.x + size.width, location.y + size.height, 0.0));
                // Second Triangle
                st.add_uv(Vector2::new(0.0, 0.0)); // Top Left
                st.add_vertex(Vector3::new(location.x, location.y, 0.0));
                st.add_uv(Vector2::new(0.0, 1.0)); // Bottom Left
                st.add_vertex(Vector3::new(location.x, location.y + size.height, 0.0));
                st.add_uv(Vector2::new(1.0, 1.0)); // Bottom Right
                st.add_vertex(Vector3::new(location.x + size.width, location.y + size.height, 0.0));
            }
        }

        let blending = self.base_background.blending;
        let material = Material::allocate(configuration.sprite_shader.clone());
        blending.configure_material(&material);

        let actual_z_index = if blending.is_none() { z_index } else { z_index + 64 };
        let max_z_index = match self.base_background.layer {
            BackgroundLayer::Back => BG_LAYER_BACK_Z_INDEX_MAX,
            BackgroundLayer::Front => BG_LAYER_FRONT_Z_INDEX_MAX,
        };
        let z_index = i32::min(actual_z_index, max_z_index);

        gdnative::godot_print!("z_index: {:?}", z_index);

        commands.spawn_bundle(Mesh2dBundle {
            texture: self.texture.clone(),
            mesh: Mesh2d {
                primitive_type: PrimitiveType::TRIANGLES,
                surface_array: st.commit_to_arrays(),
            },
            clip_rect: self.base_background.get_window_clip_rect(),
            back_buffer_copy: BackBufferCopy {
                enabled: true,
                ..Default::default()
            },
            material: Some(material),
            z_index: z_index.into(),
            ..Default::default()
        })
        .insert(self.clone())
        .id()
    }

    pub fn update(&self, mut transform: Mut<Transform2D>) {
        let velocity = self.base_background.velocity;

        if velocity == Vector2::new(0.0, 0.0) {
            return;
        }

        *transform = transform.then_translate(velocity);

        let size = self.sprite.size;
        let location = transform.transform_point(Point2::default());
        let startlocation = self.base_background.startlocation;

        if location.x >= startlocation.x + size.width || location.x <= startlocation.x - size.width {
            *transform = transform
                .then_translate(Vector2::new(-location.x, 0.0))
                .then_translate(Vector2::new(startlocation.x, 0.0));
        }

        if location.y >= startlocation.y + size.height || location.y <= startlocation.y - size.height {
            *transform = transform
                .then_translate(Vector2::new(0.0, -location.y))
                .then_translate(Vector2::new(0.0, startlocation.y));
        }
    }
}
