# ailang — Project State

Last activity: 2026-06-08 - Phase 0 tasks 0001-0004 complete, 28 tests passing

---

## Current Phase

**Phase 0 — Foundation**
Status: In Progress

### Completed
- 0001: NodeId (blake3, Ord/PartialOrd)
- 0002: Type enum (Text, Int, Float, Bool, Bytes, Option, Result, Var, Union, Fold)
- 0003: Type::unify (one-way, static edge-legality check)
- 0004: Graph (NodeDef, PortDef, Edge, GraphError, add_edge with unify)

### In Progress
- 0005: EffectSet + CapToken
- 0006: Node effects
- 0007: Serialization

### Test count
28 tests passing (ailang-core)

---

## Blockers/Concerns

- Executor models (qwen3.6, devstral:24b) repeatedly hallucinate `TypeEnum` — fixed by embedding actual API in executor.md prompt
- Stale background tasks can overwrite working code — always kill old tasks before relaunching orchestrator

---

### Quick Tasks Completed

| # | Description | Date | Commit | Directory |
|---|-------------|------|--------|-----------|
| 260608-f5y | Fix lib.rs module collision after task-0004 executor overwrite (28 tests restored) | 2026-06-08 | 1e04ea4 | 260608-f5y-fix-orchestrator-file-corruption-and-ens |
