# Project "ailang" — A Programming Language Built for AI, Not Humans

*(codename, totally negotiable — see §10)*
*Status: v0.1 — living document. Throw new ideas in §12 and we promote them later.*

---

## 0. The one-paragraph pitch

Every mainstream language (Python, JS, Rust, etc.) is a compromise shaped by **human** limitations: we get tired, we can't hold 50 things in our head, we need verbose keywords and whitespace to not get lost. An AI has none of those constraints but a *completely different* one: it pays per **token**. So we design a language where the unit of cost is the token, the unit of meaning is the token, and the whole thing compiles down to something fast, sandboxed, and portable (WebAssembly). Dense to emit, safe to run, painful to write by hand — which is fine, because no human is supposed to.

---

## 1. Why bother (the premise)

| Human languages assume | An LLM actually has |
|---|---|
| Limited working memory → need names, comments, structure | Effectively perfect recall of grammar + full file in context |
| Reading fatigue → whitespace, formatting | No fatigue; density is free |
| Typing cost ≈ keystrokes | "Typing cost" ≈ **tokens**, a totally different metric |
| Ambiguity tolerated, fixed later | Can emit a fully-specified canonical form first try |
| Style is subjective | One canonical form = perfectly diffable + cacheable |

So the design target isn't "readable." It's **minimum tokens to express correct, safe intent.**

---

## 2. Hard requirements — decoded, with a reality check on each

### 2a. "Incredibly few tokens" — the headline feature
The non-obvious trap: **fewer characters ≠ fewer tokens.** LLM tokenizers (BPE) merge *frequent* sequences into single tokens. Common English words and code patterns are often 1 token each; weird symbol soup (`}=>{|>>`) can tokenize to *one token per character* — the opposite of what we want. APL-style hieroglyphics would actually be a disaster on a standard tokenizer.

Two honest paths:
- **Path A (tokenizer-agnostic):** primitives are chosen to already be single high-frequency tokens. Surface syntax ends up looking more like terse pidgin-English than symbol soup. Portable across models.
- **Path B (co-designed tokenizer):** ship the language *with* a custom tokenizer/vocabulary where every keyword and primitive is exactly 1 token. Maximum density, but locks us to models we can retokenize for.

→ **UPDATE — target is FUTURE AI (DECIDED §8), which changes this entirely.** If the AI emits a *structured graph* directly instead of a text token-stream, there's **no tokenizer in the loop at all**. The character/token problem evaporates. Efficiency now comes from a different, better place:

- **The program IS a typed, foldable graph** (not text). The canonical form is a content-addressed node graph. (Prior art: Weft, see §11 — it proves graph-native + folding works.)
- **Folding = the token-efficiency mechanism.** Any subgraph collapses into a single node with a typed interface. The AI working at the top level only "loads" (pays attention/compute for) the 5 blocks it sees; the 95 nodes inside stay folded behind a hash. *Deep detail is free until you open it.* This is a far bigger win than shaving characters ever was.
- **Content-addressed nodes** → identical subgraphs stored once, referenced by hash everywhere. Massive dedup across a whole codebase.
- Path A/B (custom text tokenizer) survives only as a **fallback rendering** for today's text-based models. Not the primary form anymore.

Other efficiency wins (still apply):
- One token = one rich intent; the compiler expands it ("macro at the speed of meaning"). `srv` → a whole typed HTTP server skeleton.
- No imports section; symbols resolve from a content-addressed global index.
- No formatting/whitespace tokens (canonical binary-ish surface form).
- Implicit, inferred types — declared only when they *add* information.

### 2b. "Fast"
Compile to **WebAssembly** (near-native, runs everywhere) with an optional native AOT path for desktop. Pure core enables aggressive optimization (no aliasing surprises).

### 2c. "Web, mobile, desktop"
WASM is the portability spine. The genuinely *hard* part isn't compute — it's **UI** (WASM has no native widgets). See §4. This is the riskiest requirement, not the token stuff.

