use crate::ty::Type;
use thiserror::Error;
#[derive(Debug, Error)]
pub enum UnifyError {
    #[error("cannot unify {0:?} with {1:?}")]
    Mismatch(Type, Type),
}
impl Type {
    pub fn unify(a: &Type, b: &Type) -> Result<Type, UnifyError> {
        if a == b {
            return Ok(a.clone());
        }
        match (a, b) {
            (Type::Var(_), _) => Ok(b.clone()),
            (Type::Option(inner_a), Type::Option(inner_b)) => {
                let unified_inner = Self::unify(inner_a, inner_b)?;
                Ok(Type::Option(Box::new(unified_inner)))
            }
            (Type::Result(ok_a, err_a), Type::Result(ok_b, err_b)) => {
                let unified_ok = Self::unify(ok_a, ok_b)?;
                let unified_err = Self::unify(err_a, err_b)?;
                Ok(Type::Result(Box::new(unified_ok), Box::new(unified_err)))
            }
            (Type::Union(variants), _) => {
                for v in variants {
                    if Self::unify(v, b).is_ok() {
                        return Ok(b.clone());
                    }
                }
                Err(UnifyError::Mismatch(a.clone(), b.clone()))
            }
            _ => Err(UnifyError::Mismatch(a.clone(), b.clone())),
        }
    }
}
