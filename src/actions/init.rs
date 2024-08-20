use serde_json::{json, Map, Value};
use std::{
    collections::{HashMap, HashSet},
    fs::{self, DirEntry},
    io::Read,
    path::Path,
};
use xxhash_rust::xxh3::xxh3_128;

use crate::{
    constants::{COMPRESS_FLAG, LZ4_FLAG},
    utils::{lz4_compress, zlib_compress},
};

#[derive(PartialEq)]
enum CompressionType {
    ZLIB,
    LZ4,
}

pub struct InitAction {
    ignore: HashSet<String>,
    compression_type: CompressionType,
}

impl InitAction {
    fn handle_dir(&self, parent: &str, dirname: String, json: &mut Map<String, Value>) {
        let new_parent = parent.to_owned() + &dirname + "/";

        let current_dir = fs::read_dir(&new_parent)
            .unwrap_or_else(|_| panic!("Failed to read directory: {new_parent}"));

        let mut structure_json = json!({});
        let obj_o = structure_json.as_object_mut();
        let obj = obj_o.unwrap();

        for file in current_dir.flatten() {
            self.handle_item(&new_parent, &file, obj);
        }

        json.insert(dirname, json!(obj));
    }

    fn write_object_file(&self, hash_value: &String, contents: &[u8]) {
        let dir_name: String = hash_value.chars().take(2).collect();
        let full_dir_name = "./.gud/objects/".to_owned() + &dir_name;

        if !Path::new(&full_dir_name).exists() {
            match fs::create_dir(&full_dir_name) {
                Ok(()) => {}
                Err(e) => {
                    panic!("Failed to initialize directory: {full_dir_name} with error: {e}");
                }
            }
        }

        let filename = full_dir_name + "/" + hash_value;

        let compressed = if self.compression_type == CompressionType::LZ4 {
            match lz4_compress(contents) {
                Ok(comp) => comp,
                Err(e) => {
                    println!("Failed to compress object file: {filename} with error: {e}");
                    contents.to_vec()
                }
            }
        } else {
            zlib_compress(contents)
        };

        match fs::write(&filename, compressed) {
            Ok(()) => {}
            Err(e) => {
                panic!("Failed to write object file: {filename} with error: {e}");
            }
        }
    }

    fn handle_file(&self, parent: &str, filename: String, json: &mut Map<String, Value>) {
        let path = parent.to_owned() + &filename;
        let contents_r = fs::read_to_string(&path);

        match contents_r {
            Ok(contents) => {
                let contents_bytes = contents.as_bytes();
                let hash_value = xxh3_128(contents_bytes).to_string();

                self.write_object_file(&hash_value, contents_bytes);
                json.insert(filename, hash_value.into());
            }
            Err(e) => {
                println!("Failed to read file: {path} with error: {e}");
            }
        }
    }

    fn handle_item(&self, parent: &str, file: &DirEntry, json: &mut Map<String, Value>) {
        // We explicitly want the program to panic if this fails, so that we can break out of whatever
        // parent loop we're currently in.
        let filename = file.file_name().into_string().unwrap();

        let new_path = parent.to_owned() + &filename;
        if self.ignore.contains(&new_path) {
            return;
        }

        if let Ok(file_type) = file.file_type() {
            if file_type.is_file() {
                self.handle_file(parent, filename, json);
            } else if file_type.is_dir() {
                self.handle_dir(parent, filename, json);
            }
        }
    }

    pub fn run(&self, name: &String) {
        let current_dir = fs::read_dir(".").expect("Failed to read directory: .");

        // TODO: panic if these fail
        let _ = fs::create_dir("./.gud");
        let _ = fs::create_dir("./.gud/objects");

        let mut structure_json = json!({name: {}});
        let obj_o = structure_json[name].as_object_mut();
        let obj = obj_o.unwrap();

        for file in current_dir.flatten() {
            self.handle_item("./", &file, obj);
        }

        let _ = fs::write("./.gud/hash", structure_json.to_string());
    }

    pub fn new(ignore: HashSet<String>, args_map: &HashMap<String, String>) -> InitAction {
        if args_map.contains_key(COMPRESS_FLAG) && args_map[COMPRESS_FLAG] == LZ4_FLAG {
            return InitAction {
                ignore,
                compression_type: CompressionType::LZ4,
            };
        }

        InitAction {
            ignore,
            compression_type: CompressionType::ZLIB,
        }
    }
}
