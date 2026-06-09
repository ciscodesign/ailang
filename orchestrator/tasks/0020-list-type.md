# TASK 0020: List type — add Type::List and Value::List
Phase: 4
Crates: ailang-core (ty.rs, unify.rs), ailang-exec (value.rs)
Depends on: 0002 (Type), 0003 (unify), 0008 (Value)

## Goal
Add `Type::List(Box<Type>)` to the Type enum and `Value::List(Vec<Value>)` to Value.
Update unify and matches_type accordingly.

## ty.rs changes
Add variant after Fold:
```rust
List(Box<Type>),
```

## unify.rs changes
Add arm in unify():
```rust
(Type::List(a), Type::List(b)) => {
    unify(a, b)?;
    Ok(())
}
```

## value.rs changes
Add variant:
```rust
List(Vec<Value>),
```
Add arm in matches_type():
```rust
(Value::List(_), Type::List(_)) => true,
```

## Acceptance tests

### ty_tests.rs — ADD inside mod tests:
```rust
#[test]
fn list_type_distinct() {
    assert_ne!(Type::List(Box::new(Type::Int)), Type::List(Box::new(Type::Text)));
}
```

### unify_tests.rs — ADD inside mod tests:
```rust
#[test]
fn list_same_inner_unifies() {
    assert!(unify(&Type::List(Box::new(Type::Int)), &Type::List(Box::new(Type::Int))).is_ok());
}
#[test]
fn list_different_inner_fails() {
    assert!(unify(&Type::List(Box::new(Type::Int)), &Type::List(Box::new(Type::Text))).is_err());
}
```

### value_tests.rs — ADD inside mod tests:
```rust
#[test]
fn list_matches_list_type() {
    assert!(Value::List(vec![Value::Int(1)]).matches_type(&Type::List(Box::new(Type::Int))));
}
#[test]
fn list_does_not_match_int() {
    assert!(!Value::List(vec![]).matches_type(&Type::Int));
}
```
