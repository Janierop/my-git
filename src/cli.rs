use clap::{Parser,Subcommand};
use crate::init::init;
use crate::cat_file::cat_file;
use crate::hash_object::hash_object;

#[derive(Parser)]
#[command(name = "mygit")]
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

pub fn parse() {
    let args = CLi::parse();

    match &args.command {
        Commands::Init => init(),
        Commands::CatFile { hash } => cat_file(hash.to_owned()),
        Commands::HashObject { write, file_path } => hash_object(file_path.to_owned(), write)
    }
}