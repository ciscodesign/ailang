---
phase: quick-260609-ele
plan: 01
type: execute
wave: 1
depends_on: []
files_modified:
  - crates/ailang-transpile/src/codegen.rs
  - crates/ailang-transpile/src/lib.rs
  - crates/ailang-core/src/builder.rs
  - crates/ailang-cli/src/main.rs
autonomous: true
requirements: [0013, 0014, 0015]

must_haves:
  truths:
    - "codegen emits correct let-bindings for Const, Code, and other node kinds"
    - "codegen emits edge rebindings before each node's own binding"
    - "GraphBuilder compiles and all 5 builder_tests pass"
    - "ailang-cli compiles and its 2 integration tests pass"
    - "all 59+ existing tests still pass after changes"
  artifacts:
    - path: "crates/ailang-transpile/src/codegen.rs"
      provides: "Full codegen with CodegenError defined locally, Kahn sort, let-bindings, edge wiring"
    - path: "crates/ailang-transpile/src/lib.rs"
      provides: "Correct module declarations — pub mod codegen, cfg(test) mod codegen_tests"
    - path: "crates/ailang-core/src/builder.rs"
      provides: "GraphBuilder with new/const_node/code_node/node/edge/build"
    - path: "crates/ailang-cli/src/main.rs"
      provides: "ailang eval and emit subcommands with value_to_json helper"
  key_links:
    - from: "codegen.rs"
      to: "graph.edges()"
      via: "edge loop before node emit"
      pattern: "for edge in edges.*dst_node == node_idx"
    - from: "main.rs eval branch"
      to: "value_to_json"
      via: "outputs converted before serde_json::to_string"
      pattern: "value_to_json"
---

<objective>
Implement ailang Phase 2 tasks 0013–0015 by restoring correct codegen with edge wiring,
verifying GraphBuilder correctness, and fixing the CLI's Value serialization.

Purpose: The orchestrator left codegen.rs broken (stub-only, wrong CodegenError import),
lib.rs with a bad re-export, and the CLI using serde_json directly on Value (which does
not implement Serialize). This plan fixes all three files so the workspace compiles
cleanly and all tests pass.

Output: Correct codegen.rs + lib.rs, verified builder.rs, fixed main.rs — workspace
compiles with cargo test passing for all crates.
</objective>

<execution_context>
export PATH="/opt/homebrew/opt/rustup/bin:$PATH"
</execution_context>

<context>
@/Users/cisco/Documents/1_Progetti/ailang/orchestrator/tasks/0013-codegen-wiring.md
@/Users/cisco/Documents/1_Progetti/ailang/orchestrator/tasks/0014-graph-builder.md
@/Users/cisco/Documents/1_Progetti/ailang/orchestrator/tasks/0015-cli.md
@/Users/cisco/Documents/1_Progetti/ailang/crates/ailang-core/src/graph.rs
@/Users/cisco/Documents/1_Progetti/ailang/crates/ailang-transpile/src/codegen_tests.rs
@/Users/cisco/Documents/1_Progetti/ailang/crates/ailang-core/src/builder_tests.rs

<interfaces>
<!-- From ailang-core/src/graph.rs — key types the executor needs -->

pub type NodeIdx = usize;

pub struct NodeDef {
    pub id: NodeId,
    pub kind: String,
    pub inputs: Vec<PortDef>,
    pub outputs: Vec<PortDef>,
    pub effects: EffectSet,
}

pub struct PortDef {
    pub name: String,
    pub ty: Type,
}

pub struct Edge {
    pub src_node: NodeIdx,
    pub src_port: usize,
    pub dst_node: NodeIdx,
    pub dst_port: usize,
}

impl Graph {
    pub fn nodes(&self) -> &Vec<NodeDef>;
    pub fn edges(&self) -> &Vec<Edge>;
    pub fn add_node(&mut self, node: NodeDef) -> NodeIdx;
    pub fn add_edge(&mut self, src_node: NodeIdx, src_port: usize,
                    dst_node: NodeIdx, dst_port: usize) -> Result<(), GraphError>;
}
</interfaces>
</context>

