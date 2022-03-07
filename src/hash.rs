use std::ops::Deref;

type HashSlice = [u8; 20];

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Hash(pub HashSlice);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_deref() {
        let hash = Hash::from("ebe851dcb690a570305736fd3ec8533228ae4d3f".to_string());
        let slice: HashSlice = *hash;

        assert_eq!(slice.len(), 20);
    }

    #[test]
    fn hash_from_hash_slice() {
        let hash0 = Hash::from("ebe851dcb690a570305736fd3ec8533228ae4d3f".to_string());
        let value: HashSlice = hash0.as_slice().try_into().unwrap();
        let hash1 = Hash::from(value);

        assert_eq!(hash0, hash1);
    }

    #[test]
    fn hash_to_string() {
        let hash_string = "ebe851dcb690a570305736fd3ec8533228ae4d3f".to_string();
        let hash = Hash::from(&hash_string);

        assert_eq!(hash.to_string(), hash_string);
    }
}
