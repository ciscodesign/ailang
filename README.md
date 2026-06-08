# ailang

**A programming language built for AI, not humans.**

Most languages were shaped by human limits — memory, fatigue, the need to read line by line. ailang starts from a different question: *if the thing writing the program is an AI that thinks in structure, what should the program actually be?*

The answer: a program is a **typed graph of building blocks**, and any cluster of blocks **folds** into a single block. The AI only ever looks at the level it's working on; everything else stays folded and costs nothing. It compiles to **WebAssembly** so it runs fast and sandboxed on web, desktop, and (later) mobile. No human ever reads the source — humans get diagrams and a verifier they can trust instead.

> **Status:** design complete, pre-implementation. Building in public.

---

## What's in here

| Path | What it is |
|---|---|
| `docs/00-WORKING-NOTES.md` | The original scratchpad / decision log — how we got here. Background reading. |
| `docs/01-DESIGN.md` | The full design. Start here. Has a plain-English intro before the dense parts. |
| `docs/02-BUILD-ORCHESTRATION.md` | How we build it: Claude as controller, Ollama models as executors, with a runnable harness. |
| `docs/03-PROMPTS.md` | The prompt library explained + prompting best practices. |
| `prompts/` | The actual system prompts (controller, executor, reviewer) + task template. |
| `site/` | The human-facing landing page (`index.html`) — what we'll link from GitHub. |
| `deck/` | The pitch deck. |

## The 60-second version

- **Source of truth is a graph, not text.** Nodes (typed building blocks) connected by typed edges. Wires only connect if types match.
- **Folding** = collapse a subgraph into one node. This is abstraction, attention-efficiency, dedup, and security-at-a-glance, all in one move.
- **Effects live in the type.** A node that touches the network *says so*. No hidden IO. Effect ordering comes from a linear token, so determinism is a typing rule.
- **Trust moves to the verifier.** No human reads the code, so trust comes from a tiny audited verifier + machine-checkable proofs + AI auditors — not from reading.
- **Honest about "bug-free":** memory-safe, type-safe, deterministic, contract-checked, verifiable. Not magic.

## How it gets built

Claude is the **controller** (architecture, tiny task specs, acceptance tests, review). Local **Ollama** models are the **executors** (they write the code). A dumb **harness** runs `cargo build`/`test` and is the only judge of facts. Nothing is accepted until it compiles, passes tests, and survives an adversarial review by a *different* model. See `docs/02-BUILD-ORCHESTRATION.md`.

## Roadmap

0. **Foundations** — graph data model, types, content-addressing.
1. **Walking skeleton** — effects, primitive nodes, fold/unfold, durable executor, graph→Rust→WASM. *Runs in a browser.*
2. **Safety** — capabilities, contracts, architecture checks, reviewer-in-CI.
3. **UI / apps** — web-first cross-platform demo. (Beyond where prior art stops.)
4. **Verification** — proof-carrying nodes, shrink the trusted compiler.
5. **Ecosystem** — AI defines new nodes in the language itself; FFI; registry.

## Prior art & credit

The graph-native + recursively-foldable + durable-execution direction is shared with **[Weft](https://github.com/WeaveMindAI/weft)** (© Quentin Feuillade--Montixi), a build-in-public language for AI systems. We borrow *ideas* (foldable typed graphs, durability as a primitive, humans/LLMs/APIs as first-class nodes), not code. Where we diverge: ailang is AI-only (no human source view), targets compiled cross-platform apps, and adds a formal-verification / proof-carrying / tiny-trusted-verifier security model.

## License

TBD before going public (consider permissive vs. source-available). Add a `LICENSE` file when the repo is created.
