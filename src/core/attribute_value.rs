use gdnative::{core_types::Vector2};

use super::error::DataError;

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
        let text_raw = &self.value.to_string();
        let text = text_raw.trim();
        let mut result: Vec<String> = Vec::new();
        for item in text.split(',') {
            result.push(item.trim().to_string());
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
            let y = pieces[0].parse::<i32>().map_err(|_| error.clone())?;

            return Ok(Vector2::new(x as f32, y as f32));
        }

        Err(error)
    }
}
