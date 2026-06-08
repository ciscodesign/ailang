# TASK 0011: Codegen — emit Rust source from a Graph
Phase: 1
Crate: ailang-transpile
Depends on: 0010 (eval working)

## Goal
Implement `codegen(graph, &str) -> String` in the `ailang-transpile` crate.
Given a `Graph` and a function name, emit valid Rust source code for a function
that executes the graph's dataflow inline — each node becomes a `let` binding,
edges become variable references.

This is the "transpile-first" path: graph → Rust source → `rustc --target wasm32`.
For now, only `Const:*` and `Code:*` nodes need to emit real code.
Other kinds emit a `todo!()` stub.

## Existing API (DO NOT CHANGE OR RE-IMPLEMENT)
```rust
// ailang-core::graph
pub type NodeIdx = usize;
pub struct NodeDef  { pub id: NodeId, pub kind: String,
                      pub inputs: Vec<PortDef>, pub outputs: Vec<PortDef>,
                      pub effects: EffectSet }
pub struct Edge     { pub src_node: NodeIdx, pub src_port: PortIdx,
                      pub dst_node: NodeIdx, pub dst_port: PortIdx, pub ty: Type }
pub struct Graph    { pub fn nodes(&self) -> &[NodeDef]; pub fn edges(&self) -> &[Edge]; }

// ailang-core::ty
pub enum Type { Text, Int, Float, Bool, Bytes,
    Option(Box<Type>), Result(Box<Type>, Box<Type>), Var(u32), Union(Vec<Type>), Fold(NodeId) }
```

## Interface
```rust
// FILE: crates/ailang-transpile/src/codegen.rs
use ailang_core::graph::Graph;

#[derive(Debug, thiserror::Error)]
pub enum CodegenError {
    #[error("cycle detected — cannot emit sequential code")]
    Cycle,
}

/// Emit a Rust function named `fn_name` that executes the graph.
/// Returns the complete function source as a String.
pub fn codegen(graph: &Graph, fn_name: &str) -> Result<String, CodegenError>;
```

## Codegen rules
- Perform a topological sort (Kahn's). Return `CodegenError::Cycle` if graph has a cycle.
- Each node `i` becomes `let node_{i}_out: <type> = ...;`
- Kind `"Const:out"` with no inputs → `let node_{i}_out = todo!("Const");`
  (actual value not available at codegen time — caller must substitute)
- Kind `"Code:<expr>"` → emit `<expr>` literally as the RHS
- Any other kind → `todo!("<kind>")`
- The emitted function signature is:
  `pub fn <fn_name>() -> () { ... }` (no inputs/outputs yet — Phase 1 scaffold)
- Nodes with no outputs are still emitted as `let _node_{i} = ...;`

## Acceptance tests
```rust
// FILE: crates/ailang-transpile/src/codegen_tests.rs
#[cfg(test)]
mod tests {
    use ailang_core::{graph::{Graph, NodeDef, PortDef}, node_id::NodeId, ty::Type};
    use ailang_effects::EffectSet;
    use crate::codegen::codegen;

    #[test]
    fn empty_graph_emits_empty_fn() {
        let g = Graph::new();
        let src = codegen(&g, "run").unwrap();
        assert!(src.contains("pub fn run()"));
        assert!(src.contains("{}") || src.contains("{ }") || src.contains("{\n}"));
    }

    #[test]
    fn const_node_emits_todo() {
        let mut g = Graph::new();
        g.add_node(NodeDef {
            id: NodeId::of(b"c"), kind: "Const:out".into(),
            inputs: vec![],
            outputs: vec![PortDef { name: "out".into(), ty: Type::Int }],
            effects: EffectSet::empty(),
        });
        let src = codegen(&g, "run").unwrap();
        assert!(src.contains("node_0") || src.contains("Const"));
    }

    #[test]
    fn code_node_emits_expr() {
        let mut g = Graph::new();
        g.add_node(NodeDef {
            id: NodeId::of(b"e"), kind: "Code:1 + 1".into(),
            inputs: vec![],
            outputs: vec![PortDef { name: "out".into(), ty: Type::Int }],
            effects: EffectSet::empty(),
        });
        let src = codegen(&g, "run").unwrap();
        assert!(src.contains("1 + 1"));
    }
}
```

## Cargo.toml
```toml
// FILE: crates/ailang-transpile/Cargo.toml
[package]
name = "ailang-transpile"
version.workspace = true
edition.workspace = true

[dependencies]
ailang-core    = { path = "../ailang-core" }
ailang-effects = { path = "../ailang-effects" }
thiserror.workspace = true
```

## lib.rs
```rust
// FILE: crates/ailang-transpile/src/lib.rs
pub mod codegen;
#[cfg(test)]
mod codegen_tests;
```
