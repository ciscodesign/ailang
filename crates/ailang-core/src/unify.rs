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
            (Type::Var(_), t) => Ok(t.clone()),
            (Type::Option(ia), Type::Option(ib)) => {
                Self::unify(ia, ib).map(|t| Type::Option(Box::new(t)))
            }
            (Type::Result(oa, ea), Type::Result(ob, eb)) => {
                let ok = Self::unify(oa, ob)?;
                let err = Self::unify(ea, eb)?;
                Ok(Type::Result(Box::new(ok), Box::new(err)))
            }
            (Type::List(ia), Type::List(ib)) => {
                Self::unify(ia, ib).map(|t| Type::List(Box::new(t)))
            }
            (Type::Map(ka, va), Type::Map(kb, vb)) => {
                let k = Self::unify(ka, kb)?;
                let v = Self::unify(va, vb)?;
                Ok(Type::Map(Box::new(k), Box::new(v)))
            }
            (Type::Union(variants), t) => variants
                .iter()
                .find(|v| Self::unify(v, t).is_ok())
                .map(|_| t.clone())
                .ok_or_else(|| UnifyError::Mismatch(a.clone(), b.clone())),
            _ => Err(UnifyError::Mismatch(a.clone(), b.clone())),
        }
    }
}
