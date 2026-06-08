---
phase: quick
plan: 260608-lon
type: execute
wave: 1
depends_on: []
files_modified:
  - crates/ailang-nodes/src/builtins.rs
  - crates/ailang-nodes/src/builtins_tests.rs
autonomous: true
requirements: [0012]
must_haves:
  truths:
    - "cargo test -p ailang-nodes passes all 9 tests"
    - "builtins.rs compiles without errors"
    - "orchestrator is killed and relaunched for tasks 0013-0015"
  artifacts:
    - path: "crates/ailang-nodes/src/builtins.rs"
      provides: "register_builtins() with 8 nodes"
      exports: ["register_builtins"]
    - path: "crates/ailang-nodes/src/builtins_tests.rs"
      provides: "9 acceptance tests"
  key_links:
    - from: "builtins_tests.rs"
      to: "builtins::register_builtins"
      via: "crate::builtins::register_builtins"
---

<objective>
Fix task 0012 (builtin nodes) that the orchestrator left broken after 6 failed attempts.

Three concrete bugs in the orchestrator-generated file must be corrected:
1. Double `??` — replace with single `?`
2. `inputs` not declared `mut` — add `mut`
3. HashMap construction via `.iter().cloned().collect()` — replace with `HashMap::from([(...)])`

Write correct builtins.rs and builtins_tests.rs from scratch per the task spec, verify they compile and all 9 tests pass, kill the running orchestrator, then relaunch it for the remaining tasks (0013-0015).

Purpose: Unblock Phase 2 execution. The orchestrator is spinning on 0012 with a broken file it cannot self-correct.
Output: Working builtins.rs + builtins_tests.rs, 9 green tests, orchestrator running on 0013-0015.
</objective>

<execution_context>
@/Users/cisco/.claude/get-shit-done/workflows/execute-plan.md
@/Users/cisco/.claude/get-shit-done/templates/summary.md
</execution_context>

<context>
@/Users/cisco/Documents/1_Progetti/ailang/.planning/STATE.md
@/Users/cisco/Documents/1_Progetti/ailang/orchestrator/tasks/0012-builtin-nodes.md
@/Users/cisco/Documents/1_Progetti/ailang/crates/ailang-nodes/src/lib.rs
</context>

<tasks>

<task type="auto">
  <name>Task 1: Write correct builtins.rs and builtins_tests.rs</name>
  <files>
    crates/ailang-nodes/src/builtins.rs
    crates/ailang-nodes/src/builtins_tests.rs
  </files>
  <action>
Write crates/ailang-nodes/src/builtins.rs with this exact content — no variations:

```rust
use std::collections::HashMap;
use ailang_exec::registry::{ExecError, Inputs, NodeRegistry, Outputs};
use ailang_exec::value::Value;

pub fn register_builtins(registry: &mut NodeRegistry) {
    // add_int: a + b
    registry.register("add_int", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let a = inputs.remove("a").ok_or_else(|| ExecError::MissingInput("a".into()))?;
        let b = inputs.remove("b").ok_or_else(|| ExecError::MissingInput("b".into()))?;
        match (a, b) {
            (Value::Int(x), Value::Int(y)) => Ok(HashMap::from([("out".into(), Value::Int(x + y))])),
            _ => Err(ExecError::Failed("add_int: expected Int inputs".into())),
        }
    }));

    // sub_int: a - b
    registry.register("sub_int", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let a = inputs.remove("a").ok_or_else(|| ExecError::MissingInput("a".into()))?;
        let b = inputs.remove("b").ok_or_else(|| ExecError::MissingInput("b".into()))?;
        match (a, b) {
            (Value::Int(x), Value::Int(y)) => Ok(HashMap::from([("out".into(), Value::Int(x - y))])),
            _ => Err(ExecError::Failed("sub_int: expected Int inputs".into())),
        }
    }));

    // mul_int: a * b
    registry.register("mul_int", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let a = inputs.remove("a").ok_or_else(|| ExecError::MissingInput("a".into()))?;
        let b = inputs.remove("b").ok_or_else(|| ExecError::MissingInput("b".into()))?;
        match (a, b) {
            (Value::Int(x), Value::Int(y)) => Ok(HashMap::from([("out".into(), Value::Int(x * y))])),
            _ => Err(ExecError::Failed("mul_int: expected Int inputs".into())),
        }
    }));

    // neg_int: -a
    registry.register("neg_int", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let a = inputs.remove("a").ok_or_else(|| ExecError::MissingInput("a".into()))?;
        match a {
            Value::Int(x) => Ok(HashMap::from([("out".into(), Value::Int(-x))])),
            _ => Err(ExecError::Failed("neg_int: expected Int input".into())),
        }
    }));

    // concat_text: a ++ b
    registry.register("concat_text", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let a = inputs.remove("a").ok_or_else(|| ExecError::MissingInput("a".into()))?;
        let b = inputs.remove("b").ok_or_else(|| ExecError::MissingInput("b".into()))?;
        match (a, b) {
            (Value::Text(x), Value::Text(y)) => Ok(HashMap::from([("out".into(), Value::Text(x + &y))])),
            _ => Err(ExecError::Failed("concat_text: expected Text inputs".into())),
        }
    }));

    // not_bool: !a
    registry.register("not_bool", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let a = inputs.remove("a").ok_or_else(|| ExecError::MissingInput("a".into()))?;
        match a {
            Value::Bool(x) => Ok(HashMap::from([("out".into(), Value::Bool(!x))])),
            _ => Err(ExecError::Failed("not_bool: expected Bool input".into())),
        }
    }));

    // and_bool: a && b
    registry.register("and_bool", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let a = inputs.remove("a").ok_or_else(|| ExecError::MissingInput("a".into()))?;
        let b = inputs.remove("b").ok_or_else(|| ExecError::MissingInput("b".into()))?;
        match (a, b) {
            (Value::Bool(x), Value::Bool(y)) => Ok(HashMap::from([("out".into(), Value::Bool(x && y))])),
            _ => Err(ExecError::Failed("and_bool: expected Bool inputs".into())),
        }
    }));

    // or_bool: a || b
    registry.register("or_bool", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let a = inputs.remove("a").ok_or_else(|| ExecError::MissingInput("a".into()))?;
        let b = inputs.remove("b").ok_or_else(|| ExecError::MissingInput("b".into()))?;
        match (a, b) {
            (Value::Bool(x), Value::Bool(y)) => Ok(HashMap::from([("out".into(), Value::Bool(x || y))])),
            _ => Err(ExecError::Failed("or_bool: expected Bool inputs".into())),
        }
    }));
}
```

