use super::git_object::SerializedGitObject;

pub struct Tree {
    entries: Vec<TreeEntry>,
}

impl Tree {
    pub fn entries(&self) -> &Vec<TreeEntry> {
        &self.entries
    }
}

impl From<SerializedGitObject> for Tree {
    fn from(value: SerializedGitObject) -> Self {
        let mut tree_entries = Vec::new();
        let mut remaining = &value.bytes[..];
        while let Some(null_pos) = remaining.iter().position(|&b| b == 0) {
            let entry_head = &remaining[..null_pos];
            let (entry_mode, entry_path) = std::str::from_utf8(entry_head)
                .unwrap()
                .split_once(' ')
                .unwrap();
            let hash_start = null_pos + 1;
            let hash = &remaining[hash_start .. hash_start + 20];
            let mut hash_array = [0; 20];
            hash_array.copy_from_slice(hash);
            tree_entries.push(
                TreeEntry {
                    mode: entry_mode.to_owned(),
                    path: entry_path.to_owned(),
                    hash: hash_array,
                }
            );
            remaining = &remaining[hash_start + 20 ..];
        }
        Tree { entries: tree_entries }
    }
}

pub struct TreeEntry {
    mode: String,
    path: String,
    hash: [u8; 20],
}

impl TreeEntry {
    pub fn get_path(&self) -> &str {
        self.path.as_str()
    }
}