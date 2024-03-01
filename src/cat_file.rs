// use std::env;
use std::{fs, io::Read};
use flate2::read::ZlibDecoder;

pub fn cat_file(hash: String) {
    // first two characters of the hash is the subdirectory name
    let sub_dir = &hash[..2];
    // remaining 38 characters is the name of the file containing the content
    let file_name = &hash[2..];
    let path = format!(".git/objects/{}/{}", sub_dir, file_name);
    let file = fs::File::open(path).expect("File not found.");
    let mut decoder = ZlibDecoder::new(file);
    let mut content_buffer = Vec::new();
    decoder.read_to_end(&mut content_buffer).unwrap();

    // git gives blobs a header that ends with a null bite
    let split_index = content_buffer.iter().position(|&b| b == 0).expect("Could not find a zero byte");
    let (_head, content) = content_buffer.split_at(split_index);
    // trim the first byte since its a zero byte
    let content = &content[1..];

    // use String::from_utf8_lossy to replace invalid utf8 chars which is what git also does
    let content = String::from_utf8_lossy(&content);
    
    print!("{}", content)
}
