use bevy_ecs::prelude::Res;
use gdnative::core_types::{Point2, Rect2, Vector2, Size2};

use crate::{core::{blending::Blending, configuration::Configuration, enumerations::BackgroundLayer, error::DataError, regex::{RegEx, RegExFlags}}, io::text_section::TextSection};

#[derive(Clone)]
pub struct BaseBackground {
    pub id: i32,
    pub name: String,
    pub currentlocation: Vector2,
    pub startlocation: Vector2,
    pub delta: Vector2,
    pub tiling: Vector2,
    pub tilingspacing: Vector2,
    pub velocity: Vector2,
    pub masking: bool,
    pub layer: BackgroundLayer,
    pub blending: Blending,
    pub drawrect: Rect2,
}

impl BaseBackground {
    pub fn build(
        configuration: &Configuration,
        textsection: &TextSection
    ) -> Result<Self, DataError> {
        let startlocation = textsection.get_attribute_or_default("start");

        Ok(BaseBackground {
            name: get_background_name(textsection),
            id: textsection.get_attribute_or("id", 0),
            startlocation,
            currentlocation: startlocation,
            delta: textsection.get_attribute_or_default("delta"),
            tiling: textsection.get_attribute_or_default("tile"),
            tilingspacing: textsection.get_attribute_or_default("tilespacing"),
            velocity: textsection.get_attribute_or_default("velocity"),
            masking: textsection.get_attribute_or("masking", false),
            layer: textsection.get_attribute_or("layerno", BackgroundLayer::Back),
            blending: textsection.get_attribute_or_default("trans"),
            drawrect: textsection.get_attribute_or(
                "window",
                Rect2::new(
                    Point2::new(0 as f32, 0 as f32),
                    configuration.screen_size
                )
            ),
        })
    }

    pub fn get_tile_length(&self, size: Size2, configuration: &Res<Configuration>) -> (Vector2, Vector2) {
        if self.tiling == Vector2::new(0.0, 0.0) {
            return (
                Vector2::new(0.0, 0.0),
                Vector2::new(1.0, 1.0),
            )
        }

        let mut t = Vector2::new(0.0, 0.0);
        t.x = f32::ceil(1.0 + configuration.screen_size.width / size.width);
        t.y = f32::ceil(1.0 + configuration.screen_size.height / size.height);

        let mut start = Vector2::new(0.0, 0.0);
        let mut end = Vector2::new(0.0, 0.0);

        if self.tiling.x == 0.0 {
            start.x = 0.0;
            end.x = 1.0;
        } else if self.tiling.x == 1.0 {
            start.x = -f32::max(3.0, t.x);
            end.x = f32::max(3.0, t.x);
        } else {
            start.x = 0.0;
            end.x = self.tiling.x;
        }

        if self.tiling.y == 0.0 {
            start.y = 0.0;
            end.y = 1.0;
        } else if self.tiling.y == 1.0 {
            start.y = -f32::max(3.0, t.y);
            end.y = f32::max(3.0, t.y);
        } else {
            start.y = 0.0;
            end.y = self.tiling.y;
        }

        (start, end)
    }
}

fn get_background_name(textsection: &TextSection) -> String {
    let titleregex = RegEx::new(r".*BG\s*(\S.*)", RegExFlags::IgnoreCase);
    if let Some(matches) = titleregex.search(&textsection.title) {
        return matches.get_string(1);
    }
    "".to_string()
}
