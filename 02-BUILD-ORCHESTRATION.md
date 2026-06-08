# ailang — Build Orchestration Plan

**How we actually build this thing.** Claude is the *controller* (architect, decomposer, reviewer). Local models via **Ollama** (plus optional cloud tools) are the *executors* (they write the code). A dumb, reliable *harness* runs the compiler and tests and feeds results back. Nothing gets accepted until it compiles, passes tests, and survives review.

The orchestration philosophy is deliberately the same as ailang's own security model: **never trust an output you haven't verified.** Code from an executor is guilty until the compiler and tests prove it innocent.

---

## 1. The roles

| Role | Who | Job |
|---|---|---|
| **Controller** | Claude (this is me) | Own the spec. Break work into tiny verifiable tasks. Write each task spec + acceptance tests. Review executor output. Decide accept/reject/retry. Integrate. |
| **Executor** | Ollama coding models | Take one tiny spec → produce code + tests that satisfy it. Nothing more. |
| **Reviewer** | A *second*, different model | Red-team accepted code against the security/correctness properties. Adversarial, not agreeable. |
| **Harness** | A plain script (no AI) | `cargo build`, `cargo test`, lint, capture output. Deterministic. The source of ground truth. |
| **Human (you)** | You | Approve phase boundaries, resolve genuine design forks, hold the keys/secrets, push to GitHub. |

**Golden rule:** the executor never decides if its own work is correct. The harness does. The controller and reviewer judge *taste and safety*; the harness judges *facts*.

---

## 2. Recommended executor models (Ollama)

Pick the largest that fits your hardware; bigger = fewer retries.

- **Coding (primary):** `qwen2.5-coder` (32b if you can, else 14b/7b) — currently one of the strongest open coding models.
- **Coding (alt / reviewer):** `deepseek-coder-v2`, `codellama`, or `llama3.x` for a *different* model family as the reviewer (diversity matters — a model rarely catches its own blind spots).
- **Reasoning/planning fallback:** a general instruct model (`qwen2.5`, `llama3.x`) for non-code tasks.

> ⚠️ Model availability and names change fast. Before you start, run `ollama list` and check the Ollama library for the current best coding model — don't trust a model name in a doc written months ago. Use **two different families** (one executor, one reviewer) on purpose.

Settings: low temperature for code (`0.1–0.3`), deterministic where possible, generous context window, hard output-format constraints (see prompt library).

---

## 3. The build loop (per task)

```
┌─────────────────────────────────────────────────────────────┐
│ CONTROLLER picks next task from the queue                    │
│   └─ writes: spec + interface + acceptance tests + context   │
│                                                              │
│ EXECUTOR (Ollama) generates: implementation + tests          │
│                                                              │
│ HARNESS runs: cargo build → cargo test → clippy              │
│   ├─ FAIL → send errors back to executor (max N retries)     │
│   └─ PASS ↓                                                  │
│                                                              │
│ REVIEWER (2nd model) red-teams against the property list     │
│   ├─ finds issue → back to executor with the critique        │
│   └─ clean ↓                                                 │
│                                                              │
│ CONTROLLER integrates, commits, marks task done, logs it     │
└─────────────────────────────────────────────────────────────┘
```

If a task fails the loop more than `N` times (suggest 3), it's **too big or under-specified** — the controller splits it and tries again. Thrashing is a spec smell, not an executor failure.

---

## 4. Best practices (hard-won, non-negotiable)

