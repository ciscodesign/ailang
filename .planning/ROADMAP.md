# ailang — Roadmap

**Project:** ailang — a graph-native, foldable, formally-checkable programming language for AI.
**Goal:** Build a working graph-native language runtime that compiles to WebAssembly. Source of truth is a typed node-graph an AI edits directly.

---

## Phase 0 — Foundation (active)

**Goal:** Core Rust workspace with typed node-graph primitives, effect system, and serialization.

**Crates:** ailang-core, ailang-effects, ailang-nodes, ailang-fold, ailang-exec, ailang-transpile

### Tasks
- [x] 0001: NodeId — blake3 content-addressed identity
- [x] 0002: Type system — Type enum with variants
- [x] 0003: Type::unify — static edge-legality check
- [ ] 0004: Graph — nodes, ports, edges, well-formedness
- [ ] 0005: EffectSet + linear CapToken
- [ ] 0006: Node effects — add EffectSet to NodeDef
- [ ] 0007: Serialization — canonical JSON + round-trip

---

## Phase 1 — Transpiler (planned)

**Goal:** graph → Rust → WASM transpilation pipeline. First runnable ailang programs.

---

## Phase 2 — Verifying Compiler (planned)

**Goal:** Native verifying compiler with proof-carrying code and formal contracts.
