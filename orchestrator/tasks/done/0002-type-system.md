# TASK 0002: Type — the ailang type system enum
Phase: 0
Depends on: 0001 (NodeId)

## Goal
Implement the `Type` enum — the complete set of types a port or edge can carry.
Must cover: primitives (Text, Int, Float, Bool, Bytes), Option<T>, Result<T,E>,
generics (type variables), union types, and a recursive Fold reference type.
Done when all acceptance tests pass and clippy is clean.

## Interface
```
// FILE: crates/ailang-core/src/ty.rs
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Type {
    Text,
    Int,
    Float,
    Bool,
    Bytes,
    Option(Box<Type>),
    Result(Box<Type>, Box<Type>),  // (ok, err)
    Var(u32),                       // type variable, identified by index
    Union(Vec<Type>),               // ordered, deduplicated
    Fold(NodeId),                   // reference to a fold node by content-hash
}
```

## Constraints
- No `unsafe`. No IO.
- `Union` variants must be stored deduplicated and sorted by a stable ordering
  (implement `Ord`/`PartialOrd` or a canonical form helper).
- Capabilities granted: none.

## Acceptance tests
```rust
// FILE: crates/ailang-core/src/ty_tests.rs
#[cfg(test)]
mod tests {
    use crate::ty::Type;
    use crate::node_id::NodeId;

    #[test]
    fn primitives_are_distinct() {
        assert_ne!(Type::Text, Type::Int);
        assert_ne!(Type::Int, Type::Bool);
    }
    #[test]
    fn option_wrapping() {
        let t = Type::Option(Box::new(Type::Text));
        assert_ne!(t, Type::Text);
        assert_eq!(t.clone(), t);
    }
    #[test]
    fn union_deduplication() {
        let a = Type::union(vec![Type::Text, Type::Int, Type::Text]);
        let b = Type::union(vec![Type::Int, Type::Text]);
        assert_eq!(a, b);  // same canonical form
    }
    #[test]
    fn fold_ref_uses_node_id() {
        let id = NodeId::of(b"myfold");
        let t = Type::Fold(id);
        assert_eq!(t, Type::Fold(id));
    }
    #[test]
    fn type_var_distinct_by_index() {
        assert_ne!(Type::Var(0), Type::Var(1));
        assert_eq!(Type::Var(0), Type::Var(0));
    }
}
```

Add a `pub fn union(types: Vec<Type>) -> Type` constructor on `Type` that
deduplicates and returns either the single element or a `Union`.

## Context
Type is the second fundamental primitive. Every port carries a Type; every edge
checks that its two endpoint Types are compatible (that check is Task 0003).
The `Fold(NodeId)` variant is how the type system refers to a folded subgraph
by its content hash, enabling recursive structure.
