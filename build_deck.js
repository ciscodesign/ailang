const pptxgen = require("pptxgenjs");
const p = new pptxgen();
p.layout = "LAYOUT_WIDE"; // 13.33 x 7.5
p.author = "ailang";
p.title = "ailang — a programming language for AI";

const W = 13.33, H = 7.5;
const BG = "0A0B0D", PANEL = "13161C", PANEL2 = "171A21";
const INK = "ECE8DE", MUTED = "9B9C93", DIM = "6F7069", ACCENT = "C2F04A";
const SERIF = "Georgia", BODY = "Calibri", MONO = "Consolas";

const shadow = () => ({ type: "outer", color: "000000", blur: 9, offset: 3, angle: 135, opacity: 0.35 });

function base(slide) { slide.background = { color: BG }; }
function eyebrow(slide, t, x, y) {
  slide.addText(t.toUpperCase(), { x, y, w: 8, h: 0.3, fontFace: MONO, fontSize: 11.5, color: ACCENT, charSpacing: 3, margin: 0 });
}
function title(slide, t, x, y, w, size = 40) {
  slide.addText(t, { x, y, w, h: 1.4, fontFace: SERIF, fontSize: size, color: INK, bold: false, lineSpacingMultiple: 0.98, margin: 0 });
}
function foot(slide, n) {
  slide.addText("ailang", { x: 0.6, y: H - 0.5, w: 3, h: 0.3, fontFace: MONO, fontSize: 9.5, color: DIM, charSpacing: 3, margin: 0 });
  slide.addText(String(n).padStart(2, "0"), { x: W - 1.1, y: H - 0.5, w: 0.5, h: 0.3, fontFace: MONO, fontSize: 9.5, color: DIM, align: "right", margin: 0 });
}
// node-graph motif: dots + connecting lines (decorative)
function motif(slide, ox, oy, s, pts, links) {
  links.forEach(([a, b]) => {
    const A = pts[a], B = pts[b];
    slide.addShape(p.shapes.LINE, { x: ox + A[0]*s, y: oy + A[1]*s, w: (B[0]-A[0])*s, h: (B[1]-A[1])*s, line: { color: "2E332B", width: 1 } });
  });
  pts.forEach((pt, i) => {
    const hot = pt[2];
    const r = hot ? 0.12 : 0.085;
    slide.addShape(p.shapes.OVAL, { x: ox + pt[0]*s - r, y: oy + pt[1]*s - r, w: r*2, h: r*2,
      fill: { color: hot ? ACCENT : "5A5E54" }, line: { type: "none" } });
  });
}
const GP = [[0,0.4,0],[0.9,0,1],[1.7,0.6,0],[1.2,1.5,1],[0.2,1.4,0],[2.4,1.3,0],[2.0,0.1,0]];
const GL = [[0,1],[1,2],[2,3],[3,4],[4,0],[2,5],[5,3],[1,6],[6,2]];

function card(slide, x, y, w, h, fill = PANEL) {
  slide.addShape(p.shapes.RECTANGLE, { x, y, w, h, fill: { color: fill }, line: { color: "23272F", width: 1 }, shadow: shadow() });
}

/* ---------- 1 TITLE ---------- */
let s = p.addSlide(); base(s);
motif(s, 8.4, 1.0, 1.7, GP, GL);
s.addText("A LANGUAGE FOR MACHINES THAT DON'T THINK LIKE US", { x: 0.7, y: 2.15, w: 10, h: 0.3, fontFace: MONO, fontSize: 12, color: ACCENT, charSpacing: 3, margin: 0 });
s.addText([
  { text: "ai", options: { color: INK } },
  { text: "l", options: { color: ACCENT } },
  { text: "ang", options: { color: INK } },
], { x: 0.6, y: 2.5, w: 9, h: 2.0, fontFace: SERIF, fontSize: 130, bold: false, margin: 0 });
s.addText("A programming language built for AI, not humans.", { x: 0.7, y: 4.75, w: 11, h: 0.6, fontFace: SERIF, italic: true, fontSize: 27, color: INK, margin: 0 });
s.addText("Design v1.0 · pre-implementation · building in public", { x: 0.7, y: 5.5, w: 11, h: 0.4, fontFace: MONO, fontSize: 12, color: MUTED, charSpacing: 1, margin: 0 });

