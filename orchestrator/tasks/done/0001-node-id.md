# TASK 0001: NodeId — content-addressed identifier
Phase: 0
Depends on: none

## Goal
Implement `NodeId`, the identity of a node. A `NodeId` is the blake3 hash of a
node's canonical byte encoding. Two nodes with identical bytes must produce
identical `NodeId`s; any byte difference must produce a different `NodeId` with
overwhelming probability. Done when all acceptance tests pass and clippy is clean.

## Interface
```
// FILE: crates/ailang-core/src/node_id.rs
pub struct NodeId([u8; 32]);

impl NodeId {
    pub fn of(bytes: &[u8]) -> Self;
    pub fn as_bytes(&self) -> &[u8; 32];
}

// Derive: Clone, Copy, PartialEq, Eq, Hash, Debug
// Display: "blake3:" + first 12 hex chars of the hash
```

## Constraints
- Use only the `blake3` workspace crate. No other dependencies.
- No `unsafe`. No IO. Fully deterministic.
- Capabilities granted: none.

## Acceptance tests
```rust
// FILE: crates/ailang-core/src/node_id_tests.rs
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
}
```

## Context
This is the first task in ailang-core. NodeId is the content-addressing primitive
used by every node and fold in the system (design §4.1, §4.4). Keep it tiny and
dependency-light — everything builds on it.
Add `blake3.workspace = true` to crates/ailang-core/Cargo.toml if not already present.
