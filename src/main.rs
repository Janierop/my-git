// use std::env;
use std::{fs, io::Read};
use clap::{Parser,Subcommand};
use flate2::read::{ZlibDecoder, ZlibEncoder};
use flate2::Compression;
use hex::ToHex;
use sha1::{Digest, Sha1};

#[derive(Parser)]
#[command(name = "my-git")]
#[command(about = "my own implementation of git")]
struct CLi {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    /// Initializes a local repository
    #[command(name = "init")]
    Init,
    #[command(name = "cat-file")]
    CatFile {
        /// The hash of the commit to be printed
        #[arg(short = 'p')]
        #[arg(id = "blob_sha")]
        hash: String,
    },
    #[command(name = "hash-object")]
    HashObject {
        #[arg(short = 'w')]
        write: bool,

        #[arg(id = "file")]
        file_path: String,
    }
}

fn init() {
    fs::create_dir(".git").unwrap();
    fs::create_dir(".git/objects").unwrap();
    fs::create_dir(".git/refs").unwrap();
    fs::write(".git/HEAD", "ref: refs/heads/master\n").unwrap();
    println!("Initialized git directory")
}

fn cat_file(hash: String) {
    // first two characters of the hash is the subdirectory name
    let sub_dir = &hash[..2];
    // remaining 38 characters is the name of the file containing the content
    let file_name = &hash[2..];
    let path = format!(".git/objects/{}/{}", sub_dir, file_name);
    let file = fs::File::open(path).expect("File not found.");
    let mut decoder = ZlibDecoder::new(file);
    let mut string_buf = String::new();
    decoder.read_to_string(&mut string_buf).expect("Error decoding file");
    // git gives blobs a header that ends with a null bite
    let (_header, result) = string_buf.split_once("\x00").unwrap();
    print!("{}", result)
}

fn hash_object(file_path: String) {
    let mut file = fs::File::open(file_path).expect("Error: File not found");
    let mut content = String::new();
    let content_length = file.read_to_string(&mut content).unwrap();
    let final_content = format!("blob {}\x00{}", content_length, content);

    let mut hasher = Sha1::new();
    hasher.update(final_content.clone());
    let hash: String = hasher.finalize().encode_hex();

    // encode zlib
    let mut encoder = ZlibEncoder::new(final_content.as_bytes(), Compression::fast());
    let mut encoded_contents = Vec::new();
    encoder.read_to_end(&mut encoded_contents).unwrap();

    let dir = &hash[..2];
    let file_name = &hash[2..];
    let _ = fs::create_dir(format!(".git/objects/{}", dir)); // Returns Error if dir already exists but doesnt panic
    fs::write(format!(".git/objects/{}/{}", dir, file_name), encoded_contents).unwrap();

    println!("{}", hash)
}

fn main() {
    let args = CLi::parse();

    match &args.command {
        Commands::Init => init(),
        Commands::CatFile { hash } => cat_file(hash.to_owned()),
        Commands::HashObject { write: _, file_path } => hash_object(file_path.to_owned())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let v = b"Hello\x00, World!";
        let v2 = b"Hello\0, World!";
        println!("{}", v == v2);
        println!("{}", String::from_utf8_lossy(v2));
    }
}