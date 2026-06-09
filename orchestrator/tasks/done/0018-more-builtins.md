# TASK 0018: More builtins — comparisons and conditionals
Phase: 3
Crate: ailang-nodes (builtins.rs only)
Depends on: 0012 (builtin-nodes)

## Goal
Extend `register_builtins` with 4 more node kinds: integer comparisons and a
conditional. All follow the same pattern as existing builtins — no new deps.

## New nodes

| kind      | inputs                                  | output port | operation              |
|-----------|-----------------------------------------|-------------|------------------------|
| `eq_int`  | `a: Int, b: Int`                        | `out: Bool` | a == b                 |
| `lt_int`  | `a: Int, b: Int`                        | `out: Bool` | a < b                  |
| `if_int`  | `cond: Bool, then: Int, else_: Int`     | `out: Int`  | if cond { then } else { else_ } |
| `len_text`| `a: Text`                               | `out: Int`  | a.len() as i64 (byte length) |

Note: `else` is a Rust keyword — use `else_` as the input port name.

## Implementation pattern (copy exactly for each node)

```rust
registry.register("eq_int", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
    let a = inputs.remove("a").ok_or_else(|| ExecError::MissingInput("a".into()))?;
    let b = inputs.remove("b").ok_or_else(|| ExecError::MissingInput("b".into()))?;
    match (a, b) {
        (Value::Int(x), Value::Int(y)) => Ok(HashMap::from([("out".into(), Value::Bool(x == y))])),
        _ => Err(ExecError::Failed("eq_int: expected Int inputs".into())),
    }
}));

registry.register("if_int", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
    let cond  = inputs.remove("cond") .ok_or_else(|| ExecError::MissingInput("cond".into()))?;
    let then  = inputs.remove("then") .ok_or_else(|| ExecError::MissingInput("then".into()))?;
    let else_ = inputs.remove("else_").ok_or_else(|| ExecError::MissingInput("else_".into()))?;
    match (cond, then, else_) {
        (Value::Bool(c), Value::Int(t), Value::Int(e)) =>
            Ok(HashMap::from([("out".into(), Value::Int(if c { t } else { e }))])),
        _ => Err(ExecError::Failed("if_int: type mismatch".into())),
    }
}));
```

## Cargo.toml — NO CHANGES

## lib.rs — NO CHANGES (builtins module already declared)

## Acceptance tests — ADD to existing builtins_tests.rs (keep all existing tests)

```rust
// FILE: crates/ailang-nodes/src/builtins_tests.rs
// Add inside `mod tests { ... }`:

#[test]
fn eq_int_true() {
    let r = reg();
    let inputs = HashMap::from([("a".into(), Value::Int(5)), ("b".into(), Value::Int(5))]);
    let out = r.call("eq_int", inputs).unwrap();
    assert_eq!(out["out"], Value::Bool(true));
}

#[test]
fn eq_int_false() {
    let r = reg();
    let inputs = HashMap::from([("a".into(), Value::Int(3)), ("b".into(), Value::Int(4))]);
    let out = r.call("eq_int", inputs).unwrap();
    assert_eq!(out["out"], Value::Bool(false));
}

#[test]
fn lt_int() {
    let r = reg();
    let inputs = HashMap::from([("a".into(), Value::Int(2)), ("b".into(), Value::Int(9))]);
    let out = r.call("lt_int", inputs).unwrap();
    assert_eq!(out["out"], Value::Bool(true));
}

#[test]
fn if_int_true_branch() {
    let r = reg();
    let inputs = HashMap::from([
        ("cond".into(),  Value::Bool(true)),
        ("then".into(),  Value::Int(10)),
        ("else_".into(), Value::Int(20)),
    ]);
    let out = r.call("if_int", inputs).unwrap();
    assert_eq!(out["out"], Value::Int(10));
}

#[test]
fn if_int_false_branch() {
    let r = reg();
    let inputs = HashMap::from([
        ("cond".into(),  Value::Bool(false)),
        ("then".into(),  Value::Int(10)),
        ("else_".into(), Value::Int(20)),
    ]);
    let out = r.call("if_int", inputs).unwrap();
    assert_eq!(out["out"], Value::Int(20));
}

#[test]
fn len_text() {
    let r = reg();
    let inputs = HashMap::from([("a".into(), Value::Text("hello".into()))]);
    let out = r.call("len_text", inputs).unwrap();
    assert_eq!(out["out"], Value::Int(5));
}
```
