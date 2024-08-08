use serde_json::{json, Map, Value};
use std::{
    collections::HashSet,
    fs::{self, DirEntry},
};
use xxhash_rust::xxh3::xxh3_128;

pub struct InitAction {
    ignore: HashSet<String>,
}

impl From<HashSet<String>> for InitAction {
    fn from(ignore: HashSet<String>) -> InitAction {
        Self { ignore }
    }
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

    fn handle_file(parent: &str, filename: String, json: &mut Map<String, Value>) {
        let path = parent.to_owned() + &filename;
        let contents_r = fs::read_to_string(&path);

        match contents_r {
            Ok(contents) => {
                let hash_value: u128 = xxh3_128(contents.as_bytes());
                json.insert(filename, hash_value.to_string().into());
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
                Self::handle_file(parent, filename, json);
            } else if file_type.is_dir() {
                self.handle_dir(parent, filename, json);
            }
        }
    }

    pub fn run(&self, name: &String) {
        let current_dir = fs::read_dir(".").expect("Failed to read directory: .");
        let mut structure_json = json!({name: {}});
        let obj_o = structure_json[name].as_object_mut();
        let obj = obj_o.unwrap();

        for file in current_dir.flatten() {
            self.handle_item("./", &file, obj);
        }

        let _ = fs::create_dir("./.gud");
        let _ = fs::write("./.gud/hash", structure_json.to_string());
    }
}
