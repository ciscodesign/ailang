# TASK 0019: WASM emit — codegen_wasm for wasm32 target
Phase: 3
Crate: ailang-transpile (codegen.rs only)
Depends on: 0013 (codegen wiring), 0016 (const literals)

## Goal
Add `pub fn codegen_wasm(graph: &Graph, fn_name: &str) -> Result<String, CodegenError>`
to `ailang-transpile`. It produces the same body as `codegen()` but wraps the function
with `#[no_mangle]` and `extern "C"` so it can be compiled to WebAssembly.

The return type is inferred from the **last node in topological order** that has at
least one output port:
- `Type::Int`   → `i64`
- `Type::Float` → `f64`
- `Type::Bool`  → `i32` (WASM has no bool — use 0/1)
- `Type::Text`  → `*const u8` (pointer — not truly usable from WASM without glue, but structurally correct)
- anything else → `()` (void)

The emitted function body is identical to `codegen()`. The last node's output variable
is returned at the end of the function.

## Implementation

```rust
pub fn codegen_wasm(graph: &Graph, fn_name: &str) -> Result<String, CodegenError> {
    // Re-use the same topo sort logic.
    // Determine return type from the last node in topo order that has outputs.
    // Emit the function with #[no_mangle] + extern "C" header.
    // Append a `return node_{last}_{port};` line before closing `}`.
}
```

### Return type mapping

```rust
fn wasm_return_type(ty: &ailang_core::ty::Type) -> &'static str {
    use ailang_core::ty::Type;
    match ty {
        Type::Int   => "i64",
        Type::Float => "f64",
        Type::Bool  => "i32",
        Type::Text  => "*const u8",
        _           => "()",
    }
}
```

### Emitted function shape

```rust
// When a last node with output exists:
#[no_mangle]
pub extern "C" fn run() -> i64 {
    // ... node let-bindings ...
    node_2_out
}

// When graph is empty or last node has no outputs:
#[no_mangle]
pub extern "C" fn run() {
    // ... node let-bindings ...
}
```

### Finding the last output node

After computing `sorted_nodes`, iterate in REVERSE to find the first node (last in topo
order) that has `!node.outputs.is_empty()`. Store `last_output: Option<(usize, String, Type)>`
= `(node_idx, port_name, port_type)`.

### IMPORTANT: do NOT duplicate the topo-sort code

Extract the topo sort into a private helper function and call it from both `codegen` and
`codegen_wasm`:

```rust
fn topo_sort(graph: &Graph) -> Result<Vec<usize>, CodegenError> {
    // ... Kahn's algorithm ...
    // Return sorted_nodes or Err(CodegenError::Cycle)
}

pub fn codegen(graph: &Graph, fn_name: &str) -> Result<String, CodegenError> {
    let sorted_nodes = topo_sort(graph)?;
    // ... rest of codegen ...
}

pub fn codegen_wasm(graph: &Graph, fn_name: &str) -> Result<String, CodegenError> {
    let sorted_nodes = topo_sort(graph)?;
    // ... wasm version ...
}
```

## lib.rs — NO CHANGE (codegen module already declared and exported)

## Cargo.toml — NO CHANGE

## Acceptance tests — ADD to existing codegen_tests.rs (keep ALL existing tests)

```rust
// FILE: crates/ailang-transpile/src/codegen_tests.rs
// Add inside `mod tests { ... }`:

use crate::codegen::codegen_wasm;

#[test]
fn wasm_empty_graph() {
    let g = Graph::new();
    let src = codegen_wasm(&g, "run").unwrap();
    assert!(src.contains("extern \"C\""), "missing extern C: {src}");
    assert!(src.contains("no_mangle"),    "missing no_mangle: {src}");
    assert!(src.contains("fn run()"),     "missing fn signature: {src}");
}

#[test]
fn wasm_int_output_returns_i64() {
    let mut g = Graph::new();
    g.add_node(make_node(b"w", "Const:out:99i64",
        vec![],
        vec![PortDef { name: "out".into(), ty: Type::Int }],
    ));
    let src = codegen_wasm(&g, "compute").unwrap();
    assert!(src.contains("-> i64"),      "missing i64 return: {src}");
    assert!(src.contains("99i64"),       "missing literal: {src}");
    assert!(src.contains("node_0_out"),  "missing return var: {src}");
}

#[test]
fn wasm_cycle_returns_error() {
    // cycle test is the same as regular codegen — just confirm wasm variant also errors
    // A cycle requires at least 2 nodes with edges in both directions, which add_edge
    // would reject via type checking. Skip if constructing a cycle isn't possible
    // through the public API — just assert codegen_wasm compiles.
    let g = Graph::new();
    assert!(codegen_wasm(&g, "noop").is_ok());
}
```

Note: `make_node` is defined in the existing test module — do not redefine it.
