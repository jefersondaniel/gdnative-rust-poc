#[derive(Clone)]
pub struct TextSection {
    pub title: String,
    pub lines: Vec<String>,
    parsedlines: Vec<(String, String)>
}

pub struct AttributeValue {
    value: String,
}

impl TextSection {
    pub fn new(
        title: String,
        lines: Vec<String>,
        parsedlines: Vec<(String, String)>
    ) -> Self {
        TextSection {
            title: title,
            lines: lines,
            parsedlines: parsedlines,
        }
    }

    pub fn has_attribute(&self, key: &String) -> bool {
        if let Some(_) = self.get_value(key) {
            return true
        }

        false
    }

    pub fn get_attribute(&self, key: &String) -> Option<AttributeValue> {
        if let Some(value) = self.get_value(key) {
            return Some(AttributeValue{ value: value })
        }

        None
    }

    pub fn get_value(&self, key: &String) -> Option<String> {
        for (data_key, data_value) in self.parsedlines.iter() {
            if key.to_lowercase() == data_key.to_lowercase() {
                return Some(data_value.to_string())
            }
        }

        None
    }
}