1. **Tasks must be tiny.** "Implement the `NodeId` content-hash type and its tests" — not "implement the type system." If a task can't be stated in a paragraph with a clear done-condition, it's too big.
2. **Acceptance tests come first.** The controller writes the tests *before* the executor writes the code. Tests are the contract. Green tests = done.
3. **Always-compile gate.** Code that doesn't build is never even reviewed. Non-negotiable.
4. **One file / one unit per task** where possible. Small blast radius = easy rollback.
5. **Deterministic prompts.** Same task → same prompt → reproducible. Pin model + temperature.
6. **Diverse reviewer.** Reviewer ≠ executor model family. Self-review is theater.
7. **Version control discipline.** One task = one commit, message references the task id. Easy to bisect, easy to revert.
8. **Content-addressed tasks.** Each task has a stable id + hash of its spec, so re-running is idempotent and caching works (very on-brand for ailang).
9. **Human checkpoints at phase boundaries only.** Inside a phase, let the loop run. Don't babysit individual tasks.
10. **Secrets never touch the executor.** API keys, tokens, signing keys stay with the human/harness. Executors get code problems, not credentials.
11. **Log everything.** Every task: spec, model, output, test result, review, decision. This is your audit trail and your dataset.
12. **Stop on ambiguity.** If a task hides a real design decision, the controller escalates to the human instead of guessing. (Auto mode is for *building*, not for *deciding the language's soul*.)

---

## 5. Phase-by-phase build assignments

Mapped to the roadmap. Phases 0–1 are concrete and buildable now; later phases are sketched.

### Phase 0 — Foundations (the data model)
- `NodeId` / content-addressing (blake3 hash of a canonical node encoding).
- Core type representation: primitives, generics, unions, type variables, option.
- `Type::unify(a, b)` + exhaustive tests (the edge legality check).
- Graph data structure: nodes, ports, edges; well-formedness checks.
- Serialization (the canonical binary form) + round-trip tests.
**Done when:** you can build a graph in code, serialize it, reload it, and the typechecker accepts/rejects edges correctly.

### Phase 1 — Walking skeleton (it runs)
- Effect/capability representation + linear-token threading + checks.
- A handful of primitive nodes: `Const`, `Code` (sandboxed), `LLM`, `HTTP`, `DB`, `Gate`.
- **Fold/unfold** with effect-union bubbling.
- Durable executor (reference Restate-style durability; can start with a simple persisted event log).
- **Transpiler: graph → Rust → WASM.** Compile a tiny graph and run it in the browser.
**Done when:** the §4.5 "Assistant" example graph compiles and runs end-to-end.

### Phase 2 — Safety
Capability enforcement, contracts (pre/post), graph-level architecture checks, the reviewer-model gate wired into CI.

### Phase 3 — UI / apps
Pick a UI strategy (web-first), ship one cross-platform demo. This is where we leave Weft's scope.

### Phase 4 — Verification
Proof-carrying nodes; shrink the trusted compiler; formalize the verifier.

### Phase 5 — Ecosystem
AI defines new node types in the language itself; FFI; content-addressed node registry.

---

## 6. Proposed repository layout

```
ailang/
├── README.md                  # start here (provided)
├── docs/
│   ├── 01-DESIGN.md           # the full design (provided)
│   ├── 02-BUILD-ORCHESTRATION.md
│   └── 03-PROMPTS.md
├── crates/
│   ├── ailang-core/            # types, graph, content-addressing, serialization
│   ├── ailang-effects/         # capabilities + linear tokens
│   ├── ailang-nodes/           # primitive node catalog + sandbox
│   ├── ailang-fold/            # fold/unfold + effect bubbling
│   ├── ailang-exec/            # durable executor
│   └── ailang-transpile/       # graph → Rust → WASM
├── orchestrator/
│   ├── orchestrate.py         # the build harness (reference impl, §7)
│   ├── tasks/                 # one file per task: spec + acceptance tests
│   └── logs/                  # full audit trail
├── site/                      # the human-facing landing page (provided)
└── Cargo.toml
```

---

## 7. Reference orchestrator harness

A minimal, honest starting point. It is intentionally dumb: it shuttles text between Ollama and `cargo`, enforces the compile/test gate, and logs. The *intelligence* lives in the controller's specs (which Claude writes) — not in this script. Treat it as a skeleton to harden, not production code.

```python
#!/usr/bin/env python3
"""ailang build harness — controller/executor loop. Reference implementation."""
import json, subprocess, time, pathlib, hashlib, urllib.request

OLLAMA = "http://localhost:11434/api/generate"
EXECUTOR_MODEL = "qwen2.5-coder:32b"     # check `ollama list` first
REVIEWER_MODEL = "deepseek-coder-v2"     # DIFFERENT family on purpose
MAX_RETRIES = 3
ROOT = pathlib.Path(__file__).resolve().parent.parent

def ollama(model: str, prompt: str, temperature: float = 0.2) -> str:
    body = json.dumps({"model": model, "prompt": prompt, "stream": False,
                       "options": {"temperature": temperature}}).encode()
    req = urllib.request.Request(OLLAMA, data=body,
                                 headers={"Content-Type": "application/json"})
    with urllib.request.urlopen(req, timeout=600) as r:
        return json.loads(r.read())["response"]

def harness_check() -> tuple[bool, str]:
    """Ground truth. No AI. Returns (ok, combined_output)."""
    out = []
    for cmd in (["cargo", "build"], ["cargo", "test"],
                ["cargo", "clippy", "--", "-D", "warnings"]):
        p = subprocess.run(cmd, cwd=ROOT, capture_output=True, text=True)
        out.append(f"$ {' '.join(cmd)}\n{p.stdout}\n{p.stderr}")
        if p.returncode != 0:
            return False, "\n".join(out)
    return True, "\n".join(out)

def write_code(response: str):
    """Executor must emit fenced blocks tagged `// FILE: path`. Parse & write."""
    for block in response.split("```"):
        if "// FILE:" in block:
            lines = block.strip().splitlines()
            path = next(l for l in lines if "// FILE:" in l).split("// FILE:")[1].strip()
            code = "\n".join(l for l in lines if "// FILE:" not in l and l not in ("rust",))
            fp = ROOT / path
            fp.parent.mkdir(parents=True, exist_ok=True)
            fp.write_text(code)

def run_task(task_path: pathlib.Path, controller_sys: str,
             executor_sys: str, reviewer_sys: str):
    spec = task_path.read_text()
    tid = hashlib.blake2b(spec.encode(), digest_size=6).hexdigest()
    log = ROOT / "orchestrator" / "logs" / f"{task_path.stem}-{tid}.log"
    log.parent.mkdir(parents=True, exist_ok=True)
    feedback = ""
    for attempt in range(1, MAX_RETRIES + 1):
        prompt = f"{executor_sys}\n\n## TASK\n{spec}\n\n## PRIOR FEEDBACK\n{feedback or 'none'}"
        code = ollama(EXECUTOR_MODEL, prompt)
        write_code(code)
        ok, report = harness_check()
        if not ok:
            feedback = f"Build/test FAILED on attempt {attempt}:\n{report[-4000:]}"
            log.write_text(log.read_text() + feedback if log.exists() else feedback)
            continue
        # diverse reviewer red-teams the passing code
        review = ollama(REVIEWER_MODEL,
                        f"{reviewer_sys}\n\n## SPEC\n{spec}\n\n## CODE\n{code}")
        if review.strip().upper().startswith("APPROVED"):
            print(f"✓ {task_path.name} accepted on attempt {attempt}")
            return True
        feedback = f"Reviewer rejected:\n{review}"
    print(f"✗ {task_path.name} failed after {MAX_RETRIES} attempts — split this task.")
    return False

if __name__ == "__main__":
    P = ROOT / "orchestrator" / "prompts"
    csys = (P / "controller.md").read_text()
    esys = (P / "executor.md").read_text()
    rsys = (P / "reviewer.md").read_text()
    for task in sorted((ROOT / "orchestrator" / "tasks").glob("*.md")):
        if not run_task(task, csys, esys, rsys):
            print("Stopping for human review."); break
```

**What's deliberately missing (harden these):** sandboxing the executor's file writes, git commit-per-task, retry backoff, streaming output, parallel tasks, and a real task dependency graph. Start simple, add as it hurts.

---

## 8. How Claude (the controller) plugs in

Three practical setups, pick by how hands-off you want to be:
- **Manual-controller:** you paste each phase's tasks to me here; I emit specs + acceptance tests + reviews; you run the harness. Most control, most copy-paste.
- **Claude Code:** run me as an agent in the repo; I write specs/tests/reviews and drive `cargo` directly, delegating bulk codegen to Ollama via the harness. Best balance.
- **Full-auto:** harness runs Ollama for codegen and a Claude API call for the controller/reviewer steps. Most autonomous, needs the most guardrails (and a budget).

The prompt library (03-PROMPTS.md) is written so any of these work — the prompts are the contract, the setup is just plumbing.
