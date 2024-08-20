use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{self, Result};

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

pub fn process_args(args: &[String]) -> HashMap<String, String> {
    let mut args_map = HashMap::new();

    for i in 2..args.len() {
        if i < args.len() - 1 {
            args_map.insert(args[i].clone(), args[i+1].clone());
        } 
    }

    args_map
}

pub fn lz4_compress(mut uncompressed: &[u8]) -> Result<Vec<u8>>{
    let writer: Vec<u8> = vec![];
    let mut encoder = EncoderBuilder::new().build(writer)?;
    io::copy(&mut uncompressed, &mut encoder)?;
    let (output, _) = encoder.finish();

    Ok(output)
}

pub fn lz4_decompress(compressed: &[u8]) -> Result<Vec<u8>> {
    let mut decoder = Decoder::new(compressed)?;
    let mut output: Vec<u8> = vec![];
    io::copy(&mut decoder, &mut output)?;

    Ok(output)
}

pub fn zlib_compress(uncompressed: &[u8]) -> Vec<u8> {
    compress_to_vec(uncompressed, 6)
}

// pub fn zlib_decompress(compressed: &[u8])-> Result<Vec<u8>, DecompressError> {
//     return decompress_to_vec_with_limit(compressed, 5000000);
// }
