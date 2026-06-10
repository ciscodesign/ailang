#[cfg(test)]
mod tests {
    use ailang_core::ty::Type;
    use crate::value::Value;
    #[test]
    fn text_matches_text() {
        assert!(Value::Text("hi".into()).matches_type(&Type::Text));
    }
    #[test]
    fn int_does_not_match_text() {
        assert!(!Value::Int(42).matches_type(&Type::Text));
    }
    #[test]
    fn any_value_matches_var() {
        assert!(Value::Bool(true).matches_type(&Type::Var(0)));
    }
    #[test]
    fn option_matches_option() {
        assert!(Value::Option(None).matches_type(&Type::Option(Box::new(Type::Text))));
    }
    #[test]
    fn result_ok_matches_result() {
        let v = Value::Result(Ok(Box::new(Value::Int(1))));
        assert!(v.matches_type(&Type::Result(Box::new(Type::Int), Box::new(Type::Text))));
    }
    #[test]
    fn list_matches_list_type() {
        assert!(Value::List(vec![Value::Int(1)]).matches_type(&Type::List(Box::new(Type::Int))));
    }
    #[test]
    fn list_does_not_match_int() {
        assert!(!Value::List(vec![]).matches_type(&Type::Int));
    }
    #[test]
    fn map_matches_map_type() {
        use std::collections::BTreeMap;
        let t = Type::Map(Box::new(Type::Text), Box::new(Type::Int));
        assert!(Value::Map(BTreeMap::new()).matches_type(&t));
    }
    #[test]
    fn map_does_not_match_list() {
        use std::collections::BTreeMap;
        assert!(!Value::Map(BTreeMap::new()).matches_type(&Type::List(Box::new(Type::Int))));
    }
}
