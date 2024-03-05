// use std::env;
use std::{fs, io::{self, Read}};
use flate2::read::ZlibDecoder;

/**
 * Print the contents of the git object
 */
pub fn cat_file_print(hash: &str) {
    let data = read_object(&hash).unwrap();

    let (_, content) = split_object(&data);

    // use String::from_utf8_lossy to replace invalid utf8 chars which is what git also does
    let content = String::from_utf8_lossy(&content);
    
    print!("{}", content)
}

/**
 * Print the size if the git object in bytes
 */
pub fn cat_file_size(hash: &str) {
    let data = read_object(&hash).unwrap();
    let (header, _) = split_object(&data);
    let (_, size) = split_header(header);
    println!("{}", String::from_utf8_lossy(size));
}

/**
 * Print the object type.
 */
pub fn cat_file_type(hash: &str) {
    let data = read_object(&hash).unwrap();
    let (header, _) = split_object(&data);
    let (object_type, _) = split_header(header);
    println!("{}", String::from_utf8_lossy(object_type));
}

pub fn ls_tree(hash: &str) {
    let data = read_object(&hash).unwrap();
    let (header, content) = split_object(&data);
    let (_object_type, _size) = split_header(header);

    let mut entries = Vec::new();
    let mut remainder = content;
    while let Some(null_index) = remainder.iter().position(|&b| b == 0) {
        let entry_header = &remainder[..null_index];
        let (entry_type, path) = std::str::from_utf8(entry_header).unwrap().split_once(' ').unwrap();
        let hash = &remainder[null_index + 1 .. null_index + 21];
        entries.push((entry_type, path, hash));
        remainder = &remainder[null_index + 21 ..];
    }
    // sort by alpha betical name
    entries.sort_by_key(|a| a.1 );
    for (_, path, _) in entries {
        println!("{}", path);
    }
}

/**
 * Reads the contents of the objet file and decompresses it
 */
fn read_object(hash: &str) -> Result<Vec<u8>, io::Error> {
    // first two characters of the hash is the subdirectory name
    let sub_dir = &hash[..2];
    // remaining 38 characters is the name of the file containing the content
    let file_name = &hash[2..];
    let path = format!(".git/objects/{}/{}", sub_dir, file_name);
    let file = fs::File::open(path)?;
    let mut decoder = ZlibDecoder::new(file);
    let mut content_buffer = Vec::new();
    decoder.read_to_end(&mut content_buffer)?;
    Ok(content_buffer)
}

/**
 * Git objects consist of a header and the contents separated by a zero byte
 */
fn split_object(data: &[u8]) -> (&[u8], &[u8]) {
    // git gives blobs a header that ends with a null bite
    let split_index = data.iter().position(|&b| b == 0).expect("Could not find a zero byte");
    // leave out the zero byte
    (&data[..split_index], &data[split_index + 1..])
}

/**
 * Git object headers consist of a type and size in bytes separated by a whitespace.
 */
fn split_header(header: &[u8]) -> (&[u8], &[u8]) {
    let split_index = header.iter().position(|&b| b == b' ').unwrap();
    (&header[..split_index], &header[split_index + 1..])
}