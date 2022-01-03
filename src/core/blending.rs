use std::{fmt::Display, sync::{RwLock, Arc}};

use gdnative::{godot_warn, core_types::ToVariant, api::animation_node_blend_space_2d::BlendMode};

use crate::{core::{error::DataError, regex::RegExFlags}, systems::visual_server::material::Material};

use super::{attribute_value::{AttributeValue, ParseAttributeValue}, enumerations::BlendType, regex::RegEx};

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Blending {
    blend_type: BlendType,
    source: u8,
    destination: u8,
}

impl Blending {
    pub fn new(
        blend_type: BlendType,
        source: u8,
        destination: u8
    ) -> Self {
        Blending {
            blend_type: blend_type,
            source: if blend_type != BlendType::None { source } else { 0 },
            destination: if blend_type != BlendType::None { destination } else { 0 }
        }
    }

    pub fn is_none(&self) -> bool {
        return self.blend_type == BlendType::None;
    }

    pub fn configure_material(&self, material: &Arc<RwLock<Material>>) {
        let mut material_write = material.write().expect("Could not lock material");
        material_write.set_shader_param("blend_type", (self.blend_type as i32).to_variant());
        material_write.set_shader_param("blend_source", (self.source as f32 / 255.0).to_variant());
        material_write.set_shader_param("blend_destination", (self.destination as f32 / 255.0).to_variant());
    }
}

fn parse_blending(raw_text: &str) -> Result<Blending, DataError> {
    let text = raw_text.to_string().trim().to_lowercase();

    if text.is_empty() || "none" == text {
        return Ok(Blending::default());
    }

    if text == "addalpha" {
        return Ok(Blending::new(BlendType::Add, 0, 0));
    }

    if text == "add" || text == "a" {
        return Ok(Blending::new(BlendType::Add, 255, 255));
    }

    if text == "add1" || text == "a1" {
        return Ok(Blending::new(BlendType::Add, 255, 127));
    }

    if text == "subtract" || text == "s" || text == "sub" {
        return Ok(Blending::new(BlendType::Subtract, 255, 255));
    }

    let regex = RegEx::new(r"^as(\d+)d(\d+)$", RegExFlags::IgnoreCase);

    if let Some(regex_match) = regex.search(&text.to_lowercase()) {
        let source_option = regex_match.get_u16(1);
        let destination_option = regex_match.get_u16(2);

        if let Some(source) = source_option {
            if let Some(destination) = destination_option {
                return Ok(Blending::new(
                    BlendType::Add,
                    source as u8,
                    destination as u8
                ));
            }
        }
    }

    Err(DataError::new(format!("Invalid blending format: {}", text)))
}

impl From<&str> for Blending {
    fn from(raw_text: &str) -> Blending {
        match parse_blending(raw_text) {
            Ok(blending) => blending,
            Err(error) => {
                godot_warn!("{}", error);
                Blending::default()
            }
        }
    }
}

impl ParseAttributeValue for Blending {
    fn parse_attribute_value(value: AttributeValue) -> Result<Blending, DataError> {
        parse_blending(&value.to_string())
    }
}

impl Display for Blending {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.blend_type == BlendType::None {
            return f.write_str("");
        }

        if self.blend_type == BlendType::Add && self.source == 0 && self.destination == 0 {
            return f.write_str("addalpha");
        }

        if self.blend_type == BlendType::Add && self.source == 255 && self.destination == 255 {
            return f.write_str("add");
        }

        if self.blend_type == BlendType::Add && self.source == 255 && self.destination == 127 {
            return f.write_str("add1");
        }

        if self.blend_type == BlendType::Subtract && self.source == 255 && self.destination == 255 {
            return f.write_str("sub");
        }

        f.write_str(&format!(
            "{}S{}D{}",
            if self.blend_type == BlendType::Add { "A" } else { "S" },
            self.source,
            self.destination
        ))
    }
}
