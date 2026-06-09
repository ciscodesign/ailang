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
