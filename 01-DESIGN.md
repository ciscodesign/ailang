# ailang — A Programming Language Built for AI, Not Humans

**Status:** Design v1.0 (pre-implementation). Name: *ailang* — see §13.
**One line:** A graph-native, foldable, formally-checkable programming language whose source of truth is a typed node-graph an AI edits directly — never text a human reads.

---

## 0. Read this first (plain English)

Every language you know — Python, JavaScript, Rust — was shaped by *human* limits. We get tired, we can't hold a hundred things in our head at once, so we invented keywords, indentation, and tidy little files we can scroll through. An AI has none of those limits, but it has a different one: it pays for everything it has to "look at" (tokens / attention / compute).

So ailang throws out the human assumptions and asks one question: *if the thing writing the program is an AI that thinks in structure, not in lines of text, what should the program actually be?*

The answer this document lands on: **the program is a graph of typed building blocks ("nodes") wired together, and any cluster of blocks can be "folded" into a single block.** The AI only ever looks at the level it's working on; everything else stays folded away and costs nothing. It compiles down to WebAssembly so it can run fast and safely on the web, desktop, and (eventually) mobile.

If the rest of this doc gets dense, that paragraph is the whole idea. Everything else is detail.

---

## 1. The premise

| Human languages assume… | …but an AI actually has |
|---|---|
| Limited working memory → needs names, comments, structure | Effectively perfect recall + the whole program in context |
| Reading fatigue → whitespace, formatting | No fatigue; density is fine |
| Cost ≈ keystrokes | Cost ≈ **attention/tokens** — a totally different metric |
| Ambiguity, fixed later | Can emit a fully-specified, checkable artifact in one go |
| Style is subjective | One canonical form → perfectly diffable, cacheable, dedup-able |

**Conclusion:** stop optimizing for "readable." Optimize for *minimum attention to express correct, safe intent* — and let the structure of the program, not its text, carry the meaning.

---

## 2. What we're building (the decisions, in one place)

| Decision | Choice | Why |
|---|---|---|
| Audience | **AI only.** No human-readable source form. | Frees us from text/syntax entirely. Humans get *projections* (diagrams, summaries) when they need them, never the source. |
| Target AI | **Future AI** that emits/edits a graph directly. | No tokenizer in the loop. Efficiency comes from structure, not character-shaving. |
| Source representation | **Typed, foldable, content-addressed node graph.** | Folding = abstraction + efficiency + security, all at once. |
| Speed | Compile to **WebAssembly** (+ native AOT later). | Near-native, sandboxed, portable. |
| Platforms | Web → Desktop → Mobile (in that order). | WASM is the portability spine; UI is the hard part, deferred. |
| Security | Capabilities + tiny trusted verifier + proof-carrying code + AI auditors. | No human reads the code, so trust moves to math + a small audited core. |
| "Bug-free" | Honestly: **memory-safe, type-safe, deterministic, contract-checked, verifiable.** | Truly bug-free is a fantasy; this is the achievable, real version. |
| Build approach | **Transpile-first:** graph → Rust → WASM now; native verifying compiler later. | Working artifact in weeks, not a 2-year compiler death-march. |
| v1 scope | **Narrow: AI-systems / workflows**, then grow into full apps. | Scope creep kills new languages. Prove the model first. |
| Priority order | Security > Bug-resistance > Reach > Speed > Raw efficiency. | Safety is unrecoverable if wrong; efficiency mostly falls out of folding for free. |

---

## 3. Design principles

1. **Attention = the unit of cost.** If a construct costs the AI attention but adds no meaning, it dies.
2. **One canonical form.** No style options. The same program is byte-identical every time → perfect diffs, caching, deduplication.
3. **Intent-first / declarative core.** Say *what*, let the compiler decide *how*. Fewer moving parts, fewer bugs.
4. **Pure core, effects at the edges.** Dangerous stuff (IO, network, time, randomness) lives in a thin, typed, explicit shell.
5. **Contracts are first-class**, not comments.
6. **No human face.** The source is a machine graph. Humans get *views*, never the source. Trust comes from the verifier, not from reading.
7. **The program is a typed foldable graph, not text.** Abstraction = collapse a subgraph into one node with a typed interface. Content-addressed, so identical subgraphs dedup.
8. **Durability is a primitive, not a library.** A step that waits 3 days is the same code as one that waits 3ms. Programs survive crashes and resume exactly where they paused. (Inspired by Weft, §12.)
9. **Humans, LLMs, APIs, infrastructure are base node types**, not imported plumbing. "Ask a human and wait" is one node, governed by a capability like everything else.

---

## 4. The graph model (the heart of it)

