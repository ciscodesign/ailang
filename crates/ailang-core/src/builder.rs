use crate::graph::{Graph, GraphError, NodeDef, NodeIdx, PortDef};
use crate::node_id::NodeId;
use crate::ty::Type;
use ailang_effects::{EffectSet};
pub struct GraphBuilder {
    graph: Graph,
    counter: u64,   // monotonic seed for NodeId::of
}
impl GraphBuilder {
    pub fn new() -> Self {
        Self {
            graph: Graph::new(),
            counter: 0,
        }
    }
    /// Add a Const node for a single output port.
    /// kind will be "Const:{port_name}".
    pub fn const_node(&mut self, port_name: impl Into<String>, ty: Type) -> NodeIdx {
        let port_name = port_name.into();
        let id = NodeId::of(&self.counter.to_le_bytes());
        self.counter += 1;
        let node_def = NodeDef {
            id,
            kind: format!("Const:{}", port_name),
            inputs: vec![],
            outputs: vec![PortDef { name: port_name, ty }],
            effects: EffectSet::empty(),
        };
        self.graph.add_node(node_def)
    }
    /// Add a Code:<expr> node with the given input ports and a single "out" output.
    pub fn code_node(
        &mut self,
        expr: impl Into<String>,
        inputs: Vec<(String, Type)>,   // (port_name, type) pairs
        out_ty: Type,
    ) -> NodeIdx {
        let expr = expr.into();
        let id = NodeId::of(&self.counter.to_le_bytes());
        self.counter += 1;
        let input_ports = inputs.iter().map(|(name, ty)| PortDef { name: name.clone(), ty: ty.clone() }).collect();
        let output_port = vec![PortDef { name: "out".into(), ty: out_ty }];
        let node_def = NodeDef {
            id,
            kind: format!("Code:{}", expr),
            inputs: input_ports,
            outputs: output_port,
            effects: EffectSet::empty(),
        };
        self.graph.add_node(node_def)
    }
    /// Add a generic node with explicit kind, inputs, and outputs.
    pub fn node(
        &mut self,
        kind: impl Into<String>,
        inputs: Vec<(String, Type)>,
        outputs: Vec<(String, Type)>,
        effects: EffectSet,
    ) -> NodeIdx {
        let id = NodeId::of(&self.counter.to_le_bytes());
        self.counter += 1;
        let input_ports = inputs.iter().map(|(name, ty)| PortDef { name: name.clone(), ty: ty.clone() }).collect();
        let output_ports = outputs.iter().map(|(name, ty)| PortDef { name: name.clone(), ty: ty.clone() }).collect();
        let node_def = NodeDef {
            id,
            kind: kind.into(),
            inputs: input_ports,
            outputs: output_ports,
            effects,
        };
        self.graph.add_node(node_def)
    }
    /// Wire src_node's output port `src_port` to dst_node's input port `dst_port`.
    pub fn edge(
        &mut self,
        src_node: NodeIdx, src_port: usize,
        dst_node: NodeIdx, dst_port: usize,
    ) -> Result<(), GraphError> {
        self.graph.add_edge(src_node, src_port, dst_node, dst_port)
    }
    /// Consume the builder and return the finished Graph.
    pub fn build(self) -> Graph {
        self.graph
    }
}
impl Default for GraphBuilder {
    fn default() -> Self { Self::new() }
}
