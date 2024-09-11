use std::fs;

use serde_json::Value;
use xxhash_rust::xxh3::xxh3_128;

use crate::{
    command_line_processor::{FlagOption, Options},
    constants::{
        COMPRESSION_TYPE_KEY, GUD_FOLDER, GUD_FOLDER_NAME, GUD_INFO_FILENAME,
        GUD_OBJECTS_FOLDER_NAME, LZ4_FLAG, TREE_KEY, ZLIB_FLAG,
    },
    utils::{get_object_file_dir_name, get_path_to_project_root},
};

use super::{action::Action, types::CompressionType};

pub struct DiffAction {
    compression_type: CompressionType,
    filename: String,
    root_path: Vec<String>,
    json_struct: Value,
}

impl DiffAction {
    fn file_has_changed(curr: &[u8], orig: &str) -> bool {
        xxh3_128(curr).to_string() != orig
    }

    fn get_original_hash(&self, json_struct: &Value) -> String {
        // naive split on '/' for now until i decide on a better way of doing it
        let full_filename = self.root_path.join("") + &self.filename;
        let path = full_filename.split('/').peekable();
        let mut current_json = json_struct;

        for str in path {
            match current_json.get(str) {
                Some(val) => {
                    if val.is_object() {
                        current_json = val;
                    } else if val.is_string() {
                        return val.to_string().trim_matches('"').to_string();
                    } else {
                        panic!("Got unknown val type: {val}");
                    }
                }
                None => break,
            }
        }

        // No original hash found, entirely new file
        String::new()
    }

    fn get_original_text(&self, orig_hash: String) -> String {
        let to_project_root = get_path_to_project_root(&self.root_path);
        // TODO: replace all of the  `dir + "/" + dir2 + "/" + dir3` with this style
        // TODO: use a method with varargs instead of this
        let obj_file_name = vec![
            to_project_root,
            GUD_FOLDER_NAME.to_string(),
            GUD_OBJECTS_FOLDER_NAME.to_string(),
            get_object_file_dir_name(&orig_hash),
            orig_hash,
        ]
        .join("/");

        let compressed_str = fs::read_to_string(obj_file_name).expect("Failed to read object file");
        // TODO: decompress

        String::new()
    }

    fn perform_diff(&self, curr_text: String, orig_text: String) {}

    pub fn new(options: Options) -> Self {
        let to_project_root = get_path_to_project_root(&options.root_path);
        let info_obj = fs::read_to_string(to_project_root + GUD_FOLDER + GUD_INFO_FILENAME)
            .expect("Encountered error while attempting to read gud hash file");

        let json_struct: Value = serde_json::from_str(info_obj.as_str())
            .expect("Encountered error while attempting to parse gud hash json object");

        let compression_type = match json_struct.get(COMPRESSION_TYPE_KEY) {
            Some(ct) => {
                assert!(
                    ct.is_string(),
                    "Unable to identify compression type from info file compression_type={ct}"
                );

                match ct.as_str() {
                    Some(LZ4_FLAG) => CompressionType::Lz4,
                    Some(ZLIB_FLAG) => CompressionType::Zlib,
                    _ => CompressionType::Default,
                }
            }
            None => CompressionType::Default,
        };

        match options.flags[&FlagOption::Name].first() {
            Some(name) => Self {
                compression_type,
                filename: name.to_string(),
                root_path: options.root_path,
                json_struct,
            },
            None => {
                panic!("Usage: gud diff [flags] <filename>");
            }
        }
    }
}

impl Action for DiffAction {
    fn run(&self) {
        let curr_text = fs::read_to_string(&self.filename).expect(&format!(
            "Encountered error when trying to read file {}",
            self.filename
        ));

        match self.json_struct.get(TREE_KEY) {
            Some(tree_struct) => {
                let orig_hash = self.get_original_hash(tree_struct);

                if !orig_hash.is_empty() && Self::file_has_changed(curr_text.as_bytes(), &orig_hash)
                {
                    println!("file has changed");
                    let orig_text = self.get_original_text(orig_hash);

                    self.perform_diff(curr_text, orig_text);
                } else {
                    println!("\n");
                }
            }
            None => panic!("Error in parsing gud tree. Repository is not a gud repository."),
        }
    }
}
