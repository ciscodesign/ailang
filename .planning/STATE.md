# ailang — Project State

Last activity: 2026-06-09 - Phase 3 COMPLETE — all 4 tasks done, 81 tests passing

---

## Current Phase

**Phase 3 — Extended Codegen & Validation**
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

### Completed — Phase 2 (Runnable Programs)
- 0012: builtin-nodes (add_int, sub_int, mul_int, neg_int, concat_text, not_bool, and_bool, or_bool)
- 0013: codegen-wiring (edge rebinding emitted before each node, local CodegenError)
- 0014: graph-builder (GraphBuilder fluent API, 5 tests)
- 0015: cli (ailang eval/emit binary, value_to_json serialization)

### Completed — Phase 3 (Extended Codegen & Validation)
- 0016: const-literals (Const:<port>:<literal> embedded directly in codegen emit)
- 0017: graph-validator (validate() returns all errors: FanIn, SelfLoop, SrcPortOob, DstPortOob)
- 0018: more-builtins (eq_int, lt_int, if_int, len_text added to register_builtins)
- 0019: wasm-emit (codegen_wasm: #[no_mangle] extern "C" fn, return type from last output node)

### Test count
81 tests passing (44 ailang-core + 4 ailang-effects + 10 ailang-exec + 5 ailang-transpile + 16 ailang-nodes + 2 ailang-cli)

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
| 260609-ele | Implement Phase 2 tasks 0013-0015 manually (codegen wiring, graph builder, CLI binary) | 2026-06-09 | 7e888e7 | 260609-ele-implement-ailang-phase-2-tasks-0013-0015 |
| 260609-fdw | Implement Phase 3 tasks 0016-0019 manually (const literals, graph validator, more builtins, wasm emit) | 2026-06-09 | pending | 260609-fdw-implement-phase-3-tasks-0016-0019 |
