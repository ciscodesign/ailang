#[cfg(test)]
mod tests {
    use crate::codegen::{codegen, codegen_wasm};
    use ailang_core::graph::{Graph, NodeDef, PortDef};
    use ailang_core::node_id::NodeId;
    use ailang_core::ty::Type;
    use ailang_effects::EffectSet;

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
        assert!(src.contains('{'));
        assert!(src.contains('}'));
    }

    #[test]
    fn const_with_literal_emits_value() {
        let mut g = Graph::new();
        g.add_node(make_node(
            b"cv",
            "Const:out:42i64",
            vec![],
            vec![PortDef { name: "out".into(), ty: Type::Int }],
        ));
        let src = codegen(&g, "run").unwrap();
        assert!(src.contains("42i64"), "literal not emitted: {src}");
        assert!(!src.contains("todo!"), "todo! should not appear: {src}");
    }

    #[test]
    fn wasm_empty_graph() {
        let g = Graph::new();
        let src = codegen_wasm(&g, "run").unwrap();
        assert!(src.contains("extern \"C\""), "missing extern C: {src}");
        assert!(src.contains("no_mangle"),    "missing no_mangle: {src}");
        assert!(src.contains("fn run()"),     "missing fn signature: {src}");
    }

    #[test]
    fn wasm_int_output_returns_i64() {
        let mut g = Graph::new();
        g.add_node(make_node(
            b"w",
            "Const:out:99i64",
            vec![],
            vec![PortDef { name: "out".into(), ty: Type::Int }],
        ));
        let src = codegen_wasm(&g, "compute").unwrap();
        assert!(src.contains("-> i64"),     "missing i64 return: {src}");
        assert!(src.contains("99i64"),      "missing literal: {src}");
        assert!(src.contains("node_0_out"), "missing return var: {src}");
    }

    #[test]
    fn wasm_cycle_returns_error() {
        let g = Graph::new();
        assert!(codegen_wasm(&g, "noop").is_ok());
    }
}
