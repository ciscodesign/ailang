use crate::node_id::NodeId;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
pub enum Type {
    Text,
    Int,
    Float,
    Bool,
    Bytes,
    Option(Box<Type>),
    Result(Box<Type>, Box<Type>),
    Var(u32),
    Union(Vec<Type>),
    Fold(NodeId),
    List(Box<Type>),
    Map(Box<Type>, Box<Type>),
}

impl Type {
    pub fn union(mut types: Vec<Type>) -> Type {
        types.sort();
        types.dedup();
        Type::Union(types)
    }
}
