use crate::node_id::NodeId;
use crate::ty::Type;
use crate::unify::UnifyError;
use ailang_effects::EffectSet;

pub type NodeIdx = usize;
pub type PortIdx = usize;

#[derive(Clone, Debug)]
pub struct PortDef {
    pub name: String,
    pub ty:   Type,
}

#[derive(Clone, Debug)]
pub struct NodeDef {
    pub id:      NodeId,
    pub kind:    String,
    pub inputs:  Vec<PortDef>,
    pub outputs: Vec<PortDef>,
    pub effects: EffectSet,
}

#[derive(Clone, Debug)]
pub struct Edge {
    pub src_node: NodeIdx,
    pub src_port: PortIdx,
    pub dst_node: NodeIdx,
    pub dst_port: PortIdx,
    pub ty:       Type,
}

#[derive(Debug, thiserror::Error)]
pub enum GraphError {
    #[error("node index {0} out of range")]
    NoSuchNode(NodeIdx),
    #[error("port index {0} out of range on node {1}")]
    NoSuchPort(PortIdx, NodeIdx),
    #[error("type mismatch: {0}")]
    TypeMismatch(#[from] UnifyError),
}

#[derive(Default)]
pub struct Graph {
    nodes: Vec<NodeDef>,
    edges: Vec<Edge>,
}

impl Graph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_node(&mut self, node: NodeDef) -> NodeIdx {
        let idx = self.nodes.len();
        self.nodes.push(node);
        idx
    }

    pub fn add_edge(
        &mut self,
        src_node: NodeIdx,
        src_port: PortIdx,
        dst_node: NodeIdx,
        dst_port: PortIdx,
    ) -> Result<(), GraphError> {
        if src_node >= self.nodes.len() {
            return Err(GraphError::NoSuchNode(src_node));
        }
        if dst_node >= self.nodes.len() {
            return Err(GraphError::NoSuchNode(dst_node));
        }
        let src_ty = self.nodes[src_node].outputs
            .get(src_port)
            .ok_or(GraphError::NoSuchPort(src_port, src_node))?
            .ty.clone();
        let dst_ty = self.nodes[dst_node].inputs
            .get(dst_port)
            .ok_or(GraphError::NoSuchPort(dst_port, dst_node))?
            .ty.clone();
        let unified = Type::unify(&src_ty, &dst_ty)?;
        self.edges.push(Edge { src_node, src_port, dst_node, dst_port, ty: unified });
        Ok(())
    }

    pub fn nodes(&self) -> &[NodeDef] {
        &self.nodes
    }

    pub fn edges(&self) -> &[Edge] {
        &self.edges
    }

    pub fn total_effects(&self) -> EffectSet {
        self.nodes.iter().fold(EffectSet::empty(), |acc, n| acc.union(&n.effects))
    }
}
