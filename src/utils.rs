use std::collections::HashSet;
use std::fs::{self, ReadDir};
use std::io;

use lz4::{Decoder, EncoderBuilder};
use miniz_oxide::deflate::compress_to_vec;
use miniz_oxide::inflate::{decompress_to_vec_with_limit, DecompressError};

pub fn process_ignore_file() -> HashSet<String> {
    let contents_r = fs::read_to_string(".gudignore");

    let mut ignore: HashSet<String> = HashSet::new();
    ignore.insert("./.gud".to_string());

    if let Ok(contents) = contents_r {
        let lines = contents.lines();

        for line in lines {
            ignore.insert("./".to_owned() + line.trim().trim_end_matches('/'));
        }
    }

    ignore
}

pub fn lz4_compress(mut uncompressed: &[u8]) -> std::io::Result<Vec<u8>> {
    let writer: Vec<u8> = vec![];
    let mut encoder = EncoderBuilder::new().build(writer)?;
    io::copy(&mut uncompressed, &mut encoder)?;
    let (output, _) = encoder.finish();

    Ok(output)
}

pub fn lz4_decompress(compressed: &[u8]) -> std::io::Result<Vec<u8>> {
    let mut decoder = Decoder::new(compressed)?;
    let mut output: Vec<u8> = vec![];
    io::copy(&mut decoder, &mut output)?;

    Ok(output)
}

pub fn zlib_compress(uncompressed: &[u8]) -> Vec<u8> {
    compress_to_vec(uncompressed, 6)
}

pub fn zlib_decompress(compressed: &[u8]) -> core::result::Result<Vec<u8>, DecompressError> {
    // TODO: pick a good number
    decompress_to_vec_with_limit(compressed, 5_000_000)
}

fn is_project_root(mut dir: ReadDir) -> bool {
    dir.any(|file_r| {
        file_r.is_ok_and(|file| {
            file.file_type().is_ok_and(|file_type| file_type.is_dir()) && file.file_name() == ".gud"
        })
    })
}

pub fn find_path_from_project_root() -> Vec<String> {
    let mut path_to_current_dir = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(e) => panic!(
            "Encountered error {e} when trying to get path to working directory. Terminating"
        ),
    };

    let mut root_path: Vec<String> = vec![];

    while let Ok(dir) = path_to_current_dir.read_dir() {
        if is_project_root(dir) {
            return root_path;
        }

        if let Some(dir_name) = path_to_current_dir.file_name() {
            if let Some(dir_str) = dir_name.to_str() {
                root_path.insert(0, dir_str.to_string() + "/");

                if let Some(path) = path_to_current_dir.parent() {
                    path_to_current_dir = path.to_path_buf();

                    // this is the only case where we don't want to panic
                    continue;
                }
            }
        }

        panic!("Unable to get information for directory {}", path_to_current_dir.to_str().unwrap_or("UNKNOWN_DIRECTORY_PATH"));
    }

    root_path
}
