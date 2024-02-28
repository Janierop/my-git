// use std::env;
use std::fs;
use clap::{Parser,Subcommand};

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
        sha: String,
    },
}


fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let args = CLi::parse();

    match &args.command {
        Commands::Init => {
            fs::create_dir(".git").unwrap();
            fs::create_dir(".git/objects").unwrap();
            fs::create_dir(".git/refs").unwrap();
            fs::write(".git/HEAD", "ref: refs/heads/master\n").unwrap();
            println!("Initialized git directory")
        },
        Commands::CatFile { sha: _ } => todo!()
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
