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

    #[test]
    fn list_empty_returns_empty() {
        let r = reg();
        let out = r.call("list_empty", HashMap::new()).unwrap();
        assert_eq!(out["out"], Value::List(vec![]));
    }

    #[test]
    fn list_push_appends() {
        let r = reg();
        let inputs = HashMap::from([
            ("list".into(), Value::List(vec![Value::Int(1)])),
            ("item".into(), Value::Int(2)),
        ]);
        let out = r.call("list_push", inputs).unwrap();
        assert_eq!(out["out"], Value::List(vec![Value::Int(1), Value::Int(2)]));
    }

    #[test]
    fn list_head_some() {
        let r = reg();
        let inputs = HashMap::from([("list".into(), Value::List(vec![Value::Int(42), Value::Int(7)]))]);
        let out = r.call("list_head", inputs).unwrap();
        assert_eq!(out["out"], Value::Option(Some(Box::new(Value::Int(42)))));
    }

    #[test]
    fn list_head_empty() {
        let r = reg();
        let inputs = HashMap::from([("list".into(), Value::List(vec![]))]);
        let out = r.call("list_head", inputs).unwrap();
        assert_eq!(out["out"], Value::Option(None));
    }

    #[test]
    fn list_tail_removes_first() {
        let r = reg();
        let inputs = HashMap::from([("list".into(), Value::List(vec![Value::Int(1), Value::Int(2), Value::Int(3)]))]);
        let out = r.call("list_tail", inputs).unwrap();
        assert_eq!(out["out"], Value::List(vec![Value::Int(2), Value::Int(3)]));
    }

    #[test]
    fn list_len() {
        let r = reg();
        let inputs = HashMap::from([("list".into(), Value::List(vec![Value::Int(1), Value::Int(2)]))]);
        let out = r.call("list_len", inputs).unwrap();
        assert_eq!(out["out"], Value::Int(2));
    }

    #[test]
    fn list_int_sum() {
        let r = reg();
        let inputs = HashMap::from([("list".into(), Value::List(vec![Value::Int(10), Value::Int(20), Value::Int(5)]))]);
        let out = r.call("list_int_sum", inputs).unwrap();
        assert_eq!(out["out"], Value::Int(35));
    }

    #[test]
    fn neg_int() {
        let r = reg();
        let out = r.call("neg_int", HashMap::from([("a".into(), Value::Int(7))])).unwrap();
        assert_eq!(out["out"], Value::Int(-7));
    }

    #[test]
    fn div_int() {
        let r = reg();
        let out = r.call("div_int", HashMap::from([("a".into(), Value::Int(10)), ("b".into(), Value::Int(3))])).unwrap();
        assert_eq!(out["out"], Value::Int(3));
    }

    #[test]
    fn div_int_by_zero() {
        let r = reg();
        assert!(r.call("div_int", HashMap::from([("a".into(), Value::Int(1)), ("b".into(), Value::Int(0))])).is_err());
    }

    #[test]
    fn mod_int() {
        let r = reg();
        let out = r.call("mod_int", HashMap::from([("a".into(), Value::Int(17)), ("b".into(), Value::Int(5))])).unwrap();
        assert_eq!(out["out"], Value::Int(2));
    }

    #[test]
    fn gt_int_true() {
        let r = reg();
        let out = r.call("gt_int", HashMap::from([("a".into(), Value::Int(9)), ("b".into(), Value::Int(3))])).unwrap();
        assert_eq!(out["out"], Value::Bool(true));
    }

    #[test]
    fn abs_int() {
        let r = reg();
        let out = r.call("abs_int", HashMap::from([("a".into(), Value::Int(-42))])).unwrap();
        assert_eq!(out["out"], Value::Int(42));
    }

    #[test]
    fn min_int() {
        let r = reg();
        let out = r.call("min_int", HashMap::from([("a".into(), Value::Int(3)), ("b".into(), Value::Int(7))])).unwrap();
        assert_eq!(out["out"], Value::Int(3));
    }

    #[test]
    fn max_int() {
        let r = reg();
        let out = r.call("max_int", HashMap::from([("a".into(), Value::Int(3)), ("b".into(), Value::Int(7))])).unwrap();
        assert_eq!(out["out"], Value::Int(7));
    }

    #[test]
    fn int_to_text() {
        let r = reg();
        let out = r.call("int_to_text", HashMap::from([("a".into(), Value::Int(42))])).unwrap();
        assert_eq!(out["out"], Value::Text("42".into()));
    }

    #[test]
    fn bool_to_text() {
        let r = reg();
        let out = r.call("bool_to_text", HashMap::from([("a".into(), Value::Bool(true))])).unwrap();
        assert_eq!(out["out"], Value::Text("true".into()));
    }

    #[test]
    fn map_empty_and_set_get() {
        let r = reg();
        let empty = r.call("map_empty", HashMap::new()).unwrap();
        let map = empty["out"].clone();
        let after_set = r.call("map_set", HashMap::from([
            ("map".into(), map),
            ("key".into(), Value::Text("x".into())),
            ("val".into(), Value::Int(99)),
        ])).unwrap();
        let out = r.call("map_get", HashMap::from([
            ("map".into(), after_set["out"].clone()),
            ("key".into(), Value::Text("x".into())),
        ])).unwrap();
        assert_eq!(out["out"], Value::Option(Some(Box::new(Value::Int(99)))));
    }

    #[test]
    fn map_contains() {
        let r = reg();
        use std::collections::BTreeMap;
        let mut m = BTreeMap::new();
        m.insert("hello".to_string(), Value::Int(1));
        let inputs = HashMap::from([
            ("map".into(), Value::Map(m)),
            ("key".into(), Value::Text("hello".into())),
        ]);
        let out = r.call("map_contains", inputs).unwrap();
        assert_eq!(out["out"], Value::Bool(true));
    }

    #[test]
    fn map_keys_returns_list() {
        let r = reg();
        use std::collections::BTreeMap;
        let mut m = BTreeMap::new();
        m.insert("a".to_string(), Value::Int(1));
        m.insert("b".to_string(), Value::Int(2));
        let out = r.call("map_keys", HashMap::from([("map".into(), Value::Map(m))])).unwrap();
        assert_eq!(out["out"], Value::List(vec![Value::Text("a".into()), Value::Text("b".into())]));
    }

    #[test]
    fn map_len() {
        let r = reg();
        use std::collections::BTreeMap;
        let mut m = BTreeMap::new();
        m.insert("k".to_string(), Value::Bool(true));
        let out = r.call("map_len", HashMap::from([("map".into(), Value::Map(m))])).unwrap();
        assert_eq!(out["out"], Value::Int(1));
    }
}
