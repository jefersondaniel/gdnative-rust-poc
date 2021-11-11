pub struct TextSection {
  title: String,
  lines: Vec<String>,
  parsedlines: Vec<(String, String)>
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

  pub fn get_value(&self, key: &String) -> Option<String> {
    for (data_key, data_value) in self.parsedlines.iter() {
      if key.to_lowercase() == data_key.to_lowercase() {
        return Some(data_value.to_string())
      }
    }

    None
  }
}
