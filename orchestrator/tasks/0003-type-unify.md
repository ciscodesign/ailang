# TASK 0003: Type::unify — static edge-legality check
Phase: 0
Depends on: 0002 (Type)

## Goal
Implement `Type::unify(a: &Type, b: &Type) -> Result<Type, UnifyError>` — the
check that determines whether an output port of type `a` can legally connect to
an input port of type `b`. Returns the unified type on success, `UnifyError` on
failure. Done when all acceptance tests pass and clippy is clean.

## Interface
```
// FILE: crates/ailang-core/src/unify.rs
#[derive(Debug, thiserror::Error)]
pub enum UnifyError {
    #[error("cannot unify {0:?} with {1:?}")]
    Mismatch(Type, Type),
}

impl Type {
    pub fn unify(a: &Type, b: &Type) -> Result<Type, UnifyError>;
}
```

## Unification rules (implement all)
- Identical types unify to themselves.
- `Var(n)` unifies with any type T → returns T (one-way: Var on the left, concrete on the right).
- `Option<T>` unifies with `Option<U>` if T unifies with U → `Option<unified>`.
- `Result<T,E>` unifies with `Result<U,F>` if T~U and E~F → `Result<unified_ok, unified_err>`.
- `Union(vs)` unifies with T if T unifies with any variant in vs → returns T.
- Everything else: `UnifyError::Mismatch`.

## Constraints
- No `unsafe`. No IO. Pure function.
- Capabilities granted: none.

## Acceptance tests
```rust
// FILE: crates/ailang-core/src/unify_tests.rs
#[cfg(test)]
mod tests {
    use crate::ty::Type;

    #[test]
    fn identical_unifies() {
        assert!(Type::unify(&Type::Text, &Type::Text).is_ok());
    }
    #[test]
    fn mismatch_fails() {
        assert!(Type::unify(&Type::Text, &Type::Int).is_err());
    }
    #[test]
    fn var_unifies_with_concrete() {
        let r = Type::unify(&Type::Var(0), &Type::Text).unwrap();
        assert_eq!(r, Type::Text);
    }
    #[test]
    fn option_unifies_same_inner() {
        let a = Type::Option(Box::new(Type::Text));
        let b = Type::Option(Box::new(Type::Text));
        assert!(Type::unify(&a, &b).is_ok());
    }
    #[test]
    fn option_fails_different_inner() {
        let a = Type::Option(Box::new(Type::Text));
        let b = Type::Option(Box::new(Type::Int));
        assert!(Type::unify(&a, &b).is_err());
    }
    #[test]
    fn union_accepts_member() {
        let u = Type::union(vec![Type::Text, Type::Int]);
        assert!(Type::unify(&u, &Type::Text).is_ok());
        assert!(Type::unify(&u, &Type::Bool).is_err());
    }
    #[test]
    fn result_unifies() {
        let a = Type::Result(Box::new(Type::Text), Box::new(Type::Int));
        let b = Type::Result(Box::new(Type::Text), Box::new(Type::Int));
        assert!(Type::unify(&a, &b).is_ok());
    }
}
```

## Context
This is the compile-time edge-legality gate. In the graph, when the controller
adds an edge from output port (type A) to input port (type B), `Type::unify(A, B)`
is called. Failure = compile error at the graph level, before anything runs.
Add `thiserror.workspace = true` to Cargo.toml if not already present.
