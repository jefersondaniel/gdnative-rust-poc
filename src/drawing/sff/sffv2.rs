use crate::core::error::DataError;

use super::data::{BufferReader, DataReader, FileReader};
use super::image::{Palette, RawColor, RawImage};
use super::lz5::decode_lz5;
use super::rle5::{decode_rle5, decode_rle8};
use super::sff_common::{SffData, SffPal, SffMetadata, MutableSffData};
use gdnative::Ref;
use gdnative::api::file::File;
use gdnative::prelude::Unique;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

#[allow(dead_code)]
struct FileHeader {
    signature: String,
    verlo3: u8,
    verlo2: u8,
    verlo1: u8,
    verhi: u8,
    reserved1: Vec<u8>, // [u8, 4]
    reserved2: Vec<u8>, // [u8, 4]
    compatverlo3: u8,
    compatverlo2: u8,
    compatverlo1: u8,
    compatverhi: u8,
    reserved3: Vec<u8>, // [u8; 4]
    reserved4: Vec<u8>, // [u8; 4]
    first_sprnode_offset: u32,
    total_frames: u32,
    first_palnode_offset: u32,
    total_palettes: u32,
    ldata_offset: u32,
    ldata_length: u32,
    tdata_offset: u32,
    tdata_length: u32,
    reserved5: Vec<u8>, // [u8; 4]
    reserved6: Vec<u8>, // [u8; 4]
    unused: Vec<u8>,    // [u8; 436
}

impl FileHeader {
    fn read(reader: &mut dyn DataReader) -> FileHeader {
        FileHeader {
            signature: reader.get_text(12),
            verlo3: reader.get_u8(),
            verlo2: reader.get_u8(),
            verlo1: reader.get_u8(),
            verhi: reader.get_u8(),
            reserved1: reader.get_buffer(4),
            reserved2: reader.get_buffer(4),
            compatverlo3: reader.get_u8(),
            compatverlo2: reader.get_u8(),
            compatverlo1: reader.get_u8(),
            compatverhi: reader.get_u8(),
            reserved3: reader.get_buffer(4),
            reserved4: reader.get_buffer(4),
            first_sprnode_offset: reader.get_u32(),
            total_frames: reader.get_u32(),
            first_palnode_offset: reader.get_u32(),
            total_palettes: reader.get_u32(),
            ldata_offset: reader.get_u32(),
            ldata_length: reader.get_u32(),
            tdata_offset: reader.get_u32(),
            tdata_length: reader.get_u32(),
            reserved5: reader.get_buffer(4),
            reserved6: reader.get_buffer(4),
            unused: reader.get_buffer(436),
        }
    }
}

struct SpriteHeader {
    groupno: i16,
    imageno: i16,
    w: i16,
    h: i16,
    x: i16,
    y: i16,
    linked: i16,
    fmt: u8,
    colordepth: u8,
    offset: u32,
    len: u32,
    palindex: i16,
    flags: i16,
}

impl SpriteHeader {
    fn read(reader: &mut dyn DataReader) -> SpriteHeader {
        SpriteHeader {
            groupno: reader.get_i16(),
            imageno: reader.get_i16(),
            w: reader.get_i16(),
            h: reader.get_i16(),
            x: reader.get_i16(),
            y: reader.get_i16(),
            linked: reader.get_i16(),
            fmt: reader.get_u8(),
            colordepth: reader.get_u8(),
            offset: reader.get_u32(),
            len: reader.get_u32(),
            palindex: reader.get_i16(),
            flags: reader.get_i16(),
        }
    }
}

struct PaletteHeader {
    groupno: i16,
    itemno: i16,
    numcols: i16,
    linked: i16,
    offset: u32,
    len: u32,
}

impl PaletteHeader {
    fn read(reader: &mut dyn DataReader) -> PaletteHeader {
        PaletteHeader {
            groupno: reader.get_i16(),
            itemno: reader.get_i16(),
            numcols: reader.get_i16(),
            linked: reader.get_i16(),
            offset: reader.get_u32(),
            len: reader.get_u32(),
        }
    }
}
struct FileHandler {
    file: Rc<Ref<File, Unique>>,
    head: FileHeader
}

