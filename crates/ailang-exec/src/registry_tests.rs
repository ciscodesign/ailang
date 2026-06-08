#[cfg(test)]
mod tests {
    use crate::{value::Value, registry::{NodeRegistry, ExecError, Inputs}};
    use std::collections::HashMap;
    #[test]
    fn unknown_kind_errors() {
        let r = NodeRegistry::new();
        assert!(matches!(r.call("NoSuch", HashMap::new()), Err(ExecError::UnknownKind(_))));
    }
    #[test]
    fn const_node_returns_value() {
        let mut r = NodeRegistry::new();
        r.register_const("out", Value::Int(42));
        let out = r.call("Const:out", HashMap::new()).unwrap();
        assert_eq!(out["out"], Value::Int(42));
    }
    #[test]
    fn custom_node_registered_and_called() {
        let mut r = NodeRegistry::new();
        r.register("double", Box::new(|inputs: Inputs| {
            let n = match inputs.get("x").cloned() {
                Some(Value::Int(n)) => n,
                _ => return Err(crate::registry::ExecError::MissingInput("x".into())),
            };
            let mut out = std::collections::HashMap::new();
            out.insert("y".to_string(), Value::Int(n * 2));
            Ok(out)
        }));
        let mut inp = HashMap::new();
        inp.insert("x".to_string(), Value::Int(7));
        let out = r.call("double", inp).unwrap();
        assert_eq!(out["y"], Value::Int(14));
    }
}
