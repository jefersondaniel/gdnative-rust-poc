use gdnative::core_types::{Angle, Rotation2D, Transform2D, Vector2};

pub struct Transform {
    pub translation: Vector2,
    pub rotation: Angle,
    pub scale: Vector2,
}

impl Default for Transform {
    fn default() -> Self {
        Transform {
            translation: Vector2::new(0.0, 0.0),
            rotation: Angle::radians(0.0),
            scale: Vector2::new(1.0, 1.0),
        }
    }
}

impl From<&Transform> for Transform2D {
    fn from(transform: &Transform) -> Self {
        Transform2D::translation(transform.translation.x, transform.translation.y)
            .then_rotate(transform.rotation)
            .then_scale(transform.scale.x, transform.scale.y)
    }
}
