use crate::core::{attribute_value::{AttributeValue, ParseAttributeValue}, error::DataError};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BackgroundType {
    None,
    Static,
    Parallax,
    Animated
}

impl Default for BackgroundType {
    fn default() -> Self { BackgroundType::None }
}

impl ParseAttributeValue for BackgroundType {
    fn parse_attribute_value(value: AttributeValue) -> Result<BackgroundType, DataError> {
        let text  = value.to_string();

        if text.to_lowercase().trim() == "normal" {
            return Ok(BackgroundType::Static);
        }

        if text.to_lowercase().trim() == "parallax" {
            return Ok(BackgroundType::Parallax);
        }

        if text.to_lowercase().trim() == "anim" {
            return Ok(BackgroundType::Animated);
        }

        if text.to_lowercase().trim() == "none" {
            return Ok(BackgroundType::None);
        }

        Err(DataError::new(format!("Invalid background type: {}", text)))
    }
}

