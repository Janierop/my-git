use crate::{
    repo::LocalRepo,
    objects::GitObject,
    objects::PrettyPrintable,
};

/**
 * Print the contents of the git object
 */
pub fn cat_file_print(repo: LocalRepo, hash: &str) {
    let object = repo.fetch(hash).deserialize();

    let text = match object {
        GitObject::Blob(blob) => blob.pretty_print(),
        GitObject::Tree(_) => todo!(),
        GitObject::Commit(_) => todo!(),
    };
    
    print!("{}", text)
}

/**
 * Print the size if the git object in bytes
 */
pub fn cat_file_size(repo: LocalRepo, hash: &str) {
    let object = repo.fetch(hash);
    let size_bytes = object.get_size_bytes();
    println!("{}", String::from_utf8_lossy(size_bytes));
}

/**
 * Print the object type.
 */
pub fn cat_file_type(repo: LocalRepo, hash: &str) {
    let object = repo.fetch(hash);
    let file_type_bytes = object.get_type_bytes();
    println!("{}", String::from_utf8_lossy(file_type_bytes));
}

pub fn ls_tree(repo: LocalRepo, hash: &str) {
    let object = repo.fetch(hash).deserialize();
    match object {
        GitObject::Tree(tree) => {
            let mut names: Vec<_> = tree.entries()
            .iter()
            .map(|e| e.get_path())
            .collect();
            names.sort();
            for entry in names {
                println!("{}", entry)
            }
        },
        _ => panic!("Command 'ls-tree' can only be used with git trees.")
    }
}
