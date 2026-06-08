# ailang — Project State

Last activity: 2026-06-08 - Phase 2 in progress — 0012 (builtin-nodes) done, 59 tests passing

---

## Current Phase

**Phase 1 — Execution Layer**
Status: COMPLETE

### Completed — Phase 0 (Foundation)
- 0001: NodeId (blake3 content-addressed, Ord/PartialOrd, serde hex)
- 0002: Type enum (Text, Int, Float, Bool, Bytes, Option, Result, Var, Union, Fold) + serde
- 0003: Type::unify (one-way static edge-legality check)
- 0004: Graph (NodeDef, PortDef, Edge, GraphError, add_edge with unify)
- 0005: EffectSet + CapToken (ailang-effects crate)
- 0006: NodeDef.effects: EffectSet, Graph::total_effects()
- 0007: serial::encode/decode (canonical JSON, round-trip)

### Completed — Phase 1 (Execution Layer)
- 0008: Value enum (runtime data: Text/Int/Float/Bool/Bytes/Option/Result + matches_type)
- 0009: NodeRegistry (kind → ExecFn dispatch, register_const helper)
- 0010: eval (Kahn topological sort, executes graph, passes outputs as inputs)
- 0011: codegen (emit Rust source from Graph: Const:* → todo!, Code:<expr> → expr)

### Phase 2 in progress (Runnable Programs)
- 0012: builtin-nodes (add_int, sub_int, mul_int, neg_int, concat_text, not_bool, and_bool, or_bool) ✓
- 0013: codegen-wiring — in progress (orchestrator running)
- 0014: graph-builder — pending
- 0015: cli — pending

### Test count
59 tests passing (33 ailang-core + 4 ailang-effects + 10 ailang-exec + 3 ailang-transpile + 9 ailang-nodes)

---

## Blockers/Concerns

- Executor models (qwen3.6, devstral:24b) repeatedly hallucinate `TypeEnum` — fixed by embedding actual API in executor.md prompt
- Stale background tasks can overwrite working code — always kill old tasks before relaunching orchestrator

---

### Quick Tasks Completed

| # | Description | Date | Commit | Directory |
|---|-------------|------|--------|-----------|
| 260608-f5y | Fix lib.rs module collision after task-0004 executor overwrite (28 tests restored) | 2026-06-08 | 1e04ea4 | 260608-f5y-fix-orchestrator-file-corruption-and-ens |
| 260608-lon | Implement 0012-builtin-nodes manually (fix double-??, mut inputs, HashMap construction) | 2026-06-08 | 98ac264 | 260608-lon-implement-0012-builtin-nodes-fix-broken- |
