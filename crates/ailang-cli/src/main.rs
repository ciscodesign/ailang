use std::process;

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {e}");
        process::exit(1);
    }
}

fn run() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let subcmd = args.first().map(|s| s.as_str()).unwrap_or("");

    match subcmd {
        "eval" | "emit" => {
            let path = args.get(1).ok_or_else(|| anyhow::anyhow!("usage: ailang {subcmd} <graph.json>"))?;
            let bytes = std::fs::read(path)?;
            let graph = ailang_core::serial::decode(&bytes)?;
            if subcmd == "eval" {
                let mut registry = ailang_exec::registry::NodeRegistry::new();
                ailang_nodes::builtins::register_builtins(&mut registry);
                // auto-register any Const:<port>:<literal> nodes found in the graph
                for node in graph.nodes() {
                    if node.kind.starts_with("Const:") {
                        ailang_nodes::builtins::register_const_literal(&mut registry, &node.kind);
                    }
                }
                let result = ailang_exec::eval::eval(&graph, &registry)?;
                let mut pairs: Vec<_> = result.into_iter().collect();
                pairs.sort_by_key(|(k, _)| *k);
                for (node_idx, outputs) in pairs {
                    let json_map: serde_json::Map<String, serde_json::Value> = outputs
                        .iter()
                        .map(|(k, v)| (k.clone(), value_to_json(v)))
                        .collect();
                    println!("node {node_idx}: {}", serde_json::to_string(&json_map).unwrap());
                }
            } else {
                let src = ailang_transpile::codegen::codegen(&graph, "run")?;
                print!("{src}");
            }
        }
        "save" => {
            let path = args.get(1).ok_or_else(|| anyhow::anyhow!("usage: ailang save <output.json>"))?;
            let g = demo_graph();
            let json = ailang_core::serial::encode(&g)?;
            std::fs::write(path, &json)?;
            println!("saved {} bytes to {path}", json.len());
        }
        "load" => {
            let path = args.get(1).ok_or_else(|| anyhow::anyhow!("usage: ailang load <input.json>"))?;
            let json = std::fs::read(path)?;
            let g = ailang_core::serial::decode(&json)?;
            println!("loaded graph: {} nodes, {} edges", g.nodes().len(), g.edges().len());
        }
        "inspect" => {
            let path = args.get(1).ok_or_else(|| anyhow::anyhow!("usage: ailang inspect <graph.json>"))?;
            let bytes = std::fs::read(path)?;
            let graph = ailang_core::serial::decode(&bytes)?;
            println!("Graph: {} nodes, {} edges", graph.nodes().len(), graph.edges().len());
            println!();
            for (i, node) in graph.nodes().iter().enumerate() {
                let inputs: Vec<String> = node.inputs.iter()
                    .map(|p| format!("{}: {:?}", p.name, p.ty))
                    .collect();
                let outputs: Vec<String> = node.outputs.iter()
                    .map(|p| format!("{}: {:?}", p.name, p.ty))
                    .collect();
                println!(
                    "  node {:>2}  kind={}\n           in=[{}]\n           out=[{}]",
                    i, node.kind,
                    inputs.join(", "),
                    outputs.join(", ")
                );
            }
            if !graph.edges().is_empty() {
                println!();
                for edge in graph.edges() {
                    println!(
                        "  edge  node{}[{}] → node{}[{}]",
                        edge.src_node, edge.src_port,
                        edge.dst_node, edge.dst_port
                    );
                }
            }
        }
        "validate" => {
            let path = args.get(1).ok_or_else(|| anyhow::anyhow!("usage: ailang validate <graph.json>"))?;
            let bytes = std::fs::read(path)?;
            let graph = ailang_core::serial::decode(&bytes)?;
            match ailang_core::validator::validate(&graph) {
                Ok(()) => println!("ok — graph is valid"),
                Err(errors) => {
                    eprintln!("{} validation error(s):", errors.len());
                    for e in &errors {
                        eprintln!("  - {e:?}");
                    }
                    std::process::exit(1);
                }
            }
        }
        "dot" => {
            let path = args.get(1).ok_or_else(|| anyhow::anyhow!("usage: ailang dot <graph.json>"))?;
            let bytes = std::fs::read(path)?;
            let graph = ailang_core::serial::decode(&bytes)?;
            println!("digraph ailang {{");
            println!("  rankdir=LR;");
            println!("  node [shape=box fontname=\"monospace\"];");
            for (i, node) in graph.nodes().iter().enumerate() {
                let label = node.kind.replace('"', "\\\"");
                println!("  n{i} [label=\"{i}: {label}\"];");
            }
            for edge in graph.edges() {
                let src_port = &graph.nodes()[edge.src_node].outputs[edge.src_port].name;
                let dst_port = &graph.nodes()[edge.dst_node].inputs[edge.dst_port].name;
                println!(
                    "  n{} -> n{} [label=\"{} → {}\"];",
                    edge.src_node, edge.dst_node, src_port, dst_port
                );
            }
            println!("}}");
        }
        "optimize" => {
            let in_path  = args.get(1).ok_or_else(|| anyhow::anyhow!("usage: ailang optimize <in.json> <out.json>"))?;
            let out_path = args.get(2).ok_or_else(|| anyhow::anyhow!("usage: ailang optimize <in.json> <out.json>"))?;
            let bytes = std::fs::read(in_path)?;
            let graph = ailang_core::serial::decode(&bytes)?;
            let before = (graph.nodes().len(), graph.edges().len());
            let optimized = ailang_fold::prune_dead(&graph);
            let after = (optimized.nodes().len(), optimized.edges().len());
            let json = ailang_core::serial::encode(&optimized)?;
            std::fs::write(out_path, &json)?;
            println!(
                "optimized: {} → {} nodes, {} → {} edges (wrote {})",
                before.0, after.0, before.1, after.1, out_path
            );
        }
        other => anyhow::bail!("unknown subcommand: {other}. usage: ailang <eval|emit|inspect|validate|dot|optimize|save|load> <file>"),
    }
    Ok(())
}

