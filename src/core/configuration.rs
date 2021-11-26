use gdnative::core_types::{Size2};

pub struct Configuration {
    pub screen_size: Size2,
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            screen_size: Size2::new(1280.0, 720.0)
        }
    }
}
