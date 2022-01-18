use std::fmt::Display;

use super::{attribute_value::{AttributeValue, ParseAttributeValue}, error::DataError};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SoundId {
    pub group: i16,
    pub sample: i16,
}

impl SoundId {
    pub fn new(group: i16, sample: i16) -> Self {
        SoundId { group: group, sample: sample }
    }

    pub fn invalid() -> Self { SoundId::new(i16::MIN, i16::MIN) }
}

impl From<&SoundId> for String {
    fn from(sprite_id: &SoundId) -> String {
        format!("{}, {}", sprite_id.group, sprite_id.sample)
    }
}

impl Display for SoundId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&String::from(self))
    }
}

impl ParseAttributeValue for SoundId {
    fn parse_attribute_value(value: AttributeValue) -> Result<SoundId, DataError> {
        let pieces  = value.split_values();
        let error = DataError::new(format!("Invalid sound id format: {}", value.to_string()));

        if pieces.len() == 2 {
            let x = pieces[0].parse::<i16>().map_err(|_| error.clone())?;
            let y = pieces[1].parse::<i16>().map_err(|_| error.clone())?;

            return Ok(SoundId::new(x, y));
        }

        Err(error)
    }
}
