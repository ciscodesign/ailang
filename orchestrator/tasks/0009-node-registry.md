# TASK 0009: NodeRegistry — map kind strings to execution functions
Phase: 1
Crate: ailang-exec
Depends on: 0008 (Value)

## Goal
Implement a `NodeRegistry` that maps a node's `kind: String` to a boxed function
that takes named input `Value`s and returns named output `Value`s. Also implement
the built-in `Const` node kind, which takes no inputs and returns a single output
whose value was fixed at registration time.

## Existing API (DO NOT CHANGE OR RE-IMPLEMENT)
```rust
// ailang-exec::value
pub enum Value { Text(String), Int(i64), Float(f64), Bool(bool), Bytes(Vec<u8>),
    Option(Option<Box<Value>>), Result(Result<Box<Value>, Box<Value>>) }

// ailang-core::graph — NodeDef.kind is a String
pub struct NodeDef { pub id: NodeId, pub kind: String,
    pub inputs: Vec<PortDef>, pub outputs: Vec<PortDef>,
    pub effects: EffectSet }
```

## Interface
```rust
// FILE: crates/ailang-exec/src/registry.rs
use crate::value::Value;
use std::collections::HashMap;

pub type Inputs  = HashMap<String, Value>;
pub type Outputs = HashMap<String, Value>;

#[derive(Debug, thiserror::Error)]
pub enum ExecError {
    #[error("unknown node kind: {0}")]
    UnknownKind(String),
    #[error("missing input: {0}")]
    MissingInput(String),
    #[error("execution failed: {0}")]
    Failed(String),
}

pub type ExecFn = Box<dyn Fn(Inputs) -> Result<Outputs, ExecError> + Send + Sync>;

pub struct NodeRegistry {
    fns: HashMap<String, ExecFn>,
}

impl NodeRegistry {
    pub fn new() -> Self;
    /// Register a node kind with its execution function.
    pub fn register(&mut self, kind: impl Into<String>, f: ExecFn);
    /// Look up and call a node's function.
    pub fn call(&self, kind: &str, inputs: Inputs) -> Result<Outputs, ExecError>;
    /// Register the built-in Const node: kind "Const:<port_name>",
    /// takes no inputs, returns { port_name → value }.
    pub fn register_const(&mut self, port_name: impl Into<String>, value: Value);
}
```

## Constraints
- No `unsafe`. No IO.
- `register_const` should register under key `"Const:{port_name}"` and return
  a single output keyed by `port_name`.
- `call` returns `ExecError::UnknownKind` if the kind is not registered.

## Acceptance tests
```rust
// FILE: crates/ailang-exec/src/registry_tests.rs
#[cfg(test)]
mod tests {
    use crate::{value::Value, registry::{NodeRegistry, ExecError, Inputs}};
    use std::collections::HashMap;

    #[test]
    fn unknown_kind_errors() {
        let r = NodeRegistry::new();
        assert!(matches!(r.call("NoSuch", HashMap::new()), Err(ExecError::UnknownKind(_))));
    }
    #[test]
    fn const_node_returns_value() {
        let mut r = NodeRegistry::new();
        r.register_const("out", Value::Int(42));
        let out = r.call("Const:out", HashMap::new()).unwrap();
        assert_eq!(out["out"], Value::Int(42));
    }
    #[test]
    fn custom_node_registered_and_called() {
        let mut r = NodeRegistry::new();
        r.register("double", Box::new(|inputs: Inputs| {
            let n = match inputs.get("x").cloned() {
                Some(Value::Int(n)) => n,
                _ => return Err(crate::registry::ExecError::MissingInput("x".into())),
            };
            let mut out = std::collections::HashMap::new();
            out.insert("y".to_string(), Value::Int(n * 2));
            Ok(out)
        }));
        let mut inp = HashMap::new();
        inp.insert("x".to_string(), Value::Int(7));
        let out = r.call("double", inp).unwrap();
        assert_eq!(out["y"], Value::Int(14));
    }
}
```

## lib.rs update
```rust
// FILE: crates/ailang-exec/src/lib.rs
pub mod value;
pub mod registry;
#[cfg(test)]
mod value_tests;
#[cfg(test)]
mod registry_tests;
```
