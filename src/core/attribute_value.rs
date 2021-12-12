use gdnative::{core_types::{Point2, Rect2, Size2, Vector2}};

use super::{enumerations::BackgroundLayer, error::DataError};

#[derive(Default, Clone, PartialEq)]
pub struct AttributeValue {
    value: String,
}

impl AttributeValue {
    pub fn new(
        value: &str,
    ) -> Self {
        AttributeValue {
            value: String::from(value)
        }
    }

    pub fn compare(&self, value: &str, index_a: usize, index_b: usize, length: usize) -> bool {
        let value_a = self.to_string().to_lowercase();
        let value_b = value.to_lowercase();

        value_a[index_a..(index_a + length)] == value_b[index_b..(index_b + length)]
    }

    pub fn to_string(&self) -> String {
        let mut value = self.value.clone();
        if value.len() >= 2 && value.starts_with('"') && value.ends_with('"') {
            value = value[1..(value.len() - 1)].to_string();
        }
        value
    }

    pub fn split_values(&self) -> Vec<String> {
        self.split_with_separator(',', true)
    }

    pub fn split_with_separator(&self, separator: char, trim: bool) -> Vec<String> {
        let text_raw = &self.value.to_string();
        let text = if trim { text_raw.trim() } else { &text_raw };
        let mut result: Vec<String> = Vec::new();
        for raw_item in text.split(separator) {
            let item = if trim { raw_item.trim().to_string() } else { raw_item.to_string() };
            result.push(item);
        }
        result
    }
}

pub trait ParseAttributeValue: Sized {
    fn parse_attribute_value(value: AttributeValue) -> Result<Self, DataError>;
}

impl ParseAttributeValue for AttributeValue {
    fn parse_attribute_value(value: AttributeValue) -> Result<AttributeValue, DataError> {
        Ok(value)
    }
}

impl ParseAttributeValue for String {
    fn parse_attribute_value(value: AttributeValue) -> Result<String, DataError> {
        Ok(value.to_string())
    }
}

impl ParseAttributeValue for i32 {
    fn parse_attribute_value(value: AttributeValue) -> Result<i32, DataError> {
        let text = value.to_string();

        match text.parse::<i32>() {
            Ok(value) => Ok(value),
            Err(err) => Err(DataError::new(format!("Invalid integer: {}", text)))
        }
    }
}

impl ParseAttributeValue for usize {
    fn parse_attribute_value(value: AttributeValue) -> Result<usize, DataError> {
        let text = value.to_string();

        match text.parse::<usize>() {
            Ok(value) => Ok(value),
            Err(err) => Err(DataError::new(format!("Invalid index: {}", text)))
        }
    }
}

impl ParseAttributeValue for f32 {
    fn parse_attribute_value(value: AttributeValue) -> Result<f32, DataError> {
        let text = value.to_string();

        match text.parse::<f32>() {
            Ok(value) => Ok(value),
            Err(err) => Err(DataError::new(format!("Invalid float: {}", text)))
        }
    }
}

impl ParseAttributeValue for bool {
    fn parse_attribute_value(value: AttributeValue) -> Result<bool, DataError> {
        let text = value.to_string();

        if text.trim() == "1" {
            return Ok(true);
        }

        if text.trim() == "0" {
            return Ok(false);
        }

        return Err(DataError::new(format!("Invalid bool: {}", text)));
    }
}

impl ParseAttributeValue for Vector2 {
    fn parse_attribute_value(value: AttributeValue) -> Result<Vector2, DataError> {
        let pieces  = value.split_values();
        let error = DataError::new(format!("Invalid vector: {}", value.to_string()));

        if pieces.len() == 2 {
            let x = pieces[0].parse::<i32>().map_err(|_| error.clone())?;
            let y = pieces[1].parse::<i32>().map_err(|_| error.clone())?;

            return Ok(Vector2::new(x as f32, y as f32));
        }

        Err(error)
    }
}

impl ParseAttributeValue for Size2 {
    fn parse_attribute_value(value: AttributeValue) -> Result<Size2, DataError> {
        let vector = Vector2::parse_attribute_value(value)?;

        Ok(Size2::new(vector.x, vector.y))
    }
}

impl ParseAttributeValue for Point2 {
    fn parse_attribute_value(value: AttributeValue) -> Result<Point2, DataError> {
        let vector = Vector2::parse_attribute_value(value)?;

        Ok(Point2::new(vector.x, vector.y))
    }
}

impl ParseAttributeValue for Rect2 {
    fn parse_attribute_value(value: AttributeValue) -> Result<Rect2, DataError> {
        let pieces  = value.split_values();
        let error = DataError::new(format!("Invalid vector: {}", value.to_string()));

        if pieces.len() > 3 {
            let x1 = pieces[0].parse::<i32>().map_err(|_| error.clone())?;
            let y1 = pieces[1].parse::<i32>().map_err(|_| error.clone())?;
            let x2 = pieces[2].parse::<i32>().map_err(|_| error.clone())?;
            let y2 = pieces[3].parse::<i32>().map_err(|_| error.clone())?;

            return Ok(
                Rect2::new(
                    Point2::new(x1 as f32, y1 as f32),
                    Size2::new((x2 - x1) as f32, (y2 - y1) as f32),
                )
            );
        }

        Err(error)
    }
}

impl ParseAttributeValue for BackgroundLayer {
    fn parse_attribute_value(value: AttributeValue) -> Result<BackgroundLayer, DataError> {
        let value  = value.to_string();

        if value.trim() == "0" {
            return Ok(BackgroundLayer::Back);
        }

        if value.trim() == "1" {
            return Ok(BackgroundLayer::Front);
        }

        Err(DataError::new(format!("Invalid layer: {}", value)))
    }
}
