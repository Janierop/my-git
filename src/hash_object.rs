use std::{fs, io::Read, path::PathBuf};

use crate::{objects::{Blob, SerializedGitObject}, repo::LocalRepo};

pub fn hash_object(file_path: PathBuf, write: bool) {
    let mut file = fs::File::open(file_path).unwrap();
    let mut file_contents = Vec::new();
    let _ = file.read_to_end(&mut file_contents).unwrap();

    let blob = Blob::from(&file_contents);
    let blob = SerializedGitObject::from(blob);

    let repo = LocalRepo::new(None);

    let hash = if write {
        repo.store(&blob)
    } else {
        blob.calculate_hash()
    };
    println!("{}", hash);
}
