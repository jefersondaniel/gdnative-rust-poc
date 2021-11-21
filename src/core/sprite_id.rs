use std::fmt::Display;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpriteId {
    pub group: i16,
    pub image: i16,
}

impl SpriteId {
    pub fn new(group: i16, image: i16) -> Self {
        SpriteId { group: group, image: image }
    }
}

impl From<&SpriteId> for String {
    fn from(sprite_id: &SpriteId) -> String {
        format!("{}, {}", sprite_id.group, sprite_id.image)
    }
}

impl Display for SpriteId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&String::from(self))
    }
}
