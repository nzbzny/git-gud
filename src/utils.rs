use std::collections::HashSet;
use std::fs;
use std::io::{self, Result};

use lz4::{Decoder, EncoderBuilder};

pub fn process_ignore_file() -> HashSet<String> {
    let contents_r = fs::read_to_string(".gudignore");

    if let Ok(contents) = contents_r {
        let lines = contents.lines();
        let mut ignore: HashSet<String> = HashSet::new();

        for line in lines {
            ignore.insert("./".to_owned() + line.trim().trim_end_matches('/'));
        }

        return ignore;
    }

    HashSet::new()
}

pub fn compress(mut uncompressed: &[u8]) -> Result<Vec<u8>>{
    let writer: Vec<u8> = vec![];
    let mut encoder = EncoderBuilder::new().build(writer)?;
    io::copy(&mut uncompressed, &mut encoder)?;
    let (output, _) = encoder.finish();

    Ok(output)
}

pub fn decompress(compressed: &[u8]) -> Result<Vec<u8>> {
    let mut decoder = Decoder::new(compressed)?;
    let mut output: Vec<u8> = vec![];
    io::copy(&mut decoder, &mut output)?;

    Ok(output)
}