fn demo_graph() -> ailang_core::graph::Graph {
    ailang_core::graph::Graph::new()
}

fn value_to_json(v: &ailang_exec::value::Value) -> serde_json::Value {
    use ailang_exec::value::Value;
    match v {
        Value::Text(s)             => serde_json::Value::String(s.clone()),
        Value::Int(n)              => serde_json::json!(n),
        Value::Float(f)            => serde_json::json!(f),
        Value::Bool(b)             => serde_json::Value::Bool(*b),
        Value::Bytes(b)            => serde_json::Value::String(
            b.iter().map(|x| format!("{x:02x}")).collect::<String>()
        ),
        Value::Option(Some(inner)) => value_to_json(inner),
        Value::Option(None)        => serde_json::Value::Null,
        Value::Result(Ok(v))       => serde_json::json!({"ok": value_to_json(v)}),
        Value::Result(Err(e))      => serde_json::json!({"err": value_to_json(e)}),
        Value::List(items)         => serde_json::Value::Array(items.iter().map(value_to_json).collect()),
        Value::Map(m)              => serde_json::Value::Object(
            m.iter().map(|(k, v)| (k.clone(), value_to_json(v))).collect()
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emit_empty_graph_contains_fn() {
        let g = ailang_core::graph::Graph::new();
        let bytes = ailang_core::serial::encode(&g).unwrap();
        let graph = ailang_core::serial::decode(&bytes).unwrap();
        let src = ailang_transpile::codegen::codegen(&graph, "run").unwrap();
        assert!(src.contains("pub fn run()"));
    }

    #[test]
    fn eval_empty_graph_succeeds() {
        let g = ailang_core::graph::Graph::new();
        let bytes = ailang_core::serial::encode(&g).unwrap();
        let graph = ailang_core::serial::decode(&bytes).unwrap();
        let mut registry = ailang_exec::registry::NodeRegistry::new();
        ailang_nodes::builtins::register_builtins(&mut registry);
        let result = ailang_exec::eval::eval(&graph, &registry).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn save_and_load_roundtrip() {
        let g = demo_graph();
        let json = ailang_core::serial::encode(&g).unwrap();
        let dir = std::env::temp_dir();
        let path = dir.join("ailang_test_roundtrip.json");
        std::fs::write(&path, &json).unwrap();
        let loaded_json = std::fs::read(&path).unwrap();
        let g2 = ailang_core::serial::decode(&loaded_json).unwrap();
        assert_eq!(g.nodes().len(), g2.nodes().len());
        assert_eq!(g.edges().len(), g2.edges().len());
        let _ = std::fs::remove_file(&path);
    }
}
