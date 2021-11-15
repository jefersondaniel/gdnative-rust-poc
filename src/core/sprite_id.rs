#[derive(Clone, Copy)]
pub struct SpriteId {
    pub group: i32,
    pub image: i32,
}

impl PartialEq for SpriteId {
    fn eq(&self, other: &Self) -> bool {
        self.group == other.group && self.image == other.image
    }
}

impl From<&SpriteId> for String {
    fn from(sprite_id: &SpriteId) -> String {
        format!("{}, {}", sprite_id.group, sprite_id.image)
    }
}
