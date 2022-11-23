/*
   Appellation: utils <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use std::io::{self, BufRead, BufReader};

pub fn convert_hash_into_binary(hash: &[u8]) -> Vec<u8> {
    let mut res: String = String::default();
    for c in hash {
        res.push_str(&format!("{:b}", c));
    }
    res.into_bytes()
}

/// From the given path, open the file and gathers its contents into a vector
pub fn file_to_vec(filename: String) -> io::Result<Vec<String>> {
    let file_in = std::fs::File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}
