# TASK 0022: Graph passes — dead node elimination in ailang-fold
Phase: 4
Crate: ailang-fold (new implementation)
Depends on: 0004 (Graph)

## Goal
Implement `pub fn dead_nodes(graph: &Graph) -> Vec<usize>` in ailang-fold.
Returns indices of nodes that have no outgoing path to any sink (node with no outputs).
A "live" node is one that is a sink OR has a path to a sink via edges.
Dead nodes are those not reachable backwards from any sink.

## Interface

```rust
// FILE: crates/ailang-fold/src/lib.rs
use ailang_core::graph::Graph;

/// Returns indices of nodes that are dead (no path to any sink).
/// A sink is a node with no output ports.
pub fn dead_nodes(graph: &Graph) -> Vec<usize> {
    // BFS/DFS backwards from sinks
}
```

## Implementation

Sinks = nodes whose outputs.is_empty(). Mark them live.
Then walk edges in reverse: if dst_node is live, mark src_node live.
Repeat until no new nodes are marked.
Return all node indices NOT in the live set.

```rust
use std::collections::HashSet;

pub fn dead_nodes(graph: &Graph) -> Vec<usize> {
    let nodes = graph.nodes();
    if nodes.is_empty() { return vec![]; }

    let mut live: HashSet<usize> = HashSet::new();

    // sinks are always live
    for (i, node) in nodes.iter().enumerate() {
        if node.outputs.is_empty() {
            live.insert(i);
        }
    }

    // propagate liveness backwards through edges
    let mut changed = true;
    while changed {
        changed = false;
        for edge in graph.edges() {
            if live.contains(&edge.dst_node) && !live.contains(&edge.src_node) {
                live.insert(edge.src_node);
                changed = true;
            }
        }
    }

    (0..nodes.len()).filter(|i| !live.contains(i)).collect()
}
```

## Cargo.toml — ailang-fold/Cargo.toml
```toml
[package]
name = "ailang-fold"
version.workspace = true
edition.workspace = true
[dependencies]
ailang-core = { path = "../ailang-core" }
```

## Acceptance tests

```rust
// FILE: crates/ailang-fold/src/lib.rs — add at bottom:
#[cfg(test)]
mod tests {
    use super::*;
    use ailang_core::graph::{Graph, NodeDef, PortDef};
    use ailang_core::node_id::NodeId;
    use ailang_core::ty::Type;
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
    fn empty_graph_no_dead_nodes() {
        let g = Graph::new();
        assert!(dead_nodes(&g).is_empty());
    }

    #[test]
    fn sink_node_is_live() {
        let mut g = Graph::new();
        g.add_node(node(b"s", vec![Type::Int], vec![])); // sink
        assert!(dead_nodes(&g).is_empty());
    }

    #[test]
    fn source_feeding_sink_both_live() {
        let mut g = Graph::new();
        g.add_node(node(b"a", vec![], vec![Type::Int])); // source
        g.add_node(node(b"b", vec![Type::Int], vec![])); // sink
        g.add_edge(0, 0, 1, 0).unwrap();
        assert!(dead_nodes(&g).is_empty());
    }

    #[test]
    fn floating_source_is_dead() {
        let mut g = Graph::new();
        g.add_node(node(b"dead", vec![], vec![Type::Int])); // has output, no downstream
        g.add_node(node(b"sink", vec![Type::Int], vec![])); // sink, not connected
        // no edge — dead has no path to sink
        let dead = dead_nodes(&g);
        assert!(dead.contains(&0), "source should be dead: {dead:?}");
        assert!(!dead.contains(&1), "sink should be live");
    }

    #[test]
    fn isolated_middle_node_is_dead() {
        let mut g = Graph::new();
        g.add_node(node(b"src", vec![], vec![Type::Int]));   // 0: dead
        g.add_node(node(b"mid", vec![Type::Int], vec![Type::Int])); // 1: dead
        g.add_node(node(b"snk", vec![Type::Int], vec![]));   // 2: sink (live, disconnected)
        g.add_edge(0, 0, 1, 0).unwrap(); // src→mid connected, but not to sink
        let dead = dead_nodes(&g);
        assert!(dead.contains(&0));
        assert!(dead.contains(&1));
        assert!(!dead.contains(&2));
    }
}
```
