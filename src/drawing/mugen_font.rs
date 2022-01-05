use gdnative::{core_types::{Rect2, Size2, Vector2, Point2}, api::visual_server::TextureFlags, godot_warn};

use crate::{core::error::DataError, io::file_system::FileSystem, systems::visual_server::{text::{font::Font, common::{FontSpacing, GlyphSpacing}, bitmap_font::BitmapFont, font_loader::load_dynamic_font}, texture::Texture}};

use super::{fnt_parser::read_fnt_file, sprite_system::SpriteSystem};

#[derive(Clone)]
pub struct MugenFont {
    font_banks: Vec<Font>,
    pub size: i32,
}

impl MugenFont {
    pub fn get_color_bank(&self, color_bank: usize) -> Font {
        if color_bank >= self.font_banks.len() {
            gdnative::godot_warn!("Color bank not found: {}", color_bank);
            return self.font_banks[0].clone()
        }

        self.font_banks[color_bank].clone()
    }

    pub fn load_font_v1(path: &str) -> Result<MugenFont, DataError> {
        let fnt_file = read_fnt_file(path)?;

        let mut bitmap_font = BitmapFont::new(
            fnt_file.textures,
            FontSpacing {
                line_gap: fnt_file.size.height + fnt_file.spacing.y,
                ..Default::default()
            }
        );

        for (character, char_data) in fnt_file.char_map.iter() {
            bitmap_font.add_character(
                *character,
                0,
                char_data.rect,
                Point2::new(0.0, 0.0),
                GlyphSpacing {
                    h_advance: char_data.rect.size.width + fnt_file.spacing.x,
                    ..Default::default()
                }
            )
        }

        bitmap_font.add_character(
            ' ',
            0,
            Rect2::default(),
            Point2::new(0.0, 0.0),
            GlyphSpacing {
                h_advance: fnt_file.size.width + fnt_file.spacing.x,
                ..Default::default()
            }
        );

        return Ok(MugenFont {
            font_banks: vec![Font::BitmapFont {
                font: bitmap_font,
            }],
            size: 0,
        })
    }

    pub fn load_font_v2(path: &str, sprite_system: &SpriteSystem) -> Result<MugenFont, DataError> {
        let file_system = FileSystem::new();
        let text_file = file_system.open_text_file(path)?;
        let def_section = text_file.get_section("def")?;
        let filename: String = def_section.get_attribute_or_fail("file")?;
        let size: Size2 = def_section.get_attribute_or_default("size");
        let spacing: Vector2 = def_section.get_attribute_or_default("spacing");
        let offset: Point2 = def_section.get_attribute_or_default("offset");
        let font_path = file_system.get_path_by_refferrer(&filename, path);

        if font_path.to_lowercase().ends_with(".sff") {
            let base_bitmap_font = BitmapFont::new(
                vec![],
                FontSpacing {
                    line_gap: size.height + spacing.y,
                    ..Default::default()
                }
            );
            let mut sprite_file = sprite_system.get_sprite_file(&font_path)?;
            let palettes = sprite_system.load_palettes(&font_path)?;
            let images = sprite_file.get_group(0)?;
            let mut font_banks: Vec<Font> = Vec::new();

            for palette in palettes.iter() {
                let mut bitmap_font = base_bitmap_font.clone();
                let mut texture_id: usize = 0;
                for sff_item in images.iter() {
                    let character = char::from_u32(sff_item.imageno as u32)
                        .ok_or(DataError::new(format!("Invalid char code: {}", sff_item.imageno)))?;
                    let mut image = sff_item.image.borrow_mut();
                    image.color_table = palette.clone();
                    bitmap_font.add_texture(Texture::allocate(image.create_image(), TextureFlags::FLAGS_DEFAULT));
                    bitmap_font.add_character(
                        character,
                        texture_id,
                        Rect2::new(
                            Point2::new(0.0, 0.0),
                            Size2::new(image.w as f32, image.h as f32)
                        ),
                        Point2::new(offset.x, offset.y - size.height),
                        GlyphSpacing {
                            h_advance: image.w as f32 + spacing.x,
                            ..Default::default()
                        }
                    );
                    texture_id += 1;
                }
                bitmap_font.add_character(
                    ' ',
                    0,
                    Rect2::default(),
                    Point2::new(0.0, 0.0),
                    GlyphSpacing {
                        h_advance: size.width + spacing.x,
                        ..Default::default()
                    }
                );
                font_banks.push(Font::BitmapFont { font: bitmap_font });
            }

            return Ok(MugenFont {
                font_banks,
                size: 0,
            })
        }

        let font_result = load_dynamic_font(&font_path);

        // TODO: Adjust spacing for TrueType fonts

        match font_result {
            Ok(font) => {
                return Ok(MugenFont {
                    font_banks: vec![font],
                    size: size.height as i32,
                })
            },
            Err(_) => {
                let font= load_dynamic_font("res://resources/roboto.ttf")?;

                godot_warn!("True type font not found: {}, using fallback", file_system.get_name(&font_path));

                Ok(MugenFont {
                    font_banks: vec![font],
                    size: size.height as i32,
                })
            }
        }
    }
}
