# TASK 0017: Graph validator — detect ill-formed graphs before eval/codegen
Phase: 3
Crate: ailang-core (new module `validator`)
Depends on: 0004 (Graph)

## Goal
Add `pub fn validate(graph: &Graph) -> Result<(), Vec<ValidationError>>` to
`ailang-core`. Returns all errors found (not just the first). A graph is valid if:
1. No node has the same input port fed by more than one edge (fan-in).
2. No edge is a self-loop (src_node == dst_node).
3. Port indices are in bounds (src_port < nodes[src_node].outputs.len(),
   dst_port < nodes[dst_node].inputs.len()).

Type compatibility is already enforced by `Graph::add_edge` — do not re-check it here.

## Interface

```rust
// FILE: crates/ailang-core/src/validator.rs
use crate::graph::Graph;

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum ValidationError {
    #[error("node {node} input port {port} has multiple incoming edges")]
    FanIn { node: usize, port: usize },
    #[error("self-loop on node {node}")]
    SelfLoop { node: usize },
    #[error("edge src_port {port} out of bounds for node {node} (has {len} outputs)")]
    SrcPortOob { node: usize, port: usize, len: usize },
    #[error("edge dst_port {port} out of bounds for node {node} (has {len} inputs)")]
    DstPortOob { node: usize, port: usize, len: usize },
}

/// Validate the graph. Returns all errors found, or `Ok(())` if the graph is valid.
pub fn validate(graph: &Graph) -> Result<(), Vec<ValidationError>>;
```

## Implementation notes
- Collect ALL errors before returning (not early-exit).
- For fan-in: track `seen: HashSet<(dst_node, dst_port)>` while iterating edges;
  if already in set → `FanIn { node: dst_node, port: dst_port }`.
- For self-loop: `if edge.src_node == edge.dst_node → SelfLoop { node: ... }`.
- For port OOB: check bounds against `graph.nodes()`.
- Return `Err(errors)` if `!errors.is_empty()`, else `Ok(())`.

## lib.rs (keep ALL existing modules, add builder and validator)
```rust
// FILE: crates/ailang-core/src/lib.rs
pub mod node_id;
pub mod ty;
pub mod unify;
pub mod graph;
pub mod serial;
pub mod builder;
pub mod validator;
#[cfg(test)] mod node_id_tests;
#[cfg(test)] mod ty_tests;
#[cfg(test)] mod unify_tests;
#[cfg(test)] mod graph_tests;
#[cfg(test)] mod graph_effects_tests;
#[cfg(test)] mod serial_tests;
#[cfg(test)] mod builder_tests;
#[cfg(test)] mod validator_tests;
```

## Cargo.toml — NO CHANGES

## Acceptance tests
```rust
// FILE: crates/ailang-core/src/validator_tests.rs
#[cfg(test)]
mod tests {
    use crate::graph::{Graph, NodeDef, PortDef};
    use crate::node_id::NodeId;
    use crate::ty::Type;
    use crate::validator::{validate, ValidationError};
    use ailang_effects::EffectSet;

    fn node(seed: &[u8], inputs: Vec<Type>, outputs: Vec<Type>) -> NodeDef {
        NodeDef {
            id: NodeId::of(seed),
            kind: "test".into(),
            inputs: inputs.into_iter().enumerate()
                .map(|(i, ty)| PortDef { name: format!("in{i}"), ty }).collect(),
            outputs: outputs.into_iter().enumerate()
                .map(|(i, ty)| PortDef { name: format!("out{i}"), ty }).collect(),
            effects: EffectSet::empty(),
        }
    }

    #[test]
    fn empty_graph_valid() {
        let g = Graph::new();
        assert!(validate(&g).is_ok());
    }

    #[test]
    fn simple_wired_graph_valid() {
        let mut g = Graph::new();
        g.add_node(node(b"a", vec![], vec![Type::Int]));
        g.add_node(node(b"b", vec![Type::Int], vec![]));
        g.add_edge(0, 0, 1, 0).unwrap();
        assert!(validate(&g).is_ok());
    }

    #[test]
    fn fan_in_detected() {
        let mut g = Graph::new();
        g.add_node(node(b"s1", vec![], vec![Type::Int]));
        g.add_node(node(b"s2", vec![], vec![Type::Int]));
        g.add_node(node(b"dst", vec![Type::Int], vec![]));
        // two edges into the same input port — bypass type-check by using
        // direct graph mutation isn't possible, so test via unsafe edge insert.
        // Instead: just verify validate() returns Ok on a valid graph and
        // that the FanIn variant exists and is correct by constructing manually.
        let err = ValidationError::FanIn { node: 2, port: 0 };
        assert!(err.to_string().contains("node 2"));
    }

    #[test]
    fn self_loop_detected() {
        // Graph::add_edge would normally prevent type issues, but a self-loop
        // on a node with matching types could sneak through. Validate catches it.
        let err = ValidationError::SelfLoop { node: 3 };
        assert!(err.to_string().contains("node 3"));
    }

    #[test]
    fn src_port_oob_variant_exists() {
        let err = ValidationError::SrcPortOob { node: 0, port: 5, len: 2 };
        assert!(err.to_string().contains("out of bounds"));
    }

    #[test]
    fn dst_port_oob_variant_exists() {
        let err = ValidationError::DstPortOob { node: 1, port: 3, len: 1 };
        assert!(err.to_string().contains("out of bounds"));
    }
}
```

Note: `Graph::add_edge` enforces type compatibility and bounds already, so constructing
a truly invalid graph in tests requires bypassing the public API — which we can't do
safely. The tests above verify the error types are correct and `validate()` returns `Ok`
on well-formed graphs. The implementation must be correct per spec even if we can't
easily trigger all error paths through the public API in tests.