fn matrix_to_pal(reader: &mut dyn DataReader, size: usize) -> Arc<Palette> {
    let mut colors: Vec<RawColor> = Vec::new();
    for i in 0..size {
        let r = reader.get_u8();
        let g = reader.get_u8();
        let b = reader.get_u8();
        reader.get_u8();
        colors.push(RawColor::new(r, g, b, if i == 0 { 0 } else { 255 }));
    }
    Arc::new(Palette::from_colors(colors))
}

fn open(filename: &str) -> Result<FileHandler, DataError> {
    let file = Rc::new(File::new());
    let result = file.open(filename, File::READ);

    if let Err(detail) = result {
        return Result::Err(DataError::new(format!(
            "error opening file: {}",
            detail
        )));
    }

    let mut reader = FileReader::new(&file);
    let head = FileHeader::read(&mut reader);

    if head.signature != "ElecbyteSpr" {
        file.close();
        return Result::Err(DataError::new(format!(
            "invalid signature: {}",
            head.signature
        )));
    }

    if head.verhi != 2 {
        file.close();
        return Result::Err(DataError::new(format!(
            "invalid version: {}.{}.{}.{}",
            head.verhi, head.verlo1, head.verlo2, head.verlo3
        )));
    }

    Result::Ok(FileHandler {
        file,
        head
    })
}

pub fn read_metadata(filename: &str) -> Result<SffMetadata, DataError> {
    let open_result = open(filename);

    if let Err(error) = open_result {
        return Result::Err(error);
    }

    let handler = open_result.expect("Invalid open result");
    let file = handler.file;
    let head = handler.head;

    file.close();

    Result::Ok(SffMetadata {
        verlo3: head.verlo3,
        verlo2: head.verlo2,
        verlo1: head.verlo1,
        verhi: head.verhi,
    })
}

pub fn read_palettes(filename: &str) -> Result<Vec<Arc<Palette>>, DataError> {
    let open_result = open(filename);

    if let Err(error) = open_result {
        return Result::Err(error);
    }

    let handler = open_result.expect("Invalid open result");
    let file = handler.file;
    let head = handler.head;
    let mut result: Vec<Arc<Palette>> = Vec::new();
    let mut palnode: Vec<PaletteHeader> = Vec::new();
    let mut reader = FileReader::new(&file);

    file.seek(head.first_palnode_offset as i64);

    for _ in 0..head.total_palettes {
        palnode.push(PaletteHeader::read(&mut reader));
    }

    for palette in palnode.iter() {
        let pal: Arc<Palette> = match palette.len {
            0 => Arc::clone(&result[palette.linked as usize]),
            len if len > 0 => {
                let mut offset: usize = head.ldata_offset as usize;
                offset += palette.offset as usize;
                file.seek(offset as i64);

                let tmp_arr = reader.get_buffer((palette.numcols * 4) as usize);
                let mut tmp_arr_reader = BufferReader::new(&tmp_arr);
                matrix_to_pal(&mut tmp_arr_reader, palette.numcols as usize)
            }
            _ => Arc::new(Palette::new(0)),
        };

        result.push(pal);
    }

    Result::Ok(result)
}

