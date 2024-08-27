use std::{collections::HashMap, fs};

use serde_json::{json, Value};

use crate::{
    command_line_processor::FlagOption,
    constants::LZ4_FLAG,
    utils::{lz4_compress, zlib_compress},
};

use super::{action::Action, types::CompressionType};

pub struct DiffAction {
    compression_type: CompressionType,
    filename: String,
    flags: HashMap<FlagOption, Vec<String>>,
}

impl DiffAction {
    pub fn file_has_changed(&self, curr: &[u8], orig: String) -> bool {
        let curr_hash = if self.compression_type == CompressionType::Lz4 {
            lz4_compress(curr).unwrap()
        } else {
            zlib_compress(curr)
        };

        curr_hash == orig.as_bytes().to_vec()
    }

    pub fn get_original_hash(&self, json_struct: &Value) -> String {
        // naive split on '/' for now until i decide on a better way of doing it
        // TODO: also assumes we're currently in the root directory of the project
        let mut path = self.filename.split("/").peekable();
        let mut current_json = json_struct;
        while let Some(str) = path.next() {
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
                    panic!("Failed to get original hash for {}", self.filename);
                }
            }

        }

        // No original hash found, entirely new file
        "".to_string()
    }

    pub fn new(flags: HashMap<FlagOption, Vec<String>>) -> Self {
        let mut compression_type: CompressionType = CompressionType::Default;
        if flags.contains_key(&FlagOption::Compression) {
            if let Some(val) = flags[&FlagOption::Compression].first() {
                if val == LZ4_FLAG {
                    compression_type = CompressionType::Lz4;
                }
            }
        }

        match flags[&FlagOption::Name].first() {
            Some(name) => Self {
                compression_type,
                filename: name.to_string(),
                flags,
            },
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

        let mut json_struct: Value = match fs::read_to_string("./.gud/hash") {
            Ok(obj) => {
                // TODO: error handling if this fails
                serde_json::from_str(obj.as_str()).unwrap()
            }
            Err(e) => {
                println!("Encountered error {e} while trying to read gud hash file");
                return;
            }
        };

        // TODO: might want to restructure the original json so that the repo name is stored
        // somewhere else, otherwise we're going to be doing this a lot
        let repo_name = if let Some(map) = json_struct.as_object() {
            // TODO: error handling
            map.keys().next().unwrap()
        } else {
            panic!("Unable to properly deserialize hash object");
        };
        json_struct = json_struct.get(repo_name).unwrap().clone();

        let orig = self.get_original_hash(&json_struct);
        println!("Original hash: {orig}");
        if self.file_has_changed(curr.as_bytes(), orig) {
            // do diff
        } else {
            println!("\n");
        }
    }
}
