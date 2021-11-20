use crate::core::error::DataError;
use crate::core::attribute_value::AttributeValue;
use crate::core::regex::{RegEx, RegExFlags};
use crate::io::text_file::TextFile;
use crate::io::text_section::TextSection;
use std::path::PathBuf;
use gdnative::prelude::*;
use gdnative::api::file::File;

pub struct FileSystem {
    titleregex: RegEx,
    parsedlineregex: RegEx,
}

impl FileSystem {
    #[allow(unused_must_use)]
    pub fn new() -> Self {
        let titleregex = RegEx::new(r"^\s*\[(.+?)\]\s*$", RegExFlags::IgnoreCase);
        let parsedlineregex = RegEx::new(r"^\s*(.+?)\s*=\s*(.+?)\s*$", RegExFlags::IgnoreCase);

        FileSystem {
            titleregex: titleregex,
            parsedlineregex: parsedlineregex,
        }
    }

    pub fn does_file_exist(&self, filepath: &str) -> bool {
        let file = File::new();
        return file.file_exists(filepath);
    }

    pub fn get_path_by_refferrer(&self, name: &str, referrer: &str) -> String {
        let mut directory = self.get_directory(referrer);
        let mut path = self.combine_paths(&directory, name);

        for _ in 0..1 {
            if !self.does_file_exist(&path) {
                directory = self.get_directory(&directory);
                path = self.combine_paths(&directory, name);
                continue;
            }

            break;
        }

        path
    }

    pub fn combine_paths(&self, lhs: &str, rhs: &str) -> String {
        return format!("{}/{}", lhs.trim_end_matches('/'), rhs.trim_start_matches('/'))
    }

    pub fn get_directory(&self, filepath: &str) -> String {
        let mut path_buf = PathBuf::from(filepath);
        path_buf.pop();
        path_buf.to_str().unwrap().to_string()
    }

    pub fn open_file(&self, filepath: &str) -> Result<Ref<File, Unique>, DataError> {
        let file = File::new();

        if let Err(detail) = file.open(filepath, File::READ) {
            return Result::Err(DataError::new(format!(
              "Error opening file: {}",
                detail
            )));
        }

        Result::Ok(file)
    }

    pub fn open_text_file(&self, filepath: &str) -> Result<TextFile, DataError> {
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

            if let Some(title_match) = self.titleregex.search(&line) {
                if !sectiontitle.is_empty() {
                    sections.push(TextSection::new(
                        sectiontitle.clone(),
                        sectionlines.clone(),
                        sectionparsedlines.clone()
                    ));
                }

                sectiontitle = title_match.get_string(1).to_string();
                sectionlines = Vec::new();
                sectionparsedlines = Vec::new();
                continue;
            }

            if sectiontitle.is_empty() {
                continue;
            }

            sectionlines.push(AttributeValue::new(&line));

            if let Some(line_match) = self.parsedlineregex.search(&line) {
                let key = line_match.get_string(1).to_string();
                let value = line_match.get_string(2).to_string();

                sectionparsedlines.push((key, AttributeValue::new(&value)));
            }
        }

        if !sectiontitle.is_empty() {
            sections.push(TextSection::new(
                sectiontitle.clone(),
                sectionlines.clone(),
                sectionparsedlines.clone()
            ));
        }

        TextFile::new(file.get_path().to_string(), sections)
    }
}
