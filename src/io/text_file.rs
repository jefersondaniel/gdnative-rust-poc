use crate::io::text_section::TextSection;

pub struct TextFile {
    pub filepath: String,
    pub sections: Vec<TextSection>
}

impl TextFile {
    pub fn new(
        filepath: String,
        sections: Vec<TextSection>
    ) -> Self {
        TextFile {
          filepath: filepath,
          sections: sections
        }
    }

    pub fn get_section(&self, key: &String) -> Option<TextSection> {
        for section in self.sections.iter() {
            if section.title.to_lowercase() == key.to_lowercase() {
                return Some(section.clone())
            }
        }

        None
    }
}
