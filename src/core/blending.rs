use gdnative::godot_warn;

use crate::core::regex::RegExFlags;

use super::{attribute_value::AttributeValue, enumerations::BlendType, regex::RegEx};

#[derive(Default, Copy, Clone, PartialEq)]
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
}

impl From<&AttributeValue> for Blending {
    fn from(attribute_value: &AttributeValue) -> Blending {
        let text = attribute_value.to_string().trim().to_lowercase();

        if text.is_empty() || "none" == text {
            return Blending::default();
        }

        if text == "addalpha" {
            return Blending::new(BlendType::Add, 0, 0);
        }

        if text == "add" || text == "a" {
            return Blending::new(BlendType::Add, 255, 255);
        }

        if text == "add1" || text == "a1" {
            return Blending::new(BlendType::Add, 255, 127);
        }

        if text == "subtract" || text == "s" || text == "sub" {
            return Blending::new(BlendType::Subtract, 255, 255);
        }

        let regex = RegEx::new(r"^as(\d+)d(\d+)$", RegExFlags::IgnoreCase);

        if let Some(regex_match) = regex.search(&text.to_lowercase()) {
            let source_option = regex_match.get_u8(1);
            let destination_option = regex_match.get_u8(2);

            if let Some(source) = source_option {
                if let Some(destination) = destination_option {
                    return Blending::new(
                        BlendType::Add,
                        source,
                        destination
                    );
                }
            }
        }

        godot_warn!("Invalid blending format: {}", text);

        Blending::default()
    }
}
