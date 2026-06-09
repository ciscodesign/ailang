# TASK 0021: List builtins — push, head, tail, len, empty, int_sum
Phase: 4
Crate: ailang-nodes (builtins.rs only)
Depends on: 0020 (List type/value), 0018 (register_builtins)

## Goal
Add 6 list operations to register_builtins. All use Value::List.

## New nodes

| kind       | inputs               | output      | operation                        |
|------------|----------------------|-------------|----------------------------------|
| list_empty | (none)               | out: List   | Value::List(vec![])              |
| list_push  | list: List, item: T  | out: List   | append item to end               |
| list_head  | list: List           | out: Option | first item or None               |
| list_tail  | list: List           | out: List   | all but first (empty if empty)   |
| list_len   | list: List           | out: Int    | number of items                  |
| list_int_sum | list: List         | out: Int    | sum all Int items                |

Note: list_push, list_head, list_tail, list_len accept any List (use Value::List(_) pattern match).
list_int_sum expects List of Int values — error on any non-Int item.

## Implementation

```rust
registry.register("list_empty", Box::new(|_inputs: Inputs| -> Result<Outputs, ExecError> {
    Ok(HashMap::from([("out".into(), Value::List(vec![]))]))
}));

registry.register("list_push", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
    let list = inputs.remove("list").ok_or_else(|| ExecError::MissingInput("list".into()))?;
    let item = inputs.remove("item").ok_or_else(|| ExecError::MissingInput("item".into()))?;
    match list {
        Value::List(mut v) => { v.push(item); Ok(HashMap::from([("out".into(), Value::List(v))])) }
        _ => Err(ExecError::Failed("list_push: expected List".into())),
    }
}));

registry.register("list_head", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
    let list = inputs.remove("list").ok_or_else(|| ExecError::MissingInput("list".into()))?;
    match list {
        Value::List(v) => {
            let head = v.into_iter().next().map(|x| Box::new(x));
            Ok(HashMap::from([("out".into(), Value::Option(head))]))
        }
        _ => Err(ExecError::Failed("list_head: expected List".into())),
    }
}));

registry.register("list_tail", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
    let list = inputs.remove("list").ok_or_else(|| ExecError::MissingInput("list".into()))?;
    match list {
        Value::List(mut v) => {
            if !v.is_empty() { v.remove(0); }
            Ok(HashMap::from([("out".into(), Value::List(v))]))
        }
        _ => Err(ExecError::Failed("list_tail: expected List".into())),
    }
}));

registry.register("list_len", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
    let list = inputs.remove("list").ok_or_else(|| ExecError::MissingInput("list".into()))?;
    match list {
        Value::List(v) => Ok(HashMap::from([("out".into(), Value::Int(v.len() as i64))])),
        _ => Err(ExecError::Failed("list_len: expected List".into())),
    }
}));

registry.register("list_int_sum", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
    let list = inputs.remove("list").ok_or_else(|| ExecError::MissingInput("list".into()))?;
    match list {
        Value::List(v) => {
            let mut sum = 0i64;
            for item in v {
                match item {
                    Value::Int(n) => sum += n,
                    _ => return Err(ExecError::Failed("list_int_sum: non-Int item".into())),
                }
            }
            Ok(HashMap::from([("out".into(), Value::Int(sum))]))
        }
        _ => Err(ExecError::Failed("list_int_sum: expected List".into())),
    }
}));
```

## Acceptance tests — ADD to builtins_tests.rs

```rust
#[test]
fn list_empty_returns_empty() {
    let r = reg();
    let out = r.call("list_empty", HashMap::new()).unwrap();
    assert_eq!(out["out"], Value::List(vec![]));
}

#[test]
fn list_push_appends() {
    let r = reg();
    let inputs = HashMap::from([
        ("list".into(), Value::List(vec![Value::Int(1)])),
        ("item".into(), Value::Int(2)),
    ]);
    let out = r.call("list_push", inputs).unwrap();
    assert_eq!(out["out"], Value::List(vec![Value::Int(1), Value::Int(2)]));
}

#[test]
fn list_head_some() {
    let r = reg();
    let inputs = HashMap::from([("list".into(), Value::List(vec![Value::Int(42), Value::Int(7)]))]);
    let out = r.call("list_head", inputs).unwrap();
    assert_eq!(out["out"], Value::Option(Some(Box::new(Value::Int(42)))));
}

#[test]
fn list_head_empty() {
    let r = reg();
    let inputs = HashMap::from([("list".into(), Value::List(vec![]))]);
    let out = r.call("list_head", inputs).unwrap();
    assert_eq!(out["out"], Value::Option(None));
}

#[test]
fn list_tail_removes_first() {
    let r = reg();
    let inputs = HashMap::from([("list".into(), Value::List(vec![Value::Int(1), Value::Int(2), Value::Int(3)]))]);
    let out = r.call("list_tail", inputs).unwrap();
    assert_eq!(out["out"], Value::List(vec![Value::Int(2), Value::Int(3)]));
}

#[test]
fn list_len() {
    let r = reg();
    let inputs = HashMap::from([("list".into(), Value::List(vec![Value::Int(1), Value::Int(2)]))]);
    let out = r.call("list_len", inputs).unwrap();
    assert_eq!(out["out"], Value::Int(2));
}

#[test]
fn list_int_sum() {
    let r = reg();
    let inputs = HashMap::from([("list".into(), Value::List(vec![Value::Int(10), Value::Int(20), Value::Int(5)]))]);
    let out = r.call("list_int_sum", inputs).unwrap();
    assert_eq!(out["out"], Value::Int(35));
}
```
