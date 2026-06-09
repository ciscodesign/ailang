use std::process;

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {e}");
        process::exit(1);
    }
}

fn run() -> anyhow::Result<()> {
    let mut args = std::env::args().skip(1);
    let subcmd = args.next().ok_or_else(|| anyhow::anyhow!("usage: ailang <eval|emit> <graph.json>"))?;
    let path   = args.next().ok_or_else(|| anyhow::anyhow!("usage: ailang <eval|emit> <graph.json>"))?;

    let bytes = std::fs::read(&path)?;
    let graph = ailang_core::serial::decode(&bytes)?;

    match subcmd.as_str() {
        "eval" => {
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
        }
        "emit" => {
            let src = ailang_transpile::codegen::codegen(&graph, "run")?;
            print!("{src}");
        }
        other => anyhow::bail!("unknown subcommand: {other}"),
    }
    Ok(())
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
}
