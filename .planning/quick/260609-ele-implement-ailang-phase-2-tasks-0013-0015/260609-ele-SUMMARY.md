---
phase: quick-260609-ele
plan: 01
subsystem: ailang-transpile, ailang-core, ailang-cli
tags: [codegen, graph-builder, cli, rust]
dependency_graph:
  requires: [0011-codegen, 0007-serial, 0010-eval, 0012-builtin-nodes]
  provides: [0013-codegen-wiring, 0014-graph-builder, 0015-cli]
  affects: [ailang-transpile, ailang-core, ailang-cli]
tech_stack:
  patterns: [Kahn-topo-sort, edge-rebinding, fluent-builder, value-to-json]
key_files:
  created:
    - crates/ailang-cli/src/main.rs
    - crates/ailang-cli/Cargo.toml
    - crates/ailang-core/src/builder.rs
    - crates/ailang-core/src/builder_tests.rs
  modified:
    - crates/ailang-transpile/src/codegen.rs
    - crates/ailang-transpile/src/lib.rs
decisions:
  - "CodegenError defined locally in codegen.rs — no re-export via lib.rs"
  - "strip_prefix used for Const:/Code: matching — clippy::manual_strip compliant"
  - "Bytes arm in value_to_json uses format! hex — no hex crate needed"
  - "GraphBuilder verified correct as-written — no changes needed to builder.rs"
metrics:
  duration: ~12 minutes
  completed: 2026-06-09
  tasks_completed: 3
  files_changed: 8
---

# Phase quick-260609-ele Plan 01: Implement ailang Phase 2 Tasks 0013-0015 Summary

**One-liner:** Restored codegen with Kahn-sort edge wiring, verified GraphBuilder fluent API, and fixed CLI Value serialization — 67 workspace tests passing, clippy clean.

## Tasks Completed

| Task | Name | Commit | Files |
|------|------|--------|-------|
| 1 | Restore codegen.rs with edge wiring + fix lib.rs | bc6ebd8, 5a4933e | codegen.rs, lib.rs |
| 2 | Verify GraphBuilder (0014) | fbd692e | builder.rs, builder_tests.rs, core/lib.rs |
| 3 | Fix CLI main.rs Value serialization (0015) | f474fc5 | main.rs, Cargo.toml, Cargo.lock |

## What Was Built

### Task 1 — codegen.rs (0013)
- `CodegenError` now defined locally in `codegen.rs` — removed broken `use crate::CodegenError` import
- `lib.rs` simplified to `pub mod codegen` + `#[cfg(test)] mod codegen_tests` — no re-export
- Kahn topological sort emits edge rebindings before each node's own let-binding:
  `let node_{dst}_{dst_port} = node_{src}_{src_port};`
- `Const:` → `let node_{i}_{port}: {ty} = todo!("Const");`
- `Code:` → `let node_{i}_out: {ty} = {expr};`
- Other → `let _node_{i} = todo!("{kind}");`
- All 4 codegen_tests pass

### Task 2 — GraphBuilder (0014)
- Builder was already correctly implemented by the orchestrator
- `NodeId::of(&self.counter.to_le_bytes())` for unique IDs — correct
- All 5 builder tests pass: empty_build, const_node, code_node_with_inputs, generic_node, edge_type_mismatch_returns_error
- ailang-core total: 38 tests passing

### Task 3 — CLI (0015)
- Replaced `serde_json::to_string(&outputs)` (where `Value` doesn't impl Serialize) with `value_to_json` helper
- `value_to_json` converts `ailang_exec::value::Value` to `serde_json::Value` for all variants
- Bytes arm uses `format!("{x:02x}")` inline — no `hex` crate dependency needed
- Removed unused `use std::collections::HashMap` and `use anyhow::Context` imports
- Both CLI tests pass: emit_empty_graph_contains_fn, eval_empty_graph_succeeds

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Clippy manual_strip errors in codegen.rs**
- **Found during:** Post-Task-1 clippy run (`cargo clippy --workspace -- -D warnings`)
- **Issue:** `kind["Const:".len()..]` and `kind["Code:".len()..]` pattern triggers `clippy::manual_strip` as error under `-D warnings`
- **Fix:** Replaced with idiomatic `if let Some(port) = kind.strip_prefix("Const:")` pattern
- **Files modified:** `crates/ailang-transpile/src/codegen.rs`
- **Commit:** 5a4933e

## Test Results

| Crate | Tests |
|-------|-------|
| ailang-cli | 2 passing |
| ailang-core | 38 passing |
| ailang-effects | 4 passing |
| ailang-exec | 10 passing |
| ailang-fold | 0 (no tests) |
| ailang-nodes | 9 passing |
| ailang-transpile | 4 passing |
| **Total** | **67 passing** |

## Self-Check: PASSED

- [x] crates/ailang-transpile/src/codegen.rs — FOUND
- [x] crates/ailang-transpile/src/lib.rs — FOUND
- [x] crates/ailang-core/src/builder.rs — FOUND
- [x] crates/ailang-cli/src/main.rs — FOUND
- [x] Commit bc6ebd8 — Task 1 codegen
- [x] Commit fbd692e — Task 2 builder
- [x] Commit f474fc5 — Task 3 CLI
- [x] Commit 5a4933e — clippy fix
- [x] `cargo test --workspace` — 67 tests passing
- [x] `cargo clippy --workspace -- -D warnings` — clean