### 2d. "Secure like hell"
- Compiles to a **sandbox** (WASM) — no ambient access to disk/network/OS by default.
- **Capability-based security:** code can only touch what's explicitly handed to it. No global `fs`, no `import os`. If a function didn't receive the network capability, it *cannot* make a request. Period.
- No `eval`, no dynamic code loading, no FFI without an explicit capability.
- Effects are typed (see 2e), so "this function secretly writes files" is a *type error*, not a surprise.

### 2e. "Bug-free" — let's be honest about this one
Nothing is truly bug-free; promising that is how you ship the bug. What's *actually* achievable and worth chasing:
- **Memory-safe** (no manual memory; WASM + managed model).
- **Type-safe + total core** (functions must handle all cases; no `null`, no unchecked exceptions — `Result`/option types instead).
- **Deterministic** (same input → same output; effects quarantined at the edges).
- **Contracts/specs** attached to functions, checked at compile time and/or runtime.
- **Formally verifiable core** — a subset where the AI emits code *and* a proof obligation the compiler discharges.
So the real claim is: **"memory-safe, type-safe, sandboxed, contract-checked, and verifiable."** That's the honest, sellable version.

---

## 3. Core design principles

1. **Token = semantic unit.** If a construct costs tokens but adds no meaning, it dies.
2. **One canonical form.** No style options, ever. Every AI emits byte-identical output for the same program → perfect diffs, caching, dedup.
3. **Intent-first / declarative core.** Say *what*, let the compiler decide *how*. Fewer tokens, fewer bugs (less imperative state to corrupt).
4. **Pure core, effects at the edges.** The dangerous stuff (IO, net, time, randomness) lives in a thin typed shell.
5. **Contracts are first-class**, not comments.
6. **No human face.** *(DECIDED)* The canonical form is whatever's densest for the machine. Trust does **not** come from humans reading the code (see §5).
7. **The program is a typed foldable graph, not text.** *(DECIDED — target is future AI.)* Nodes + typed edges. Abstraction = collapse a subgraph into one node with a described interface. Content-addressed, so identical subgraphs dedup. Text is at most an optional fallback rendering, never the source of truth.
8. **Durability is a primitive, not a library.** *(borrowed from Weft)* A step that waits 3 days for an external event is the same code as one that waits 3ms. Programs survive crashes and resume exactly where they paused. State management isn't the AI's problem — it's the runtime's.
9. **Humans, LLMs, APIs, infra are base node types**, not imported plumbing. *(borrowed from Weft)* "Ask a human and wait" is one node with a typed in/out, governed by a capability like everything else.

---

## 4. Architecture sketch

```
AI emits/edits ─► typed foldable node-graph (content-addressed)   ← the source of truth, no text
              ─► type check + effect check + contract/proof check + architecture check
              ─► IR (optimizer: purity + graph structure = aggressive)
              ─► WASM core  ──► browser (web)
                             ──► Wasmtime / native AOT (desktop)
                             ──► WASM-on-mobile runtime (mobile)
              + durable executor (crash-survival, human-wait, resume)
              + UI layer (the hard part) ──► DOM/Canvas (web)
                                          ──► native widget bridge (desktop/mobile)
```

**The UI problem (flag this loudly):** WASM does logic, not buttons. Options:
- (a) Declarative UI tree emitted by the program; host renders it per platform (think "one UI spec → many renderers"). Cross-platform but we build all the renderers.
- (b) Lean on an existing cross-platform renderer and just target it.
- (c) Web-only first (DOM/Canvas), expand later.
This is where most of the *actual* engineering budget goes. Token efficiency is the easy 20%.

---

## 5. Security model (expanded)

- **Default deny.** A program starts with zero capabilities.
- Capabilities are unforgeable tokens passed in at entry (`net`, `fs(scope)`, `clock`, `rand`, `ui`).
- Effect types make capability use visible in signatures → static audit.
- Supply chain: every dependency is content-addressed + hashed; no "latest" floating versions, no install-time code execution.

