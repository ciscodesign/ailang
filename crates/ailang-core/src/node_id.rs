use std::fmt;
use blake3::hash;
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct NodeId([u8; 32]);
impl NodeId {
    pub fn of(bytes: &[u8]) -> Self {
        let hash_bytes: [u8; 32] = hash(bytes).into();
        NodeId(hash_bytes)
    }
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}
impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "blake3:")?;
        for byte in self.0.iter().take(6) {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}
