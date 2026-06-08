use crate::node_id::NodeId;
use std::cmp::Ordering;
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Type {
    Text,
    Int,
    Float,
    Bool,
    Bytes,
    Option(Box<Type>),
    Result(Box<Type>, Box<Type>),  // (ok, err)
    Var(u32),                       // type variable, identified by index
    Union(Vec<Type>),               // ordered, deduplicated
    Fold(NodeId),                   // reference to a fold node by content-hash
}
impl Type {
    pub fn union(mut types: Vec<Type>) -> Type {
        if types.len() <= 1 {
            return types.into_iter().next().unwrap_or(Type::Text);
        }
        types.sort();
        types.dedup();
        Type::Union(types)
    }
}
impl Ord for Type {
    fn cmp(&self, other: &Self) -> Ordering {
        use std::fmt::Debug;
        use std::fmt::Formatter;
        struct DebugWrapper<'a>(&'a dyn Debug);
        impl PartialEq for DebugWrapper<'_> {
            fn eq(&self, other: &Self) -> bool {
                format!("{:?}", self.0) == format!("{:?}", other.0)
            }
        }
        impl Eq for DebugWrapper<'_> {}
        impl Ord for DebugWrapper<'_> {
            fn cmp(&self, other: &Self) -> Ordering {
                format!("{:?}", self.0).cmp(&format!("{:?}", other.0))
            }
        }
        impl PartialOrd for DebugWrapper<'_> {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }
        let a = DebugWrapper(self);
        let b = DebugWrapper(other);
        a.cmp(&b)
    }
}
impl PartialOrd for Type {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
