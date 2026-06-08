#[cfg(test)]
mod tests {
    use ailang_effects::EffectSet;
    use crate::{graph::{Graph, NodeDef, PortDef, GraphError}, node_id::NodeId, ty::Type};

    fn text_node(kind: &str, inputs: &[&str], outputs: &[&str]) -> NodeDef {
        NodeDef {
            id:      NodeId::of(kind.as_bytes()),
            kind:    kind.to_string(),
            inputs:  inputs.iter().map(|n|  PortDef { name: n.to_string(), ty: Type::Text }).collect(),
            outputs: outputs.iter().map(|n| PortDef { name: n.to_string(), ty: Type::Text }).collect(),
            effects: EffectSet::empty(),
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
            effects: EffectSet::empty(),
        });
        let b = g.add_node(text_node("B", &["in"], &[]));
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

    #[test]
    fn add_multiple_edges() {
        let mut g = Graph::new();
        let a = g.add_node(text_node("A", &[], &["out"]));
        let b = g.add_node(text_node("B", &["in"], &[]));
        let c = g.add_node(text_node("C", &["in"], &[]));
        assert!(g.add_edge(a, 0, b, 0).is_ok());
        assert!(g.add_edge(a, 0, c, 0).is_ok());
        assert_eq!(g.edges().len(), 2);
    }

    #[test]
    fn reject_edge_on_empty_graph() {
        let mut g = Graph::new();
        assert!(matches!(g.add_edge(0, 0, 0, 0), Err(GraphError::NoSuchNode(0))));
    }
}
