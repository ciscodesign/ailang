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
    // Additional coverage tests
    #[test]
    fn var_on_right_fails() {
        assert!(Type::unify(&Type::Text, &Type::Var(0)).is_err());
    }
    #[test]
    fn nested_option_unifies() {
        let a = Type::Option(Box::new(Type::Option(Box::new(Type::Text))));
        let b = Type::Option(Box::new(Type::Option(Box::new(Type::Text))));
        assert!(Type::unify(&a, &b).is_ok());
    }
    #[test]
    fn union_with_var_inner() {
        let u = Type::union(vec![Type::Var(0), Type::Int]);
        let r = Type::unify(&u, &Type::Text).unwrap();
        assert_eq!(r, Type::Text);
    }
    #[test]
    fn result_mismatch_ok_fails() {
        let a = Type::Result(Box::new(Type::Text), Box::new(Type::Int));
        let b = Type::Result(Box::new(Type::Bool), Box::new(Type::Int));
        assert!(Type::unify(&a, &b).is_err());
    }
    #[test]
    fn result_mismatch_err_fails() {
        let a = Type::Result(Box::new(Type::Text), Box::new(Type::Int));
        let b = Type::Result(Box::new(Type::Text), Box::new(Type::Bool));
        assert!(Type::unify(&a, &b).is_err());
    }
}
