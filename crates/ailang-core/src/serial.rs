use crate::graph::{Graph, NodeDef, PortDef};
use crate::node_id::NodeId;
use crate::ty::Type;
use ailang_effects::EffectSet;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SerialError {
    #[error("encode error: {0}")]
    Encode(String),
    #[error("decode error: {0}")]
    Decode(String),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct SerialPortDef {
    name: String,
    ty:   Type,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct SerialNodeDef {
    id:      NodeId,   // serde impl: hex string round-trips without re-hashing
    kind:    String,
    inputs:  Vec<SerialPortDef>,
    outputs: Vec<SerialPortDef>,
}

impl From<&NodeDef> for SerialNodeDef {
    fn from(n: &NodeDef) -> Self {
        Self {
            id:      n.id,
            kind:    n.kind.clone(),
            inputs:  n.inputs.iter().map(|p| SerialPortDef { name: p.name.clone(), ty: p.ty.clone() }).collect(),
            outputs: n.outputs.iter().map(|p| SerialPortDef { name: p.name.clone(), ty: p.ty.clone() }).collect(),
        }
    }
}

pub fn encode(graph: &Graph) -> Result<Vec<u8>, SerialError> {
    let nodes: Vec<SerialNodeDef> = graph.nodes().iter().map(SerialNodeDef::from).collect();
    serde_json::to_string_pretty(&nodes)
        .map(|s| s.into_bytes())
        .map_err(|e| SerialError::Encode(e.to_string()))
}

pub fn decode(bytes: &[u8]) -> Result<Graph, SerialError> {
    let json_str = std::str::from_utf8(bytes).map_err(|e| SerialError::Decode(e.to_string()))?;
    let nodes: Vec<SerialNodeDef> = serde_json::from_str(json_str)
        .map_err(|e| SerialError::Decode(e.to_string()))?;
    let mut graph = Graph::new();
    for node in nodes {
        graph.add_node(NodeDef {
            id:      node.id,
            kind:    node.kind,
            inputs:  node.inputs.into_iter().map(|p| PortDef { name: p.name, ty: p.ty }).collect(),
            outputs: node.outputs.into_iter().map(|p| PortDef { name: p.name, ty: p.ty }).collect(),
            effects: EffectSet::empty(),
        });
    }
    Ok(graph)
}
