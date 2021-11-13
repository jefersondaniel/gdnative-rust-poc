use crate::core::error::DataError;
use crate::io::attribute_value::AttributeValue;
use crate::io::text_file::TextFile;
use crate::io::text_section::TextSection;
use std::path::PathBuf;
use gdnative::prelude::*;
use gdnative::api::{RegEx};
use gdnative::api::file::File;

pub struct FileSystem {
    titleregex: Ref<RegEx, Unique>,
    parsedlineregex: Ref<RegEx, Unique>,
}

impl FileSystem {
    #[allow(unused_must_use)]
    pub fn new() -> Self {
        let titleregex = RegEx::new();
        titleregex.compile(r"^\s*\[(.+?)\]\s*$");

        let parsedlineregex = RegEx::new();
        parsedlineregex.compile(r"^\s*(.+?)\s*=\s*(.+?)\s*$");

        FileSystem {
            titleregex: titleregex,
            parsedlineregex: parsedlineregex,
        }
    }

    pub fn does_file_exist(filepath: String) -> bool {
        let file = File::new();
        return file.file_exists(filepath);
    }

    pub fn combine_paths(lhs: String, rhs: String) -> String {
        return format!("{}/{}", lhs.trim_end_matches('/'), rhs.trim_start_matches('/'))
    }

    pub fn get_directory(filepath: String) -> String {
        let mut path_buf = PathBuf::from(filepath);
        path_buf.pop();
        path_buf.to_str().unwrap().to_string()
    }

    pub fn open_file(&self, filepath: String) -> Result<Ref<File, Unique>, DataError> {
        let file = File::new();

        if let Err(detail) = file.open(filepath, File::READ) {
            return Result::Err(DataError::new(format!(
              "Error opening file: {}",
                detail
            )));
        }

        Result::Ok(file)
    }

    pub fn open_text_file(&self, filepath: String) -> Result<TextFile, DataError> {
        let result = self.open_file(filepath);

        match result {
            Ok(file) => Result::Ok(self.build_text_file(file)),
            Err(error) => Result::Err(error)
        }
    }

    pub fn build_text_file(&self, file: Ref<File, Unique>) -> TextFile {
        let mut sections: Vec<TextSection> = Vec::new();
        let mut sectiontitle: String = "".to_string();
        let mut sectionlines: Vec<AttributeValue> = Vec::new();
        let mut sectionparsedlines: Vec<(String, AttributeValue)> = Vec::new();

        while !file.eof_reached() {
            let mut line = file.get_line().to_string();
            line = line.trim().to_string();

            if let Some(commentindex) = line.find(';') {
                line = line[..commentindex].to_string();
            }

            line = line.trim().to_string();

            if line.is_empty() {
                continue;
            }

            if let Some(match_shared) = self.titleregex.search(line.clone(), 0, -1) {
                if !sectiontitle.is_empty() {
                    sections.push(TextSection::new(
                        sectiontitle.clone(),
                        sectionlines.clone(),
                        sectionparsedlines.clone()
                    ));
                }

                let match_unique = unsafe { match_shared.assume_unique() };
                sectiontitle = match_unique.get_string(1).to_string();
                sectionlines = Vec::new();
                sectionparsedlines = Vec::new();
                continue;
            }

            if sectiontitle.is_empty() {
                continue;
            }

            sectionlines.push(AttributeValue::new(&line));

            if let Some(match_shared) = self.parsedlineregex.search(line.clone(), 0, -1) {
                let match_unique = unsafe { match_shared.assume_unique() };
                let key = match_unique.get_string(1).to_string();
                let mut value = match_unique.get_string(2).to_string();

                if value.len() >= 2 && value.starts_with('"') && value.ends_with('"') {
                    value = value[1..(value.len() - 1)].to_string();
                }

                sectionparsedlines.push((key, AttributeValue::new(&value)));
            }
        }

        TextFile::new(file.get_path().to_string(), sections)
    }
}
