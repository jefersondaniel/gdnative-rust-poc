use gdnative::core_types::{Point2, Rect2, Vector2};

use crate::{core::{blending::Blending, configuration::Configuration, enumerations::BackgroundLayer, error::DataError, regex::{RegEx, RegExFlags}}, io::text_section::TextSection};

#[derive(Clone)]
pub struct BaseBackground {
    pub id: i32,
    pub name: String,
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
        Ok(BaseBackground {
            name: get_background_name(textsection),
            id: textsection.get_attribute_or("id", 0),
            startlocation: textsection.get_attribute_or_default("start"),
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
}

fn get_background_name(textsection: &TextSection) -> String {
    let titleregex = RegEx::new(r".*BG\\s*(\\S.*)", RegExFlags::IgnoreCase);
    if let Some(matches) = titleregex.search(&textsection.title) {
        return matches.get_string(1);
    }
    "".to_string()
}
