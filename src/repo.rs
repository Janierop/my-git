use std::{fs, io::Read};
use flate2::{read::{ZlibDecoder, ZlibEncoder}, Compression};
use crate::objects::SerializedGitObject;

pub struct LocalRepo {
    root: String,
}

impl LocalRepo {
    pub fn new(root: Option<String>) -> Self {
        match root {
            Some(path) => LocalRepo { root: path },
            None => LocalRepo { root: ".".to_owned() },
        }
    }
    // fetch data from the repo db and decompresses it
    pub fn fetch(&self, hash: &str) -> SerializedGitObject {
        let path = format!("{}/.git/objects/{}/{}",self.root, &hash[..2], &hash[2..]);
        let file = fs::File::open(path).unwrap();
        let mut decoder = ZlibDecoder::new(file);
        let mut decoded = Vec::new();
        decoder.read_to_end(&mut decoded).unwrap();
        SerializedGitObject::from(decoded)
    }
    // store data as a file in the repo db
    pub fn store(&self, object: &SerializedGitObject) -> String {
        let hash = object.calculate_hash();
        let bytes = object.bytes();

        //compress and stoZlibDecoder
        let mut encoder = ZlibEncoder::new(&bytes[..], Compression::fast());
        let mut encoded_content = Vec::with_capacity(bytes.len() / 2);
        let _ = encoder.read_to_end(&mut encoded_content);
        let dir = &hash[..2];
        let file_name = &hash[2..];
        let _ = fs::create_dir_all(format!("{}/.git/objects/{}", self.root, dir));
        fs::write(
            format!("{}/.git/objects/{}/{}", self.root, dir, file_name),
            encoded_content,
        )
        .unwrap();

        hash
    }
}