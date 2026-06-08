#!/usr/bin/env python3
"""ailang build harness — controller/executor/reviewer loop."""

import argparse
import hashlib
import json
import os
import pathlib
import subprocess
import sys
import time
import urllib.request
from datetime import datetime

# ensure cargo is on PATH
os.environ["PATH"] = "/opt/homebrew/opt/rustup/bin:" + os.environ.get("PATH", "")

# ── config ──────────────────────────────────────────────────────────────────
OLLAMA        = "http://localhost:11434/api/generate"
EXECUTOR      = "qwen3.6"       # primary code writer
EXECUTOR_ALT  = "devstral:24b"  # fallback if primary hits MAX_RETRIES
REVIEWER      = "deepseek-r1:8b"
MAX_RETRIES   = 3               # per executor before switching to alt
TEMP_CODE     = 0.15
TEMP_REVIEW   = 0.2

ROOT      = pathlib.Path(__file__).resolve().parent.parent
TASKS_DIR = ROOT / "orchestrator" / "tasks"
LOGS_DIR  = ROOT / "orchestrator" / "logs"
PROMPTS   = ROOT / "orchestrator" / "prompts"

EXECUTOR_SYS = (PROMPTS / "executor.md").read_text()  if (PROMPTS / "executor.md").exists()  else ""
REVIEWER_SYS = (PROMPTS / "reviewer.md").read_text()  if (PROMPTS / "reviewer.md").exists()  else ""


# ── ollama ───────────────────────────────────────────────────────────────────
def ollama(model: str, prompt: str, temperature: float = 0.2) -> str:
    # prepend /no_think for Qwen3 models to skip chain-of-thought and emit code directly
    if "qwen3" in model.lower():
        prompt = "/no_think\n\n" + prompt
    body = json.dumps({
        "model": model,
        "prompt": prompt,
        "stream": False,
        "options": {"temperature": temperature, "num_ctx": 32768},
    }).encode()
    req = urllib.request.Request(
        OLLAMA, data=body, headers={"Content-Type": "application/json"}
    )
    try:
        with urllib.request.urlopen(req, timeout=600) as r:
            raw = json.loads(r.read())["response"]
    except Exception as e:
        return f"OLLAMA_ERROR: {e}"
    # strip <think>…</think> blocks (deepseek-r1 and other reasoning models)
    # keep content AFTER the last </think> tag; if empty, fall back to thinking content
    import re as _re
    after = _re.split(r"</think>", raw, flags=_re.IGNORECASE)
    if len(after) > 1:
        actual = after[-1].strip()
        return actual if actual else raw  # non-empty actual response preferred
    return raw


# ── harness ───────────────────────────────────────────────────────────────────
def harness() -> tuple[bool, str]:
    """Ground truth. No AI. Returns (ok, output)."""
    out = []
    for cmd in (
        ["cargo", "build", "--workspace"],
        ["cargo", "test",  "--workspace"],
        ["cargo", "clippy", "--workspace", "--", "-D", "warnings"],
    ):
        p = subprocess.run(cmd, cwd=ROOT, capture_output=True, text=True)
        block = f"$ {' '.join(cmd)}\n{p.stdout}\n{p.stderr}".strip()
        out.append(block)
        if p.returncode != 0:
            return False, "\n\n".join(out)
    # guard: fail if cargo test ran zero tests across the whole workspace
    combined = "\n".join(out)
    import re
    counts = re.findall(r"test result: ok\. (\d+) passed", combined)
    total = sum(int(c) for c in counts)
    if total == 0:
        return False, combined + "\n\nHARNESS GUARD: 0 tests ran — executor must declare modules in lib.rs and include test files."
    return True, combined


# ── file writer ───────────────────────────────────────────────────────────────
def write_files(response: str) -> list[str]:
    """Parse `// FILE: path` fenced blocks and write them."""
    written = []
    chunks = response.split("```")
    for chunk in chunks:
        if "// FILE:" not in chunk:
            continue
        lines = chunk.strip().splitlines()
        # first line may be lang tag (rust, toml, …) — skip it if no FILE:
        file_line = next((l for l in lines if "// FILE:" in l), None)
        if not file_line:
            continue
        rel = file_line.split("// FILE:")[1].strip()
        code_lines = [
            l for l in lines
            if "// FILE:" not in l and l not in ("rust", "toml", "")
        ]
        fp = ROOT / rel
        fp.parent.mkdir(parents=True, exist_ok=True)
        fp.write_text("\n".join(code_lines) + "\n")
        written.append(rel)
    return written


# ── git ───────────────────────────────────────────────────────────────────────
def git_commit(task_id: str, title: str) -> bool:
    r = subprocess.run(
        ["git", "add", "-A"],
        cwd=ROOT, capture_output=True
    )
    if r.returncode != 0:
        return False
    msg = f"task({task_id}): {title}"
    r = subprocess.run(
        ["git", "commit", "-m", msg],
        cwd=ROOT, capture_output=True
    )
    return r.returncode == 0


