// use std::env;
use std::{fs, io::Read};
use clap::{Parser,Subcommand};
use flate2::read::ZlibDecoder;

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

fn main() {
    let args = CLi::parse();

    match &args.command {
        Commands::Init => init(),
        Commands::CatFile { hash } => cat_file(hash.to_owned())
    }

    // Uncomment this block to pass the first stage
    // let args: Vec<String> = env::args().collect();
    // if args[1] == "init" {
    //     fs::create_dir(".git").unwrap();
    //     fs::create_dir(".git/objects").unwrap();
    //     fs::create_dir(".git/refs").unwrap();
    //     fs::write(".git/HEAD", "ref: refs/heads/master\n").unwrap();
    //     println!("Initialized git directory")
    // } else {
    //     println!("unknown command: {}", args[1])
    // }
}
