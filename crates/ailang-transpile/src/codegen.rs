use ailang_core::graph::Graph;
use ailang_core::ty::Type;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, thiserror::Error)]
pub enum CodegenError {
    #[error("cycle detected — cannot emit sequential code")]
    Cycle,
}

fn rust_type(ty: &Type) -> String {
    match ty {
        Type::Int   => "i64".into(),
        Type::Float => "f64".into(),
        Type::Bool  => "bool".into(),
        Type::Text  => "String".into(),
        Type::Bytes => "Vec<u8>".into(),
        Type::List(inner) => format!("Vec<{}>", rust_type(inner)),
        Type::Option(inner) => format!("Option<{}>", rust_type(inner)),
        Type::Result(ok, err) => format!("Result<{}, {}>", rust_type(ok), rust_type(err)),
        _ => "()".into(),
    }
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

fn builtin_expr(kind: &str, node_idx: usize) -> Option<(&'static str, String)> {
    let i = node_idx;
    Some(match kind {
        "add_int"      => ("i64",  format!("node_{i}_a + node_{i}_b")),
        "sub_int"      => ("i64",  format!("node_{i}_a - node_{i}_b")),
        "mul_int"      => ("i64",  format!("node_{i}_a * node_{i}_b")),
        "div_int"      => ("i64",  format!("node_{i}_a / node_{i}_b")),
        "mod_int"      => ("i64",  format!("node_{i}_a % node_{i}_b")),
        "neg_int"      => ("i64",  format!("-node_{i}_a")),
        "abs_int"      => ("i64",  format!("node_{i}_a.abs()")),
        "min_int"      => ("i64",  format!("node_{i}_a.min(node_{i}_b)")),
        "max_int"      => ("i64",  format!("node_{i}_a.max(node_{i}_b)")),
        "eq_int"       => ("bool", format!("node_{i}_a == node_{i}_b")),
        "lt_int"       => ("bool", format!("node_{i}_a < node_{i}_b")),
        "gt_int"       => ("bool", format!("node_{i}_a > node_{i}_b")),
        "not_bool"     => ("bool", format!("!node_{i}_a")),
        "and_bool"     => ("bool", format!("node_{i}_a && node_{i}_b")),
        "or_bool"      => ("bool", format!("node_{i}_a || node_{i}_b")),
        "concat_text"       => ("String", format!("format!(\"{{}}{{}}\" , node_{i}_a, node_{i}_b)")),
        "len_text"          => ("i64",    format!("node_{i}_a.len() as i64")),
        "int_to_text"       => ("String", format!("node_{i}_a.to_string()")),
        "bool_to_text"      => ("String", format!("node_{i}_a.to_string()")),
        "eq_float"          => ("bool",   format!("node_{i}_a == node_{i}_b")),
        "lt_float"          => ("bool",   format!("node_{i}_a < node_{i}_b")),
        "gt_float"          => ("bool",   format!("node_{i}_a > node_{i}_b")),
        "if_float"          => ("f64",    format!("if node_{i}_cond {{ node_{i}_then }} else {{ node_{i}_else_ }}")),
        "if_text"           => ("String", format!("if node_{i}_cond {{ node_{i}_then }} else {{ node_{i}_else_ }}")),
        "if_bool"           => ("bool",   format!("if node_{i}_cond {{ node_{i}_then }} else {{ node_{i}_else_ }}")),
        "trim_text"         => ("String", format!("node_{i}_a.trim().to_string()")),
        "to_upper_text"     => ("String", format!("node_{i}_a.to_uppercase()")),
        "to_lower_text"     => ("String", format!("node_{i}_a.to_lowercase()")),
        "contains_text"     => ("bool",   format!("node_{i}_a.contains(node_{i}_b.as_str())")),
        "starts_with_text"  => ("bool",   format!("node_{i}_a.starts_with(node_{i}_b.as_str())")),
        "ends_with_text"    => ("bool",   format!("node_{i}_a.ends_with(node_{i}_b.as_str())")),
        "replace_text"      => ("String", format!("node_{i}_a.replace(node_{i}_from.as_str(), node_{i}_to.as_str())")),
        "add_float"         => ("f64",    format!("node_{i}_a + node_{i}_b")),
        "sub_float"         => ("f64",    format!("node_{i}_a - node_{i}_b")),
        "mul_float"         => ("f64",    format!("node_{i}_a * node_{i}_b")),
        "div_float"         => ("f64",    format!("node_{i}_a / node_{i}_b")),
        "neg_float"         => ("f64",    format!("-node_{i}_a")),
        "abs_float"         => ("f64",    format!("node_{i}_a.abs()")),
        "floor_float"       => ("i64",    format!("node_{i}_a.floor() as i64")),
        "ceil_float"        => ("i64",    format!("node_{i}_a.ceil() as i64")),
        "round_float"       => ("i64",    format!("node_{i}_a.round() as i64")),
        "int_to_float"      => ("f64",    format!("node_{i}_a as f64")),
        "float_to_int"      => ("i64",    format!("node_{i}_a as i64")),
        "float_to_text"     => ("String", format!("node_{i}_a.to_string()")),
        _ => return None,
    })
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

    let node = &graph.nodes()[node_idx];
    let kind = node.kind.as_str();

    if let Some(rest) = kind.strip_prefix("Const:") {
        let (port, literal_opt) = match rest.find(':') {
            Some(pos) => (&rest[..pos], Some(&rest[pos + 1..])),
            None => (rest, None),
        };
        let ty_str = node.outputs.first().map_or("()".into(), |p| rust_type(&p.ty));
        match literal_opt {
            Some(lit) => {
                let expr = if ty_str == "String" {
                    format!("\"{}\".to_string()", lit.replace('\\', "\\\\").replace('"', "\\\""))
                } else {
                    lit.to_string()
                };
                code.push_str(&format!(
                    "    let node_{}_{}: {} = {};\n",
                    node_idx, port, ty_str, expr
                ));
            }
            None => code.push_str(&format!(
                "    let node_{}_{}: {} = todo!(\"Const\");\n",
                node_idx, port, ty_str
            )),
        }
    } else if let Some(expr) = kind.strip_prefix("Code:") {
        let ty_str = node.outputs.first().map_or("()".into(), |p| rust_type(&p.ty));
        code.push_str(&format!(
            "    let node_{}_out: {} = {};\n",
            node_idx, ty_str, expr
        ));
    } else if let Some((ty_str, expr)) = builtin_expr(kind, node_idx) {
        let out_port = node.outputs.first().map(|p| p.name.as_str()).unwrap_or("out");
        code.push_str(&format!(
            "    let node_{}_{}: {} = {};\n",
            node_idx, out_port, ty_str, expr
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

fn wasm_ret_type(ty: &Type) -> &'static str {
    match ty {
        Type::Int   => "i64",
        Type::Float => "f64",
        Type::Bool  => "i32",
        Type::Text  => "*const u8",
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