<tasks>

<task type="auto">
  <name>Task 1: Restore codegen.rs with edge wiring + fix lib.rs</name>
  <files>
    crates/ailang-transpile/src/codegen.rs
    crates/ailang-transpile/src/lib.rs
  </files>
  <action>
The orchestrator broke codegen.rs (emits only comments, uses `crate::CodegenError` which
isn't defined) and lib.rs (has `pub use codegen::CodegenError` which requires CodegenError
to be pub-exported from codegen.rs — the spec says it must be defined in codegen.rs only).

**Write lib.rs exactly as:**
```rust
pub mod codegen;
#[cfg(test)]
mod codegen_tests;
```
No `pub use`. No re-export of CodegenError.

**Write codegen.rs from scratch** with these exact semantics:

1. Define `CodegenError` locally (do NOT import from crate root):
```rust
use thiserror::Error;
#[derive(Debug, Error)]
pub enum CodegenError {
    #[error("graph contains a cycle")]
    Cycle,
}
```

2. Imports needed:
```rust
use std::collections::{HashMap, VecDeque};
use ailang_core::graph::{Graph, NodeIdx};
```

3. `pub fn codegen(graph: &Graph, fn_name: &str) -> Result<String, CodegenError>`

4. Kahn's topological sort (same structure as currently in codegen.rs — the sort itself
   is already correct, keep it). Return `Err(CodegenError::Cycle)` if `order.len() != nodes.len()`.

5. Build a per-destination-node edge lookup before emitting. For the emit loop, for each
   node in topo order:
   a. Collect all edges where `edge.dst_node == node_idx`. For each such edge, emit:
      `    let node_{dst_node}_{dst_port_name} = node_{src_node}_{src_port_name};\n`
      where `dst_port_name = nodes[edge.dst_node].inputs[edge.dst_port].name`
      and   `src_port_name = nodes[edge.src_node].outputs[edge.src_port].name`
   b. Then emit the node's own binding:
      - kind starts with `"Const:"`: extract port = kind after "Const:", get ty from
        `nodes[i].outputs[0].ty`, emit:
        `    let node_{i}_{port}: {ty_json} = todo!("Const");\n`
        where `ty_json = serde_json::to_string(&ty).unwrap()`
      - kind starts with `"Code:"`: extract expr = kind after "Code:", get ty from
        `nodes[i].outputs[0].ty`, emit:
        `    let node_{i}_out: {ty_json} = {expr};\n`
      - other: emit:
        `    let _node_{i} = todo!("{kind}");\n`

6. Wrap in `pub fn {fn_name}() {{\n ... }}\n`

Note: The existing Kahn sort in codegen.rs is correct — reuse its structure. The only
additions are: local CodegenError definition, the edge-rebinding emit step (5a), and
correct let-binding emit replacing the current comment-only output (5b).
  </action>
  <verify>
    <automated>export PATH="/opt/homebrew/opt/rustup/bin:$PATH" && cd /Users/cisco/Documents/1_Progetti/ailang && cargo test -p ailang-transpile 2>&1</automated>
  </verify>
  <done>
    `cargo test -p ailang-transpile` passes all 4 tests:
    empty_graph_emits_empty_fn, const_node_emits_todo, code_node_emits_expr,
    wired_nodes_emit_binding. No compile errors.
  </done>
</task>

<task type="auto">
  <name>Task 2: Verify and fix GraphBuilder (0014)</name>
  <files>
    crates/ailang-core/src/builder.rs
  </files>
  <action>
The orchestrator wrote builder.rs which looks structurally correct. Run the tests first.
If `cargo test -p ailang-core` passes all 5 builder tests plus all existing core tests
(33 total previously), no changes needed.

If it fails, the likely issues are:
- Missing `Default` impl → already present in the file, should be fine
- `EffectSet` import — the file uses `use ailang_effects::{EffectSet}` with curly braces
  around a single item; this is valid Rust but if EffectSet path changed, fix to
  `use ailang_effects::EffectSet;`
