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
    fn display_hex_chars_are_valid() {
        let s = format!("{}", NodeId::of(b"x"));
        let hex_part = &s["blake3:".len()..];
        assert!(hex_part.chars().all(|c| c.is_ascii_hexdigit()));
    }
}