/* ---------- 2 THE PROBLEM ---------- */
s = p.addSlide(); base(s); eyebrow(s, "01 — the problem", 0.7, 0.7);
title(s, "Languages built for limits we don't have.", 0.7, 1.05, 11.5, 38);
card(s, 0.7, 2.5, 5.8, 4.2);
card(s, 6.83, 2.5, 5.8, 4.2, PANEL2);
s.addText("HUMAN LANGUAGES ASSUME…", { x: 1.05, y: 2.85, w: 5, h: 0.3, fontFace: MONO, fontSize: 11, color: DIM, charSpacing: 2, margin: 0 });
s.addText([
  { text: "Limited memory → names, comments, structure", options: { breakLine: true } },
  { text: "Reading fatigue → whitespace, formatting", options: { breakLine: true } },
  { text: "Cost ≈ keystrokes", options: { breakLine: true } },
  { text: "Ambiguity, patched up later", options: {} },
], { x: 1.05, y: 3.35, w: 5.1, h: 3.0, fontFace: BODY, fontSize: 15.5, color: MUTED, paraSpaceAfter: 12, margin: 0 });
s.addText("…AN AI ACTUALLY HAS", { x: 7.18, y: 2.85, w: 5, h: 0.3, fontFace: MONO, fontSize: 11, color: ACCENT, charSpacing: 2, margin: 0 });
s.addText([
  { text: "Near-perfect recall of the whole program", options: { breakLine: true } },
  { text: "No fatigue — density is fine", options: { breakLine: true } },
  { text: "Cost ≈ attention — a different metric entirely", options: { breakLine: true } },
  { text: "Emits a fully-checkable artifact in one go", options: {} },
], { x: 7.18, y: 3.35, w: 5.1, h: 3.0, fontFace: BODY, fontSize: 15.5, color: INK, paraSpaceAfter: 12, margin: 0 });
foot(s, 2);

/* ---------- 3 THE INSIGHT ---------- */
s = p.addSlide(); base(s); eyebrow(s, "02 — the insight", 0.7, 0.7);
s.addText([
  { text: "So stop optimizing for ", options: { color: MUTED } },
  { text: "readable", options: { color: MUTED, italic: true } },
  { text: ".", options: { color: MUTED, breakLine: true } },
  { text: "Optimize for ", options: { color: INK } },
  { text: "minimum attention", options: { color: ACCENT } },
  { text: " to express", options: { color: INK, breakLine: true } },
  { text: "correct, safe intent — and let ", options: { color: INK } },
  { text: "structure", options: { color: ACCENT } },
  { text: ",", options: { color: INK, breakLine: true } },
  { text: "not text, carry the meaning.", options: { color: INK } },
], { x: 0.7, y: 2.3, w: 12, h: 3, fontFace: SERIF, fontSize: 44, lineSpacingMultiple: 1.05, margin: 0 });
motif(s, 9.6, 5.4, 1.4, GP, GL);
foot(s, 3);

/* ---------- 4 WHAT IT IS ---------- */
s = p.addSlide(); base(s); eyebrow(s, "03 — what ailang is", 0.7, 0.7);
title(s, "Three commitments.", 0.7, 1.05, 11, 38);
const trip = [
  ["Graph-native", "The source of truth is a typed node-graph an AI edits directly. No syntax. No text. Humans get diagrams, never the source."],
  ["Foldable", "Any cluster of nodes collapses into one. The AI only looks at the level it's on; the rest stays folded and costs nothing."],
  ["Verifiable", "Memory-safe, type-safe, deterministic, contract-checked. Trust comes from a tiny verifier and proofs — not from reading."],
];
trip.forEach((t, i) => {
  const x = 0.7 + i * 4.13;
  card(s, x, 2.6, 3.8, 4.0);
  s.addShape(p.shapes.OVAL, { x: x + 0.4, y: 3.0, w: 0.34, h: 0.34, fill: { color: ACCENT }, line: { type: "none" } });
  s.addText(t[0], { x: x + 0.4, y: 3.55, w: 3.1, h: 0.5, fontFace: SERIF, fontSize: 23, color: INK, margin: 0 });
  s.addText(t[1], { x: x + 0.4, y: 4.15, w: 3.05, h: 2.2, fontFace: BODY, fontSize: 14, color: MUTED, margin: 0 });
});
foot(s, 4);

