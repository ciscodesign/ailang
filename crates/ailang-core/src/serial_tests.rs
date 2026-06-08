#[cfg(test)]
mod tests {
    use ailang_effects::EffectSet;
    use crate::{
        graph::{Graph, NodeDef, PortDef},
        node_id::NodeId,
        serial::{encode, decode},
        ty::Type,
    };
    fn simple_graph() -> Graph {
        let mut g = Graph::new();
        let a = g.add_node(NodeDef {
            id:      NodeId::of(b"A"),
            kind:    "Const".into(),
            inputs:  vec![],
            outputs: vec![PortDef { name: "out".into(), ty: Type::Text }],
            effects: EffectSet::empty(),
        });
        let b = g.add_node(NodeDef {
            id:      NodeId::of(b"B"),
            kind:    "Sink".into(),
            inputs:  vec![PortDef { name: "in".into(), ty: Type::Text }],
            outputs: vec![],
            effects: EffectSet::empty(),
        });
        g.add_edge(a, 0, b, 0).unwrap();
        g
    }
    #[test]
    fn round_trip() {
        let g = simple_graph();
        let bytes = encode(&g).unwrap();
        let g2    = decode(&bytes).unwrap();
        let bytes2 = encode(&g2).unwrap();
        assert_eq!(bytes, bytes2);  // deterministic
    }
    #[test]
    fn content_hash_stable() {
        let g = simple_graph();
        let id1 = NodeId::of(&encode(&g).unwrap());
        let id2 = NodeId::of(&encode(&g).unwrap());
        assert_eq!(id1, id2);
    }
    #[test]
    fn decode_rejects_garbage() {
        assert!(decode(b"not valid json at all!!!").is_err());
    }
}
