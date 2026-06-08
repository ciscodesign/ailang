# ailang — Project State

Last activity: 2026-06-08 - Phase 0 COMPLETE — all 7 tasks done, 37 tests passing

---

## Current Phase

**Phase 0 — Foundation**
Status: In Progress

### Completed — Phase 0
- 0001: NodeId (blake3 content-addressed, Ord/PartialOrd, serde hex)
- 0002: Type enum (Text, Int, Float, Bool, Bytes, Option, Result, Var, Union, Fold) + serde
- 0003: Type::unify (one-way static edge-legality check)
- 0004: Graph (NodeDef, PortDef, Edge, GraphError, add_edge with unify)
- 0005: EffectSet + CapToken (ailang-effects crate)
- 0006: NodeDef.effects: EffectSet, Graph::total_effects()
- 0007: serial::encode/decode (canonical JSON, round-trip)

### Test count
37 tests passing (33 ailang-core + 4 ailang-effects)

---

## Blockers/Concerns

- Executor models (qwen3.6, devstral:24b) repeatedly hallucinate `TypeEnum` — fixed by embedding actual API in executor.md prompt
- Stale background tasks can overwrite working code — always kill old tasks before relaunching orchestrator

---

### Quick Tasks Completed

| # | Description | Date | Commit | Directory |
|---|-------------|------|--------|-----------|
| 260608-f5y | Fix lib.rs module collision after task-0004 executor overwrite (28 tests restored) | 2026-06-08 | 1e04ea4 | 260608-f5y-fix-orchestrator-file-corruption-and-ens |
