# TASK 0007: Canonical serialization — Graph encode/decode round-trip
Phase: 0
Depends on: 0006 (NodeDef with effects)

## Goal
Implement canonical binary serialization for `Graph` such that:
- `encode(graph)` produces a deterministic byte vector (same graph = same bytes, always).
- `decode(bytes)` reconstructs the graph exactly.
- `NodeId::of(encode(graph))` gives the graph's content hash.

Use JSON as the interchange format for now (true binary comes in Phase 2).
Determinism requires sorted keys and stable ordering throughout.
Done when all acceptance tests pass and clippy is clean.

## Interface
```
// FILE: crates/ailang-core/src/serial.rs
use crate::graph::Graph;

#[derive(Debug, thiserror::Error)]
pub enum SerialError {
    #[error("encode error: {0}")]
    Encode(String),
    #[error("decode error: {0}")]
    Decode(String),
}

pub fn encode(graph: &Graph) -> Result<Vec<u8>, SerialError>;
pub fn decode(bytes: &[u8])  -> Result<Graph,  SerialError>;
```

## Constraints
- Output must be deterministic: same `Graph` value → same bytes, every time,
  regardless of insertion order. Sort all collections by a stable key before encoding.
- Use `serde_json` with sorted keys (enable the `preserve_order` feature or sort manually).
- No `unsafe`. No IO.
- Capabilities granted: none.

## Acceptance tests
```rust
// FILE: crates/ailang-core/src/serial_tests.rs
#[cfg(test)]
mod tests {
    use ailang_effects::EffectSet;
    use crate::{
        graph::{Graph, NodeDef, PortDef},
        node_id::NodeId,
        serial::{encode, decode},
        ty::Type,
    };

    fn simple_graph() -> Graph {
        let mut g = Graph::new();
        let a = g.add_node(NodeDef {
            id:      NodeId::of(b"A"),
            kind:    "Const".into(),
            inputs:  vec![],
            outputs: vec![PortDef { name: "out".into(), ty: Type::Text }],
            effects: EffectSet::empty(),
        });
        let b = g.add_node(NodeDef {
            id:      NodeId::of(b"B"),
            kind:    "Sink".into(),
            inputs:  vec![PortDef { name: "in".into(), ty: Type::Text }],
            outputs: vec![],
            effects: EffectSet::empty(),
        });
        g.add_edge(a, 0, b, 0).unwrap();
        g
    }

    #[test]
    fn round_trip() {
        let g = simple_graph();
        let bytes = encode(&g).unwrap();
        let g2    = decode(&bytes).unwrap();
        let bytes2 = encode(&g2).unwrap();
        assert_eq!(bytes, bytes2);  // deterministic
    }
    #[test]
    fn content_hash_stable() {
        let g = simple_graph();
        let id1 = NodeId::of(&encode(&g).unwrap());
        let id2 = NodeId::of(&encode(&g).unwrap());
        assert_eq!(id1, id2);
    }
    #[test]
    fn decode_rejects_garbage() {
        assert!(decode(b"not valid json at all!!!").is_err());
    }
}
```

## Context
Phase 0 is complete when this task passes. The `NodeId::of(encode(graph))`
pattern is how folds get their content hash — a fold IS a graph, and its
identity IS the hash of its encoded form. Determinism is non-negotiable.
Add `serde_json.workspace = true` to `ailang-core/Cargo.toml` if not already present.
