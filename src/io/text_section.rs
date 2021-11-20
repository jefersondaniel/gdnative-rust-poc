use crate::core::attribute_value::AttributeValue;

#[derive(Clone)]
pub struct TextSection {
    pub title: String,
    pub lines: Vec<AttributeValue>,
    pub parsedlines: Vec<(String, AttributeValue)>,
}

impl TextSection {
    pub fn new(
        title: String,
        lines: Vec<AttributeValue>,
        parsedlines: Vec<(String, AttributeValue)>,
    ) -> Self {
        TextSection {
            title: title,
            lines: lines,
            parsedlines: parsedlines,
        }
    }

    pub fn has_attribute(&self, key: &String) -> bool {
        if let Some(_) = self.get_attribute(key) {
            return true
        }

        false
    }

    fn get_attribute(&self, key: &String) -> Option<&AttributeValue> {
        for (data_key, data_value) in self.parsedlines.iter() {
            if key.to_lowercase() == data_key.to_lowercase() {
                return Some(data_value)
            }
        }

        None
    }
}
