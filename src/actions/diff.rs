use std::fs;

use serde_json::Value;

use crate::{
    command_line_processor::{FlagOption, Options},
    constants::{LZ4_FLAG, TREE_KEY},
    utils::{lz4_compress, zlib_compress},
};

use super::{action::Action, types::CompressionType};

pub struct DiffAction {
    compression_type: CompressionType,
    filename: String,
    root_path: Vec<String>,
}

impl DiffAction {
    pub fn file_has_changed(&self, curr: &[u8], orig: &str) -> bool {
        let curr_hash = if self.compression_type == CompressionType::Lz4 {
            lz4_compress(curr).unwrap()
        } else {
            zlib_compress(curr)
        };

        curr_hash == orig.as_bytes().to_vec()
    }

    pub fn get_original_hash(&self, json_struct: &Value) -> String {
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
                        return val.to_string();
                    } else {
                        panic!("Got unknown val type: {val}");
                    }
                }
                None => {
                    panic!("Failed to get original hash for {full_filename}");
                }
            }
        }

        // No original hash found, entirely new file
        String::new()
    }

    pub fn new(options: Options) -> Self {
        let mut compression_type: CompressionType = CompressionType::Default;
        if options.flags.contains_key(&FlagOption::Compression) {
            if let Some(val) = options.flags[&FlagOption::Compression].first() {
                if val == LZ4_FLAG {
                    compression_type = CompressionType::Lz4;
                }
            }
        }

        match options.flags[&FlagOption::Name].first() {
            Some(name) => {
                Self {
                    compression_type,
                    filename: name.to_string(),
                    root_path: options.root_path,
                }
            }
            None => {
                panic!("Usage: gud diff [flags] <filename>");
            }
        }
    }
}

impl Action for DiffAction {
    fn run(&self) {
        let curr = match fs::read_to_string(&self.filename) {
            Ok(val) => val,
            Err(e) => {
                panic!(
                    "Encountered error {e} when trying to read file {}",
                    self.filename
                );
            }
        };

        let to_project_root = "../".repeat(self.root_path.len());
        let json_struct: Value = match fs::read_to_string(to_project_root + ".gud/hash") {
            Ok(obj) => {
                // TODO: error handling if this fails
                serde_json::from_str(obj.as_str()).unwrap()
            }
            Err(e) => {
                println!("Encountered error {e} while trying to read gud hash file");
                return;
            }
        };

        match json_struct.get(TREE_KEY) {
            Some(tree_struct) => {
                let orig = self.get_original_hash(tree_struct);
                //TODO: NOAH delete
                println!("Original hash: {orig}");

                if self.file_has_changed(curr.as_bytes(), &orig) {
                    // do diff
                } else {
                    println!("\n");
                }
            }
            None => panic!("Error in parsing gud tree. Repository is not a gud repository."),
        }
    }
}