# ── task runner ───────────────────────────────────────────────────────────────
def run_task(task_path: pathlib.Path, use_alt: bool = False) -> bool:
    spec    = task_path.read_text()
    tid     = hashlib.blake2b(spec.encode(), digest_size=6).hexdigest()
    title   = task_path.stem
    executor = EXECUTOR_ALT if use_alt else EXECUTOR
    stamp   = datetime.now().strftime("%Y%m%d-%H%M%S")
    log     = LOGS_DIR / f"{task_path.stem}-{tid}-{stamp}.log"
    LOGS_DIR.mkdir(parents=True, exist_ok=True)

    entries = []
    def log_entry(label: str, text: str):
        entries.append(f"=== {label} ===\n{text}\n")
        log.write_text("\n".join(entries))

    log_entry("SPEC", spec)
    log_entry("META", f"task_id={tid}  executor={executor}  ts={stamp}")

    feedback = ""
    for attempt in range(1, MAX_RETRIES + 1):
        print(f"\n[{title}] attempt {attempt}/{MAX_RETRIES} ({executor})")

        prompt = (
            f"{EXECUTOR_SYS}\n\n"
            f"## TASK\n{spec}\n\n"
            f"## PRIOR FEEDBACK\n{feedback or 'none'}"
        )
        print(f"  → sending to {executor}…", end="", flush=True)
        code = ollama(executor, prompt, TEMP_CODE)
        print(" done")

        if code.strip().startswith("BLOCKED:"):
            print(f"  ✗ BLOCKED: {code.strip()}")
            log_entry(f"BLOCKED (attempt {attempt})", code)
            return False

        written = write_files(code)
        log_entry(f"GENERATED FILES (attempt {attempt})", "\n".join(written) or "(none parsed)")
        log_entry(f"RAW CODE (attempt {attempt})", code)

        if not written:
            feedback = "Your response contained no parseable `// FILE:` blocks. Emit code in fenced blocks starting with `// FILE: path`."
            log_entry(f"PARSE FAIL (attempt {attempt})", feedback)
            continue

        print("  → running harness…", end="", flush=True)
        ok, report = harness()
        log_entry(f"HARNESS (attempt {attempt})", report)
        print(" pass" if ok else " FAIL")

        if not ok:
            feedback = f"Build/test/clippy FAILED on attempt {attempt}:\n{report[-3000:]}"
            continue

        # harness passed — send to reviewer
        print(f"  → reviewing with {REVIEWER}…", end="", flush=True)
        review_prompt = (
            f"{REVIEWER_SYS}\n\n"
            f"## SPEC\n{spec}\n\n"
            f"## CODE\n{code}"
        )
        review = ollama(REVIEWER, review_prompt, TEMP_REVIEW)
        log_entry(f"REVIEW (attempt {attempt})", review)
        print(" done")

        if review.strip().upper().startswith("APPROVED"):
            print(f"  ✓ accepted on attempt {attempt}")
            committed = git_commit(tid, title)
            if committed:
                print(f"  ✓ committed: task({tid}): {title}")
            return True
        else:
            feedback = f"Reviewer REJECTED on attempt {attempt}:\n{review}"
            print(f"  ✗ reviewer rejected — feeding back")

    # primary exhausted — try alt executor once before giving up
    if not use_alt and executor != EXECUTOR_ALT:
        print(f"\n  → switching to alt executor {EXECUTOR_ALT} for one more pass")
        return run_task(task_path, use_alt=True)

    print(f"\n  ✗ {title} failed — task is too big or under-specified. Split it.")
    return False


# ── CLI ───────────────────────────────────────────────────────────────────────
def main():
    ap = argparse.ArgumentParser(description="ailang orchestrator")
    ap.add_argument("task", nargs="?", help="single task file to run (default: all in tasks/)")
    ap.add_argument("--dry-run", action="store_true", help="print tasks without running")
    ap.add_argument("--best-effort", action="store_true",
                    help="skip failed tasks and continue (unattended mode)")
    args = ap.parse_args()

    if args.task:
        tasks = [pathlib.Path(args.task)]
    else:
        tasks = sorted(TASKS_DIR.glob("*.md"))

    if not tasks:
        print("No tasks found in", TASKS_DIR)
        sys.exit(1)

    print(f"ailang orchestrator — {len(tasks)} task(s)"
          + (" [best-effort]" if args.best_effort else ""))
    for t in tasks:
        print(f"  {t.name}")

    if args.dry_run:
        sys.exit(0)

    failed = []
    for task in tasks:
        if not run_task(task):
            if args.best_effort:
                print(f"\n  ⚠ skipping {task.name} — continuing in best-effort mode")
                failed.append(task.name)
            else:
                print(f"\nStopping for human review — fix or split: {task.name}")
                sys.exit(1)

    if failed:
        print(f"\nAll tasks attempted. {len(failed)} failed (needs human review):")
        for name in failed:
            print(f"  ✗ {name}")
        sys.exit(1)

    print("\nAll tasks complete.")


if __name__ == "__main__":
    main()
