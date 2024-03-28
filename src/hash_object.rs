use std::{fs, io::Read, path::PathBuf};

use crate::{objects::{Blob, SerializedGitObject}, repo::LocalRepo};

pub fn hash_object(repo: LocalRepo, file_path: PathBuf, write: bool) {
    // PathBuff joins self with path, unless path is absolute in which case it takes over completely.
    let file_path = repo.repo_path().join(file_path);

    let mut file = fs::File::open(file_path).unwrap();
    let mut file_contents = Vec::new();
    let _ = file.read_to_end(&mut file_contents).unwrap();

    let blob = Blob::from(&file_contents);
    let blob = SerializedGitObject::from(blob);
    
    let hash = if write {
        repo.store(&blob)
    } else {
        blob.calculate_hash()
    };
    println!("{}", hex::encode(hash));
}
