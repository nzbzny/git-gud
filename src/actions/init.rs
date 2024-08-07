use std::fs::{self, DirEntry};
use serde_json::{json, Map, Value};
use xxhash_rust::xxh3::xxh3_128;

fn handle_dir(parent: &str, dirname: String, json: &mut Map<String, Value>) {
    let new_parent = parent.to_owned() + &dirname + "/";
    
    let current_dir = fs::read_dir(&new_parent).expect(format!("Failed to read directory: {}", new_parent).as_str());

    let mut structure_json = json!({});
    let obj_o = structure_json.as_object_mut();
    let obj = obj_o.unwrap(); 

    for file_r in current_dir {
        if let Ok(file) = file_r {
            handle_item(&new_parent, file, obj);
        }
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

fn handle_item(parent: &str, file: DirEntry, json: &mut Map<String, Value>) {
    // We explicitly want the program to panic if this fails, so that we can break out of whatever
    // parent loop we're currently in.
    let filename = file.file_name().into_string().unwrap();

    if let Ok(file_type) = file.file_type() {
        if file_type.is_file() {
            handle_file(parent, filename, json);
        } else if file_type.is_dir() {
            handle_dir(parent, filename, json);
        }
    }
}

pub fn init(name: &String) {
    let current_dir = fs::read_dir(".").expect("Failed to read directory: .");
    let mut structure_json = json!({name: {}});
    let obj_o = structure_json[name].as_object_mut();
    let obj = obj_o.unwrap(); 

    for file_r in current_dir {
        if let Ok(file) = file_r {
            handle_item("./", file, obj);
        }
    }

    let _ = fs::create_dir("./.gud");
    let _ = fs::write("./.gud/hash", structure_json.to_string());
}