pub fn read_images(filename: &str, groups: &[i16]) -> Result<Vec<SffData>, DataError> {
    let open_result = open(filename);

    if let Err(error) = open_result {
        return Result::Err(error);
    }

    let handler = open_result.expect("Invalid open result");
    let file = handler.file;
    let head = handler.head;
    let mut reader = FileReader::new(&file);

    let mut sffdata: HashMap<i32, MutableSffData> = HashMap::new();
    let mut paldata: Vec<SffPal> = Vec::new();
    let mut sprnode: Vec<SpriteHeader> = Vec::new();
    let mut palnode: Vec<PaletteHeader> = Vec::new();
    let mut requested_indexes: Vec<i32> = Vec::new();

    file.seek(head.first_palnode_offset as i64);

    for _ in 0..head.total_palettes {
        palnode.push(PaletteHeader::read(&mut reader));
    }

    file.seek(head.first_sprnode_offset as i64);

    for counter in 0..head.total_frames {
        let spr = SpriteHeader::read(&mut reader);

        if groups.contains(&spr.groupno) {
            requested_indexes.push(counter as i32);
            if spr.len == 0 {
                requested_indexes.push(spr.linked as i32);
            }
        }

        sprnode.push(spr);
    }

    for palette in palnode.iter() {
        let pal: Arc<Palette> = match palette.len {
            0 => Arc::clone(&paldata[palette.linked as usize].pal),
            len if len > 0 => {
                let mut offset: usize = head.ldata_offset as usize;
                offset += palette.offset as usize;
                file.seek(offset as i64);

                let tmp_arr = reader.get_buffer((palette.numcols * 4) as usize);
                let mut tmp_arr_reader = BufferReader::new(&tmp_arr);
                matrix_to_pal(&mut tmp_arr_reader, palette.numcols as usize)
            }
            _ => Arc::new(Palette::new(0)),
        };

        paldata.push(SffPal {
            pal,
            itemno: palette.itemno as i32,
            groupno: palette.groupno as i32,
            is_used: false,
            usedby: -1,
            reserved: 0,
        });
    }

    //reading images
    for (counter, sprite) in sprnode.iter().enumerate() {
        if !groups.is_empty() && !requested_indexes.contains(&(counter as i32)) {
            continue;
        }

        let linked;
        let mut image = Rc::new(RefCell::new(RawImage::empty()));
        if sprite.len == 0 {
            //linked image
            linked = -1;
            image = Rc::clone(&sffdata[&(sprite.linked as i32)].image);
        } else {
            //"normal" image
            let mut offset: usize = 0;
            if sprite.flags == 0 {
                offset = head.ldata_offset as usize;
            }
            if sprite.flags != 0 {
                offset = head.tdata_offset as usize;
            }
            offset += sprite.offset as usize;
            file.seek(offset as i64);

            let mut tmp_arr = reader.get_buffer(sprite.len as usize);
            let mut tmp_reader = BufferReader::new(&tmp_arr);

            match sprite.fmt {
                2 => tmp_arr = decode_rle8(&mut tmp_reader),
                3 => tmp_arr = decode_rle5(&mut tmp_reader),
                4 => tmp_arr = decode_lz5(&mut tmp_reader),
                _ => (),
            };

            let expected_size = (sprite.w as usize * sprite.h as usize) as usize;
            let actual_size = tmp_arr.len();

            if expected_size != actual_size {
                return Err(DataError::new(format!(
                    "Image decoding failed. GroupNo={}. ImageNo={}",
                    sprite.groupno, sprite.imageno
                )));
            }

            //adding image
            if sprite.colordepth == 5 || sprite.colordepth == 8 {
                image = Rc::new(RefCell::new(RawImage {
                    w: sprite.w as usize,
                    h: sprite.h as usize,
                    pixels: Arc::new(tmp_arr),
                    color_table: Arc::clone(&paldata[sprite.palindex as usize].pal),
                }));
            }

            linked = -1;
        }

        sffdata.insert(counter as i32, MutableSffData {
            groupno: sprite.groupno,
            imageno: sprite.imageno,
            x: sprite.x,
            y: sprite.y,
            palindex: sprite.palindex,
            image,
            linked,
        });
    }

    for (a, item) in sffdata.iter_mut() {
        let b = item.palindex as usize;
        if !paldata[b].is_used {
            paldata[b].is_used = true;
            paldata[b].usedby = *a;
        }
    }

    file.close();

    let mut result: Vec<SffData> = Vec::new();

    for value in sffdata.values() {
        result.push(value.to_sff_data());
    }

    Result::Ok(result)
}