/* ---------- 5 GRAPH MODEL ---------- */
s = p.addSlide(); base(s); eyebrow(s, "04 — the model", 0.7, 0.7);
title(s, "A program is a graph you can fold.", 0.7, 1.05, 11.5, 38);
// left: node + edge explainer
card(s, 0.7, 2.6, 6.0, 4.0);
s.addText("NODE + EDGE", { x: 1.0, y: 2.9, w: 5, h: 0.3, fontFace: MONO, fontSize: 11, color: ACCENT, charSpacing: 2, margin: 0 });
// two nodes + an edge
s.addShape(p.shapes.ROUNDED_RECTANGLE, { x: 1.0, y: 3.7, w: 1.9, h: 0.9, rectRadius: 0.08, fill: { color: PANEL2 }, line: { color: ACCENT, width: 1.2 } });
s.addText([{text:"Topic",options:{breakLine:true,color:INK,bold:true}},{text:": Text",options:{color:MUTED}}], { x: 1.0, y: 3.85, w: 1.9, h: 0.6, fontFace: MONO, fontSize: 11, align:"center", margin: 0 });
s.addShape(p.shapes.LINE, { x: 2.9, y: 4.15, w: 1.5, h: 0, line: { color: ACCENT, width: 1.6, endArrowType: "triangle" } });
s.addText("Text ▶ Text ✓", { x: 2.85, y: 3.75, w: 1.7, h: 0.3, fontFace: MONO, fontSize: 9, color: DIM, align: "center", margin: 0 });
s.addShape(p.shapes.ROUNDED_RECTANGLE, { x: 4.4, y: 3.7, w: 1.9, h: 0.9, rectRadius: 0.08, fill: { color: PANEL2 }, line: { color: ACCENT, width: 1.2 } });
s.addText([{text:"Poet",options:{breakLine:true,color:INK,bold:true}},{text:": LLM «llm»",options:{color:MUTED}}], { x: 4.4, y: 3.85, w: 1.9, h: 0.6, fontFace: MONO, fontSize: 11, align:"center", margin: 0 });
s.addText("Wires only connect if the types match. A bad connection is a compile error before anything runs.", { x: 1.0, y: 5.0, w: 5.4, h: 1.3, fontFace: BODY, fontSize: 14, color: MUTED, margin: 0 });
// right: fold explainer
card(s, 6.83, 2.6, 5.8, 4.0, PANEL2);
s.addText("FOLD", { x: 7.13, y: 2.9, w: 5, h: 0.3, fontFace: MONO, fontSize: 11, color: ACCENT, charSpacing: 2, margin: 0 });
// 5 small nodes cluster
const fc = [[7.4,4.0],[7.9,3.7],[8.4,4.0],[8.2,4.55],[7.6,4.55]];
[[0,1],[1,2],[2,3],[3,4],[4,0]].forEach(([a,b])=>{
  s.addShape(p.shapes.LINE,{x:fc[a][0],y:fc[a][1],w:fc[b][0]-fc[a][0],h:fc[b][1]-fc[a][1],line:{color:"3A4030",width:1}});
});
fc.forEach(c=>s.addShape(p.shapes.OVAL,{x:c[0]-0.1,y:c[1]-0.1,w:0.2,h:0.2,fill:{color:ACCENT},line:{type:"none"}}));
s.addShape(p.shapes.LINE, { x: 9.0, y: 4.2, w: 1.1, h: 0, line: { color: MUTED, width: 1.6, endArrowType: "triangle" } });
s.addShape(p.shapes.OVAL, { x: 10.3, y: 3.8, w: 0.8, h: 0.8, fill: { color: BG }, line: { color: ACCENT, width: 2 } });
s.addText("1", { x: 10.3, y: 3.92, w: 0.8, h: 0.5, fontFace: SERIF, fontSize: 22, color: ACCENT, align: "center", margin: 0 });
s.addText("Five nodes collapse into one node with a typed interface. The innards stay folded behind a hash — invisible until opened.", { x: 7.13, y: 5.05, w: 5.2, h: 1.3, fontFace: BODY, fontSize: 14, color: MUTED, margin: 0 });
foot(s, 5);

