#[cfg(test)]
mod tests {
    use crate::ty::Type;
    use crate::node_id::NodeId;
    #[test]
    fn primitives_are_distinct() {
        assert_ne!(Type::Text, Type::Int);
        assert_ne!(Type::Int, Type::Bool);
    }
    #[test]
    fn option_wrapping() {
        let t = Type::Option(Box::new(Type::Text));
        assert_ne!(t, Type::Text);
        assert_eq!(t.clone(), t);
    }
    #[test]
    fn union_deduplication() {
        let a = Type::union(vec![Type::Text, Type::Int, Type::Text]);
        let b = Type::union(vec![Type::Int, Type::Text]);
        assert_eq!(a, b);  // same canonical form
    }
    #[test]
    fn fold_ref_uses_node_id() {
        let id = NodeId::of(b"myfold");
        let t = Type::Fold(id);
        assert_eq!(t, Type::Fold(id));
    }
    #[test]
    fn type_var_distinct_by_index() {
        assert_ne!(Type::Var(0), Type::Var(1));
        assert_eq!(Type::Var(0), Type::Var(0));
    }
    #[test]
    fn list_type_distinct() {
        assert_ne!(Type::List(Box::new(Type::Int)), Type::List(Box::new(Type::Text)));
        assert_eq!(Type::List(Box::new(Type::Int)), Type::List(Box::new(Type::Int)));
    }
}
