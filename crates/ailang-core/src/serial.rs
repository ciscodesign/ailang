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
    id:      NodeId,
    kind:    String,
    inputs:  Vec<SerialPortDef>,
    outputs: Vec<SerialPortDef>,
}

#[derive(Serialize, Deserialize, Debug)]
struct SerialEdge {
    src_node: usize,
    src_port: usize,
    dst_node: usize,
    dst_port: usize,
}

#[derive(Serialize, Deserialize, Debug)]
struct SerialGraph {
    nodes: Vec<SerialNodeDef>,
    edges: Vec<SerialEdge>,
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
    let sg = SerialGraph {
        nodes: graph.nodes().iter().map(SerialNodeDef::from).collect(),
        edges: graph.edges().iter().map(|e| SerialEdge {
            src_node: e.src_node,
            src_port: e.src_port,
            dst_node: e.dst_node,
            dst_port: e.dst_port,
        }).collect(),
    };
    serde_json::to_string_pretty(&sg)
        .map(|s| s.into_bytes())
        .map_err(|e| SerialError::Encode(e.to_string()))
}

pub fn decode(bytes: &[u8]) -> Result<Graph, SerialError> {
    let json_str = std::str::from_utf8(bytes).map_err(|e| SerialError::Decode(e.to_string()))?;

    // Try new format first, fall back to legacy node-array format
    let sg: SerialGraph = if json_str.trim_start().starts_with('[') {
        let nodes: Vec<SerialNodeDef> = serde_json::from_str(json_str)
            .map_err(|e| SerialError::Decode(e.to_string()))?;
        SerialGraph { nodes, edges: vec![] }
    } else {
        serde_json::from_str(json_str)
            .map_err(|e| SerialError::Decode(e.to_string()))?
    };

    let mut graph = Graph::new();
    for node in sg.nodes {
        graph.add_node(NodeDef {
            id:      node.id,
            kind:    node.kind,
            inputs:  node.inputs.into_iter().map(|p| PortDef { name: p.name, ty: p.ty }).collect(),
            outputs: node.outputs.into_iter().map(|p| PortDef { name: p.name, ty: p.ty }).collect(),
            effects: EffectSet::empty(),
        });
    }
    for edge in sg.edges {
        graph.add_edge(edge.src_node, edge.src_port, edge.dst_node, edge.dst_port)
            .map_err(|e| SerialError::Decode(e.to_string()))?;
    }
    Ok(graph)
}