/* ---------- 6 FOLDING WINS ---------- */
s = p.addSlide(); base(s); eyebrow(s, "05 — why folding matters", 0.7, 0.7);
title(s, "One mechanism. Four wins.", 0.7, 1.05, 11, 38);
const wins = [
  ["Efficiency", "The AI only loads the interface. Folded detail costs zero attention until opened."],
  ["Security at a glance", "Effects bubble up. A 100-node system still honestly says «llm, db» at the top — nothing hides three folds deep."],
  ["Free dedup", "Identical subgraphs share a hash. Stored once, cached forever."],
  ["Infinite nesting", "Folds contain folds. A huge program is five blocks at the top, each opening into five more."],
];
wins.forEach((wn, i) => {
  const x = 0.7 + (i % 2) * 6.13, y = 2.55 + Math.floor(i / 2) * 2.1;
  card(s, x, y, 5.8, 1.85);
  s.addText(String(i+1).padStart(2,"0"), { x: x+0.35, y: y+0.3, w: 0.9, h: 0.8, fontFace: SERIF, fontSize: 30, color: ACCENT, margin: 0 });
  s.addText(wn[0], { x: x+1.3, y: y+0.28, w: 4.2, h: 0.4, fontFace: SERIF, fontSize: 19, color: INK, margin: 0 });
  s.addText(wn[1], { x: x+1.3, y: y+0.72, w: 4.3, h: 1.0, fontFace: BODY, fontSize: 13, color: MUTED, margin: 0 });
});
foot(s, 6);

/* ---------- 7 EFFECTS / DETERMINISM ---------- */
s = p.addSlide(); base(s); eyebrow(s, "06 — determinism for free", 0.7, 0.7);
title(s, "Effects live in the type.", 0.7, 1.05, 11, 38);
s.addText("A node that touches the network says so in its signature. There is no hidden IO. Ordering comes from a single linear token threaded through effectful nodes — so two side-effects physically cannot run in an undefined order.", { x: 0.7, y: 2.2, w: 11.8, h: 1.3, fontFace: BODY, fontSize: 17, color: MUTED, margin: 0 });
// token threading diagram
const ty = 4.4;
const boxes = ["⟨net⟩", "FetchA", "FetchB", "FetchC"];
boxes.forEach((b, i) => {
  const x = 0.9 + i * 3.05;
  const isTok = i === 0;
  s.addShape(p.shapes.ROUNDED_RECTANGLE, { x, y: ty, w: 2.2, h: 1.0, rectRadius: 0.08,
    fill: { color: isTok ? "1E2410" : PANEL }, line: { color: isTok ? ACCENT : "2E333C", width: isTok ? 1.5 : 1 } });
  s.addText(b, { x, y: ty + 0.3, w: 2.2, h: 0.4, fontFace: MONO, fontSize: isTok ? 15 : 14, color: isTok ? ACCENT : INK, align: "center", margin: 0 });
  if (i < boxes.length - 1) s.addShape(p.shapes.LINE, { x: x + 2.2, y: ty + 0.5, w: 0.85, h: 0, line: { color: ACCENT, width: 1.6, endArrowType: "triangle" } });
});
s.addText("the single «net» token flows through, one call after another — determinism becomes a typing rule, not a convention.", { x: 0.9, y: 5.7, w: 11.5, h: 0.6, fontFace: BODY, italic: true, fontSize: 13.5, color: DIM, margin: 0 });
foot(s, 7);

/* ---------- 8 SECURITY ---------- */
s = p.addSlide(); base(s); eyebrow(s, "07 — security", 0.7, 0.7);
title(s, "Nobody reads the code. So trust moves.", 0.7, 1.05, 12, 38);
const sec = [
  ["The verifier is the anchor", "Humans trust one small, fixed, audited checker — not the oceans of AI-generated code it inspects."],
  ["Proof-carrying code", "The AI ships code with machine-checkable proofs. You trust the math, not the author."],
  ["Adversarial AI auditors", "A different model red-teams every graph. Cheap, scalable, no readability required."],
];
sec.forEach((t, i) => {
  const y = 2.55 + i * 1.4;
  card(s, 0.7, y, 11.9, 1.18);
  s.addShape(p.shapes.OVAL, { x: 1.1, y: y + 0.35, w: 0.5, h: 0.5, fill: { color: "1E2410" }, line: { color: ACCENT, width: 1.5 } });
  s.addText(String(i+1), { x: 1.1, y: y + 0.42, w: 0.5, h: 0.36, fontFace: SERIF, fontSize: 18, color: ACCENT, align: "center", margin: 0 });
  s.addText(t[0], { x: 1.9, y: y + 0.26, w: 4.4, h: 0.7, fontFace: SERIF, fontSize: 20, color: INK, valign: "middle", margin: 0 });
  s.addText(t[1], { x: 6.4, y: y + 0.2, w: 6.0, h: 0.8, fontFace: BODY, fontSize: 13.5, color: MUTED, valign: "middle", margin: 0 });
});
foot(s, 8);

