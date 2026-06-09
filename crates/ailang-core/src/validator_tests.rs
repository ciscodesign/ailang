#[cfg(test)]
mod tests {
    use crate::graph::{Graph, NodeDef, PortDef};
    use crate::node_id::NodeId;
    use crate::ty::Type;
    use crate::validator::{validate, ValidationError};
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
    fn empty_graph_valid() {
        let g = Graph::new();
        assert!(validate(&g).is_ok());
    }

    #[test]
    fn simple_wired_graph_valid() {
        let mut g = Graph::new();
        g.add_node(node(b"a", vec![], vec![Type::Int]));
        g.add_node(node(b"b", vec![Type::Int], vec![]));
        g.add_edge(0, 0, 1, 0).unwrap();
        assert!(validate(&g).is_ok());
    }

    #[test]
    fn fan_in_variant_exists() {
        let err = ValidationError::FanIn { node: 2, port: 0 };
        assert!(err.to_string().contains("node 2"));
    }

    #[test]
    fn self_loop_variant_exists() {
        let err = ValidationError::SelfLoop { node: 3 };
        assert!(err.to_string().contains("node 3"));
    }

    #[test]
    fn src_port_oob_variant_exists() {
        let err = ValidationError::SrcPortOob { node: 0, port: 5, len: 2 };
        assert!(err.to_string().contains("out of bounds"));
    }

    #[test]
    fn dst_port_oob_variant_exists() {
        let err = ValidationError::DstPortOob { node: 1, port: 3, len: 1 };
        assert!(err.to_string().contains("out of bounds"));
    }
}