- `NodeDef` field order mismatch — verify against graph.rs struct definition

The ailang-core lib.rs already has the correct module declarations including
`pub mod builder` and `#[cfg(test)] mod builder_tests` — do NOT modify lib.rs.

Only make changes if `cargo test -p ailang-core` reports actual errors.
  </action>
  <verify>
    <automated>export PATH="/opt/homebrew/opt/rustup/bin:$PATH" && cd /Users/cisco/Documents/1_Progetti/ailang && cargo test -p ailang-core 2>&1</automated>
  </verify>
  <done>
    `cargo test -p ailang-core` passes all tests including the 5 new builder tests
    (empty_build, const_node, code_node_with_inputs, generic_node,
    edge_type_mismatch_returns_error). Total should be 38+ tests passing.
  </done>
</task>

<task type="auto">
  <name>Task 3: Fix CLI main.rs Value serialization (0015)</name>
  <files>
    crates/ailang-cli/src/main.rs
  </files>
  <action>
The orchestrator wrote most of main.rs correctly but has a bug in the eval branch:
`serde_json::to_string(&outputs)?` where `outputs` is `HashMap<String, Value>` and
`Value` does not derive `Serialize`. This will fail to compile.

Fix the eval branch to use `value_to_json`. The corrected eval output block:
```rust
for (node_idx, outputs) in pairs {
    let json_map: serde_json::Map<String, serde_json::Value> = outputs
        .iter()
        .map(|(k, v)| (k.clone(), value_to_json(v)))
        .collect();
    println!("node {node_idx}: {}", serde_json::to_string(&json_map).unwrap());
}
```

Also remove the unused `use std::collections::HashMap` and `use anyhow::Context` imports
if they cause `unused_imports` warnings that fail the build. Keep all other code as-is:
the `value_to_json` function, the `run()` function structure, the `emit` branch, and
the `#[cfg(test)] mod tests` block at the bottom (which tests codegen/eval on empty
graphs and does not use `value_to_json` directly).

The `ailang-cli/Cargo.toml` already has `hex = "0.4"` — do NOT modify it.
The workspace `Cargo.toml` already has `ailang-cli` as a member — do NOT modify it.
  </action>
  <verify>
    <automated>export PATH="/opt/homebrew/opt/rustup/bin:$PATH" && cd /Users/cisco/Documents/1_Progetti/ailang && cargo test -p ailang-cli 2>&1</automated>
  </verify>
  <done>
    `cargo test -p ailang-cli` compiles and passes both tests:
    emit_empty_graph_contains_fn, eval_empty_graph_succeeds.
    Then run `cargo test --workspace` to confirm all 59+ tests still pass.
  </done>
</task>

</tasks>

<verification>
After all three tasks complete, run the full workspace test suite:

```
export PATH="/opt/homebrew/opt/rustup/bin:$PATH"
cd /Users/cisco/Documents/1_Progetti/ailang
cargo test --workspace 2>&1
```

Expected: all crates compile, 64+ tests pass (59 pre-existing + 4 new transpile tests +
5 new builder tests - overlap with pre-existing). Zero compile errors, zero test failures.
</verification>

<success_criteria>
- `cargo test --workspace` exits 0
- `ailang-transpile` tests: 4 passing (empty_graph, const_node, code_node, wired_nodes)
- `ailang-core` tests: includes 5 new builder tests
- `ailang-cli` tests: 2 passing (emit_empty_graph, eval_empty_graph)
- codegen.rs defines CodegenError locally (no `use crate::CodegenError`)
- codegen emits `let node_{i}_{port}` bindings, not comments
- edge wiring emits `let node_{dst}_{dst_port} = node_{src}_{src_port};` before each node
</success_criteria>

<output>
After completion, update /Users/cisco/Documents/1_Progetti/ailang/.planning/STATE.md:
- Mark tasks 0013, 0014, 0015 as done under Phase 2
- Update test count to reflect new total
</output>
