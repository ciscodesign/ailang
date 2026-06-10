use ailang_core::graph::Graph;
use std::collections::{HashMap, HashSet};

/// Returns a new Graph with all dead nodes (and their edges) removed.
/// Node indices are remapped; edges are preserved between surviving nodes.
pub fn prune_dead(graph: &Graph) -> Graph {
    let dead: HashSet<usize> = dead_nodes(graph).into_iter().collect();
    let nodes = graph.nodes();

    // Build old_idx → new_idx map for surviving nodes
    let mut remap: HashMap<usize, usize> = HashMap::new();
    let mut new_graph = Graph::new();
    for (old_idx, node) in nodes.iter().enumerate() {
        if !dead.contains(&old_idx) {
            let new_idx = new_graph.nodes().len();
            remap.insert(old_idx, new_idx);
            new_graph.add_node(node.clone());
        }
    }

    // Re-add edges whose both endpoints survived
    for edge in graph.edges() {
        if let (Some(&src), Some(&dst)) = (remap.get(&edge.src_node), remap.get(&edge.dst_node)) {
            let _ = new_graph.add_edge(src, edge.src_port, dst, edge.dst_port);
        }
    }

    new_graph
}

/// Returns node_idx → (port_name, literal_string) for every `Const:<port>:<literal>` node.
/// These values are statically known and can be inlined by codegen or an optimizer.
pub fn const_values(graph: &Graph) -> HashMap<usize, (String, String)> {
    graph.nodes().iter().enumerate().filter_map(|(i, node)| {
        let rest = node.kind.strip_prefix("Const:")?;
        let pos = rest.find(':')?;
        Some((i, (rest[..pos].to_string(), rest[pos + 1..].to_string())))
    }).collect()
}

/// Returns indices of nodes with no path to any sink (node with no output ports).
pub fn dead_nodes(graph: &Graph) -> Vec<usize> {
    let nodes = graph.nodes();
    if nodes.is_empty() {
        return vec![];
    }

    let mut live: HashSet<usize> = HashSet::new();

    for (i, node) in nodes.iter().enumerate() {
        if node.outputs.is_empty() {
            live.insert(i);
        }
    }

    let mut changed = true;
    while changed {
        changed = false;
        for edge in graph.edges() {
            if live.contains(&edge.dst_node) && !live.contains(&edge.src_node) {
                live.insert(edge.src_node);
                changed = true;
            }
        }
    }

    (0..nodes.len()).filter(|i| !live.contains(i)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ailang_core::graph::{Graph, NodeDef, PortDef};
    use ailang_core::node_id::NodeId;
    use ailang_core::ty::Type;
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

    fn const_node(seed: &[u8], kind: &str) -> NodeDef {
        NodeDef {
            id: NodeId::of(seed),
            kind: kind.into(),
            inputs: vec![],
            outputs: vec![PortDef { name: "out".into(), ty: Type::Int }],
            effects: EffectSet::empty(),
        }
    }

    #[test]
    fn const_values_finds_literals() {
        let mut g = Graph::new();
        g.add_node(const_node(b"c1", "Const:out:42"));
        g.add_node(const_node(b"c2", "Const:val:99"));
        let cv = const_values(&g);
        assert_eq!(cv[&0], ("out".into(), "42".into()));
        assert_eq!(cv[&1], ("val".into(), "99".into()));
    }

    #[test]
    fn const_values_ignores_non_const() {
        let mut g = Graph::new();
        g.add_node(node(b"x", vec![], vec![Type::Int])); // generic node, no Const: prefix
        let cv = const_values(&g);
        assert!(cv.is_empty());
    }

    #[test]
    fn empty_graph_no_dead_nodes() {
        let g = Graph::new();
        assert!(dead_nodes(&g).is_empty());
    }

    #[test]
    fn sink_node_is_live() {
        let mut g = Graph::new();
        g.add_node(node(b"s", vec![Type::Int], vec![]));
        assert!(dead_nodes(&g).is_empty());
    }

    #[test]
    fn source_feeding_sink_both_live() {
        let mut g = Graph::new();
        g.add_node(node(b"a", vec![], vec![Type::Int]));
        g.add_node(node(b"b", vec![Type::Int], vec![]));
        g.add_edge(0, 0, 1, 0).unwrap();
        assert!(dead_nodes(&g).is_empty());
    }

    #[test]
    fn floating_source_is_dead() {
        let mut g = Graph::new();
        g.add_node(node(b"dead", vec![], vec![Type::Int]));
        g.add_node(node(b"sink", vec![Type::Int], vec![]));
        let dead = dead_nodes(&g);
        assert!(dead.contains(&0), "source should be dead: {dead:?}");
        assert!(!dead.contains(&1), "sink should be live");
    }

    #[test]
    fn isolated_middle_node_is_dead() {
        let mut g = Graph::new();
        g.add_node(node(b"src", vec![], vec![Type::Int]));
        g.add_node(node(b"mid", vec![Type::Int], vec![Type::Int]));
        g.add_node(node(b"snk", vec![Type::Int], vec![]));
        g.add_edge(0, 0, 1, 0).unwrap();
        let dead = dead_nodes(&g);
        assert!(dead.contains(&0));
        assert!(dead.contains(&1));
        assert!(!dead.contains(&2));
    }

    #[test]
    fn prune_dead_removes_floating_source() {
        let mut g = Graph::new();
        g.add_node(node(b"dead", vec![], vec![Type::Int])); // dead — no path to sink
        g.add_node(node(b"src",  vec![], vec![Type::Int]));
        g.add_node(node(b"snk",  vec![Type::Int], vec![]));
        g.add_edge(1, 0, 2, 0).unwrap();
        let pruned = prune_dead(&g);
        assert_eq!(pruned.nodes().len(), 2, "dead node should be removed");
        assert_eq!(pruned.edges().len(), 1, "live edge should survive");
    }

    #[test]
    fn prune_dead_empty_graph() {
        let g = Graph::new();
        let pruned = prune_dead(&g);
        assert!(pruned.nodes().is_empty());
        assert!(pruned.edges().is_empty());
    }

    #[test]
    fn prune_dead_all_live() {
        let mut g = Graph::new();
        g.add_node(node(b"a", vec![], vec![Type::Int]));
        g.add_node(node(b"b", vec![Type::Int], vec![]));
        g.add_edge(0, 0, 1, 0).unwrap();
        let pruned = prune_dead(&g);
        assert_eq!(pruned.nodes().len(), 2);
        assert_eq!(pruned.edges().len(), 1);
    }
}
