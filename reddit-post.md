# ailang — a programming language designed for AI, not humans

**TL;DR:** What if you stopped pretending AI would write code for humans to read, and instead designed a language from scratch around how AI actually works? That's ailang.

---

## The core idea

Every language you know — Python, Rust, TypeScript — was shaped by *human* constraints. We get tired. We can't hold hundreds of interconnected pieces in mind. So we invented keywords, indentation, files, and namespaces to help us manage complexity.

An AI has none of those limits. But it has a different one: **attention is the unit of cost.** Everything an AI has to "look at" costs tokens, compute, and latency.

So ailang asks the obvious question nobody has cleanly answered yet: *if the thing writing programs is an AI that thinks in structure, not text, what should the program actually be?*

The answer: **a typed, foldable, content-addressed node graph.**

---

## What that means in practice

A program is a graph of typed nodes wired together by typed edges. Types must match — no silent coercions. Any subgraph can be "folded" into a single node that exposes only its typed interface; the internal structure is hidden until you need it. Content-addressed means two nodes that do the same thing *are literally the same node* — free deduplication, caching, and diffing.

There is no human-readable source form. The AI edits the graph directly. Humans get *projections* (diagrams, summaries) when they need to understand what's happening — but never the source, because the source isn't text.

Effects are explicit and part of a node's type — not tucked away in a call stack. A node that touches the network declares `net` on its port. A node that calls an LLM declares `llm`. No hidden IO, ever.

Durability is a primitive, not a library. A workflow that waits 3 days is the same code as one that waits 3ms. Programs survive restarts and resume exactly where they left off.

Humans, LLMs, HTTP APIs, databases — these are base node types in the type system, not bolted-on libraries.

---

## Why now / why bother

Current AI coding tools (Copilot, Cursor, etc.) help AI write text that *looks like* programs. That's a reasonable transitional step. But the underlying representation is still text files designed for human eyes, and there's a ceiling on how good AI code generation can get when the output format is fundamentally the wrong shape for the generator.

ailang is a bet that the ceiling exists, and that removing it matters.

The V1 target is narrow: AI workflows and agent systems — not general-purpose apps. The goal is to prove the model works before expanding scope.

---

## How we're building it

Phase 0 (done today): Rust workspace, 6 crates. Orchestrator pipeline with local Ollama models: qwen3.6 as primary executor, devstral:24b as fallback, deepseek-r1:8b as adversarial reviewer. The orchestrator writes tasks as markdown specs → sends to executor → validates with `cargo build/test/clippy` → sends passing code to reviewer → commits on approval. Full retry logic, fallback executor, per-task logs.

**Current state:** 40 unit tests passing across the core crate. NodeId (blake3-content-addressed), Type system, Type::unify (sound static checker), Graph, EffectSet + linear capability tokens, node effect annotations, canonical JSON serialization — all complete and committed.

Compile target is WebAssembly. The first working programs will be graph → Rust → WASM via a transpiler; a native verifying compiler comes later, once the model is proven.

---

## The open question I'm genuinely wrestling with

The language has no human-readable form. That's a principled decision, not an oversight. But "principled" and "practically useful" aren't always the same thing, and I'm aware this is the part that makes most language designers uncomfortable.

The argument is: trust comes from a small, formally verified runtime and a type system, not from a human reading the source. The AI auditor is also a node type — you can attach formal verification or AI review as a step in the pipeline. If that's not convincing to you, I'd genuinely like to hear why.

---

## Links

- GitHub: https://github.com/ciscodesign/ailang
- Design spec: in the repo (`01-DESIGN.md`) — 3,000 words, covers the full architecture

Early days. Happy to answer questions or be told this is a terrible idea with reasons.

---

**Suggested subreddits:**
- r/ProgrammingLanguages (most appropriate — technically focused, serious)
- r/rust (building in Rust, orchestrator angle)
- r/MachineLearning (AI-native angle)
- r/programming (broadest reach)
