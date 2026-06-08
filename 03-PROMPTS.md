# ailang — Prompt Library & Prompting Best Practices

The operational prompts live as separate files (so the harness can load them directly):

- `prompts/controller.md` — Claude's role: decompose, spec, test-first, review, integrate.
- `prompts/executor.md` — the Ollama coding model: implement exactly one task, strict output format.
- `prompts/reviewer.md` — a *different* model: adversarial red-team of passing code.
- `prompts/spec-template.md` — the task format + a worked Phase 0 example.

> In the repo, copy these into `orchestrator/prompts/` (where the harness in 02-BUILD-ORCHESTRATION.md expects them) — or just point the harness at this folder.

---

## Why this split works
- **Separation of judgment and labor.** The controller (taste, architecture, safety) is a strong reasoning model; the executor (volume) can be a cheaper local model. You spend big-model budget only where it matters.
- **Verification beats trust.** The compiler/tests are the ground truth; no model's confidence overrides them. This mirrors ailang's own "trust the verifier, not the author" philosophy.
- **Diversity catches blind spots.** A model is bad at reviewing its own output. Different families fail differently — pair them.

## Prompting best practices (for the executor especially)
1. **Constrain the output format hard.** The `// FILE: path` convention lets a dumb script route code with zero parsing intelligence. Ambiguous output = broken pipeline.
2. **Low temperature for code (0.1–0.3).** You want correctness and reproducibility, not creativity.
3. **Tests in the prompt.** Give the executor the exact tests it must pass. It turns a fuzzy "implement X" into a crisp "make these green."
4. **Minimal context, maximal relevance.** Don't paste the whole design doc — paste the one §and the already-built types the task touches. Irrelevant context is noise that degrades small models fast.
5. **Feed failures back verbatim.** On a retry, give the executor the actual compiler error or reviewer critique, truncated to the relevant tail. Don't paraphrase errors.
6. **One responsibility per prompt.** The executor implements; it does not also decide design. The reviewer critiques; it does not rewrite. Mixed roles = mush.
7. **Make "I'm stuck" a first-class output.** `BLOCKED:` (executor) and escalation (controller) are features. A model guessing past ambiguity is how silent bugs are born.
8. **Pin everything.** Model name, temperature, prompt version. A reproducible pipeline is a debuggable pipeline.

## A note on autonomy
The harness can run unattended *within a phase*. It should **stop at phase boundaries** and on repeated task failure. Full lights-out autonomy across the whole language is not the goal — the human approves the shape, the loop fills it in. Build mode is automatic; design mode is not.
