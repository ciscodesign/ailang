# ailang examples

ailang is a **graph-native language**: programs are typed node-graphs, not text.
Every node is a pure function. Edges wire outputs to inputs.
The runtime evaluates the graph, or the transpiler emits Rust/WASM source.

## Running examples

```bash
# evaluate a graph (runs all nodes, prints each node's outputs as JSON)
ailang eval examples/01_hello.ailang.json

# emit Rust source from a graph
ailang emit examples/02_calculator.ailang.json

# save/load a graph
ailang save /tmp/my_graph.json
ailang load /tmp/my_graph.json
```

---

## 01_hello — string concatenation

**What it computes:** `"Hello, " ++ "World!"` → `"Hello, World!"`

```
[Const "Hello, "] ──(a)──┐
                          ├── [concat_text] ──► "Hello, World!"
[Const "World!"]  ──(b)──┘
```

```bash
ailang eval examples/01_hello.ailang.json
# node 2: {"out":"Hello, World!"}
```

---

## 02_calculator — arithmetic pipeline

**What it computes:** `(3 + 4) * 2 = 14`

```
[Const 3] ──(a)──┐
                  ├── [add_int=7] ──(a)──┐
[Const 4] ──(b)──┘                       ├── [mul_int=14]
                         [Const 2] ──(b)──┘
```

```bash
ailang eval   examples/02_calculator.ailang.json   # node 4: {"out":14}
ailang emit   examples/02_calculator.ailang.json   # Rust source
```

Emitted Rust:
```rust
pub fn run() {
    let node_0_out: i64 = 3;
    let node_1_out: i64 = 4;
    let node_2_a = node_0_out;
    let node_2_b = node_1_out;
    let node_2_out: i64 = node_2_a + node_2_b;
    let node_3_out: i64 = 2;
    let node_4_a = node_2_out;
    let node_4_b = node_3_out;
    let node_4_out: i64 = node_4_a * node_4_b;
}
```

---

## 03_lcg_random — random number generator (LCG)

**What it computes:** one step of a Linear Congruential Generator
`next = (a × seed + c) % m`
with `seed=42, a=1664525, c=1013904223, m=2³²` → **1083814273**

```
[seed=42] ──────────────────────(b)──┐
[a=1664525] ────────────────────(a)──┴── [mul_int] ──(a)──┐
[c=1013904223] ──────────────────────────────────────(b)──┴── [add_int] ──(a)──┐
[m=4294967296] ──────────────────────────────────────────────────────────(b)──┴── [mod_int]
                                                                                      │
                                                                              1083814273
```

```bash
ailang eval examples/03_lcg_random.ailang.json
# node 6: {"out":1083814273}
```

To generate a sequence, feed the output back in as the new seed (the graph is stateless
— iteration happens outside, in a host loop or by chaining more graphs).

---

## 04_clamp — value clamping

**What it computes:** `clamp(150, lo=0, hi=100)` → **100**
`min(max(value, lo), hi)`

```bash
ailang eval examples/04_clamp.ailang.json
# node 4: {"out":100}
```

---

## 05_fizzbuzz_check — conditional logic

**What it computes:** is 15 divisible by 3 AND 5? → **true**

```
[n=15] → [mod 3] → [eq 0] ──(a)──┐
[n=15] → [mod 5] → [eq 0] ──(b)──┴── [and_bool] → true
```

```bash
ailang eval examples/05_fizzbuzz_check.ailang.json
# node 9: {"out":true}
```

---

## How to build your own program

A graph is a JSON file with `nodes` and `edges`.
Each node has an `id` (blake3 hash), a `kind`, and typed `inputs`/`outputs`.

```json
{
  "nodes": [
    { "id": "...", "kind": "Const:out:42", "inputs": [], "outputs": [{"name":"out","ty":"Int"}] },
    { "id": "...", "kind": "neg_int",      "inputs": [{"name":"a","ty":"Int"}], "outputs": [{"name":"out","ty":"Int"}] }
  ],
  "edges": [
    { "src_node": 0, "src_port": 0, "dst_node": 1, "dst_port": 0 }
  ]
}
```

**Built-in node kinds:**

| kind | inputs | output | operation |
|---|---|---|---|
| `Const:<port>:<val>` | — | port | literal constant |
| `add_int` / `sub_int` / `mul_int` / `div_int` / `mod_int` | a,b: Int | out: Int | arithmetic |
| `neg_int` / `abs_int` / `min_int` / `max_int` | a (,b): Int | out: Int | unary/binary |
| `add_float` | a,b: Float | out: Float | float add |
| `eq_int` / `lt_int` / `gt_int` | a,b: Int | out: Bool | comparison |
| `not_bool` / `and_bool` / `or_bool` | a (,b): Bool | out: Bool | logic |
| `if_int` | cond: Bool, then/else\_: Int | out: Int | conditional |
| `concat_text` / `len_text` / `int_to_text` / `bool_to_text` | text/int/bool | out | string ops |
| `list_empty` / `list_push` / `list_head` / `list_tail` / `list_len` / `list_int_sum` | list (,item) | out | list ops |
| `map_empty` / `map_set` / `map_get` / `map_contains` / `map_keys` / `map_len` | map (,key,val) | out | map ops |

---

## What about a landing page or web app?

ailang targets WebAssembly. The `emit` subcommand produces Rust source with
`#[no_mangle] pub extern "C"` functions (via `codegen_wasm`). Compile that to
`.wasm` with `rustc --target wasm32-unknown-unknown`, then call it from JavaScript:

```js
// In a browser — ailang computes values, JS renders them
const { instance } = await WebAssembly.instantiateStreaming(fetch("run.wasm"));
const result = instance.exports.run();  // e.g. the LCG output
document.getElementById("random").textContent = result;
```

A landing page hero number, a price calculator, a form validator — any pure
computation can be an ailang graph compiled to WASM and wired into the page.
The page's HTML/CSS stays in JS land; ailang handles the logic.
