use crate::{io::file_system::FileSystem, core::error::DataError};

use super::{font::Font, vector_font::VectorFont};

pub fn load_dynamic_font(path: &str) -> Result<Font, DataError> {
    let filesystem = FileSystem::new();
    let file = filesystem.open_file(path)?;
    let typed_array = file.get_buffer(file.get_len());
    let font_data: Vec<u8> = typed_array.read().iter().map(|v| *v).collect();

    let result = VectorFont::try_from_bytes(font_data);

    if let Ok(font) = result {
        return Ok(Font::VectorFont {
            font: font,
        })
    }

    Err(DataError::new(format!("Invalid TrueType font: {}", path)))
}
