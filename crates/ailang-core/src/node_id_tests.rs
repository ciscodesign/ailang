#[cfg(test)]
mod tests {
    use crate::node_id::NodeId;
    #[test]
    fn same_bytes_same_id() {
        assert_eq!(NodeId::of(b"hello"), NodeId::of(b"hello"));
    }
    #[test]
    fn different_bytes_different_id() {
        assert_ne!(NodeId::of(b"hello"), NodeId::of(b"hellp"));
    }
    #[test]
    fn empty_is_stable() {
        assert_eq!(NodeId::of(b""), NodeId::of(b""));
    }
    #[test]
    fn display_prefix_and_length() {
        let s = format!("{}", NodeId::of(b"x"));
        assert!(s.starts_with("blake3:"));
        assert_eq!(s.len(), "blake3:".len() + 12);
    }
    #[test]
    fn as_bytes_len() {
        assert_eq!(NodeId::of(b"abc").as_bytes().len(), 32);
    }
    #[test]
    fn hash_consistency() {
        let id = NodeId::of(b"consistent");
        use std::collections::HashSet;
        let mut set = HashSet::new();
        assert!(set.insert(id));
        assert!(!set.insert(NodeId::of(b"consistent")));
    }
}
