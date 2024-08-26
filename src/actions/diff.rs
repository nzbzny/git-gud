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

        String::from_utf8(curr_hash).unwrap() == orig
    }

    pub fn get_original_hash(&self, json_struct: Value) -> String {
        let path = self.filename.split("/");
        let current_json = &json_struct;
        for str in path {
            match current_json.get(str) {
                Some(val) => {

                }
                None => {}
            } 
        }


        "".to_string()
    }

    pub fn new(flags: HashMap<FlagOption, Vec<String>>) -> Self {
        let mut compression_type: CompressionType = CompressionType::Default;
        if let Some(val) = flags[&FlagOption::Compression].first() {
            if val == LZ4_FLAG {
                compression_type = CompressionType::Lz4;
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

        let json_struct = match fs::read_to_string("./.gud/hash") {
            Ok(obj) => {
                json!(obj)
                // naive split on '/' for now until i decide on a better way of doing it
                // TODO: also assumes we're currently in the root directory of the project
            }
            Err(e) => {
                println!("Encountered error {e} while trying to read gud hash file");
                return;
            }
        };

        let orig = self.get_original_hash(json_struct);
        if self.file_has_changed(curr.as_bytes(), orig) {
            // do stuff
        } else {
            println!("\n");
        }
    }
}