/* ---------- 9 CROSS PLATFORM ---------- */
s = p.addSlide(); base(s); eyebrow(s, "08 — fast & portable", 0.7, 0.7);
title(s, "Compiles to WebAssembly.", 0.7, 1.05, 11, 38);
s.addText("Near-native speed, sandboxed by default, portable. The hard part — UI — is deliberately deferred past v1.", { x: 0.7, y: 2.15, w: 11.5, h: 0.7, fontFace: BODY, fontSize: 16, color: MUTED, margin: 0 });
// flow: graph -> wasm -> 3 targets
s.addShape(p.shapes.ROUNDED_RECTANGLE, { x: 0.9, y: 3.7, w: 2.6, h: 1.1, rectRadius: 0.08, fill: { color: PANEL2 }, line: { color: ACCENT, width: 1.3 } });
s.addText("typed graph", { x: 0.9, y: 4.05, w: 2.6, h: 0.4, fontFace: MONO, fontSize: 14, color: INK, align: "center", margin: 0 });
s.addShape(p.shapes.LINE, { x: 3.5, y: 4.25, w: 0.9, h: 0, line: { color: ACCENT, width: 1.6, endArrowType: "triangle" } });
s.addShape(p.shapes.ROUNDED_RECTANGLE, { x: 4.4, y: 3.7, w: 2.6, h: 1.1, rectRadius: 0.08, fill: { color: "1E2410" }, line: { color: ACCENT, width: 1.5 } });
s.addText("WASM", { x: 4.4, y: 4.0, w: 2.6, h: 0.5, fontFace: MONO, fontSize: 17, color: ACCENT, align: "center", bold: true, margin: 0 });
const tg = ["Web", "Desktop", "Mobile"];
tg.forEach((t, i) => {
  const y = 3.15 + i * 0.93;
  s.addShape(p.shapes.LINE, { x: 7.0, y: 4.25, w: 1.0, h: (y + 0.34) - 4.25, line: { color: "2E333C", width: 1.2, endArrowType: "triangle" } });
  s.addShape(p.shapes.ROUNDED_RECTANGLE, { x: 8.1, y, w: 2.4, h: 0.68, rectRadius: 0.06, fill: { color: PANEL }, line: { color: "2E333C", width: 1 } });
  s.addText(t + (i === 2 ? "  (later)" : ""), { x: 8.1, y: y + 0.16, w: 2.4, h: 0.4, fontFace: MONO, fontSize: 13, color: i===2?DIM:INK, align: "center", margin: 0 });
});
foot(s, 9);

/* ---------- 10 HOW BUILT ---------- */
s = p.addSlide(); base(s); eyebrow(s, "09 — how it's built", 0.7, 0.7);
title(s, "One AI directs. Others execute. A dumb harness judges.", 0.7, 1.05, 12.3, 34);
const pipe = [["Controller", "Claude", true], ["Executors", "local models", false], ["Harness", "compile + test", false], ["Reviewer", "a different model", false]];
pipe.forEach((c, i) => {
  const x = 0.8 + i * 3.15;
  s.addShape(p.shapes.ROUNDED_RECTANGLE, { x, y: 2.9, w: 2.7, h: 1.3, rectRadius: 0.08, fill: { color: c[2] ? "1E2410" : PANEL }, line: { color: c[2] ? ACCENT : "2E333C", width: c[2] ? 1.5 : 1 } });
  s.addText(c[0], { x, y: 3.2, w: 2.7, h: 0.4, fontFace: SERIF, fontSize: 18, color: c[2] ? ACCENT : INK, align: "center", margin: 0 });
  s.addText(c[1], { x, y: 3.65, w: 2.7, h: 0.4, fontFace: MONO, fontSize: 11.5, color: MUTED, align: "center", margin: 0 });
  if (i < 3) s.addShape(p.shapes.LINE, { x: x + 2.7, y: 3.55, w: 0.45, h: 0, line: { color: ACCENT, width: 1.6, endArrowType: "triangle" } });
});
s.addText("↺  loops on failure", { x: 0.8, y: 4.4, w: 11, h: 0.4, fontFace: MONO, fontSize: 12, color: DIM, margin: 0 });
s.addText("A strong reasoning model owns architecture and writes the acceptance tests first. Cheaper local models write the code. The compiler and tests are the only judge of facts — nothing is accepted until it builds, passes, and survives a red-team. Code is guilty until proven innocent.", { x: 0.8, y: 5.0, w: 11.7, h: 1.5, fontFace: BODY, fontSize: 14.5, color: MUTED, margin: 0 });
foot(s, 10);