Then write crates/ailang-nodes/src/builtins_tests.rs verbatim from the task spec (the full #[cfg(test)] mod tests { ... } block with all 9 tests: add_int, sub_int, mul_int, neg_int, concat_text, not_bool, and_bool, or_bool, missing_input_returns_error).

Key correctness rules (why the orchestrator failed):
- `inputs` parameter MUST be `mut inputs: Inputs` — `.remove()` requires mutability
- HashMap construction MUST use `HashMap::from([("out".into(), Value::...)])` — NOT `.iter().cloned().collect()`
- Each `.ok_or_else(...)` gets ONE `?` — double `??` is a compile error
  </action>
  <verify>
    <automated>cd /Users/cisco/Documents/1_Progetti/ailang && cargo test -p ailang-nodes 2>&1</automated>
  </verify>
  <done>All 9 tests in ailang-nodes pass. Zero compile errors.</done>
</task>

<task type="auto">
  <name>Task 2: Kill orchestrator and relaunch for tasks 0013-0015</name>
  <files></files>
  <action>
Step 1 — Kill any running orchestrator process:
  pkill -f orchestrate.py || true

Step 2 — Verify it is dead:
  sleep 1 && pgrep -f orchestrate.py && echo "STILL RUNNING" || echo "KILLED"

Step 3 — Relaunch orchestrator in background for remaining tasks:
  cd /Users/cisco/Documents/1_Progetti/ailang && python3 orchestrator/orchestrate.py --best-effort &

The orchestrator will pick up from task 0013 onward (0012 is now complete). Do NOT wait for it — it runs in the background.
  </action>
  <verify>
    <automated>sleep 2 && pgrep -f orchestrate.py && echo "ORCHESTRATOR RUNNING" || echo "NOT STARTED"</automated>
  </verify>
  <done>Old orchestrator process killed. New orchestrator process running in background targeting tasks 0013-0015.</done>
</task>

</tasks>

<verification>
After Task 1: `cargo test -p ailang-nodes` outputs "9 tests passed, 0 failed".
After Task 2: `pgrep -f orchestrate.py` returns a PID.
</verification>

<success_criteria>
- builtins.rs compiles cleanly with `cargo build -p ailang-nodes`
- All 9 acceptance tests in builtins_tests.rs pass
- Previous orchestrator process is dead
- New orchestrator process is running for 0013-0015
</success_criteria>

<output>
After completion, create `/Users/cisco/Documents/1_Progetti/ailang/.planning/quick/260608-lon-implement-0012-builtin-nodes-fix-broken-/260608-lon-SUMMARY.md`
</output>