**Where trust comes from now (this is the key shift).** Humans can't read ailang, so "I audited the code" is off the table. Trust moves to three places a human *can* still vouch for:
1. **The verifier/compiler is the trust anchor.** Humans trust a small, fixed, audited compiler — not the millions of lines of AI-emitted code it checks. Keep the trusted core *tiny*.
2. **Code ships with machine-checkable proofs.** The AI emits proof obligations alongside the code; the verifier discharges them. You trust the *math*, not the author. Illegible code + a valid proof of "never escapes its sandbox" is still trustworthy.
3. **AI auditors.** Other models review/red-team the code. Cheap, scalable, and they don't need it to be human-readable either.

→ Honest caveat: this concentrates ALL trust in the verifier. If the verifier is wrong, nothing else saves you. So the verifier itself stays small enough for humans to formally verify *it*. Turtles, but only one turtle deep.

---

## 6. Open questions — **need your call** (§8 = the steering wheel)

1. **Which AI?** Optimize for *today's LLMs* (token streams, BPE) or a hypothetical future AI that could emit structured/binary ASTs directly? Hugely changes the design.
2. **Human-auditable, or AI-only?** Full density vs. keeping a readable face for trust/security. (I strongly recommend keeping the readable face.)
3. **Greenfield or transpile?** Build a real compiler, or first transpile to an existing fast/safe target (e.g. Rust→WASM) to get a working prototype in weeks instead of years?
4. **Priority order.** These requirements *conflict.* If I had to rank: I'd guess **Security > Bug-resistance > Cross-platform > Speed > Token-tininess**. Tell me if that's wrong — it changes everything downstream.
5. **Scope of "apps."** Full GUI apps, or is "headless services + simple web UI" enough for v1?

---

## 7. Risks / things that will absolutely bite us

- ~~Tokenizer lock-in~~ → **mostly retired** by going graph-native/future-AI. New risk instead: **betting on a future AI** that can fluently emit/edit large typed graphs directly. If that capability lags, we fall back to a text rendering and the density story weakens.
- **No ecosystem.** New language = zero libraries. Need a great FFI or interop story or it's a toy. (Underrated: this kills most new languages, not syntax.)
- **UI is 80% of the work**, not the language core.
- **"Bug-free" overpromise** — manage expectations or get burned.
- **Density vs. auditability** is a permanent tradeoff; we manage it, we don't "solve" it.
- **Verification doesn't scale for free** — full formal proofs are expensive; we'll need a "verified core, trusted edges" split.

---

## 8. Decisions log (fill in as we go)
- [x] **Human-readable face: NO** — AI-only. Unlocks Path B (custom 1-token vocabulary) + shifts trust to verifier/proofs/AI-auditors.
- [ ] Path A vs B → **leaning B (decided above)**, keep A as portable fallback
- [x] **Target AI: FUTURE AI** — emits/edits the graph directly, no tokenizer. Canonical form = typed foldable graph, not text.
- [x] **Source representation: graph, not text** — folding is the efficiency mechanism; content-addressed nodes dedup.
- [x] **Build approach: TRANSPILE-FIRST.** Graph → Rust → WASM to get a running artifact in *weeks*, not years. We inherit Rust's memory safety + the WASM target for free. A native verifying compiler comes later (Phase 4), once the model's proven.
- [x] **v1 scope: NARROW — AI-systems/workflows first.** Headless graph logic + a minimal web view (à la Weft). Full cross-platform GUI apps are deferred to Phase 3+. Reason: scope creep is what actually kills new languages; prove the graph model before fighting the UI dragon.
- [x] **Requirement priority order** *(my call — trivially flippable):* **Security > Bug-resistance > Cross-platform reach > Speed > Raw token-efficiency.** Efficiency drops to last because folding gives most of it for free now; safety is the thing humans can't recover from if we get it wrong.

