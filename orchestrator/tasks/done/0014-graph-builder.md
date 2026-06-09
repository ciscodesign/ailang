# TASK 0014: Graph Builder — fluent API for constructing graphs
Phase: 2
Crate: ailang-core (new module `builder`)
Depends on: 0004 (Graph)

## Goal
Add a `GraphBuilder` to `ailang-core` that lets callers construct graphs without
manually creating `NodeDef`, `PortDef`, and `NodeId` structs. The builder provides
ergonomic methods for common patterns (constant nodes, code-expression nodes, generic
nodes) and calls `graph.add_node` / `graph.add_edge` internally.

## Interface
```rust
// FILE: crates/ailang-core/src/builder.rs
use crate::graph::{Graph, GraphError, NodeDef, NodeIdx, PortDef};
use crate::node_id::NodeId;
use crate::ty::Type;
use ailang_effects::EffectSet;

pub struct GraphBuilder {
    graph: Graph,
    counter: u64,   // monotonic seed for NodeId::of
}

impl GraphBuilder {
    pub fn new() -> Self;

    /// Add a Const node for a single output port.
    /// kind will be "Const:{port_name}".
    pub fn const_node(&mut self, port_name: impl Into<String>, ty: Type) -> NodeIdx;

    /// Add a Code:<expr> node with the given input ports and a single "out" output.
    pub fn code_node(
        &mut self,
        expr: impl Into<String>,
        inputs: Vec<(String, Type)>,   // (port_name, type) pairs
        out_ty: Type,
    ) -> NodeIdx;

    /// Add a generic node with explicit kind, inputs, and outputs.
    pub fn node(
        &mut self,
        kind: impl Into<String>,
        inputs: Vec<(String, Type)>,
        outputs: Vec<(String, Type)>,
        effects: EffectSet,
    ) -> NodeIdx;

    /// Wire src_node's output port `src_port` to dst_node's input port `dst_port`.
    pub fn edge(
        &mut self,
        src_node: NodeIdx, src_port: usize,
        dst_node: NodeIdx, dst_port: usize,
    ) -> Result<(), GraphError>;

    /// Consume the builder and return the finished Graph.
    pub fn build(self) -> Graph;
}

impl Default for GraphBuilder {
    fn default() -> Self { Self::new() }
}
```

### NodeId generation
Each call to `const_node`, `code_node`, or `node` must produce a unique NodeId.
Use `NodeId::of(&self.counter.to_le_bytes())` and increment `self.counter` each time.

## lib.rs — add builder module
```rust
// FILE: crates/ailang-core/src/lib.rs
pub mod node_id;
pub mod ty;
pub mod unify;
pub mod graph;
pub mod serial;
pub mod builder;
#[cfg(test)] mod node_id_tests;
#[cfg(test)] mod ty_tests;
#[cfg(test)] mod unify_tests;
#[cfg(test)] mod graph_tests;
#[cfg(test)] mod graph_effects_tests;
#[cfg(test)] mod serial_tests;
#[cfg(test)] mod builder_tests;
```

## Cargo.toml (unchanged — keep exactly)
```toml
// FILE: crates/ailang-core/Cargo.toml
[package]
name = "ailang-core"
version = "0.1.0"
edition = "2021"

[dependencies]
blake3.workspace = true
thiserror.workspace = true
serde.workspace = true
serde_json.workspace = true
hex = "0.4"
ailang-effects = { path = "../ailang-effects" }
```

## Acceptance tests
```rust
// FILE: crates/ailang-core/src/builder_tests.rs
#[cfg(test)]
mod tests {
    use crate::builder::GraphBuilder;
    use crate::ty::Type;
    use ailang_effects::EffectSet;

    #[test]
    fn empty_build() {
        let b = GraphBuilder::new();
        let g = b.build();
        assert_eq!(g.nodes().len(), 0);
        assert_eq!(g.edges().len(), 0);
    }

    #[test]
    fn const_node() {
        let mut b = GraphBuilder::new();
        let idx = b.const_node("out", Type::Int);
        let g = b.build();
        assert_eq!(g.nodes().len(), 1);
        assert_eq!(g.nodes()[idx].kind, "Const:out");
        assert_eq!(g.nodes()[idx].outputs[0].ty, Type::Int);
    }

    #[test]
    fn code_node_with_inputs() {
        let mut b = GraphBuilder::new();
        let c = b.const_node("val", Type::Int);
        let code = b.code_node(
            "val + 1",
            vec![("val".into(), Type::Int)],
            Type::Int,
        );
        b.edge(c, 0, code, 0).unwrap();
        let g = b.build();
        assert_eq!(g.nodes().len(), 2);
        assert_eq!(g.edges().len(), 1);
        assert!(g.nodes()[code].kind.starts_with("Code:"));
    }

    #[test]
    fn generic_node() {
        let mut b = GraphBuilder::new();
        let idx = b.node(
            "add_int",
            vec![("a".into(), Type::Int), ("b".into(), Type::Int)],
            vec![("out".into(), Type::Int)],
            EffectSet::empty(),
        );
        let g = b.build();
        assert_eq!(g.nodes()[idx].kind, "add_int");
        assert_eq!(g.nodes()[idx].inputs.len(), 2);
        assert_eq!(g.nodes()[idx].outputs.len(), 1);
    }

    #[test]
    fn edge_type_mismatch_returns_error() {
        let mut b = GraphBuilder::new();
        let src = b.const_node("out", Type::Int);
        let dst = b.node(
            "not_bool",
            vec![("a".into(), Type::Bool)],
            vec![("out".into(), Type::Bool)],
            EffectSet::empty(),
        );
        assert!(b.edge(src, 0, dst, 0).is_err());
    }
}
```
