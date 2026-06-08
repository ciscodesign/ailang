#[cfg(test)]
mod tests {
    use ailang_exec::registry::NodeRegistry;
    use ailang_exec::value::Value;
    use std::collections::HashMap;
    use crate::builtins::register_builtins;
    fn reg() -> NodeRegistry {
        let mut r = NodeRegistry::new();
        register_builtins(&mut r);
        r
    }
    #[test]
    fn add_int() {
        let r = reg();
        let inputs = HashMap::from([("a".into(), Value::Int(3)), ("b".into(), Value::Int(4))]);
        let out = r.call("add_int", inputs).unwrap();
        assert_eq!(out["out"], Value::Int(7));
    }
    #[test]
    fn sub_int() {
        let r = reg();
        let inputs = HashMap::from([("a".into(), Value::Int(10)), ("b".into(), Value::Int(3))]);
        let out = r.call("sub_int", inputs).unwrap();
        assert_eq!(out["out"], Value::Int(7));
    }
    #[test]
    fn mul_int() {
        let r = reg();
        let inputs = HashMap::from([("a".into(), Value::Int(6)), ("b".into(), Value::Int(7))]);
        let out = r.call("mul_int", inputs).unwrap();
        assert_eq!(out["out"], Value::Int(42));
    }
    #[test]
    fn neg_int() {
        let r = reg();
        let inputs = HashMap::from([("a".into(), Value::Int(5))]);
        let out = r.call("neg_int", inputs).unwrap();
        assert_eq!(out["out"], Value::Int(-5));
    }
    #[test]
    fn concat_text() {
        let r = reg();
        let inputs = HashMap::from([
            ("a".into(), Value::Text("hello".into())),
            ("b".into(), Value::Text(" world".into())),
        ]);
        let out = r.call("concat_text", inputs).unwrap();
        assert_eq!(out["out"], Value::Text("hello world".into()));
    }
    #[test]
    fn not_bool() {
        let r = reg();
        let inputs = HashMap::from([("a".into(), Value::Bool(true))]);
        let out = r.call("not_bool", inputs).unwrap();
        assert_eq!(out["out"], Value::Bool(false));
    }
    #[test]
    fn and_bool() {
        let r = reg();
        let inputs = HashMap::from([("a".into(), Value::Bool(true)), ("b".into(), Value::Bool(false))]);
        let out = r.call("and_bool", inputs).unwrap();
        assert_eq!(out["out"], Value::Bool(false));
    }
    #[test]
    fn or_bool() {
        let r = reg();
        let inputs = HashMap::from([("a".into(), Value::Bool(false)), ("b".into(), Value::Bool(true))]);
        let out = r.call("or_bool", inputs).unwrap();
        assert_eq!(out["out"], Value::Bool(true));
    }
    #[test]
    fn missing_input_returns_error() {
        let r = reg();
        let inputs = HashMap::from([("a".into(), Value::Int(1))]);
        let err = r.call("add_int", inputs).unwrap_err();
        assert!(err.to_string().contains("missing input") || err.to_string().contains("b"));
    }
}
