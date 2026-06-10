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
        other => anyhow::bail!("unknown subcommand: {other}. usage: ailang <eval|emit|save|load> <file>"),
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
