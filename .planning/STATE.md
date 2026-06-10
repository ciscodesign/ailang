# ailang — Project State

Last activity: 2026-06-10 - Phase 6 COMPLETE — 134 tests passing, float builtins, string ops, CLI inspect/validate

---

## Current Phase

**Phase 6 — Float Builtins, String Ops, CLI Inspect/Validate**
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

### Completed — Phase 4 (Lists, Fold Passes & File I/O)
- 0020: list-type (Type::List(Box<Type>) + Value::List(Vec<Value>), unify + matches_type)
- 0021: list-builtins (list_empty, list_push, list_head, list_tail, list_len, list_int_sum)
- 0022: graph-passes (ailang-fold: dead_nodes() — backward BFS liveness from sinks)
- 0023: graph-file-io (CLI save/load subcommands, value_to_json handles List)

### Completed — Phase 5 (More Builtins, Map Type, Const Fold, Examples)
- 0024: more-builtins (neg_int, div_int, mod_int, gt_int, abs_int, min_int, max_int, int_to_text, bool_to_text)
- 0025: map-type (Type::Map(k,v) + Value::Map(BTreeMap) + unify + 6 map builtins)
- 0026: const_values fold pass (ailang-fold: extract compile-time literals from graph)
- 0027: examples (5 runnable .ailang.json programs + README; serial fixed to round-trip edges)
- codegen: proper Rust type names (i64/bool/String) + builtin_expr for all known kinds

### Completed — Phase 6 (Float Builtins, String Ops, CLI Inspect/Validate)
- 0028: float-builtins (sub/mul/div/neg/abs_float, floor/ceil/round_float, int_to_float, float_to_int, float_to_text; codegen exprs)
- 0029: string-ops (trim, to_upper, to_lower, contains, starts_with, ends_with, replace, split, join, slice _text; codegen exprs)
- 0030: cli-inspect/validate (`ailang inspect` prints nodes/edges/types; `ailang validate` runs validator with exit-code 1 on error)

### Test count
134 tests passing (50 ailang-core + 4 ailang-effects + 14 ailang-exec + 7 ailang-fold + 5 ailang-transpile + 51 ailang-nodes + 3 ailang-cli)

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
| 260609-fdw | Implement Phase 3 tasks 0016-0019 manually (const literals, graph validator, more builtins, wasm emit) | 2026-06-09 | ca5238a | 260609-fdw-implement-phase-3-tasks-0016-0019 |
| 260609-p4 | Implement Phase 4 tasks 0020-0023 (List type/value, list builtins, dead-node fold pass, CLI file I/O) | 2026-06-09 | b388747 | inline |
| 260610-p5 | Implement Phase 5: more builtins, Map type, const_values, examples, codegen fix, serial edges | 2026-06-10 | 6d21dd2 | inline |
| 260610-p6 | Implement Phase 6: float builtins, string ops, CLI inspect/validate — 134 tests | 2026-06-10 | f118f5f | inline |
