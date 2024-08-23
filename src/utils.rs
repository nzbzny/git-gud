use std::collections::HashSet;
use std::fs;
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
