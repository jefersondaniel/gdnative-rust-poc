#[derive(Clone)]
pub struct AttributeValue {
    value: String,
}

impl AttributeValue {
    pub fn new(
        value: String,
    ) -> Self {
        AttributeValue {
            value: value
        }
    }
}

impl From<&AttributeValue> for String {
    fn from(attribute_value: &AttributeValue) -> String {
        let mut value = attribute_value.value.clone();
        if value.len() >= 2 && value.starts_with('"') && value.ends_with('"') {
            value = value[1..(value.len() - 1)].to_string();
        }
        value
    }
}
