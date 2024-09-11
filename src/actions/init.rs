use serde_json::{json, Map, Value};
use std::{
    collections::{HashMap, HashSet},
    fs::{self, DirEntry},
    path::Path,
};
use xxhash_rust::xxh3::xxh3_128;

use crate::{
    command_line_processor::FlagOption,
    constants::{
        COMPRESSION_TYPE_KEY, DEFAULT_FLAG, GUD_FOLDER, GUD_OBJECTS_FOLDER_NAME, LZ4_FLAG,
        REPO_NAME_KEY, TREE_KEY, ZLIB_FLAG,
    },
    utils::{get_object_file_dir_name, lz4_compress, zlib_compress},
};

use super::{action::Action, types::CompressionType};

pub struct InitAction {
    ignore: HashSet<String>,
    compression_type: CompressionType,
    flags: HashMap<FlagOption, Vec<String>>,
}

impl InitAction {
    fn handle_dir(&self, parent: &str, dirname: String, json: &mut Map<String, Value>) {
        let new_parent = parent.to_owned() + &dirname + "/";

        let current_dir =
            fs::read_dir(&new_parent).expect(&format!("Failed to read directory: {new_parent}"));

        let mut structure_json = json!({});
        let obj_o = structure_json.as_object_mut();
        let obj = obj_o.unwrap();

        for file in current_dir.flatten() {
            self.handle_item(&new_parent, &file, obj);
        }

        json.insert(dirname, json!(obj));
    }

    fn write_object_file(&self, hash_value: &String, contents: &[u8]) {
        let dir_name: String = get_object_file_dir_name(hash_value);
        let full_dir_name = "./".to_owned() + GUD_FOLDER + GUD_OBJECTS_FOLDER_NAME + &dir_name;

        if !Path::new(&full_dir_name).exists() {
            fs::create_dir(&full_dir_name)
                .expect(&format!("Failed to initialize directory: {full_dir_name}"));
        }

        let filename = full_dir_name + "/" + hash_value;

        let compressed = if self.compression_type == CompressionType::Lz4 {
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

        fs::write(&filename, compressed).expect(&format!("Failed to write object file {filename}"));
    }

    fn handle_file(&self, parent: &str, filename: String, json: &mut Map<String, Value>) {
        let path = parent.to_owned() + &filename;
        let contents = fs::read_to_string(&path).expect(&format!("Failed to read file at {path}"));

        let contents_bytes = contents.as_bytes();
        let hash_value = xxh3_128(contents_bytes).to_string();

        self.write_object_file(&hash_value, contents_bytes);
        json.insert(filename, hash_value.into());
    }

    fn handle_item(&self, parent: &str, file: &DirEntry, json: &mut Map<String, Value>) {
        // We explicitly want the program to panic if this fails, so that we can break out of whatever
        // parent loop we're currently in.
        // TODO: relook at this
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

    pub fn new(ignore: HashSet<String>, flags: HashMap<FlagOption, Vec<String>>) -> InitAction {
        let mut compression_type = CompressionType::Zlib;

        if flags.contains_key(&FlagOption::Compression) {
            if let Some(val) = flags[&FlagOption::Compression].first() {
                if val == LZ4_FLAG {
                    compression_type = CompressionType::Lz4;
                }
            }
        }

        InitAction {
            ignore,
            compression_type,
            flags,
        }
    }
}

fn compression_type_to_str(compression_type: &CompressionType) -> String {
    match compression_type {
        CompressionType::Lz4 => LZ4_FLAG,
        CompressionType::Zlib => ZLIB_FLAG,
        CompressionType::Default => DEFAULT_FLAG,
    }
    .to_string()
}

impl Action for InitAction {
    fn run(&self) {
        let current_dir = fs::read_dir(".").expect("Failed to read directory: .");

        let gud_path = "./".to_owned() + GUD_FOLDER;
        fs::create_dir(&gud_path)
            .expect("Encountered error while attempting to make .gud directory");
        fs::create_dir(gud_path + GUD_OBJECTS_FOLDER_NAME)
            .expect("Encountered error while attempting to initialize objects directory");

        let name = self.flags[&FlagOption::Name].first().unwrap();

        let mut structure_json = json!({
            TREE_KEY: {},
            REPO_NAME_KEY: name,
            COMPRESSION_TYPE_KEY: compression_type_to_str(&self.compression_type)
        });

        let obj_o = structure_json[TREE_KEY].as_object_mut();
        let obj = obj_o.unwrap();

        for file in current_dir.flatten() {
            self.handle_item("./", &file, obj);
        }

        if let Err(e) = fs::write("./.gud/hash", structure_json.to_string()) {
            panic!("Encountered error {e} while attempting to write gud hash file");
        }
    }
}
