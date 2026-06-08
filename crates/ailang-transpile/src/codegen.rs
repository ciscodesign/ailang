use ailang_core::graph::Graph;
use std::collections::{HashMap, VecDeque};
#[derive(Debug, thiserror::Error)]
pub enum CodegenError {
    #[error("cycle detected — cannot emit sequential code")]
    Cycle,
}
/// Emit a Rust function named `fn_name` that executes the graph.
/// Returns the complete function source as a String.
pub fn codegen(graph: &Graph, fn_name: &str) -> Result<String, CodegenError> {
    // Perform topological sort using Kahn's algorithm
    let mut in_degree = HashMap::new();
    for node_idx in 0..graph.nodes().len() {
        in_degree.insert(node_idx, 0);
    }
    for edge in graph.edges() {
        *in_degree.get_mut(&edge.dst_node).unwrap() += 1;
    }
    let mut queue = VecDeque::new();
    for (&node_idx, &degree) in &in_degree {
        if degree == 0 {
            queue.push_back(node_idx);
        }
    }
    let mut sorted_nodes = vec![];
    while let Some(node_idx) = queue.pop_front() {
        sorted_nodes.push(node_idx);
        for edge in graph.edges() {
            if edge.src_node == node_idx {
                let new_degree = in_degree.get_mut(&edge.dst_node).unwrap();
                *new_degree -= 1;
                if *new_degree == 0 {
                    queue.push_back(edge.dst_node);
                }
            }
        }
    }
    // If we didn't process all nodes, there's a cycle
    if sorted_nodes.len() != graph.nodes().len() {
        return Err(CodegenError::Cycle);
    }
    let mut code = format!("pub fn {}() {{\n", fn_name);
    for &node_idx in &sorted_nodes {
        let node = &graph.nodes()[node_idx];
        match node.kind.as_str() {
            "Const:out" => {
                code.push_str(&format!(
                    "    let node_{}_out: {} = todo!(\"Const\");\n",
                    node_idx, serde_json::to_string(&node.outputs[0].ty).unwrap()
                ));
            }
            kind if kind.starts_with("Code:") => {
                code.push_str(&format!(
                    "    let node_{}_out: {} = {};\n",
                    node_idx,
                    serde_json::to_string(&node.outputs[0].ty).unwrap(),
                    &kind["Code:".len()..]
                ));
            }
            _ => {
                code.push_str(&format!(
                    "    let _node_{} = todo!(\"{}\");\n",
                    node_idx, node.kind
                ));
            }
        }
    }
    code.push('}');
    Ok(code)
}
