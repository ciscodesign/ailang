# TASK 0016: Const literals — embed values in Const node kind string
Phase: 3
Crate: ailang-transpile (codegen.rs only) + ailang-nodes (builtins.rs only)
Depends on: 0013 (codegen wiring)

## Goal
Currently `Const:out` nodes emit `todo!("Const")` in codegen and require manual
`register_const` calls in eval. This task adds a convention: if the node kind is
`"Const:<port>:<literal>"` (three colon-separated parts), the third segment is
treated as a Rust literal and emitted directly. Eval also learns to parse these.

## Convention
```
"Const:out"          → old style, no embedded value → todo!("Const") in codegen, needs register_const for eval
"Const:out:42i64"    → new style, Int literal
"Const:out:3.14f64"  → Float literal
"Const:out:true"     → Bool literal
"Const:out:hello"    → Text literal (emitted as &str — caller wraps as needed)
```
The literal segment is everything after the second colon (may itself contain colons, though unlikely).

## Changes

### ailang-transpile/src/codegen.rs

In the `strip_prefix("Const:")` branch, after stripping the prefix, check if the
remaining string contains `':'`:
- If yes: split at the FIRST `':'` → `port` and `literal`; emit `let node_{i}_{port}: {ty} = {literal};`
- If no: existing behaviour — emit `let node_{i}_{port}: {ty} = todo!("Const");`

```rust
if let Some(rest) = kind.strip_prefix("Const:") {
    let (port, literal_opt) = match rest.find(':') {
        Some(pos) => (&rest[..pos], Some(&rest[pos + 1..])),
        None      => (rest, None),
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
```

### ailang-nodes/src/builtins.rs

Add a helper function (NOT exported, `pub(crate)` is fine or just `fn`):

```rust
/// Register a const node that reads its literal value from the kind string.
/// Kind format: "Const:<port>:<value>" where value is parseable from a string.
/// This supplements NodeRegistry::register_const for the literal-kind pattern.
pub fn register_const_literal(registry: &mut NodeRegistry, kind: &str) {
    // kind = "Const:<port>:<literal_str>"
    let rest = kind.strip_prefix("Const:").unwrap_or(kind);
    let (port, literal) = match rest.find(':') {
        Some(pos) => (&rest[..pos], &rest[pos + 1..]),
        None => return, // no literal — caller must use register_const
    };
    let port = port.to_string();
    let value: Value = if let Ok(n) = literal.parse::<i64>() {
        Value::Int(n)
    } else if let Ok(f) = literal.parse::<f64>() {
        Value::Float(f)
    } else if literal == "true" {
        Value::Bool(true)
    } else if literal == "false" {
        Value::Bool(false)
    } else {
        Value::Text(literal.to_string())
    };
    registry.register_const(port, value);
}
```

**Note:** `register_const_literal` is called on a kind like `"Const:out:42"` and registers the node under that full kind string. But `NodeRegistry::register_const` registers under `"Const:{port_name}"`. These are DIFFERENT keys. We need to register under the FULL kind string:

```rust
pub fn register_const_literal(registry: &mut NodeRegistry, kind: &str) {
    let rest = kind.strip_prefix("Const:").unwrap_or(kind);
    let (port, literal) = match rest.find(':') {
        Some(pos) => (&rest[..pos], &rest[pos + 1..]),
        None => return,
    };
    let port_owned = port.to_string();
    let kind_owned = kind.to_string();
    let value: Value = if let Ok(n) = literal.parse::<i64>() {
        Value::Int(n)
    } else if let Ok(f) = literal.parse::<f64>() {
        Value::Float(f)
    } else if literal == "true" {
        Value::Bool(true)
    } else if literal == "false" {
        Value::Bool(false)
    } else {
        Value::Text(literal.to_string())
    };
    registry.register(kind_owned, Box::new(move |_inputs: Inputs| {
        let mut out = Outputs::new();
        out.insert(port_owned.clone(), value.clone());
        Ok(out)
    }));
}
```

Make `register_const_literal` public: `pub fn register_const_literal(...)`.

## Cargo.toml — NO CHANGES to either crate

## lib.rs changes

### ailang-transpile/src/lib.rs — NO CHANGE needed (codegen module already declared)

### ailang-nodes/src/lib.rs — NO CHANGE needed (builtins already declared)

## Acceptance tests

### ailang-transpile/src/codegen_tests.rs — ADD one test (keep all existing tests)

```rust
#[test]
fn const_with_literal_emits_value() {
    let mut g = Graph::new();
    g.add_node(NodeDef {
        id: NodeId::of(b"cv"),
        kind: "Const:out:42i64".into(),
        inputs: vec![],
        outputs: vec![PortDef { name: "out".into(), ty: Type::Int }],
        effects: EffectSet::empty(),
    });
    let src = codegen(&g, "run").unwrap();
    assert!(src.contains("42i64"), "literal not emitted: {src}");
    assert!(!src.contains("todo!"), "todo! should not appear: {src}");
}
```

### ailang-nodes/src/builtins_tests.rs — ADD two tests (keep all existing tests)

```rust
#[test]
fn register_const_literal_int() {
    let mut r = NodeRegistry::new();
    register_const_literal(&mut r, "Const:out:7");
    let out = r.call("Const:out:7", HashMap::new()).unwrap();
    assert_eq!(out["out"], Value::Int(7));
}

#[test]
fn register_const_literal_bool() {
    let mut r = NodeRegistry::new();
    register_const_literal(&mut r, "Const:flag:true");
    let out = r.call("Const:flag:true", HashMap::new()).unwrap();
    assert_eq!(out["flag"], Value::Bool(true));
}
```

Remember to import `register_const_literal` in the test module.
