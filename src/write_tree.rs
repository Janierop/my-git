use std::cmp::Ordering;
use std::io::Read;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::fs::{self, File};
use crate::objects::{Blob, SerializedGitObject};
use crate::repo::LocalRepo;

pub fn write_tree(repo: &LocalRepo) {
    let hash = hash_tree(repo, repo.repo_path().clone());
    let hash_hex = hex::encode(hash.unwrap());
    println!("{}", hash_hex);
}

const GIT_IGNORE: [&str; 1] = [".git"];

// DFS hash the repo ignoring gitignored files and directories.
// Returns None if the directory is empty
fn hash_tree(repo: &LocalRepo, path: PathBuf) -> Option<[u8; 20]> {
    // get directory items.
    let entries = fs::read_dir(&path).unwrap();
    // list them 
    let mut entries = entries
        .filter_map(|v| {
            v.ok().map(
                |v| (
                    v.file_name().to_str().expect("Filename was not valid utf-8").to_owned(),
                    v.metadata().expect("Could not read file metadata"),
                )
            )
        })
        .filter(|v| {
            // if match any gitignored name then ignore
            let name = &v.0[..];
            !GIT_IGNORE.contains(&name)
        })
        .collect::<Vec<_>>();
    // return None if no non-ignored entries
    if entries.len() < 1 {
        return None;
    }
    // sort the vec because we need to DFS in lexographical order
    entries.sort_by(
        |a, b| 
            base_name_compare(&a.0, a.1.is_dir(), &b.0, b.1.is_dir())
    );
    // if is directory recursively call hashtree
    let entries_with_hash = entries.into_iter().map(
        |(name, metadata)| {
            let mode = if metadata.is_dir() {
                "40000"
            } else if metadata.is_symlink() {
                "120000"
            } else if
                /* Git only tracks the executable permissions of the owner and
                nothing else. */
                (metadata.permissions().mode() & 0o100) != 0 
            {
                "100755"
            } else {
                "100644"
            };

            let sub_path = path.join(&name);

            let hash = if metadata.is_dir() {
                // hash the folder with a recursive call
                hash_tree(repo, sub_path)
            } else {
                // hash file
                let mut file = File::open(sub_path).unwrap();
                let mut data = Vec::new();
                file.read_to_end(&mut data).unwrap();
                let blob = Blob::from(data);
                // store object
                let hash = repo.store(&SerializedGitObject::from(blob));
                Some(hash)
            };

            (mode, name, hash)
        }
    )
    .filter_map(|(mode,name,hash)| {
        match hash {
            None => None,
            Some(hash) => Some((mode, name, hash)),
        }
    });
    // build tree with entry modes, names, hashes
    let mut tree_object_data = Vec::new();
    for (mode, name, hash) in entries_with_hash {
        tree_object_data.extend_from_slice(mode.as_bytes());
        tree_object_data.push(b' ');
        tree_object_data.extend_from_slice(name.as_bytes());
        tree_object_data.push(0);
        tree_object_data.extend_from_slice(&hash);
    }
    let content_len_string = tree_object_data.len().to_string();
    let data = [b"tree ", content_len_string.as_bytes(), &[0], &tree_object_data].concat(); 
    // store tree in repo and return the hash
    let tree_hash = repo.store(&SerializedGitObject::from(data));
    Some(tree_hash)
}

// https://github.com/git/git/blob/e09f1254c54329773904fe25d7c545a1fb4fa920/tree.c#L99
fn base_name_compare(a: &str, a_is_dir: bool, b: &str, b_is_dir: bool) -> Ordering {
    let a = a.as_bytes();
    let b = b.as_bytes();
    let len = a.len().min(b.len());
    let cmp = (&a[..len]).cmp(&b[..len]);
    match cmp {
        Ordering::Equal => {
            let c1 = match a.get(len) {
                Some(&v) => v,
                None if a_is_dir => b'/',
                _ => 0,
            };
            let c2 = match b.get(len) {
                Some(&v) => v,
                None if b_is_dir => b'/',
                _ => 0,
            };
            c1.cmp(&c2)
        },
        _ => cmp
    }
}

#[cfg(test)]
#[test]
fn a() {
    use std::os::unix::fs::PermissionsExt;

    if let Ok(entries) = fs::read_dir(".") {
        for entry in entries {
            if let Ok(entry) = entry {
                let meta = entry.metadata().unwrap();
                println!("{} {:b}", entry.file_name().to_string_lossy(), meta.permissions().mode())
            }
        }
    }
}

#[cfg(test)]
#[test]
fn test2() {
    let a  = &[0u8,1,2,3,4,5,6][..];
    let b = &[0,1,2,3,4,5][..];
    let c = a.cmp(&b);
    dbg!(c);
}
