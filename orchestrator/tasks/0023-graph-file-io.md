# TASK 0023: Graph file I/O — load/save .ailang.json via CLI
Phase: 4
Crate: ailang-cli (main.rs only)
Depends on: 0007 (serial), 0015 (cli)

## Goal
Add two CLI subcommands to the `ailang` binary:
- `ailang save <output.json>` — serializes the built-in demo graph to JSON
- `ailang load <input.json>` — deserializes a graph from JSON and prints its node count

This uses the existing `ailang_core::serial::{encode, decode}` functions.

## Implementation

Add to main.rs match arms:
```rust
Some("save") => {
    let path = args.get(2).expect("usage: ailang save <file>");
    let g = demo_graph();
    let json = ailang_core::serial::encode(&g).expect("encode failed");
    std::fs::write(path, &json).expect("write failed");
    println!("saved {} bytes to {path}", json.len());
}
Some("load") => {
    let path = args.get(2).expect("usage: ailang load <file>");
    let json = std::fs::read(path).expect("read failed");
    let g = ailang_core::serial::decode(&json).expect("decode failed");
    println!("loaded graph: {} nodes, {} edges",
        g.nodes().len(), g.edges().len());
}
```

Note: `demo_graph()` is a private helper that builds the same graph used in the existing
`emit` and `eval` subcommands. Extract it from existing code rather than duplicating.

## Acceptance tests — ADD to main.rs test module

```rust
#[test]
fn save_and_load_roundtrip() {
    use std::io::Write;
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
```
