---
phase: quick-260608-f5y
plan: 01
subsystem: ailang-core
tags: [fix, orchestrator-collision, lib-rs, test-recovery]
dependency_graph:
  requires: []
  provides: [full-22-test-baseline]
  affects: [ailang-core]
tech_stack:
  added: []
  patterns: [file-per-module test pattern]
key_files:
  created: []
  modified:
    - crates/ailang-core/src/lib.rs
decisions:
  - "Restored three missing #[cfg(test)] mod declarations stripped by task-0004 executor overwrite"
metrics:
  duration: "< 5 minutes"
  completed: "2026-06-08"
  tasks_completed: 2
  files_modified: 1
---

# Quick Task 260608-f5y: Fix Orchestrator lib.rs Collision — Summary

**One-liner:** Restored node_id_tests, ty_tests, unify_tests module declarations in lib.rs after task-0004 executor overwrote the file, recovering 22+ tests from 6.

---

## What Was Fixed

The task-0004 executor (graph implementation) performed a full overwrite of `crates/ailang-core/src/lib.rs`, replacing the entire file with only the four production module declarations and `graph_tests`. This silently dropped the three `#[cfg(test)] mod` declarations wired in tasks 0001-0003:

- `mod node_id_tests` (task 0001)
- `mod ty_tests` (task 0002)
- `mod unify_tests` (task 0003)

The workspace compiled and 6 graph tests ran, but the 16-22 tests from prior tasks were invisible to the test runner.

---

## Test Count

| State | Tests passing |
|-------|--------------|
| Before fix | 6 (graph_tests only) |
| After fix | 28 (all modules) |

The final count is 28 rather than the estimated 22 — additional graph tests were present beyond the initial count in STATE.md.

---

## Commit

```
1e04ea4  fix: restore lib.rs module declarations after task-0004 executor collision
```

Pushed to `origin/main`. Remote is up to date.

---

## Root Cause Note: lib.rs Collision Pattern

This is a recurring pattern in this project. Executor agents (especially smaller models like qwen3.6 and devstral:24b) treat lib.rs as a file to be fully rewritten from scratch when adding new modules, rather than appending to existing content.

**Mitigations for future orchestration:**
1. In executor.md prompts, explicitly list ALL existing module declarations that must be preserved when modifying lib.rs
2. Use `Edit` (diff-based) rather than `Write` (overwrite) for lib.rs modifications
3. After each task that touches lib.rs, the orchestrator should verify `cargo test --workspace` shows the same or higher test count before marking the task complete

---

## Deviations from Plan

None — plan executed exactly as written.

---

## Self-Check: PASSED

- `crates/ailang-core/src/lib.rs` — confirmed modified, all 8 module declarations present
- `crates/ailang-core/Cargo.toml` — confirmed intact, [package] section present
- Commit `1e04ea4` — confirmed at HEAD, pushed to origin/main
- `cargo test --workspace` — 28 passed, 0 failed
