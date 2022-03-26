use std::fmt::Display;

use super::{attribute_value::{AttributeValue, ParseAttributeValue}, error::DataError};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpriteId {
    pub group: i16,
    pub image: i16,
}

impl SpriteId {
    pub const SMALL_PORTRAIT: SpriteId = SpriteId { group: 9000, image: 0 };

    pub fn new(group: i16, image: i16) -> Self {
        SpriteId { group: group, image: image }
    }

    pub fn invalid() -> Self { SpriteId::new(i16::MIN, i16::MIN) }
}

impl From<&SpriteId> for String {
    fn from(sprite_id: &SpriteId) -> String {
        format!("{}, {}", sprite_id.group, sprite_id.image)
    }
}

impl ParseAttributeValue for SpriteId {
    fn parse_attribute_value(value: AttributeValue) -> Result<SpriteId, DataError> {
        let pieces  = value.split_values();
        let error = DataError::new(format!("Invalid sprite id format: {}", value.to_string()));

        if pieces.len() == 2 {
            let x = pieces[0].parse::<i16>().map_err(|_| error.clone())?;
            let y = pieces[1].parse::<i16>().map_err(|_| error.clone())?;

            return Ok(SpriteId::new(x, y));
        }

        Err(error)
    }
}

impl Display for SpriteId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&String::from(self))
    }
}
