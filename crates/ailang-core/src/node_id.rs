use std::fmt;
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct NodeId([u8; 32]);
impl NodeId {
    pub fn of(bytes: &[u8]) -> Self {
        Self(blake3::hash(bytes).as_bytes())
    }
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}
impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "blake3:")?;
        for &b in &self.0[..6] {
            write!(f, "{:02x}", b)?;
        }
        Ok(())
    }
}
