/// Generate example .ailang.json graph files into the top-level examples/ directory.
/// Run: cargo run --example gen_examples
use ailang_core::graph::{Graph, NodeDef, PortDef};
use ailang_core::node_id::NodeId;
use ailang_core::ty::Type;
use ailang_effects::EffectSet;
use std::path::Path;

fn nd(seed: &[u8], kind: &str, inputs: Vec<(&str, Type)>, outputs: Vec<(&str, Type)>) -> NodeDef {
    NodeDef {
        id: NodeId::of(seed),
        kind: kind.into(),
        inputs: inputs.into_iter().map(|(n, t)| PortDef { name: n.into(), ty: t }).collect(),
        outputs: outputs.into_iter().map(|(n, t)| PortDef { name: n.into(), ty: t }).collect(),
        effects: EffectSet::empty(),
    }
}

fn save(g: &Graph, out_dir: &Path, name: &str) {
    let json = ailang_core::serial::encode(g).expect("encode");
    let path = out_dir.join(name);
    std::fs::write(&path, &json).expect("write");
    println!("wrote {}", path.display());
}

fn main() {
    let out_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap()  // crates/ailang-cli -> crates/
        .parent().unwrap()  // crates/ -> workspace root
        .join("examples");
    std::fs::create_dir_all(&out_dir).unwrap();

    // ── 01_hello.ailang.json ──────────────────────────────────────────────────
    // "Hello, " ++ "World!" → "Hello, World!"
    {
        let mut g = Graph::new();
        g.add_node(nd(b"h1", "Const:a:Hello, ", vec![], vec![("a", Type::Text)]));
        g.add_node(nd(b"h2", "Const:b:World!",  vec![], vec![("b", Type::Text)]));
        g.add_node(nd(b"h3", "concat_text",
            vec![("a", Type::Text), ("b", Type::Text)],
            vec![("out", Type::Text)],
        ));
        g.add_edge(0, 0, 2, 0).unwrap(); // const "Hello, " → concat input a
        g.add_edge(1, 0, 2, 1).unwrap(); // const "World!"  → concat input b
        save(&g, &out_dir, "01_hello.ailang.json");
    }

    // ── 02_calculator.ailang.json ─────────────────────────────────────────────
    // (3 + 4) * 2 = 14
    {
        let mut g = Graph::new();
        g.add_node(nd(b"c1", "Const:out:3", vec![], vec![("out", Type::Int)]));
        g.add_node(nd(b"c2", "Const:out:4", vec![], vec![("out", Type::Int)]));
        g.add_node(nd(b"ca", "add_int",
            vec![("a", Type::Int), ("b", Type::Int)],
            vec![("out", Type::Int)],
        ));
        g.add_node(nd(b"c3", "Const:out:2", vec![], vec![("out", Type::Int)]));
        g.add_node(nd(b"cm", "mul_int",
            vec![("a", Type::Int), ("b", Type::Int)],
            vec![("out", Type::Int)],
        ));
        g.add_edge(0, 0, 2, 0).unwrap(); // 3 → add.a
        g.add_edge(1, 0, 2, 1).unwrap(); // 4 → add.b
        g.add_edge(2, 0, 4, 0).unwrap(); // add.out → mul.a
        g.add_edge(3, 0, 4, 1).unwrap(); // 2 → mul.b
        save(&g, &out_dir, "02_calculator.ailang.json");
    }

    // ── 03_lcg_random.ailang.json ─────────────────────────────────────────────
    // LCG: next = (a * seed + c) % m
    // seed=42, a=1664525, c=1013904223, m=4294967296 → 1868576327
    {
        let mut g = Graph::new();
        // constants
        g.add_node(nd(b"lseed", "Const:out:42",          vec![], vec![("out", Type::Int)])); // 0
        g.add_node(nd(b"la",    "Const:out:1664525",      vec![], vec![("out", Type::Int)])); // 1
        g.add_node(nd(b"lc",    "Const:out:1013904223",   vec![], vec![("out", Type::Int)])); // 2
        g.add_node(nd(b"lm",    "Const:out:4294967296",   vec![], vec![("out", Type::Int)])); // 3
        // a * seed
        g.add_node(nd(b"lmul", "mul_int",
            vec![("a", Type::Int), ("b", Type::Int)], vec![("out", Type::Int)])); // 4
        // (a*seed) + c
        g.add_node(nd(b"ladd", "add_int",
            vec![("a", Type::Int), ("b", Type::Int)], vec![("out", Type::Int)])); // 5
        // ((a*seed)+c) % m
        g.add_node(nd(b"lmod", "mod_int",
            vec![("a", Type::Int), ("b", Type::Int)], vec![("out", Type::Int)])); // 6
        g.add_edge(1, 0, 4, 0).unwrap(); // a    → mul.a
        g.add_edge(0, 0, 4, 1).unwrap(); // seed → mul.b
        g.add_edge(4, 0, 5, 0).unwrap(); // mul  → add.a
        g.add_edge(2, 0, 5, 1).unwrap(); // c    → add.b
        g.add_edge(5, 0, 6, 0).unwrap(); // add  → mod.a
        g.add_edge(3, 0, 6, 1).unwrap(); // m    → mod.b
        save(&g, &out_dir, "03_lcg_random.ailang.json");
    }

    // ── 04_clamp.ailang.json ──────────────────────────────────────────────────
    // clamp(value=150, lo=0, hi=100) → 100
    // uses: min_int(max_int(value, lo), hi)
    {
        let mut g = Graph::new();
        g.add_node(nd(b"cv",  "Const:out:150", vec![], vec![("out", Type::Int)])); // 0 value
        g.add_node(nd(b"clo", "Const:out:0",   vec![], vec![("out", Type::Int)])); // 1 lo
        g.add_node(nd(b"chi", "Const:out:100", vec![], vec![("out", Type::Int)])); // 2 hi
        g.add_node(nd(b"cmx", "max_int",
            vec![("a", Type::Int), ("b", Type::Int)], vec![("out", Type::Int)])); // 3 max(value,lo)
        g.add_node(nd(b"cmn", "min_int",
            vec![("a", Type::Int), ("b", Type::Int)], vec![("out", Type::Int)])); // 4 min(prev,hi)
        g.add_edge(0, 0, 3, 0).unwrap(); // value → max.a
        g.add_edge(1, 0, 3, 1).unwrap(); // lo    → max.b
        g.add_edge(3, 0, 4, 0).unwrap(); // max   → min.a
        g.add_edge(2, 0, 4, 1).unwrap(); // hi    → min.b
        save(&g, &out_dir, "04_clamp.ailang.json");
    }

    // ── 05_fizzbuzz_check.ailang.json ─────────────────────────────────────────
    // Is 15 divisible by 3? → true; by 5? → true; both (FizzBuzz)? → true
    {
        let mut g = Graph::new();
        g.add_node(nd(b"fn",  "Const:out:15",  vec![], vec![("out", Type::Int)])); // 0 n
        g.add_node(nd(b"f3",  "Const:out:3",   vec![], vec![("out", Type::Int)])); // 1
        g.add_node(nd(b"f5",  "Const:out:5",   vec![], vec![("out", Type::Int)])); // 2
        g.add_node(nd(b"f0a", "Const:out:0",   vec![], vec![("out", Type::Int)])); // 3 zero for eq
        g.add_node(nd(b"f0b", "Const:out:0",   vec![], vec![("out", Type::Int)])); // 4 zero for eq
        // n % 3
        g.add_node(nd(b"fm3", "mod_int",
            vec![("a", Type::Int), ("b", Type::Int)], vec![("out", Type::Int)])); // 5
        // n % 5
        g.add_node(nd(b"fm5", "mod_int",
            vec![("a", Type::Int), ("b", Type::Int)], vec![("out", Type::Int)])); // 6
        // (n%3) == 0
        g.add_node(nd(b"fe3", "eq_int",
            vec![("a", Type::Int), ("b", Type::Int)], vec![("out", Type::Bool)])); // 7
        // (n%5) == 0
        g.add_node(nd(b"fe5", "eq_int",
            vec![("a", Type::Int), ("b", Type::Int)], vec![("out", Type::Bool)])); // 8
        // div_by_3 AND div_by_5
        g.add_node(nd(b"fab", "and_bool",
            vec![("a", Type::Bool), ("b", Type::Bool)], vec![("out", Type::Bool)])); // 9
        g.add_edge(0, 0, 5, 0).unwrap(); // n → mod3.a
        g.add_edge(1, 0, 5, 1).unwrap(); // 3 → mod3.b
        g.add_edge(0, 0, 6, 0).unwrap(); // n → mod5.a
        g.add_edge(2, 0, 6, 1).unwrap(); // 5 → mod5.b
        g.add_edge(5, 0, 7, 0).unwrap(); // mod3 → eq3.a
        g.add_edge(3, 0, 7, 1).unwrap(); // 0    → eq3.b
        g.add_edge(6, 0, 8, 0).unwrap(); // mod5 → eq5.a
        g.add_edge(4, 0, 8, 1).unwrap(); // 0    → eq5.b
        g.add_edge(7, 0, 9, 0).unwrap(); // div3 → and.a
        g.add_edge(8, 0, 9, 1).unwrap(); // div5 → and.b
        save(&g, &out_dir, "05_fizzbuzz_check.ailang.json");
    }

    println!("\nAll examples written to {}", out_dir.display());
}
