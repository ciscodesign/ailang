# TASK SPEC TEMPLATE + WORKED EXAMPLE

Every task the Controller emits follows this shape. The harness saves one per file in `orchestrator/tasks/`.

## Template

```markdown
# TASK <id>: <short title>
Phase: <0–5>
Depends on: <task ids or "none">

## Goal
<One paragraph. What this task produces and its done-condition.>

## Interface
<Exact signatures / types / file paths the executor must produce. No ambiguity.>

## Constraints
- <e.g. no unsafe, no external crates beyond X, must be deterministic>
- <capabilities granted: usually "none">

## Acceptance tests
<Full Rust test code, written by the Controller. Green tests = done.>

## Context
<The minimal surrounding facts the executor needs: related types already built, naming conventions, the relevant §of the design doc.>
```

---

## Worked example — a real Phase 0 task

```markdown
# TASK 0001: NodeId content-addressed identifier
Phase: 0
Depends on: none

## Goal
Implement `NodeId`, the content-addressed identity of a node. A `NodeId` is the
blake3 hash of a node's canonical byte encoding. Two nodes with identical
canonical bytes must produce identical `NodeId`s; any difference must produce a
different `NodeId` with overwhelming probability. Done when the acceptance tests
pass and clippy is clean.

## Interface
// FILE: crates/ailang-core/src/node_id.rs
- `pub struct NodeId([u8; 32]);`
- `impl NodeId { pub fn of(canonical_bytes: &[u8]) -> Self; }`
- Derive: Clone, Copy, PartialEq, Eq, Hash, Debug.
- `impl std::fmt::Display for NodeId` → `"blake3:" + first 12 hex chars`.

## Constraints
- Use the `blake3` crate only. No `unsafe`. Deterministic. No IO.
- Capabilities granted: none.

## Acceptance tests
// FILE: crates/ailang-core/src/node_id_tests.rs
#[cfg(test)]
mod tests {
    use super::super::node_id::NodeId;

    #[test]
    fn identical_bytes_same_id() {
        assert_eq!(NodeId::of(b"hello"), NodeId::of(b"hello"));
    }
    #[test]
    fn different_bytes_different_id() {
        assert_ne!(NodeId::of(b"hello"), NodeId::of(b"hellp"));
    }
    #[test]
    fn empty_is_stable() {
        assert_eq!(NodeId::of(b""), NodeId::of(b""));
    }
    #[test]
    fn display_has_prefix_and_len() {
        let s = format!("{}", NodeId::of(b"x"));
        assert!(s.starts_with("blake3:"));
        assert_eq!(s.len(), "blake3:".len() + 12);
    }
}

## Context
First task in ailang-core. Establishes the content-addressing primitive used by
nodes and folds (design §4.1, §4.4). Add `blake3` to crates/ailang-core/Cargo.toml.
Keep the type tiny and dependency-light — everything else will build on it.
```

This is the granularity. The next tasks would be `0002: canonical node encoding`, `0003: Type enum`, `0004: Type::unify`, each just as small.
