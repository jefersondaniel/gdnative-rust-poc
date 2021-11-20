#[derive(Clone, PartialEq)]
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
        let value_a = self.value.to_lowercase();
        let value_b = value.to_lowercase();

        value_a[index_a..(index_a + length)] == value_b[index_b..(index_b + length)]
    }

    pub fn to_string(&self) -> String {
        return String::from(self);
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
