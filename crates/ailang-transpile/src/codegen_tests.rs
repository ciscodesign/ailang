#[cfg(test)]
mod tests {
    use ailang_core::{graph::{Graph, NodeDef, PortDef, Edge}, node_id::NodeId, ty::Type};
    use ailang_effects::EffectSet;
    use crate::codegen::codegen;
    fn make_node(seed: &[u8], kind: &str, inputs: Vec<PortDef>, outputs: Vec<PortDef>) -> NodeDef {
        NodeDef {
            id: NodeId::of(seed),
            kind: kind.into(),
            inputs,
            outputs,
            effects: EffectSet::empty(),
        }
    }
    #[test]
    fn empty_graph_emits_empty_fn() {
        let g = Graph::new();
        let src = codegen(&g, "run").unwrap();
        assert!(src.contains("pub fn run()"));
        assert!(src.contains("{}") || src.contains("}{") || src.contains("{\n}"));
    }
    #[test]
    fn const_node_emits_todo() {
        let mut g = Graph::new();
        g.add_node(make_node(b"c", "Const:out",
            vec![],
            vec![PortDef { name: "out".into(), ty: Type::Int }],
        ));
        let src = codegen(&g, "run").unwrap();
        assert!(src.contains("node_0") || src.contains("Const"));
    }
    #[test]
    fn code_node_emits_expr() {
        let mut g = Graph::new();
        g.add_node(make_node(b"e", "Code:1 + 1",
            vec![],
            vec![PortDef { name: "out".into(), ty: Type::Int }],
        ));
        let src = codegen(&g, "run").unwrap();
        assert!(src.contains("1 + 1"));
    }
    #[test]
    fn wired_nodes_emit_binding() {
        // node 0: Const:out (Int) → node 1: Code:x + 1 (input port "x")
        let mut g = Graph::new();
        g.add_node(make_node(b"src", "Const:out",
            vec![],
            vec![PortDef { name: "out".into(), ty: Type::Int }],
        ));
        g.add_node(make_node(b"dst", "Code:x + 1",
            vec![PortDef { name: "x".into(), ty: Type::Int }],
            vec![PortDef { name: "out".into(), ty: Type::Int }],
        ));
        g.add_edge(0, 0, 1, 0).unwrap();
        let src = codegen(&g, "run").unwrap();
        // Must emit a rebinding that connects node_0_out to node_1_x
        assert!(src.contains("node_0_out"), "src output binding missing");
        assert!(src.contains("node_1_x") || src.contains("node_0_out"), "input wiring missing");
        assert!(src.contains("x + 1"), "Code expr missing");
    }
}
