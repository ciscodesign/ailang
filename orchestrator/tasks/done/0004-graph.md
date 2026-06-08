# TASK 0004: Graph — nodes, ports, edges, well-formedness
Phase: 0
Depends on: 0003 (Type::unify)

## Goal
Implement the core `Graph` data structure: a collection of nodes, each with typed
input and output ports, connected by typed edges. Adding an edge must call
`Type::unify` and fail if types don't match. Done when all acceptance tests pass
and clippy is clean.

## Interface
```
// FILE: crates/ailang-core/src/graph.rs
pub type NodeIdx = usize;
pub type PortIdx = usize;

pub struct PortDef {
    pub name: String,
    pub ty:   Type,
}

pub struct NodeDef {
    pub id:      NodeId,
    pub kind:    String,        // "Const", "LLM", "HTTP", etc. (opaque for now)
    pub inputs:  Vec<PortDef>,
    pub outputs: Vec<PortDef>,
}

pub struct Edge {
    pub src_node:  NodeIdx,
    pub src_port:  PortIdx,   // index into src.outputs
    pub dst_node:  NodeIdx,
    pub dst_port:  PortIdx,   // index into dst.inputs
    pub ty:        Type,      // the unified type
}

#[derive(Default)]
pub struct Graph {
    nodes: Vec<NodeDef>,
    edges: Vec<Edge>,
}

impl Graph {
    pub fn new() -> Self;
    pub fn add_node(&mut self, node: NodeDef) -> NodeIdx;
    pub fn add_edge(
        &mut self,
        src_node: NodeIdx, src_port: PortIdx,
        dst_node: NodeIdx, dst_port: PortIdx,
    ) -> Result<(), GraphError>;
    pub fn nodes(&self) -> &[NodeDef];
    pub fn edges(&self) -> &[Edge];
}

#[derive(Debug, thiserror::Error)]
pub enum GraphError {
    #[error("node index {0} out of range")]
    NoSuchNode(NodeIdx),
    #[error("port index {0} out of range on node {1}")]
    NoSuchPort(PortIdx, NodeIdx),
    #[error("type mismatch: {0}")]
    TypeMismatch(#[from] UnifyError),
}
```

## Constraints
- No `unsafe`. No IO.
- `add_edge` must validate both node indices and port indices before calling unify.
- Capabilities granted: none.

## Acceptance tests
```rust
// FILE: crates/ailang-core/src/graph_tests.rs
#[cfg(test)]
mod tests {
    use crate::{graph::{Graph, NodeDef, PortDef, GraphError}, node_id::NodeId, ty::Type};

    fn text_node(kind: &str, inputs: &[&str], outputs: &[&str]) -> NodeDef {
        NodeDef {
            id: NodeId::of(kind.as_bytes()),
            kind: kind.to_string(),
            inputs:  inputs.iter().map(|n|  PortDef { name: n.to_string(), ty: Type::Text }).collect(),
            outputs: outputs.iter().map(|n| PortDef { name: n.to_string(), ty: Type::Text }).collect(),
        }
    }

    #[test]
    fn add_compatible_edge() {
        let mut g = Graph::new();
        let a = g.add_node(text_node("A", &[], &["out"]));
        let b = g.add_node(text_node("B", &["in"], &[]));
        assert!(g.add_edge(a, 0, b, 0).is_ok());
        assert_eq!(g.edges().len(), 1);
    }
    #[test]
    fn reject_type_mismatch() {
        let mut g = Graph::new();
        let a = g.add_node(NodeDef {
            id: NodeId::of(b"A"), kind: "A".into(),
            inputs: vec![], outputs: vec![PortDef { name: "out".into(), ty: Type::Int }],
        });
        let b = g.add_node(text_node("B", &["in"], &[])); // input is Text
        assert!(matches!(g.add_edge(a, 0, b, 0), Err(GraphError::TypeMismatch(_))));
    }
    #[test]
    fn reject_bad_node_index() {
        let mut g = Graph::new();
        let a = g.add_node(text_node("A", &[], &["out"]));
        assert!(matches!(g.add_edge(a, 0, 99, 0), Err(GraphError::NoSuchNode(99))));
    }
    #[test]
    fn reject_bad_port_index() {
        let mut g = Graph::new();
        let a = g.add_node(text_node("A", &[], &["out"]));
        let b = g.add_node(text_node("B", &["in"], &[]));
        assert!(matches!(g.add_edge(a, 99, b, 0), Err(GraphError::NoSuchPort(99, _))));
    }
}
```

## Context
Graph is the core runtime data structure — every other crate operates on it.
Keep it simple: just a Vec<NodeDef> and Vec<Edge> with validation on mutation.
No graph-traversal logic yet; that comes with fold (Task 0007).
