use std::{collections::HashMap, sync::Arc};

use gdnative::{api::{file::File, visual_server::TextureFlags}, core_types::{Point2, Vector2, Size2, Rect2}};

use crate::{core::{error::DataError, attribute_value::{ParseAttributeValue, AttributeValue}}, systems::visual_server::texture::Texture, io::text_file::TextFile};

use super::sff::{data::{DataReader, BufferReader, FileReader}, pcx::read_pcx};

#[allow(dead_code)]
pub struct FileHeader {
    signature: String,
    verlo3: u8,
    verlo2: u8,
    verlo1: u8,
    verhi: u8,
    pcx_offset: u32,
    pcx_size: u32,
    text_offset: u32,
    text_size: u32,
    unused: Vec<u8>,
}

impl FileHeader {
    pub fn read(reader: &mut dyn DataReader) -> FileHeader {
        FileHeader {
            signature: reader.get_text(12),
            verlo3: reader.get_u8(),
            verlo2: reader.get_u8(),
            verlo1: reader.get_u8(),
            verhi: reader.get_u8(),
            pcx_offset: reader.get_u32(),
            pcx_size: reader.get_u32(),
            text_offset: reader.get_u32(),
            text_size: reader.get_u32(),
            unused: reader.get_buffer(40),
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum FntType {
    Variable,
    Fixed
}

impl ParseAttributeValue for FntType {
    fn parse_attribute_value(value: AttributeValue) -> Result<FntType, DataError> {
        let text = value.to_string().to_lowercase();

        if text.trim() == "variable" {
            return Ok(FntType::Variable);
        }

        Ok(FntType::Fixed)
    }
}

impl Default for FntType {
    fn default() -> Self { FntType::Fixed }
}

pub struct CharData {
    pub texture_index: usize,
    pub rect: Rect2,
}

pub struct FntFile {
    pub offset: Point2,
    pub size: Size2,
    pub spacing: Vector2,
    pub font_type: FntType,
    pub char_map: HashMap<char, CharData>,
    pub textures: Vec<Arc<Texture>>,
}

pub fn read_fnt_file(path: &str) -> Result<FntFile, DataError> {
    let file = File::new();
    let open_result = file.open(path.to_string(), File::READ);

    if let Err(detail) = open_result {
        return Result::Err(DataError::new(format!(
            "Error opening fnt file: {}",
            detail
        )));
    }

    let mut reader = FileReader::new(&file);
    let head = FileHeader::read(&mut reader);

    if head.signature != "ElecbyteFnt" {
        file.close();
        return Result::Err(DataError::new(format!(
            "Fnt invalid signature: {}",
            head.signature
        )));
    }

    file.seek(head.text_offset as i64);
    let text = reader.get_text(head.text_size as usize);

    file.seek(head.pcx_offset as i64);
    let pcx_arr = reader.get_buffer(head.pcx_size as usize);
    let mut pcx_arr_reader = BufferReader::new(&pcx_arr);
    let image_result = read_pcx(&mut pcx_arr_reader);

    match image_result {
        Ok(image) => {
            let image = image.borrow().create_image();

            parse_fnt_file(
                path.to_string(),
                text,
                Texture::allocate(image, TextureFlags(0))
            )
        }
        Err(message) => {
            file.close();
            Result::Err(DataError::new(message.to_string()))
        }
    }
}

fn parse_fnt_file(
    path: String,
    text: String,
    texture: Arc<Texture>
) -> Result<FntFile, DataError> {
    let text_file = TextFile::from_string(path, text);
    let def_section = text_file.get_section("def")?;
    let map_section = text_file.get_section("map")?;

    let offset: Point2 = def_section.get_attribute_or_default("offset");
    let size: Size2 = def_section.get_attribute_or_default("size");
    let spacing: Vector2 = def_section.get_attribute_or_default("spacing");
    let font_type: FntType = def_section.get_attribute_or_default("type");
    let mut char_map: HashMap<char, CharData> = HashMap::new();
    let textures = vec![texture];

    for (iterator, line) in map_section.lines.iter().enumerate() {
        let pieces = line.split_with_separator(' ', false);
        let character = parse_character(pieces[0].to_string())?;
        let mut char_start_x = iterator as f32 * size.width;
        let mut char_width = size.width;

        if font_type == FntType::Variable {
            char_start_x = pieces[1].parse()
                .map_err(|_| DataError::new(format!("Invalid char line: {}", line.to_string())))?;
            char_width = pieces[2].parse()
                .map_err(|_| DataError::new(format!("Invalid char line: {}", line.to_string())))?;
        }

        // TODO: Review char_width usage

        char_map.insert(character, CharData {
            texture_index: 0,
            rect: Rect2::new(
                Point2::new(char_start_x, 0.0),
                size
            )
        });
    }

    Ok(FntFile {
        offset,
        size,
        spacing,
        font_type,
        char_map,
        textures,
    })
}

fn parse_character(text: String) -> Result<char, DataError> {
    if text.to_lowercase().starts_with("0x") {
        let num = u32::from_str_radix(
            text.to_lowercase().trim_start_matches("0x"),
            16
        ).map_err(
            |_| DataError::new(format!("Invalid char format: {}", text))
        )?;

        return char::from_u32(num)
            .ok_or(DataError::new(format!("Invalid char code: {}", text)))
    }

    text.chars().next()
        .ok_or(DataError::new(format!("Invalid char: {}", text)))
}
