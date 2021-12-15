use gdnative::core_types::{Angle, Transform2D, Vector2};

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

impl Transform {
    pub fn translation(translation: Vector2) -> Self {
        Transform {
            translation,
            rotation: Angle::radians(0.0),
            scale: Vector2::new(1.0, 1.0),
        }
    }

    pub fn rotation(rotation: Angle) -> Self {
        Transform {
            translation: Vector2::new(0.0, 0.0),
            rotation,
            scale: Vector2::new(1.0, 1.0),
        }
    }

    pub fn scale(scale: Vector2) -> Self {
        Transform {
            translation: Vector2::new(0.0, 0.0),
            rotation: Angle::radians(0.0),
            scale,
        }
    }

    pub fn then_translate(&mut self, translation: Vector2) -> &mut Self {
        self.translation = translation;
        self
    }

    pub fn then_rotate(&mut self, rotation: Angle) -> &mut Self {
        self.rotation = rotation;
        self
    }

    pub fn than_scale(&mut self, scale: Vector2) -> &mut Self {
        self.scale = scale;
        self
    }
}

impl From<&Transform> for Transform2D {
    fn from(transform: &Transform) -> Self {
        Transform2D::translation(transform.translation.x, transform.translation.y)
            .then_rotate(transform.rotation)
            .then_scale(transform.scale.x, transform.scale.y)
    }
}

impl From<Transform> for Transform2D {
    fn from(transform: Transform) -> Self {
        Transform2D::translation(transform.translation.x, transform.translation.y)
            .then_rotate(transform.rotation)
            .then_scale(transform.scale.x, transform.scale.y)
    }
}
