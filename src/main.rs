use std::ops::Deref;

mod blob;

type HashSlice = [u8; 20];

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Hash(HashSlice);

impl Deref for Hash {
    type Target = HashSlice;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<HashSlice> for Hash {
    fn from(hash: HashSlice) -> Self {
        Self(hash)
    }
}

impl From<String> for Hash {
    fn from(string: String) -> Self {
        let bytes: HashSlice = hex::decode(string)
            .expect("Invalid Hash string")
            .as_slice()
            .try_into()
            .expect("Invalid Hash bytes length");
        Self(bytes)
    }
}

impl From<&String> for Hash {
    fn from(string: &String) -> Self {
        let bytes: HashSlice = hex::decode(string)
            .expect("Invalid Hash string")
            .as_slice()
            .try_into()
            .expect("Invalid Hash bytes length");
        Self(bytes)
    }
}

impl ToString for Hash {
    fn to_string(&self) -> String {
        hex::encode(self.0)
    }
}

fn main() {
    println!("Hello, world!");
}
