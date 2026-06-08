# TASK 0006: Add EffectSet to NodeDef
Phase: 0
Depends on: 0004 (Graph), 0005 (EffectSet)

## Goal
Extend `NodeDef` in `ailang-core` to carry an `EffectSet`, and add a method on
`Graph` that computes the total effect set of all nodes. Update the Graph tests
to set effects. Done when all acceptance tests pass and clippy is clean.

## Interface
Update `crates/ailang-core/src/graph.rs`:
```
// NodeDef gains one field:
pub struct NodeDef {
    pub id:      NodeId,
    pub kind:    String,
    pub inputs:  Vec<PortDef>,
    pub outputs: Vec<PortDef>,
    pub effects: EffectSet,     // ← new
}

// Graph gains one method:
impl Graph {
    /// Union of EffectSets across all nodes.
    pub fn total_effects(&self) -> EffectSet;
}
```

Also add `ailang-effects` as a dependency of `ailang-core` in Cargo.toml.

## Constraints
- No `unsafe`. No IO.
- All existing graph tests must continue to pass (set `effects: EffectSet::empty()` in helpers).
- Capabilities granted: none.

## Acceptance tests
```rust
// FILE: crates/ailang-core/src/graph_effects_tests.rs
#[cfg(test)]
mod tests {
    use ailang_effects::{Effect, EffectSet};
    use crate::{graph::{Graph, NodeDef, PortDef}, node_id::NodeId, ty::Type};

    fn make_node(kind: &str, effects: EffectSet) -> NodeDef {
        NodeDef {
            id: NodeId::of(kind.as_bytes()),
            kind: kind.to_string(),
            inputs: vec![],
            outputs: vec![],
            effects,
        }
    }

    #[test]
    fn total_effects_union() {
        let mut g = Graph::new();
        g.add_node(make_node("A", EffectSet::of(&[Effect::Net])));
        g.add_node(make_node("B", EffectSet::of(&[Effect::Db])));
        g.add_node(make_node("C", EffectSet::empty()));
        let total = g.total_effects();
        assert!(total.contains(Effect::Net));
        assert!(total.contains(Effect::Db));
        assert!(!total.contains(Effect::Llm));
    }
    #[test]
    fn empty_graph_has_no_effects() {
        let g = Graph::new();
        assert_eq!(g.total_effects(), EffectSet::empty());
    }
}
```

## Context
Effects bubble up — a fold's declared EffectSet is the union of its children's
EffectSets. `total_effects()` on a Graph is the primitive that fold (Task 0007)
uses when computing its interface. This is the "security at a glance" feature:
you always know what a folded system touches.
