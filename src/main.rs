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
        #[arg(value_name = "blob_sha")]
        hash: String,
    },
    #[command(name = "hash-object")]
    HashObject {
        #[arg(short = 'w')]
        write: bool,

        #[arg(value_name = "file")]
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

fn hash_object(file_path: String, write: &bool) {
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

fn main() {
    let args = CLi::parse();

    match &args.command {
        Commands::Init => init(),
        Commands::CatFile { hash } => cat_file(hash.to_owned()),
        Commands::HashObject { write, file_path } => hash_object(file_path.to_owned(), write)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        
    }
}