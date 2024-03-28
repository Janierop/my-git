use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};
use crate::init::init;
use crate::cat_file::{cat_file_print, cat_file_size, cat_file_type, ls_tree};
use crate::hash_object::hash_object;
use crate::repo::LocalRepo;
use crate::write_tree;

#[derive(Parser)]
#[command(name = "mygit")]
#[command(about = "my own implementation of git")]
struct CLi {
    #[arg(short = 'C')]
    #[arg(value_name = "path")]
    repo_path: Option<PathBuf>,

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
        #[command(flatten)]
        args: CatFileArgs,

        /// The hash of the commit to be printed
        #[arg(value_name = "blob_sha")]
        hash: String,
    },
    #[command(name = "hash-object")]
    HashObject {
        #[arg(short = 'w')]
        write: bool,

        #[arg(value_name = "file")]
        file_path: PathBuf,
    },
    #[command(name = "ls-tree")]
    LSTree {
        #[arg(long = "name-only")]
        name_only: bool,

        #[arg(value_name = "tree-sha")]
        hash: String
    },
    #[command(name = "write-tree")]
    WriteTree
}

#[derive(Args)]
#[command()]
#[group(required = true, multiple = false)]
struct CatFileArgs {
    /// Pretty print contents
    #[arg(short = 'p')]
    print: bool,

    /// Print size of file in bytes
    #[arg(short = 's')]
    size: bool,

    /// Print object type
    #[arg(short = 't')]
    type_flag: bool
}

pub fn parse() {
    let args = CLi::parse();

    let repo = LocalRepo::new(args.repo_path);

    match &args.command {
        Commands::Init => init(),
        Commands::CatFile { hash, args } => {
            match (args.print, args.size, args.type_flag) {
                (true, _, _) => cat_file_print(repo, hash),
                (_, true, _) => cat_file_size(repo, hash),
                (_, _, true) => cat_file_type(repo, hash),
                _ => unreachable!()
            }
        },
        Commands::HashObject { write, file_path } => hash_object(repo, file_path.to_owned(), *write),
        Commands::LSTree { hash, name_only } if *name_only => ls_tree(repo, hash),
        Commands::WriteTree => write_tree::write_tree(&repo),
        _ => todo!()
    }
}