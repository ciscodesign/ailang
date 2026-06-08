use crate::registry::{ExecError, NodeRegistry};
use crate::value::Value;
use ailang_core::graph::{Graph, NodeIdx};
use std::collections::{HashMap, VecDeque};
/// Result of evaluating a graph: map from NodeIdx to that node's output Values.
pub type EvalResult = HashMap<NodeIdx, HashMap<String, Value>>;
#[derive(Debug, thiserror::Error)]
pub enum EvalError {
    #[error("cycle detected in graph")]
    Cycle,
    #[error("node {0}: {1}")]
    NodeFailed(NodeIdx, ExecError),
    #[error("missing output for edge from node {0} port {1}")]
    MissingOutput(NodeIdx, usize),
}
/// Evaluate the graph using the given registry.
/// Executes nodes in topological order (Kahn's algorithm).
/// Returns the outputs of every node.
pub fn eval(graph: &Graph, registry: &NodeRegistry) -> Result<EvalResult, EvalError> {
    let mut result = EvalResult::new();
    let num_nodes = graph.nodes().len();
    // Kahn's algorithm for topological sort
    let mut in_degree = vec![0; num_nodes];
    let mut queue = VecDeque::new();
    // Calculate in-degrees and initialize the queue with nodes having zero in-degree
    for edge in graph.edges() {
        in_degree[edge.dst_node] += 1;
    }
    for (idx, _) in graph.nodes().iter().enumerate() {
        if in_degree[idx] == 0 {
            queue.push_back(idx);
        }
    }
    // Process nodes in topological order
    while let Some(node_idx) = queue.pop_front() {
        let node_def = &graph.nodes()[node_idx];
        let mut inputs = HashMap::new();
        // Collect input values from connected edges
        for edge in graph.edges() {
            if edge.dst_node == node_idx {
                let src_outputs = result.get(&edge.src_node).ok_or(EvalError::MissingOutput(edge.src_node, edge.src_port))?;
                let value = src_outputs.get(&graph.nodes()[edge.src_node].outputs[edge.src_port].name)
                    .ok_or(EvalError::MissingOutput(edge.src_node, edge.src_port))?;
                inputs.insert(graph.nodes()[node_idx].inputs[edge.dst_port].name.clone(), value.clone());
            }
        }
        // Call the node's function with collected inputs
        match registry.call(&node_def.kind, inputs) {
            Ok(outputs) => {
                result.insert(node_idx, outputs);
                for edge in graph.edges() {
                    if edge.src_node == node_idx {
                        in_degree[edge.dst_node] -= 1;
                        if in_degree[edge.dst_node] == 0 {
                            queue.push_back(edge.dst_node);
                        }
                    }
                }
            },
            Err(e) => return Err(EvalError::NodeFailed(node_idx, e)),
        }
    }
    // If we processed all nodes, return the result
    if result.len() == num_nodes {
        Ok(result)
    } else {
        Err(EvalError::Cycle)
    }
}
