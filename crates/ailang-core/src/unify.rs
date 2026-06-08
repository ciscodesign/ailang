use crate::ty::Type;
use thiserror::Error;
#[derive(Debug, Error)]
pub enum UnifyError {
    #[error("cannot unify {0:?} with {1:?}")]
    Mismatch(Type, Type),
}
impl Type {
    pub fn unify(a: &Type, b: &Type) -> Result<Type, UnifyError> {
        match (a, b) {
            (x, y) if x == y => Ok(x.clone()),
            (Type::Var(_), t) | (t, Type::Var(_)) => Ok(t.clone()),
            (Type::Option(inner_a), Type::Option(inner_b)) => {
                Self::unify(inner_a.as_ref(), inner_b.as_ref()).map(|t| Type::Option(Box::new(t)))
            }
            (Type::Result(ok_a, err_a), Type::Result(ok_b, err_b)) => {
                let unified_ok = Self::unify(ok_a.as_ref(), ok_b.as_ref())?;
                let unified_err = Self::unify(err_a.as_ref(), err_b.as_ref())?;
                Ok(Type::Result(Box::new(unified_ok), Box::new(unified_err)))
            }
            (Type::Union(variants), t) | (t, Type::Union(variants)) => {
                variants.iter().find(|v| Self::unify(v, t).is_ok()).map(|_| t.clone())
            }
            _ => Err(UnifyError::Mismatch(a.clone(), b.clone())),
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::ty::Type;
    #[test]
    fn identical_unifies() {
        assert!(Type::unify(&Type::Text, &Type::Text).is_ok());
    }
    #[test]
    fn mismatch_fails() {
        assert!(Type::unify(&Type::Text, &Type::Int).is_err());
    }
    #[test]
    fn var_unifies_with_concrete() {
        let r = Type::unify(&Type::Var(0), &Type::Text).unwrap();
        assert_eq!(r, Type::Text);
    }
    #[test]
    fn option_unifies_same_inner() {
        let a = Type::Option(Box::new(Type::Text));
        let b = Type::Option(Box::new(Type::Text));
        assert!(Type::unify(&a, &b).is_ok());
    }
    #[test]
    fn option_fails_different_inner() {
        let a = Type::Option(Box::new(Type::Text));
        let b = Type::Option(Box::new(Type::Int));
        assert!(Type::unify(&a, &b).is_err());
    }
    #[test]
    fn union_accepts_member() {
        let u = Type::union(vec![Type::Text, Type::Int]);
        assert!(Type::unify(&u, &Type::Text).is_ok());
        assert!(Type::unify(&u, &Type::Bool).is_err());
    }
    #[test]
    fn result_unifies() {
        let a = Type::Result(Box::new(Type::Text), Box::new(Type::Int));
        let b = Type::Result(Box::new(Type::Text), Box::new(Type::Int));
        assert!(Type::unify(&a, &b).is_ok());
    }
}
pub mod node_id;
pub mod ty;
pub mod unify;
#[cfg(test)]
mod unify_tests;
// Fixed unused import warning from prior feedback
use crate::node_id::NodeId;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Text,
    Int,
    Bool,
    Var(usize),
    Option(Box<Type>),
    Result(Box<Type>, Box<Type>),
    Union(Vec<Type>),
}
impl Type {
    pub fn union(vs: Vec<Type>) -> Self {
        Self::Union(vs)
    }
}
[dependencies]
thiserror.workspace = true
