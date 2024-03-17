use super::git_object::{PrettyPrintable, SerializedGitObject};

pub struct Blob {
    pub(super) data: Vec<u8>,
}

impl<T: AsRef<[u8]>> From<T> for Blob {
    fn from(value: T) -> Self {
        Self { data: value.as_ref().to_vec() }
    }
}

impl PrettyPrintable for Blob {
    fn pretty_print(&self) -> String {
        String::from_utf8_lossy(&self.data[..]).to_string()
    }
}

impl From<SerializedGitObject> for Blob {
    fn from(value: SerializedGitObject) -> Self {
        Self { data: Vec::from(value.get_content_bytes()) }
    }
}