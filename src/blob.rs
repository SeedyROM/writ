use core::fmt::Debug;
use sha1::{Digest, Sha1};
use std::{fs, path::PathBuf};

use crate::Hash;

pub struct Blob {
    hash: Hash,
    #[allow(dead_code)]
    data: Vec<u8>,
}

impl Blob {
    pub fn hash(&self) -> &Hash {
        &self.hash
    }

    #[allow(dead_code)]
    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }
}

impl PartialEq for Blob {
    fn eq(&self, other: &Self) -> bool {
        self.hash.eq(&other.hash)
    }
}

impl PartialEq<String> for Blob {
    fn eq(&self, other: &String) -> bool {
        Hash::from(other).eq(self.hash())
    }
}

impl From<String> for Blob {
    fn from(file: String) -> Self {
        // Read our file into Vec<u8> or fail
        let data = fs::read(file).expect("Failed to load blob");

        // Build the hash from the data `as_slice()`
        let mut hasher = Sha1::new();
        hasher.update(data.as_slice());
        let hash = Hash(hasher.finalize().into());

        Self { hash, data }
    }
}

impl From<PathBuf> for Blob {
    fn from(path: PathBuf) -> Self {
        Self::from(path.display().to_string())
    }
}

impl Debug for Blob {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Blob")
            .field("hash", &self.hash.to_string())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    const TEST_FILE_PATH: &str = "./test/blob_test.txt";
    const OTHER_TEST_FILE_PATH: &str = "./test/other_blob_test.txt";

    #[test]
    fn blob_creation_from_string() {
        let blob = Blob::from(TEST_FILE_PATH.to_string());

        assert_eq!(
            *blob.hash(),
            Hash::from("e2a667251544b4fd487e813cc030c661433a936a".to_string())
        );
    }

    #[test]
    fn blob_creation_from_path_buf() {
        let mut path = PathBuf::new();
        path.push(TEST_FILE_PATH);

        let blob = Blob::from(path);
        assert_eq!(
            *blob.hash(),
            Hash::from("e2a667251544b4fd487e813cc030c661433a936a".to_string())
        );
    }

    #[test]
    fn blob_partial_eq() {
        let blob0 = Blob::from(TEST_FILE_PATH.to_string());
        let blob1 = Blob::from(OTHER_TEST_FILE_PATH.to_string());

        assert_ne!(blob0, blob1);
    }
}
