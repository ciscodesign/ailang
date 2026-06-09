#[cfg(test)]
mod tests {
    use crate::builtins::{register_builtins, register_const_literal};
    use ailang_exec::registry::NodeRegistry;
    use ailang_exec::value::Value;
    use std::collections::HashMap;

    fn reg() -> NodeRegistry {
        let mut r = NodeRegistry::new();
        register_builtins(&mut r);
        r
    }

    #[test]
    fn register_const_literal_int() {
        let mut r = NodeRegistry::new();
        register_const_literal(&mut r, "Const:out:7");
        let out = r.call("Const:out:7", HashMap::new()).unwrap();
        assert_eq!(out["out"], Value::Int(7));
    }

    #[test]
    fn register_const_literal_bool() {
        let mut r = NodeRegistry::new();
        register_const_literal(&mut r, "Const:flag:true");
        let out = r.call("Const:flag:true", HashMap::new()).unwrap();
        assert_eq!(out["flag"], Value::Bool(true));
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
        let inputs = HashMap::from([("a".into(), Value::Int(10)), ("b".into(), Value::Int(4))]);
        let out = r.call("sub_int", inputs).unwrap();
        assert_eq!(out["out"], Value::Int(6));
    }

    #[test]
    fn mul_int() {
        let r = reg();
        let inputs = HashMap::from([("a".into(), Value::Int(3)), ("b".into(), Value::Int(5))]);
        let out = r.call("mul_int", inputs).unwrap();
        assert_eq!(out["out"], Value::Int(15));
    }

    #[test]
    fn add_float() {
        let r = reg();
        let inputs = HashMap::from([("a".into(), Value::Float(1.5)), ("b".into(), Value::Float(2.5))]);
        let out = r.call("add_float", inputs).unwrap();
        assert_eq!(out["out"], Value::Float(4.0));
    }

    #[test]
    fn concat_text() {
        let r = reg();
        let inputs = HashMap::from([("a".into(), Value::Text("hello".into())), ("b".into(), Value::Text(" world".into()))]);
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
    fn eq_int_true() {
        let r = reg();
        let inputs = HashMap::from([("a".into(), Value::Int(5)), ("b".into(), Value::Int(5))]);
        let out = r.call("eq_int", inputs).unwrap();
        assert_eq!(out["out"], Value::Bool(true));
    }

    #[test]
    fn eq_int_false() {
        let r = reg();
        let inputs = HashMap::from([("a".into(), Value::Int(3)), ("b".into(), Value::Int(4))]);
        let out = r.call("eq_int", inputs).unwrap();
        assert_eq!(out["out"], Value::Bool(false));
    }

    #[test]
    fn lt_int() {
        let r = reg();
        let inputs = HashMap::from([("a".into(), Value::Int(2)), ("b".into(), Value::Int(9))]);
        let out = r.call("lt_int", inputs).unwrap();
        assert_eq!(out["out"], Value::Bool(true));
    }

    #[test]
    fn if_int_true_branch() {
        let r = reg();
        let inputs = HashMap::from([
            ("cond".into(),  Value::Bool(true)),
            ("then".into(),  Value::Int(10)),
            ("else_".into(), Value::Int(20)),
        ]);
        let out = r.call("if_int", inputs).unwrap();
        assert_eq!(out["out"], Value::Int(10));
    }

    #[test]
    fn if_int_false_branch() {
        let r = reg();
        let inputs = HashMap::from([
            ("cond".into(),  Value::Bool(false)),
            ("then".into(),  Value::Int(10)),
            ("else_".into(), Value::Int(20)),
        ]);
        let out = r.call("if_int", inputs).unwrap();
        assert_eq!(out["out"], Value::Int(20));
    }

    #[test]
    fn len_text() {
        let r = reg();
        let inputs = HashMap::from([("a".into(), Value::Text("hello".into()))]);
        let out = r.call("len_text", inputs).unwrap();
        assert_eq!(out["out"], Value::Int(5));
    }
}
