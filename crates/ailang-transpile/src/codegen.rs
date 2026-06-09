use ailang_core::graph::Graph;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, thiserror::Error)]
pub enum CodegenError {
    #[error("cycle detected — cannot emit sequential code")]
    Cycle,
}

fn topo_sort(graph: &Graph) -> Result<Vec<usize>, CodegenError> {
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
    let mut sorted = vec![];
    while let Some(node_idx) = queue.pop_front() {
        sorted.push(node_idx);
        for edge in graph.edges() {
            if edge.src_node == node_idx {
                let deg = in_degree.get_mut(&edge.dst_node).unwrap();
                *deg -= 1;
                if *deg == 0 {
                    queue.push_back(edge.dst_node);
                }
            }
        }
    }
    if sorted.len() != graph.nodes().len() {
        return Err(CodegenError::Cycle);
    }
    Ok(sorted)
}

fn emit_node(graph: &Graph, node_idx: usize, code: &mut String) {
    // Input rebindings from edges
    for edge in graph.edges() {
        if edge.dst_node == node_idx {
            let src_port = &graph.nodes()[edge.src_node].outputs[edge.src_port].name;
            let dst_port = &graph.nodes()[edge.dst_node].inputs[edge.dst_port].name;
            code.push_str(&format!(
                "    let node_{}_{} = node_{}_{};\n",
                node_idx, dst_port, edge.src_node, src_port
            ));
        }
    }
    // Node's own binding
    let node = &graph.nodes()[node_idx];
    let kind = node.kind.as_str();
    if let Some(rest) = kind.strip_prefix("Const:") {
        let (port, literal_opt) = match rest.find(':') {
            Some(pos) => (&rest[..pos], Some(&rest[pos + 1..])),
            None => (rest, None),
        };
        let ty_str = if node.outputs.is_empty() {
            "()".to_string()
        } else {
            serde_json::to_string(&node.outputs[0].ty).unwrap()
        };
        match literal_opt {
            Some(lit) => code.push_str(&format!(
                "    let node_{}_{}: {} = {};\n",
                node_idx, port, ty_str, lit
            )),
            None => code.push_str(&format!(
                "    let node_{}_{}: {} = todo!(\"Const\");\n",
                node_idx, port, ty_str
            )),
        }
    } else if let Some(expr) = kind.strip_prefix("Code:") {
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

pub fn codegen(graph: &Graph, fn_name: &str) -> Result<String, CodegenError> {
    let sorted = topo_sort(graph)?;
    let mut code = format!("pub fn {}() {{\n", fn_name);
    for &node_idx in &sorted {
        emit_node(graph, node_idx, &mut code);
    }
    code.push('}');
    Ok(code)
}

fn wasm_ret_type(ty: &ailang_core::ty::Type) -> &'static str {
    use ailang_core::ty::Type;
    match ty {
        Type::Int => "i64",
        Type::Float => "f64",
        Type::Bool => "i32",
        Type::Text => "*const u8",
        _ => "()",
    }
}

pub fn codegen_wasm(graph: &Graph, fn_name: &str) -> Result<String, CodegenError> {
    let sorted = topo_sort(graph)?;

    let last_output = sorted.iter().rev().find_map(|&idx| {
        let node = &graph.nodes()[idx];
        node.outputs.first().map(|p| (idx, p.name.clone(), p.ty.clone()))
    });

    let ret = last_output
        .as_ref()
        .map(|(_, _, ty)| wasm_ret_type(ty))
        .unwrap_or("()");

    let mut code = String::new();
    code.push_str("#[no_mangle]\n");
    if ret == "()" {
        code.push_str(&format!("pub extern \"C\" fn {}() {{\n", fn_name));
    } else {
        code.push_str(&format!("pub extern \"C\" fn {}() -> {} {{\n", fn_name, ret));
    }

    for &node_idx in &sorted {
        emit_node(graph, node_idx, &mut code);
    }

    if ret != "()" {
        if let Some((last_idx, port_name, _)) = &last_output {
            code.push_str(&format!("    node_{}_{}\n", last_idx, port_name));
        }
    }

    code.push('}');
    Ok(code)
}
