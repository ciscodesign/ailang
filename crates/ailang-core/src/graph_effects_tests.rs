#[cfg(test)]
mod tests {
    use ailang_effects::{Effect, EffectSet};
    use crate::{graph::{Graph, NodeDef}, node_id::NodeId};
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
