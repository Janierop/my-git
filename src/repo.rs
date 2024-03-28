use std::{fs, io::Read, path::PathBuf};
use flate2::{read::{ZlibDecoder, ZlibEncoder}, Compression};
use crate::objects::SerializedGitObject;

pub struct LocalRepo {
    root: PathBuf,
}

impl LocalRepo {
    pub fn new(root: Option<PathBuf>) -> Self {
        match root {
            Some(path) => LocalRepo { root: path },
            None => LocalRepo { root: PathBuf::from(r"./") },
        }
    }
    pub fn repo_path(&self) -> &PathBuf {
        &self.root
    }
    // fetch data from the repo db and decompresses it
    pub fn fetch(&self, hash: &str) -> SerializedGitObject {
        // let path = format!("{}/.git/objects/{}/{}", self.root.display(), &hash[..2], &hash[2..]);
        let path: PathBuf = [self.root.to_str().unwrap(), r".git/objects/", &hash[..2], &hash[2..]].iter().collect();
        let file = fs::File::open(path).unwrap();
        let mut decoder = ZlibDecoder::new(file);
        let mut decoded = Vec::new();
        decoder.read_to_end(&mut decoded).unwrap();
        SerializedGitObject::from(decoded)
    }
    // store data as a file in the repo db
    pub fn store(&self, object: &SerializedGitObject) -> [u8; 20] {
        let hash = object.calculate_hash();
        let hash_hex = hex::encode(&hash);
        let bytes = object.bytes();

        //compress and stoZlibDecoder
        let mut encoder = ZlibEncoder::new(&bytes[..], Compression::fast());
        let mut encoded_content = Vec::with_capacity(bytes.len() / 2);
        let _ = encoder.read_to_end(&mut encoded_content);
        let dir = &hash_hex[..2];
        let file_name = &hash_hex[2..];
        let path: PathBuf = [self.root.to_str().unwrap(), r".git/objects/", dir].iter().collect();
        let _ = fs::create_dir_all(&path);
        let mut path = path.clone();
        path.push(file_name);
        dbg!(hash_hex);
        fs::write(
            path,
            encoded_content,
        )
        .unwrap();
        hash
    }
}