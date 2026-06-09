use crate::graph::Graph;
use std::collections::HashSet;

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum ValidationError {
    #[error("node {node} input port {port} has multiple incoming edges")]
    FanIn { node: usize, port: usize },
    #[error("self-loop on node {node}")]
    SelfLoop { node: usize },
    #[error("edge src_port {port} out of bounds for node {node} (has {len} outputs)")]
    SrcPortOob { node: usize, port: usize, len: usize },
    #[error("edge dst_port {port} out of bounds for node {node} (has {len} inputs)")]
    DstPortOob { node: usize, port: usize, len: usize },
}

pub fn validate(graph: &Graph) -> Result<(), Vec<ValidationError>> {
    let mut errors = Vec::new();
    let mut seen: HashSet<(usize, usize)> = HashSet::new();

    for edge in graph.edges() {
        if edge.src_node == edge.dst_node {
            errors.push(ValidationError::SelfLoop { node: edge.src_node });
        }

        let src_len = graph.nodes()[edge.src_node].outputs.len();
        if edge.src_port >= src_len {
            errors.push(ValidationError::SrcPortOob {
                node: edge.src_node,
                port: edge.src_port,
                len: src_len,
            });
        }

        let dst_len = graph.nodes()[edge.dst_node].inputs.len();
        if edge.dst_port >= dst_len {
            errors.push(ValidationError::DstPortOob {
                node: edge.dst_node,
                port: edge.dst_port,
                len: dst_len,
            });
        }

        let key = (edge.dst_node, edge.dst_port);
        if seen.contains(&key) {
            errors.push(ValidationError::FanIn { node: edge.dst_node, port: edge.dst_port });
        } else {
            seen.insert(key);
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
