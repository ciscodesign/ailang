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