> **Plain English:** a program is boxes connected by wires. Each box has typed input and output sockets. Wires only connect if the types match. Boxes that do something "dangerous" (touch the network, write a file) must declare it on the box itself — it can't be hidden. And you can shrink-wrap any group of boxes into a single new box. That's it.

### 4.1 Node — the building block
A node has:
- **Identity** = a content hash of its full definition. Two nodes that do the same thing *are* the same node (free dedup + caching).
- **Kind** = a *primitive* (built-in: `LLM`, `HTTP`, `DB`, `Human`, `Code`, `Gate`, `Timer`, …) or a *fold* (a subgraph — recursion).
- **Typed ports** = named, typed inputs and outputs.
- **Effects/capabilities** = what it's allowed to touch (`net`, `db`, `fs`, `human`, `clock`, `rand`). **This is part of its type** — there is no hidden IO.
- **Contract** (optional) = preconditions, postconditions, invariants.

*Human-readable sketch (NOT the real form — the real form is binary):*
```
node Poet : LLM
  in  prompt : Text
  in  cfg    : LlmConfig
  out reply  : Text
  fx  «llm»
  pre  prompt.len > 0
  post reply.len > 0
  id   blake3:7c1a…
```

### 4.2 Typed edge — the wire
Connects one output port to one input port. Legal only if the types unify (with generics, unions, type variables, and option/null propagation). A missing or mistyped connection is a **compile error at the graph level**, before anything runs.

```
Topic.value ──▶ Poet.prompt     # Text ▶ Text  ✓
Cfg.config  ──▶ Poet.cfg        # LlmConfig ▶ LlmConfig  ✓
```

### 4.3 The effect token — how order and determinism happen
Pure nodes have **no inherent order** — the compiler runs them however it likes (free parallelism). Effectful nodes get ordered by threading a **linear capability token** through them: there is exactly one `net` token, it must be used once and passed on, so two network calls *cannot* run in an undefined order.

```
⟨net⟩ ──▶ FetchA ──⟨net⟩──▶ FetchB ──⟨net⟩──▶ …
```

The payoff: **determinism and effect-ordering become typing rules, not conventions.** A huge slice of "no bugs" falls out of the type system for free.

### 4.4 Fold — abstraction, efficiency, and security in one move
Collapse any subgraph into a single composite node:
```
fold { Topic, Cfg, Poet, Save } as Assistant
  interface:
    in  topic : Text
    out saved : RecordId
    fx  «llm, db»          # = UNION of inner effects, automatically bubbled up
    pre  topic.len > 0
  id  blake3:9f3a…         # = hash of the whole subgraph
```

One mechanism, four wins:
- **Efficiency / attention:** at the top level the AI sees only `Assistant`'s interface. The innards stay folded behind a hash and cost nothing until opened. *This is the token-efficiency story.*
- **Security at a glance:** effects bubble up, so a folded 100-node system still honestly advertises `«llm, db»` at the top. You can't bury a network call three folds deep.
- **Dedup:** identical subgraphs → same hash → stored once.
- **Recursion:** folds contain folds. A 100-node program is 5 blocks at the top, each opening into 5 more.

### 4.5 Tiny worked example — a whole "app", folded
```
Topic:Text ─┐
            ├─▶ Poet:LLM ──▶ reply:Text ──▶ Save:DB ──▶ RecordId
Cfg:LlmConfig┘                                   ⟨db⟩ threaded in
```
Fold it → one `Assistant` node: `in topic:Text → out saved:RecordId, fx«llm,db»`. Drop *that* into a bigger graph and it's a single block. That's the entire ergonomic story.

### 4.6 Open sub-questions
- Control flow: explicit control edges, or `Gate`/`Match` nodes + pure dataflow? *(Lean: nodes only.)*
- Generics on a fold's interface: type-variables on boundary ports.
- Streaming/async values (an LLM token stream): a port type or a node kind?
- Multi-node contracts: live on the fold boundary only?

---

## 5. Type & effect system
- Sound static typing: generics, unions, type variables, option types (no `null`), `Result`-style error values (no unchecked exceptions).
- Effects are types: a node's signature includes the capabilities it uses. Folds union them upward.
- Total core: pattern matches must be exhaustive; partial functions are rejected.
- Inference everywhere; explicit types only where they add information.

---

## 6. Security model
- **Default deny.** A program starts with zero capabilities. It can only touch what's explicitly handed in at entry (`net`, `fs(scope)`, `db`, `clock`, `rand`, `human`, `ui`).
- **Capabilities are unforgeable + linear** — visible in every signature, impossible to fabricate, impossible to hide.
- **Sandboxed target (WASM):** no ambient access to OS, disk, or network.
- **No `eval`, no dynamic code loading, no FFI** without an explicit capability.
- **Supply chain:** every dependency is content-addressed + hashed. No floating "latest", no install-time code execution.

