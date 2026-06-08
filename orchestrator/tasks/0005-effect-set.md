# TASK 0005: EffectSet + capability token
Phase: 0
Depends on: 0001 (NodeId)

## Goal
Implement `EffectSet` (the set of capabilities a node declares) and `CapToken`
(the linear capability token that enforces deterministic ordering of effectful
operations). Done when all acceptance tests pass and clippy is clean.

## Interface
```
// FILE: crates/ailang-effects/src/lib.rs

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub enum Effect {
    Net,    // network access
    Db,     // database access
    Fs,     // filesystem access
    Llm,    // LLM call
    Human,  // human-in-the-loop wait
    Clock,  // wall-clock / sleep
    Rand,   // randomness
    Ui,     // user interface
}

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct EffectSet(std::collections::BTreeSet<Effect>);

impl EffectSet {
    pub fn empty() -> Self;
    pub fn of(effects: &[Effect]) -> Self;
    pub fn contains(&self, e: Effect) -> bool;
    pub fn union(&self, other: &EffectSet) -> EffectSet;
    pub fn is_subset_of(&self, other: &EffectSet) -> bool;
    pub fn iter(&self) -> impl Iterator<Item = &Effect>;
}

/// Linear capability token. Created once at entry; consumed and re-emitted
/// by each effectful step, enforcing sequential ordering.
/// The token is affine (move-only) — it cannot be copied or cloned.
pub struct CapToken {
    effect: Effect,
    seq:    u64,   // monotonically increasing sequence number
}

impl CapToken {
    pub fn new(effect: Effect) -> Self;
    /// Consume this token and return the next one in the sequence.
    pub fn next(self) -> Self;
    pub fn effect(&self) -> Effect;
    pub fn seq(&self) -> u64;
}
```

## Constraints
- `CapToken` must NOT implement `Clone` or `Copy` — linearity is the whole point.
- `EffectSet` uses `BTreeSet` for stable ordering (required for canonical serialization).
- No `unsafe`. No IO.
- Capabilities granted: none.

## Acceptance tests
```rust
// FILE: crates/ailang-effects/src/tests.rs
#[cfg(test)]
mod tests {
    use crate::{Effect, EffectSet, CapToken};

    #[test]
    fn effect_set_union() {
        let a = EffectSet::of(&[Effect::Net, Effect::Db]);
        let b = EffectSet::of(&[Effect::Db, Effect::Llm]);
        let u = a.union(&b);
        assert!(u.contains(Effect::Net));
        assert!(u.contains(Effect::Db));
        assert!(u.contains(Effect::Llm));
        assert!(!u.contains(Effect::Fs));
    }
    #[test]
    fn effect_set_subset() {
        let small = EffectSet::of(&[Effect::Net]);
        let big   = EffectSet::of(&[Effect::Net, Effect::Db]);
        assert!(small.is_subset_of(&big));
        assert!(!big.is_subset_of(&small));
    }
    #[test]
    fn cap_token_sequences() {
        let t0 = CapToken::new(Effect::Net);
        assert_eq!(t0.seq(), 0);
        let t1 = t0.next();
        assert_eq!(t1.seq(), 1);
        let t2 = t1.next();
        assert_eq!(t2.seq(), 2);
    }
    #[test]
    fn cap_token_effect_preserved() {
        let t = CapToken::new(Effect::Db);
        assert_eq!(t.effect(), Effect::Db);
    }
    // Compile-time test: uncommenting the line below must NOT compile.
    // let t = CapToken::new(Effect::Net);
    // let _copy = t.clone(); // CapToken is not Clone
}
```

## Context
EffectSet is stored on every NodeDef (not yet — that's a later task that updates
NodeDef). CapToken is the linear threading primitive that makes effect ordering
a type-level property rather than a convention. The "no Clone/Copy" constraint
on CapToken is the entire linearity story — the compiler enforces it.
This task lives in the `ailang-effects` crate.
