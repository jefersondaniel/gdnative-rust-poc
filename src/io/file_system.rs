use crate::core::error::DataError;
use crate::core::attribute_value::AttributeValue;
use crate::core::regex::{RegEx, RegExFlags};
use crate::io::text_file::TextFile;
use crate::io::text_section::TextSection;
use std::ffi::OsStr;
use std::path::{PathBuf, Path};
use gdnative::prelude::*;
use gdnative::api::file::File;

pub struct FileSystem {}

impl FileSystem {
    pub fn new() -> Self {
        FileSystem {}
    }

    pub fn does_file_exist(&self, filepath: &str) -> bool {
        let file = File::new();
        return file.file_exists(filepath);
    }

    pub fn get_path_by_refferrer(&self, name: &str, referrer: &str) -> String {
        let mut directory = self.get_directory(referrer);
        let mut path = self.combine_paths(&directory, name);

        for _ in 0..2 {
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

    pub fn get_name(&self, filepath: &str) -> String {
        let path_buff = Path::new(filepath);
        let default = OsStr::new("");
        let result = path_buff.file_name().unwrap_or(&default);
        result.to_str().unwrap_or("").to_string()
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

    pub fn open_file_as_string(&self, filepath: &str) -> Result<String, DataError> {
        let file = File::new();

        if let Err(detail) = file.open(filepath, File::READ) {
            return Result::Err(DataError::new(format!(
              "Error opening file: {}",
                detail
            )));
        }

        Ok(file.get_as_text().to_string())
    }

    pub fn open_text_file(&self, filepath: &str) -> Result<TextFile, DataError> {
        let result = self.open_file(filepath);

        match result {
            Ok(file) => Result::Ok(self.build_text_file(file)),
            Err(error) => Result::Err(error)
        }
    }

    pub fn build_text_file(&self, file: Ref<File, Unique>) -> TextFile {
        let text = file.get_as_text().to_string();

        TextFile::from_string(
            file.get_path().to_string(),
            text
        )
    }
}
