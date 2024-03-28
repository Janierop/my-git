use sha1::{Digest, Sha1};
use super::{
    blob::Blob, 
    commit::Commit, 
    tree::Tree
};

pub enum GitObject {
    Blob(Box<Blob>),
    Tree(Box<Tree>),
    Commit(Box<Commit>),
}

pub trait PrettyPrintable {
    fn pretty_print(&self) -> String;
}

/// Uncompressed data that makes up a Git Object file
pub struct SerializedGitObject {
    pub(super) bytes: Vec<u8>,
}

impl SerializedGitObject {
    pub fn get_type_bytes(&self) -> &[u8] {
        let space_pos = self.bytes.iter().position(|&b| b == b' ').unwrap();
        &self.bytes[..space_pos]
    }

    pub fn get_size_bytes(&self) -> &[u8] {
        let space_pos = self.bytes.iter().position(|&b| b == b' ').unwrap();
        let null_pos = self.bytes.iter().position(|&b| b == 0).unwrap();
        &self.bytes[space_pos + 1 .. null_pos]
    }

    pub fn get_content_bytes(&self) -> &[u8] {
        let null_pos = self.bytes.iter().position(|&b| b == 0).unwrap();
        &self.bytes[null_pos + 1 ..]
    }

    pub fn calculate_hash(&self) -> [u8; 20] {
        let mut hasher = Sha1::new();
        hasher.update(&self.bytes);
        hasher.finalize().into()
    }

    pub fn deserialize(self) -> GitObject {
        match self.get_type_bytes() {
            b"blob" => GitObject::Blob(Box::new(Blob::from(self))),
            b"tree" => GitObject::Tree(Box::new(Tree::from(self))),
            b"commit" => todo!(),
            _ => panic!("Malformed git object")
        }
    }

    pub fn bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }
}

impl From<Blob> for SerializedGitObject {
    fn from(value: Blob) -> Self {
        let object_type = b"blob ";
        let data = &value.data;
        let length_str = data.len().to_string();
        let bytes = [object_type, length_str.as_bytes(), &[0], data].concat();
        Self { bytes }
    }
}

impl From<Vec<u8>> for SerializedGitObject {
    fn from(value: Vec<u8>) -> Self {
        Self { bytes: value }
    }
}