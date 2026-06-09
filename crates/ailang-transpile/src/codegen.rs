use ailang_core::graph::Graph;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, thiserror::Error)]
pub enum CodegenError {
    #[error("cycle detected — cannot emit sequential code")]
    Cycle,
}

pub fn codegen(graph: &Graph, fn_name: &str) -> Result<String, CodegenError> {
    // Kahn's topo sort
    let mut in_degree: HashMap<usize, usize> = HashMap::new();
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
    if sorted_nodes.len() != graph.nodes().len() {
        return Err(CodegenError::Cycle);
    }

    let mut code = format!("pub fn {}() {{\n", fn_name);
    for &node_idx in &sorted_nodes {
        // Emit input rebindings from edges BEFORE the node's own binding
        for edge in graph.edges() {
            if edge.dst_node == node_idx {
                let src_port_name = &graph.nodes()[edge.src_node].outputs[edge.src_port].name;
                let dst_port_name = &graph.nodes()[edge.dst_node].inputs[edge.dst_port].name;
                code.push_str(&format!(
                    "    let node_{}_{} = node_{}_{};\n",
                    node_idx, dst_port_name, edge.src_node, src_port_name
                ));
            }
        }
        // Emit the node's own binding
        let node = &graph.nodes()[node_idx];
        let kind = node.kind.as_str();
        if kind.starts_with("Const:") {
            let port = &kind["Const:".len()..];
            let ty_str = if node.outputs.is_empty() {
                "()".to_string()
            } else {
                serde_json::to_string(&node.outputs[0].ty).unwrap()
            };
            code.push_str(&format!(
                "    let node_{}_{}: {} = todo!(\"Const\");\n",
                node_idx, port, ty_str
            ));
        } else if kind.starts_with("Code:") {
            let expr = &kind["Code:".len()..];
            let ty_str = if node.outputs.is_empty() {
                "()".to_string()
            } else {
                serde_json::to_string(&node.outputs[0].ty).unwrap()
            };
            code.push_str(&format!(
                "    let node_{}_out: {} = {};\n",
                node_idx, ty_str, expr
            ));
        } else {
            code.push_str(&format!(
                "    let _node_{} = todo!(\"{}\");\n",
                node_idx, kind
            ));
        }
    }
    code.push('}');
    Ok(code)
}
