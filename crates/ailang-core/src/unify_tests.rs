#[cfg(test)]
mod tests {
    use crate::ty::Type;
    use crate::unify::UnifyError;

    #[test]
    fn identical_unifies() {
        assert!(Type::unify(&Type::Text, &Type::Text).is_ok());
    }

    #[test]
    fn mismatch_fails() {
        assert!(matches!(
            Type::unify(&Type::Text, &Type::Int),
            Err(UnifyError::Mismatch(_, _))
        ));
    }

    #[test]
    fn var_unifies_with_concrete() {
        let r = Type::unify(&Type::Var(0), &Type::Text).unwrap();
        assert_eq!(r, Type::Text);
    }

    #[test]
    fn var_on_right_fails() {
        assert!(Type::unify(&Type::Text, &Type::Var(0)).is_err());
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
        assert!(matches!(
            Type::unify(&a, &b),
            Err(UnifyError::Mismatch(_, _))
        ));
    }

    #[test]
    fn union_accepts_member() {
        let u = Type::union(vec![Type::Text, Type::Int]);
        assert!(Type::unify(&u, &Type::Text).is_ok());
        assert!(matches!(
            Type::unify(&u, &Type::Bool),
            Err(UnifyError::Mismatch(_, _))
        ));
    }

    #[test]
    fn result_unifies() {
        let a = Type::Result(Box::new(Type::Text), Box::new(Type::Int));
        let b = Type::Result(Box::new(Type::Text), Box::new(Type::Int));
        assert!(Type::unify(&a, &b).is_ok());
    }

    #[test]
    fn result_fails_different_ok() {
        let a = Type::Result(Box::new(Type::Text), Box::new(Type::Int));
        let b = Type::Result(Box::new(Type::Bool), Box::new(Type::Int));
        assert!(Type::unify(&a, &b).is_err());
    }

    #[test]
    fn result_fails_different_err() {
        let a = Type::Result(Box::new(Type::Text), Box::new(Type::Int));
        let b = Type::Result(Box::new(Type::Text), Box::new(Type::Bool));
        assert!(Type::unify(&a, &b).is_err());
    }

    #[test]
    fn union_with_var_member() {
        let u = Type::union(vec![Type::Var(0)]);
        let r = Type::unify(&u, &Type::Text).unwrap();
        assert_eq!(r, Type::Text);
    }
    #[test]
    fn list_same_inner_unifies() {
        let a = Type::List(Box::new(Type::Int));
        let b = Type::List(Box::new(Type::Int));
        assert!(Type::unify(&a, &b).is_ok());
    }
    #[test]
    fn list_different_inner_fails() {
        let a = Type::List(Box::new(Type::Int));
        let b = Type::List(Box::new(Type::Text));
        assert!(Type::unify(&a, &b).is_err());
    }
    #[test]
    fn map_same_types_unifies() {
        let a = Type::Map(Box::new(Type::Text), Box::new(Type::Int));
        let b = Type::Map(Box::new(Type::Text), Box::new(Type::Int));
        assert!(Type::unify(&a, &b).is_ok());
    }
    #[test]
    fn map_val_mismatch_fails() {
        let a = Type::Map(Box::new(Type::Text), Box::new(Type::Int));
        let b = Type::Map(Box::new(Type::Text), Box::new(Type::Bool));
        assert!(Type::unify(&a, &b).is_err());
    }
}
