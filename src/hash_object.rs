// use std::env;
use std::{fs, io::Read};
use flate2::read::ZlibEncoder;
use flate2::Compression;
use hex::ToHex;
use sha1::{Digest, Sha1};

pub fn hash_object(file_path: String, write: &bool) {
    let mut file = fs::File::open(file_path).expect("Error: File not found");
    let mut content = Vec::new();
    let content_length = file.read_to_end(&mut content).unwrap();
    let final_content = [b"blob ", content_length.to_string().as_bytes(), &[0], &content].concat();

    let mut hasher = Sha1::new();
    hasher.update(final_content.clone());
    let hash: String = hasher.finalize().encode_hex();

    
    // write only if write flag is specified
    if *write {
        // encode zlib
        let mut encoder = ZlibEncoder::new(&final_content[..], Compression::fast());
        let mut encoded_contents = Vec::new();
        encoder.read_to_end(&mut encoded_contents).unwrap();

        let dir = &hash[..2];
        let file_name = &hash[2..];
        let _ = fs::create_dir(format!(".git/objects/{}", dir)); // Returns Error if dir already exists but doesn't panic
        fs::write(format!(".git/objects/{}/{}", dir, file_name), encoded_contents).unwrap();
    }

    println!("{}", hash)
}