### Where trust comes from (the key shift)
Humans can't read ailang, so "I audited the code" is off the table. Trust relocates to three things humans *can* vouch for:
1. **The verifier/compiler is the trust anchor** — small, fixed, audited. Humans trust *it*, not the oceans of AI-generated code it checks. Keep it tiny enough to formally verify.
2. **Proof-carrying code** — the AI emits code *and* the proof obligations; the verifier discharges them. You trust the math, not the author. Illegible code + a valid proof of "never escapes its sandbox" is still trustworthy.
3. **AI auditors** — other models red-team the graph. Cheap, scalable, and they don't need readability either.

**Honest caveat:** this concentrates all trust in the verifier. If it's wrong, nothing saves you — so the verifier stays small enough that humans can formally verify *it*. One turtle deep, not infinite turtles.

---

## 7. Execution model
- **Durable executor** (inspired by Restate/Weft): programs survive crashes and restarts; a multi-day human-wait is the same code as a millisecond call; state is the runtime's problem, not the AI's.
- **Compile pipeline:** graph → typecheck/effect-check/contract-check/architecture-check → IR (optimizer exploits purity + graph structure) → WASM.
- **Targets:** browser (web), Wasmtime / native AOT (desktop), WASM-on-mobile runtime (mobile).

---

## 8. The "bug-free" reality (manage expectations)
Nothing is truly bug-free; promising that is how you ship the bug. What ailang actually delivers:
- **Memory-safe** (managed model + WASM).
- **Type-safe + total core** (no null, no unchecked exceptions, exhaustive matches).
- **Deterministic** (effects ordered by linear tokens).
- **Contract-checked** (specs at compile and/or run time).
- **Verifiable core** (a subset where proofs are discharged automatically).

The honest, sellable claim: **"memory-safe, type-safe, sandboxed, deterministic, contract-checked, and verifiable."**

---

## 9. Cross-platform / apps (the genuinely hard part)
WASM does logic, not buttons. UI is ~80% of the eventual engineering and is **deliberately deferred past v1**. Options when we get there:
- (a) Declarative UI tree emitted by the program; per-platform renderers. Most portable, most work.
- (b) Target an existing cross-platform renderer.
- (c) Web-only first (DOM/Canvas), expand later. ← likely starting point.

---

## 10. Scope
- **v1 (now):** the graph model, type/effect system, durable executor, transpile-to-Rust→WASM, a small opinionated node catalog (LLM, HTTP, DB, Code, Gate, Human, Timer), a minimal web view for humans to *see* a program. Headless logic / AI-workflows.
- **Later:** native verifying compiler, proof-carrying code, full cross-platform UI, AI-defines-new-nodes-in-the-language-itself, package/registry.

---

## 11. Risks / things that will bite us
- **Betting on future AI** that fluently edits large typed graphs. If that lags, we fall back to a text rendering and the density story weakens.
- **No ecosystem.** New language = zero libraries. Interop/FFI story is make-or-break (it kills more languages than syntax ever does).
- **UI is the iceberg**, not the language core.
- **All trust in the verifier** — it must be tiny and audited.
- **Verification doesn't scale for free** — "verified core, trusted edges" split is mandatory.

---

## 12. Prior art — Weft
`github.com/WeaveMindAI/weft` — a 2026, build-in-public language for "AI systems." Young and explicitly unfinished, but real and overlapping. License: O'Saasy (fine to borrow *ideas*; reusing *code* in a hosted product is restricted — we take ideas only).

**Borrowed:** foldable typed graph as the program model; durable execution as a primitive; humans/LLMs/APIs/infra as first-class nodes; "one program, many renderings"; end-to-end typing with compile-time architecture checks.

**Diverged:** Weft keeps a human graph view (we go AI-only); Weft targets workflow orchestration (we want compiled cross-platform apps); no formal-verification / proof-carrying / verifier-as-trust-anchor in Weft (that's ours).

---

## 13. Naming
**ailang** — decided.

---

## 14. Glossary
- **Node** — a typed building block (a box).
- **Port** — a typed input/output socket on a node.
- **Edge** — a typed wire connecting an output port to an input port.
- **Effect / capability** — permission to touch the outside world (network, disk, etc.), declared in the type.
- **Effect token** — a linear value threaded through effectful nodes to force deterministic order.
- **Fold** — collapsing a subgraph into one composite node.
- **Content-addressed** — identity = hash of contents; identical things share identity automatically.
- **Proof-carrying code** — code shipped with a machine-checkable proof of a safety property.
- **Verifier** — the small, trusted program that checks everything; the root of trust.
