# TASK 0013: Codegen Wiring — wire edges into emitted Rust bindings
Phase: 2
Crate: ailang-transpile
Depends on: 0011 (codegen scaffold)

## Goal
Extend `codegen` in `ailang-transpile` so that edges are reflected in the emitted Rust
code. Currently each node emits a standalone `let` binding. After this task, nodes with
inputs receive the upstream port values via local variable rebinding before their own
`let` binding — making it possible for `Code:<expr>` nodes to reference their inputs
by port name.

## Current behaviour (DO NOT REMOVE)
- Topological sort via Kahn's algorithm, returns `CodegenError::Cycle` on cycle
- `"Const:<port>"` → `let node_{i}_{port}: {ty} = todo!("Const");`
- `"Code:<expr>"` → `let node_{i}_out: {ty} = <expr>;`
- other kinds → `let _node_{i} = todo!("<kind>");`

## Output naming convention (must be consistent across both tasks)
Each node output port named `P` on node index `I` is bound as:
```
node_{I}_{P}
```
Example: node 2, port named `"out"` → variable `node_2_out`.

## New behaviour: input wiring
For every edge `(src_node, src_port_name) → (dst_node, dst_port_name)`:
Before emitting node `dst_node`'s own `let` binding, emit:
```rust
    let node_{dst_node}_{dst_port_name} = node_{src_node}_{src_port_name};
```
This makes the input available under the dst port name. A `Code:` node can then
write `Code:{dst_port_name} + 1` and the variable is in scope.

## Codegen rules (full updated set)
1. Topological sort (Kahn's). Return `CodegenError::Cycle` on cycle.
2. Emit `pub fn {fn_name}() {{\n`
3. For each node in topological order:
   a. For each incoming edge to this node, emit:
      `    let node_{dst}_{dst_port} = node_{src}_{src_port};\n`
      (use the port **names** from `graph.nodes()[src].outputs[src_port].name`
       and `graph.nodes()[dst].inputs[dst_port].name`)
   b. Then emit the node's own binding:
      - `"Const:<port>"` → `    let node_{i}_{port}: {ty} = todo!(\"Const\");\n`
      - `"Code:<expr>"` → `    let node_{i}_out: {ty} = <expr>;\n`
      - other → `    let _node_{i} = todo!(\"{kind}\");\n`
4. Emit `}`

The type annotation in the `let` binding uses the first output port's type, rendered
via `serde_json::to_string(&ty).unwrap()` (same as before).

## Interface (unchanged)
```rust
// FILE: crates/ailang-transpile/src/codegen.rs
pub fn codegen(graph: &Graph, fn_name: &str) -> Result<String, CodegenError>;
```

## Cargo.toml (unchanged — keep exactly)
```toml
// FILE: crates/ailang-transpile/Cargo.toml
[package]
name = "ailang-transpile"
version.workspace = true
edition.workspace = true

[dependencies]
ailang-core    = { path = "../ailang-core" }
ailang-effects = { path = "../ailang-effects" }
thiserror.workspace = true
serde_json.workspace = true
```

## lib.rs (keep exactly)
```rust
// FILE: crates/ailang-transpile/src/lib.rs
pub mod codegen;
#[cfg(test)]
mod codegen_tests;
```

## Acceptance tests
```rust
// FILE: crates/ailang-transpile/src/codegen_tests.rs
#[cfg(test)]
mod tests {
    use ailang_core::{graph::{Graph, NodeDef, PortDef, Edge}, node_id::NodeId, ty::Type};
    use ailang_effects::EffectSet;
    use crate::codegen::codegen;

    fn make_node(seed: &[u8], kind: &str, inputs: Vec<PortDef>, outputs: Vec<PortDef>) -> NodeDef {
        NodeDef {
            id: NodeId::of(seed),
            kind: kind.into(),
            inputs,
            outputs,
            effects: EffectSet::empty(),
        }
    }

    #[test]
    fn empty_graph_emits_empty_fn() {
        let g = Graph::new();
        let src = codegen(&g, "run").unwrap();
        assert!(src.contains("pub fn run()"));
        assert!(src.contains("{}") || src.contains("{ }") || src.contains("{\n}"));
    }

    #[test]
    fn const_node_emits_todo() {
        let mut g = Graph::new();
        g.add_node(make_node(b"c", "Const:out",
            vec![],
            vec![PortDef { name: "out".into(), ty: Type::Int }],
        ));
        let src = codegen(&g, "run").unwrap();
        assert!(src.contains("node_0") || src.contains("Const"));
    }

    #[test]
    fn code_node_emits_expr() {
        let mut g = Graph::new();
        g.add_node(make_node(b"e", "Code:1 + 1",
            vec![],
            vec![PortDef { name: "out".into(), ty: Type::Int }],
        ));
        let src = codegen(&g, "run").unwrap();
        assert!(src.contains("1 + 1"));
    }

    #[test]
    fn wired_nodes_emit_binding() {
        // node 0: Const:out (Int) → node 1: Code:x + 1 (input port "x")
        let mut g = Graph::new();
        g.add_node(make_node(b"src", "Const:out",
            vec![],
            vec![PortDef { name: "out".into(), ty: Type::Int }],
        ));
        g.add_node(make_node(b"dst", "Code:x + 1",
            vec![PortDef { name: "x".into(), ty: Type::Int }],
            vec![PortDef { name: "out".into(), ty: Type::Int }],
        ));
        g.add_edge(0, 0, 1, 0).unwrap();
        let src = codegen(&g, "run").unwrap();
        // Must emit a rebinding that connects node_0_out to node_1_x
        assert!(src.contains("node_0_out"), "src output binding missing");
        assert!(src.contains("node_1_x") || src.contains("node_0_out"), "input wiring missing");
        assert!(src.contains("x + 1"), "Code expr missing");
    }
}
```