---

## 9. Naming brainstorm (because vibes matter)
- **ailang** — graph-native, AI-native. (decided)
- **CANT** — as in jargon/cant, a private language. Cheeky.
- **LUME / QUILL / CORTEX / SYNTH**
- **Æ** — pretentious, two chars, lol
- **TACL** — Token-Aligned Compiled Language (boring but descriptive)

---

## 10. Phased roadmap (proposed)
- **Phase 0 — Spec & decisions.** Answer remaining §8. Define the core graph model: node types, typed edges, folding/interface rules, content-addressing scheme.
- **Phase 1 — Walking skeleton.** Build the graph model + a durable executor; transpile the graph to Rust→WASM so it actually runs. Prove "AI emits a graph → folds cleanly → runs in browser." *Borrow Weft's executor approach (Restate-style durability) as a reference, don't reinvent it.*
- **Phase 2 — Safety.** Effect system + capabilities + contracts + architecture checks at the graph level.
- **Phase 3 — UI.** Pick a UI strategy (§4), ship one cross-platform demo app. (This is where we go *beyond* Weft, which stops at workflows.)
- **Phase 4 — Verification.** Add the verified core + proof-carrying nodes; shrink the trusted compiler.
- **Phase 5 — Ecosystem/interop.** Let the AI define new node types in the language itself (Weft's stated end-goal too); FFI; content-addressed node registry.

---

## 11. Prior art — Weft (and what we steal vs. leave)
Repo: `github.com/WeaveMindAI/weft` — a 2026, ~2-star, build-in-public language for "AI systems." Young and explicitly unfinished, but the design overlaps with ours enough to be a real reference. (License: O'Saasy — fine for borrowing *ideas*; reusing *code* in a hosted product is restricted. We take ideas.)

**Steal:**
- Recursively foldable typed graph as the program model — the abstraction mechanism *and* (for us) the efficiency mechanism.
- Durable execution as a primitive (crash-survival, multi-day waits, resume).
- Humans / LLMs / APIs / infra as first-class node types instead of imported plumbing.
- "One program, multiple renderings" — the stored form is abstract; views are projections. (We just drop the human view.)
- End-to-end typing with compile-time architecture validation.

**Leave / where we diverge:**
- Weft keeps a human graph view; **we go AI-only**, no human source form.
- Weft targets workflow/agent orchestration, not compiled cross-platform **apps**; our WASM + UI goal is out of its scope.
- No formal-verification / proof-carrying / verifier-as-trust-anchor story in Weft; that's ours.
- Token-minimality isn't its headline; folding-for-efficiency is our spin on its folding-for-abstraction.

**Worth a closer look later:** their `DESIGN.md`, `ROADMAP.md`, and the node `catalog/` layout (two files per node: backend + frontend) — a clean model for how "rich primitive" nodes are defined.

---

## 12. Core graph model — first sketch (v0.1)

> ⚠️ **The real form is a binary, content-addressed graph.** The notation below is a *human crutch* so we can talk about it today — the future AI never sees this text, it manipulates the graph directly. We're sketching the *semantics*, not a syntax.

### The three primitives

**1. Node** — the atomic unit of computation.
```
node Poet : LLM                 # kind = built-in primitive (or a fold, see below)
  in  prompt : Text             # typed input ports
  in  cfg    : LlmConfig
  out reply  : Text             # typed output ports
  fx  «llm»                     # effects/capabilities it needs (part of its type!)
  pre  prompt.len > 0           # contract: precondition
  post reply.len > 0            # contract: postcondition
  id  blake3:7c1a…              # identity = hash of the node's full definition
```
- **Kind** is either a *primitive* (LLM, HTTP, DB, Human, Code, Gate, Timer…) or a *fold* (a subgraph — recursion).
- **Effects are in the interface.** A node that touches the network *says so* in its type. No hidden IO, ever.
- **Identity = content hash.** Two nodes that do the same thing *are* the same node. Free dedup + caching.

**2. Typed edge** — connects one output port to one input port.
```
Topic.value ──▶ Poet.prompt      # legal iff types unify:  Text ▶ Text ✓
Cfg.config  ──▶ Poet.cfg         # LlmConfig ▶ LlmConfig ✓
```
- Types must unify (with generics, unions, type-variables, option/null propagation — the usual sound machinery). A missing or mistyped connection is a *compile error at the graph level* before anything runs.
- **Pure nodes are unordered** — the compiler schedules them however it likes (great for parallelism). So how do we order effects deterministically? →

**3. The effect token (the clever bit)** — ordering falls out of the type system, not out of line-order.
```
⟨net⟩ ──▶ FetchA ──⟨net⟩──▶ FetchB ──⟨net⟩──▶ …
```
- A capability like `net` or `db` is a **linear token**: it must be used exactly once and threaded onward. Two network calls *can't* run in an undefined order, because the single `net` token flows through one, then the next.
- Determinism and effect-ordering become a *typing rule*, not a convention. (Borrowed in spirit from linear types / world-token IO.) This is a big chunk of the "bug-free" claim, for free.

### Fold — abstraction *and* efficiency in one move
Collapse any subgraph into a single composite node:
```
fold { Topic, Cfg, Poet, Save } as Assistant
  interface:                    # = the subgraph's unconnected boundary ports
    in  topic : Text
    out saved : RecordId
    fx  «llm, db»               # = UNION of inner effects, bubbled up automatically
    pre  topic.len > 0          # derived/declared boundary contract
  id  blake3:9f3a…              # = hash of the whole subgraph
```
What folding buys us, all at once:
- **Efficiency / attention:** at the top level the AI sees `Assistant`'s *interface only* — ports, types, effects, contract. The innards stay folded behind a hash and cost nothing until opened. *This is our token-efficiency story.*
- **Security, visible at a glance:** because effects bubble up, a folded 100-node system still honestly advertises `«llm, db»` at the top. You can't hide a network call three folds deep.
- **Dedup:** identical subgraphs → identical hash → stored once, cached forever.
- **Recursion:** folds contain folds. A 100-node program is 5 blocks at the top, each of which opens into 5 more. Turtles, but navigable ones.

### Tiny worked example (a whole "app", folded)
```
Topic:Text ─┐
            ├─▶ Poet:LLM ──▶ reply:Text ──▶ Save:DB ──▶ RecordId
Cfg:LlmConfig┘                                   ⟨db⟩ threaded in
```
Fold it → one `Assistant` node: `in topic:Text → out saved:RecordId, fx«llm,db»`.
Drop *that* into a bigger graph and it's a single block. That's the entire ergonomic story.

### Open sub-questions for the model
- Do we need explicit **control edges** (branching/loops) or can `Gate`/`Match` nodes + dataflow cover it? (Lean: nodes, keep edges pure-dataflow.)
- How are **generics** represented on a fold's interface — type-vars on boundary ports?
- **Streaming/async** values (an LLM token stream) — a port type, or a node kind?
- Where do **contracts** that span multiple nodes live — on the fold boundary only?

---

## 13. Parking lot (raw ideas, unsorted — promote the good ones)
- Programs as content-addressed graphs → identical sub-functions stored once → near-free dedup across an entire codebase.
- The AI emits *both* code and its proof obligations in one pass; compiler just checks.
- "Cost annotations": compiler reports token-cost AND runtime-cost of each construct, so the AI can self-optimize.
- Reversible/decompilable: dense form ⇄ readable form is lossless and deterministic.
- Self-describing errors: a compile error includes the minimal patch to fix it (great for an AI loop).
- Tests as contracts: if every function carries a spec, "tests" are generated, not written.
- Streaming-friendly grammar: parseable left-to-right so it can be validated *as the model emits it* (catch bugs mid-generation).
```
*Idea? Drop it here and we'll sort it later.*
