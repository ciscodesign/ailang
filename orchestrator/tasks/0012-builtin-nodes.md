# TASK 0012: Builtin Nodes — standard arithmetic/logic/text operations
Phase: 2
Crate: ailang-nodes
Depends on: 0010 (ailang-exec with NodeRegistry)

## Goal
Implement `register_builtins(registry: &mut NodeRegistry)` in the `ailang-nodes` crate.
Registers 8 standard built-in node kinds that cover integer arithmetic, boolean logic,
and text operations. Each node reads named inputs from `Inputs` and writes named outputs
to `Outputs` (both are `HashMap<String, Value>`).

## Existing API (DO NOT CHANGE OR RE-IMPLEMENT)
```rust
// ailang-exec::registry
pub type Inputs  = HashMap<String, Value>;
pub type Outputs = HashMap<String, Value>;

#[derive(Debug, thiserror::Error)]
pub enum ExecError {
    #[error("unknown node kind: {0}")]       UnknownKind(String),
    #[error("missing input: {0}")]           MissingInput(String),
    #[error("execution failed: {0}")]        Failed(String),
}

pub type ExecFn = Box<dyn Fn(Inputs) -> Result<Outputs, ExecError> + Send + Sync>;

pub struct NodeRegistry { /* private */ }
impl NodeRegistry {
    pub fn new() -> Self;
    pub fn register(&mut self, kind: impl Into<String>, f: ExecFn);
    pub fn call(&self, kind: &str, inputs: Inputs) -> Result<Outputs, ExecError>;
    pub fn register_const(&mut self, port_name: impl Into<String>, value: Value);
}

// ailang-exec::value
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Text(String), Int(i64), Float(f64), Bool(bool), Bytes(Vec<u8>),
    Option(Option<Box<Value>>),
    Result(std::result::Result<Box<Value>, Box<Value>>),
}
```

## Interface
```rust
// FILE: crates/ailang-nodes/src/lib.rs
pub mod builtins;
#[cfg(test)]
mod builtins_tests;

// FILE: crates/ailang-nodes/src/builtins.rs
use ailang_exec::registry::{ExecError, Inputs, Outputs, NodeRegistry};
use ailang_exec::value::Value;

/// Register all standard built-in nodes into `registry`.
pub fn register_builtins(registry: &mut NodeRegistry);
```

## Nodes to register

| kind          | input ports       | output port | operation              |
|---------------|-------------------|-------------|------------------------|
| `add_int`     | `a: Int, b: Int`  | `out: Int`  | a + b                  |
| `sub_int`     | `a: Int, b: Int`  | `out: Int`  | a - b                  |
| `mul_int`     | `a: Int, b: Int`  | `out: Int`  | a * b                  |
| `neg_int`     | `a: Int`          | `out: Int`  | -a                     |
| `concat_text` | `a: Text, b: Text`| `out: Text` | a + b (concatenate)    |
| `not_bool`    | `a: Bool`         | `out: Bool` | !a                     |
| `and_bool`    | `a: Bool, b: Bool`| `out: Bool` | a && b                 |
| `or_bool`     | `a: Bool, b: Bool`| `out: Bool` | a \|\| b               |

Each node function:
1. Extracts its named inputs with `inputs.remove("a")` etc. Return `ExecError::MissingInput("a".into())` if missing.
2. Pattern-matches the `Value` variant. Return `ExecError::Failed(...)` on wrong variant.
3. Computes the result and returns `Ok(HashMap::from([("out".into(), Value::Int(result))]))`.

## Cargo.toml
```toml
// FILE: crates/ailang-nodes/Cargo.toml
[package]
name = "ailang-nodes"
version.workspace = true
edition.workspace = true

[dependencies]
ailang-exec = { path = "../ailang-exec" }
```

## Acceptance tests
```rust
// FILE: crates/ailang-nodes/src/builtins_tests.rs
#[cfg(test)]
mod tests {
    use ailang_exec::registry::NodeRegistry;
    use ailang_exec::value::Value;
    use std::collections::HashMap;
    use crate::builtins::register_builtins;

    fn reg() -> NodeRegistry {
        let mut r = NodeRegistry::new();
        register_builtins(&mut r);
        r
    }

    #[test]
    fn add_int() {
        let r = reg();
        let inputs = HashMap::from([("a".into(), Value::Int(3)), ("b".into(), Value::Int(4))]);
        let out = r.call("add_int", inputs).unwrap();
        assert_eq!(out["out"], Value::Int(7));
    }

    #[test]
    fn sub_int() {
        let r = reg();
        let inputs = HashMap::from([("a".into(), Value::Int(10)), ("b".into(), Value::Int(3))]);
        let out = r.call("sub_int", inputs).unwrap();
        assert_eq!(out["out"], Value::Int(7));
    }

    #[test]
    fn mul_int() {
        let r = reg();
        let inputs = HashMap::from([("a".into(), Value::Int(6)), ("b".into(), Value::Int(7))]);
        let out = r.call("mul_int", inputs).unwrap();
        assert_eq!(out["out"], Value::Int(42));
    }

    #[test]
    fn neg_int() {
        let r = reg();
        let inputs = HashMap::from([("a".into(), Value::Int(5))]);
        let out = r.call("neg_int", inputs).unwrap();
        assert_eq!(out["out"], Value::Int(-5));
    }

    #[test]
    fn concat_text() {
        let r = reg();
        let inputs = HashMap::from([
            ("a".into(), Value::Text("hello".into())),
            ("b".into(), Value::Text(" world".into())),
        ]);
        let out = r.call("concat_text", inputs).unwrap();
        assert_eq!(out["out"], Value::Text("hello world".into()));
    }

    #[test]
    fn not_bool() {
        let r = reg();
        let inputs = HashMap::from([("a".into(), Value::Bool(true))]);
        let out = r.call("not_bool", inputs).unwrap();
        assert_eq!(out["out"], Value::Bool(false));
    }

    #[test]
    fn and_bool() {
        let r = reg();
        let inputs = HashMap::from([("a".into(), Value::Bool(true)), ("b".into(), Value::Bool(false))]);
        let out = r.call("and_bool", inputs).unwrap();
        assert_eq!(out["out"], Value::Bool(false));
    }

    #[test]
    fn or_bool() {
        let r = reg();
        let inputs = HashMap::from([("a".into(), Value::Bool(false)), ("b".into(), Value::Bool(true))]);
        let out = r.call("or_bool", inputs).unwrap();
        assert_eq!(out["out"], Value::Bool(true));
    }

    #[test]
    fn missing_input_returns_error() {
        let r = reg();
        let inputs = HashMap::from([("a".into(), Value::Int(1))]);
        let err = r.call("add_int", inputs).unwrap_err();
        assert!(err.to_string().contains("missing input") || err.to_string().contains("b"));
    }
}
```
