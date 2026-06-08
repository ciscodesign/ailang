# TASK 0008: Value — runtime representation of data
Phase: 1
Crate: ailang-exec (new)
Depends on: Phase 0 complete

## Goal
Add a `Value` enum to the new `ailang-exec` crate. `Value` is the runtime
counterpart of `Type`: where `Type` describes the shape of data, `Value` carries
the actual data flowing through graph edges at execution time.

## Existing API (DO NOT CHANGE)
```rust
// ailang-core::ty
pub enum Type { Text, Int, Float, Bool, Bytes,
    Option(Box<Type>), Result(Box<Type>, Box<Type>),
    Var(u32), Union(Vec<Type>), Fold(NodeId) }

// ailang-core::node_id
pub struct NodeId([u8; 32]);  // derives Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize
```

## Interface
```rust
// FILE: crates/ailang-exec/src/value.rs
use ailang_core::ty::Type;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Text(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Bytes(Vec<u8>),
    Option(Option<Box<Value>>),
    Result(std::result::Result<Box<Value>, Box<Value>>),
}

impl Value {
    /// Returns true if this value is compatible with the given Type.
    /// Does not check recursively — top-level variant match only.
    pub fn matches_type(&self, ty: &Type) -> bool;
}
```

## Constraints
- No `unsafe`. No IO. Pure data type.
- `Float` uses `f64` but does NOT derive `Eq` or `Hash` (f64 is not Eq).
  The `PartialEq` impl for `Float` uses `==` (normal float equality).
- Do NOT implement `Value` for `Var`, `Union`, `Fold` — those are type-level only.
- `matches_type` maps: Text↔Text, Int↔Int, Float↔Float, Bool↔Bool, Bytes↔Bytes,
  Option(_)↔Option(_), Result(_,_)↔Result(_,_), anything↔Var(_).

## Acceptance tests
```rust
// FILE: crates/ailang-exec/src/value_tests.rs
#[cfg(test)]
mod tests {
    use ailang_core::ty::Type;
    use crate::value::Value;

    #[test]
    fn text_matches_text() {
        assert!(Value::Text("hi".into()).matches_type(&Type::Text));
    }
    #[test]
    fn int_does_not_match_text() {
        assert!(!Value::Int(42).matches_type(&Type::Text));
    }
    #[test]
    fn any_value_matches_var() {
        assert!(Value::Bool(true).matches_type(&Type::Var(0)));
    }
    #[test]
    fn option_matches_option() {
        assert!(Value::Option(None).matches_type(&Type::Option(Box::new(Type::Text))));
    }
    #[test]
    fn result_ok_matches_result() {
        let v = Value::Result(Ok(Box::new(Value::Int(1))));
        assert!(v.matches_type(&Type::Result(Box::new(Type::Int), Box::new(Type::Text))));
    }
}
```

## Cargo.toml
```toml
// FILE: crates/ailang-exec/Cargo.toml
[package]
name = "ailang-exec"
version.workspace = true
edition.workspace = true

[dependencies]
ailang-core = { path = "../ailang-core" }
thiserror.workspace = true
```

## lib.rs
```rust
// FILE: crates/ailang-exec/src/lib.rs
pub mod value;
#[cfg(test)]
mod value_tests;
```
