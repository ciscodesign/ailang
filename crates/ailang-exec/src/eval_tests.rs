#[cfg(test)]
mod tests {
    use ailang_core::{graph::{Graph, NodeDef, PortDef}, node_id::NodeId, ty::Type};
    use ailang_effects::EffectSet;
    use crate::{eval::eval, registry::NodeRegistry, value::Value};
    fn make_graph() -> (Graph, NodeRegistry) {
        let mut g = Graph::new();
        let mut r = NodeRegistry::new();
        // Source: Const node emitting Int(5)
        r.register_const("out", Value::Int(5));
        let src = g.add_node(NodeDef {
            id: NodeId::of(b"src"), kind: "Const:out".into(),
            inputs: vec![], effects: EffectSet::empty(),
            outputs: vec![PortDef { name: "out".into(), ty: Type::Int }],
        });
        // Sink: identity node that passes "in" → "out"
        r.register("identity", Box::new(|inputs| Ok(inputs)));
        let dst = g.add_node(NodeDef {
            id: NodeId::of(b"dst"), kind: "identity".into(),
            inputs: vec![PortDef { name: "out".into(), ty: Type::Int }],
            outputs: vec![PortDef { name: "out".into(), ty: Type::Int }],
            effects: EffectSet::empty(),
        });
        g.add_edge(src, 0, dst, 0).unwrap();
        (g, r)
    }
    #[test]
    fn simple_chain_evaluates() {
        let (g, r) = make_graph();
        let result = eval(&g, &r).unwrap();
        // dst node (idx 1) should have output "out" = Int(5)
        assert_eq!(result[&1]["out"], Value::Int(5));
    }
    #[test]
    fn empty_graph_evaluates() {
        let g = Graph::new();
        let r = NodeRegistry::new();
        let result = eval(&g, &r).unwrap();
        assert!(result.is_empty());
    }
}