/* ---------- 11 ROADMAP ---------- */
s = p.addSlide(); base(s); eyebrow(s, "10 — roadmap", 0.7, 0.7);
title(s, "Narrow first. Prove the model. Then grow.", 0.7, 1.05, 12, 36);
const road = [
  ["P0", "Foundations", "Graph data model, type system, content-addressing."],
  ["P1", "Walking skeleton", "Effects, primitive nodes, fold/unfold, durable executor, graph → Rust → WASM. It runs in a browser."],
  ["P2", "Safety", "Capabilities, contracts, architecture checks, reviewer-in-CI."],
  ["P3", "UI & apps", "Web-first cross-platform demo — beyond workflow-only prior art."],
  ["P4", "Verification", "Proof-carrying nodes; shrink the trusted compiler."],
  ["P5", "Ecosystem", "The AI defines new node types in the language itself. FFI. Registry."],
];
road.forEach((r, i) => {
  const y = 2.5 + i * 0.78;
  s.addText(r[0], { x: 0.7, y, w: 0.9, h: 0.6, fontFace: MONO, fontSize: 16, color: ACCENT, bold: true, valign: "middle", margin: 0 });
  s.addText(r[1], { x: 1.7, y, w: 2.9, h: 0.6, fontFace: SERIF, fontSize: 18, color: INK, valign: "middle", margin: 0 });
  s.addText(r[2], { x: 4.7, y, w: 7.9, h: 0.6, fontFace: BODY, fontSize: 13.5, color: MUTED, valign: "middle", margin: 0 });
  if (i < road.length - 1) s.addShape(p.shapes.LINE, { x: 0.7, y: y + 0.72, w: 11.9, h: 0, line: { color: "20242C", width: 1 } });
});
foot(s, 11);

/* ---------- 12 CLOSE ---------- */
s = p.addSlide(); base(s);
motif(s, 9.3, 0.9, 1.6, GP, GL);
s.addText("THE HONEST VERSION", { x: 0.7, y: 2.0, w: 8, h: 0.3, fontFace: MONO, fontSize: 12, color: ACCENT, charSpacing: 3, margin: 0 });
s.addText("\u201CBug-free\u201D is a fantasy.", { x: 0.7, y: 2.5, w: 12, h: 0.9, fontFace: SERIF, fontSize: 44, color: INK, margin: 0 });
s.addText([
  { text: "What ailang actually delivers: ", options: { color: MUTED } },
  { text: "memory-safe, type-safe, deterministic, contract-checked, verifiable", options: { color: ACCENT } },
  { text: ". That's the real, sellable claim — and it's enough.", options: { color: MUTED } },
], { x: 0.7, y: 3.6, w: 11.6, h: 1.4, fontFace: SERIF, fontSize: 24, italic: true, lineSpacingMultiple: 1.1, margin: 0 });
s.addShape(p.shapes.LINE, { x: 0.7, y: 5.4, w: 11.9, h: 0, line: { color: "20242C", width: 1 } });
s.addText("A public repository is coming.", { x: 0.7, y: 5.7, w: 7, h: 0.4, fontFace: BODY, fontSize: 16, color: INK, margin: 0 });
s.addText("graph + fold direction shared with Weft — ideas borrowed, not code", { x: 0.7, y: 6.2, w: 11, h: 0.4, fontFace: MONO, fontSize: 11, color: DIM, margin: 0 });

p.writeFile({ fileName: "/home/claude/ailang-deck.pptx" }).then(() => console.log("deck written"));
