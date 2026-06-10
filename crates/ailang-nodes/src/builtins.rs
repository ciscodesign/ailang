use std::collections::{BTreeMap, HashMap};
use ailang_exec::registry::{ExecError, Inputs, NodeRegistry, Outputs};
use ailang_exec::value::Value;

pub fn register_const_literal(registry: &mut NodeRegistry, kind: &str) {
    let rest = kind.strip_prefix("Const:").unwrap_or(kind);
    let (port, literal) = match rest.find(':') {
        Some(pos) => (&rest[..pos], &rest[pos + 1..]),
        None => return,
    };
    let port_owned = port.to_string();
    let kind_owned = kind.to_string();
    let value: Value = if let Ok(n) = literal.parse::<i64>() {
        Value::Int(n)
    } else if let Ok(f) = literal.parse::<f64>() {
        Value::Float(f)
    } else if literal == "true" {
        Value::Bool(true)
    } else if literal == "false" {
        Value::Bool(false)
    } else {
        Value::Text(literal.to_string())
    };
    registry.register(kind_owned, Box::new(move |_inputs: Inputs| {
        Ok(HashMap::from([(port_owned.clone(), value.clone())]))
    }));
}

pub fn register_builtins(registry: &mut NodeRegistry) {
    registry.register("add_int", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let a = inputs.remove("a").ok_or_else(|| ExecError::MissingInput("a".into()))?;
        let b = inputs.remove("b").ok_or_else(|| ExecError::MissingInput("b".into()))?;
        match (a, b) {
            (Value::Int(x), Value::Int(y)) => Ok(HashMap::from([("out".into(), Value::Int(x + y))])),
            _ => Err(ExecError::Failed("add_int: expected Int inputs".into())),
        }
    }));

    registry.register("sub_int", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let a = inputs.remove("a").ok_or_else(|| ExecError::MissingInput("a".into()))?;
        let b = inputs.remove("b").ok_or_else(|| ExecError::MissingInput("b".into()))?;
        match (a, b) {
            (Value::Int(x), Value::Int(y)) => Ok(HashMap::from([("out".into(), Value::Int(x - y))])),
            _ => Err(ExecError::Failed("sub_int: expected Int inputs".into())),
        }
    }));

    registry.register("mul_int", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let a = inputs.remove("a").ok_or_else(|| ExecError::MissingInput("a".into()))?;
        let b = inputs.remove("b").ok_or_else(|| ExecError::MissingInput("b".into()))?;
        match (a, b) {
            (Value::Int(x), Value::Int(y)) => Ok(HashMap::from([("out".into(), Value::Int(x * y))])),
            _ => Err(ExecError::Failed("mul_int: expected Int inputs".into())),
        }
    }));

    registry.register("add_float", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let a = inputs.remove("a").ok_or_else(|| ExecError::MissingInput("a".into()))?;
        let b = inputs.remove("b").ok_or_else(|| ExecError::MissingInput("b".into()))?;
        match (a, b) {
            (Value::Float(x), Value::Float(y)) => Ok(HashMap::from([("out".into(), Value::Float(x + y))])),
            _ => Err(ExecError::Failed("add_float: expected Float inputs".into())),
        }
    }));

    registry.register("concat_text", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let a = inputs.remove("a").ok_or_else(|| ExecError::MissingInput("a".into()))?;
        let b = inputs.remove("b").ok_or_else(|| ExecError::MissingInput("b".into()))?;
        match (a, b) {
            (Value::Text(x), Value::Text(y)) => Ok(HashMap::from([("out".into(), Value::Text(x + &y))])),
            _ => Err(ExecError::Failed("concat_text: expected Text inputs".into())),
        }
    }));

    registry.register("not_bool", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let a = inputs.remove("a").ok_or_else(|| ExecError::MissingInput("a".into()))?;
        match a {
            Value::Bool(b) => Ok(HashMap::from([("out".into(), Value::Bool(!b))])),
            _ => Err(ExecError::Failed("not_bool: expected Bool input".into())),
        }
    }));

    registry.register("and_bool", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let a = inputs.remove("a").ok_or_else(|| ExecError::MissingInput("a".into()))?;
        let b = inputs.remove("b").ok_or_else(|| ExecError::MissingInput("b".into()))?;
        match (a, b) {
            (Value::Bool(x), Value::Bool(y)) => Ok(HashMap::from([("out".into(), Value::Bool(x && y))])),
            _ => Err(ExecError::Failed("and_bool: expected Bool inputs".into())),
        }
    }));

    registry.register("or_bool", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let a = inputs.remove("a").ok_or_else(|| ExecError::MissingInput("a".into()))?;
        let b = inputs.remove("b").ok_or_else(|| ExecError::MissingInput("b".into()))?;
        match (a, b) {
            (Value::Bool(x), Value::Bool(y)) => Ok(HashMap::from([("out".into(), Value::Bool(x || y))])),
            _ => Err(ExecError::Failed("or_bool: expected Bool inputs".into())),
        }
    }));

    registry.register("eq_int", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let a = inputs.remove("a").ok_or_else(|| ExecError::MissingInput("a".into()))?;
        let b = inputs.remove("b").ok_or_else(|| ExecError::MissingInput("b".into()))?;
        match (a, b) {
            (Value::Int(x), Value::Int(y)) => Ok(HashMap::from([("out".into(), Value::Bool(x == y))])),
            _ => Err(ExecError::Failed("eq_int: expected Int inputs".into())),
        }
    }));

    registry.register("lt_int", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let a = inputs.remove("a").ok_or_else(|| ExecError::MissingInput("a".into()))?;
        let b = inputs.remove("b").ok_or_else(|| ExecError::MissingInput("b".into()))?;
        match (a, b) {
            (Value::Int(x), Value::Int(y)) => Ok(HashMap::from([("out".into(), Value::Bool(x < y))])),
            _ => Err(ExecError::Failed("lt_int: expected Int inputs".into())),
        }
    }));

    registry.register("if_int", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let cond  = inputs.remove("cond") .ok_or_else(|| ExecError::MissingInput("cond".into()))?;
        let then  = inputs.remove("then") .ok_or_else(|| ExecError::MissingInput("then".into()))?;
        let else_ = inputs.remove("else_").ok_or_else(|| ExecError::MissingInput("else_".into()))?;
        match (cond, then, else_) {
            (Value::Bool(c), Value::Int(t), Value::Int(e)) =>
                Ok(HashMap::from([("out".into(), Value::Int(if c { t } else { e }))])),
            _ => Err(ExecError::Failed("if_int: type mismatch".into())),
        }
    }));

    registry.register("len_text", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let a = inputs.remove("a").ok_or_else(|| ExecError::MissingInput("a".into()))?;
        match a {
            Value::Text(s) => Ok(HashMap::from([("out".into(), Value::Int(s.len() as i64))])),
            _ => Err(ExecError::Failed("len_text: expected Text input".into())),
        }
    }));

    registry.register("list_empty", Box::new(|_inputs: Inputs| -> Result<Outputs, ExecError> {
        Ok(HashMap::from([("out".into(), Value::List(vec![]))]))
    }));

    registry.register("list_push", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let list = inputs.remove("list").ok_or_else(|| ExecError::MissingInput("list".into()))?;
        let item = inputs.remove("item").ok_or_else(|| ExecError::MissingInput("item".into()))?;
        match list {
            Value::List(mut v) => { v.push(item); Ok(HashMap::from([("out".into(), Value::List(v))])) }
            _ => Err(ExecError::Failed("list_push: expected List".into())),
        }
    }));

    registry.register("list_head", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let list = inputs.remove("list").ok_or_else(|| ExecError::MissingInput("list".into()))?;
        match list {
            Value::List(v) => {
                let head = v.into_iter().next().map(Box::new);
                Ok(HashMap::from([("out".into(), Value::Option(head))]))
            }
            _ => Err(ExecError::Failed("list_head: expected List".into())),
        }
    }));

    registry.register("list_tail", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let list = inputs.remove("list").ok_or_else(|| ExecError::MissingInput("list".into()))?;
        match list {
            Value::List(mut v) => {
                if !v.is_empty() { v.remove(0); }
                Ok(HashMap::from([("out".into(), Value::List(v))]))
            }
            _ => Err(ExecError::Failed("list_tail: expected List".into())),
        }
    }));

    registry.register("list_len", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let list = inputs.remove("list").ok_or_else(|| ExecError::MissingInput("list".into()))?;
        match list {
            Value::List(v) => Ok(HashMap::from([("out".into(), Value::Int(v.len() as i64))])),
            _ => Err(ExecError::Failed("list_len: expected List".into())),
        }
    }));

    registry.register("list_int_sum", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let list = inputs.remove("list").ok_or_else(|| ExecError::MissingInput("list".into()))?;
        match list {
            Value::List(v) => {
                let mut sum = 0i64;
                for item in v {
                    match item {
                        Value::Int(n) => sum += n,
                        _ => return Err(ExecError::Failed("list_int_sum: non-Int item".into())),
                    }
                }
                Ok(HashMap::from([("out".into(), Value::Int(sum))]))
            }
            _ => Err(ExecError::Failed("list_int_sum: expected List".into())),
        }
    }));

    registry.register("neg_int", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let a = inputs.remove("a").ok_or_else(|| ExecError::MissingInput("a".into()))?;
        match a {
            Value::Int(x) => Ok(HashMap::from([("out".into(), Value::Int(-x))])),
            _ => Err(ExecError::Failed("neg_int: expected Int".into())),
        }
    }));

    registry.register("div_int", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let a = inputs.remove("a").ok_or_else(|| ExecError::MissingInput("a".into()))?;
        let b = inputs.remove("b").ok_or_else(|| ExecError::MissingInput("b".into()))?;
        match (a, b) {
            (Value::Int(x), Value::Int(y)) => {
                if y == 0 { return Err(ExecError::Failed("div_int: division by zero".into())); }
                Ok(HashMap::from([("out".into(), Value::Int(x / y))]))
            }
            _ => Err(ExecError::Failed("div_int: expected Int inputs".into())),
        }
    }));

    registry.register("mod_int", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let a = inputs.remove("a").ok_or_else(|| ExecError::MissingInput("a".into()))?;
        let b = inputs.remove("b").ok_or_else(|| ExecError::MissingInput("b".into()))?;
        match (a, b) {
            (Value::Int(x), Value::Int(y)) => {
                if y == 0 { return Err(ExecError::Failed("mod_int: modulo by zero".into())); }
                Ok(HashMap::from([("out".into(), Value::Int(x % y))]))
            }
            _ => Err(ExecError::Failed("mod_int: expected Int inputs".into())),
        }
    }));

    registry.register("gt_int", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let a = inputs.remove("a").ok_or_else(|| ExecError::MissingInput("a".into()))?;
        let b = inputs.remove("b").ok_or_else(|| ExecError::MissingInput("b".into()))?;
        match (a, b) {
            (Value::Int(x), Value::Int(y)) => Ok(HashMap::from([("out".into(), Value::Bool(x > y))])),
            _ => Err(ExecError::Failed("gt_int: expected Int inputs".into())),
        }
    }));

    registry.register("abs_int", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let a = inputs.remove("a").ok_or_else(|| ExecError::MissingInput("a".into()))?;
        match a {
            Value::Int(x) => Ok(HashMap::from([("out".into(), Value::Int(x.abs()))])),
            _ => Err(ExecError::Failed("abs_int: expected Int".into())),
        }
    }));

    registry.register("min_int", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let a = inputs.remove("a").ok_or_else(|| ExecError::MissingInput("a".into()))?;
        let b = inputs.remove("b").ok_or_else(|| ExecError::MissingInput("b".into()))?;
        match (a, b) {
            (Value::Int(x), Value::Int(y)) => Ok(HashMap::from([("out".into(), Value::Int(x.min(y)))])),
            _ => Err(ExecError::Failed("min_int: expected Int inputs".into())),
        }
    }));

    registry.register("max_int", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let a = inputs.remove("a").ok_or_else(|| ExecError::MissingInput("a".into()))?;
        let b = inputs.remove("b").ok_or_else(|| ExecError::MissingInput("b".into()))?;
        match (a, b) {
            (Value::Int(x), Value::Int(y)) => Ok(HashMap::from([("out".into(), Value::Int(x.max(y)))])),
            _ => Err(ExecError::Failed("max_int: expected Int inputs".into())),
        }
    }));

    registry.register("int_to_text", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let a = inputs.remove("a").ok_or_else(|| ExecError::MissingInput("a".into()))?;
        match a {
            Value::Int(x) => Ok(HashMap::from([("out".into(), Value::Text(x.to_string()))])),
            _ => Err(ExecError::Failed("int_to_text: expected Int".into())),
        }
    }));

    registry.register("bool_to_text", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let a = inputs.remove("a").ok_or_else(|| ExecError::MissingInput("a".into()))?;
        match a {
            Value::Bool(b) => Ok(HashMap::from([("out".into(), Value::Text(b.to_string()))])),
            _ => Err(ExecError::Failed("bool_to_text: expected Bool".into())),
        }
    }));

    registry.register("map_empty", Box::new(|_inputs: Inputs| -> Result<Outputs, ExecError> {
        Ok(HashMap::from([("out".into(), Value::Map(BTreeMap::new()))]))
    }));

    registry.register("map_set", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let map  = inputs.remove("map").ok_or_else(|| ExecError::MissingInput("map".into()))?;
        let key  = inputs.remove("key").ok_or_else(|| ExecError::MissingInput("key".into()))?;
        let val  = inputs.remove("val").ok_or_else(|| ExecError::MissingInput("val".into()))?;
        match (map, key) {
            (Value::Map(mut m), Value::Text(k)) => { m.insert(k, val); Ok(HashMap::from([("out".into(), Value::Map(m))])) }
            _ => Err(ExecError::Failed("map_set: expected Map and Text key".into())),
        }
    }));

    registry.register("map_get", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let map = inputs.remove("map").ok_or_else(|| ExecError::MissingInput("map".into()))?;
        let key = inputs.remove("key").ok_or_else(|| ExecError::MissingInput("key".into()))?;
        match (map, key) {
            (Value::Map(m), Value::Text(k)) => {
                let v = m.get(&k).cloned().map(Box::new);
                Ok(HashMap::from([("out".into(), Value::Option(v))]))
            }
            _ => Err(ExecError::Failed("map_get: expected Map and Text key".into())),
        }
    }));

    registry.register("map_contains", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let map = inputs.remove("map").ok_or_else(|| ExecError::MissingInput("map".into()))?;
        let key = inputs.remove("key").ok_or_else(|| ExecError::MissingInput("key".into()))?;
        match (map, key) {
            (Value::Map(m), Value::Text(k)) => Ok(HashMap::from([("out".into(), Value::Bool(m.contains_key(&k)))])),
            _ => Err(ExecError::Failed("map_contains: expected Map and Text key".into())),
        }
    }));

    registry.register("map_keys", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let map = inputs.remove("map").ok_or_else(|| ExecError::MissingInput("map".into()))?;
        match map {
            Value::Map(m) => {
                let keys = m.into_keys().map(Value::Text).collect();
                Ok(HashMap::from([("out".into(), Value::List(keys))]))
            }
            _ => Err(ExecError::Failed("map_keys: expected Map".into())),
        }
    }));

    registry.register("map_len", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let map = inputs.remove("map").ok_or_else(|| ExecError::MissingInput("map".into()))?;
        match map {
            Value::Map(m) => Ok(HashMap::from([("out".into(), Value::Int(m.len() as i64))])),
            _ => Err(ExecError::Failed("map_len: expected Map".into())),
        }
    }));

    // --- conditional ops (float / text / bool) ---

    registry.register("eq_float", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let a = inputs.remove("a").ok_or_else(|| ExecError::MissingInput("a".into()))?;
        let b = inputs.remove("b").ok_or_else(|| ExecError::MissingInput("b".into()))?;
        match (a, b) {
            (Value::Float(x), Value::Float(y)) => Ok(HashMap::from([("out".into(), Value::Bool(x == y))])),
            _ => Err(ExecError::Failed("eq_float: expected Float inputs".into())),
        }
    }));

    registry.register("lt_float", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let a = inputs.remove("a").ok_or_else(|| ExecError::MissingInput("a".into()))?;
        let b = inputs.remove("b").ok_or_else(|| ExecError::MissingInput("b".into()))?;
        match (a, b) {
            (Value::Float(x), Value::Float(y)) => Ok(HashMap::from([("out".into(), Value::Bool(x < y))])),
            _ => Err(ExecError::Failed("lt_float: expected Float inputs".into())),
        }
    }));

    registry.register("gt_float", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let a = inputs.remove("a").ok_or_else(|| ExecError::MissingInput("a".into()))?;
        let b = inputs.remove("b").ok_or_else(|| ExecError::MissingInput("b".into()))?;
        match (a, b) {
            (Value::Float(x), Value::Float(y)) => Ok(HashMap::from([("out".into(), Value::Bool(x > y))])),
            _ => Err(ExecError::Failed("gt_float: expected Float inputs".into())),
        }
    }));

    registry.register("if_float", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let cond  = inputs.remove("cond") .ok_or_else(|| ExecError::MissingInput("cond".into()))?;
        let then  = inputs.remove("then") .ok_or_else(|| ExecError::MissingInput("then".into()))?;
        let else_ = inputs.remove("else_").ok_or_else(|| ExecError::MissingInput("else_".into()))?;
        match (cond, then, else_) {
            (Value::Bool(c), Value::Float(t), Value::Float(e)) =>
                Ok(HashMap::from([("out".into(), Value::Float(if c { t } else { e }))])),
            _ => Err(ExecError::Failed("if_float: type mismatch".into())),
        }
    }));

    registry.register("if_text", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let cond  = inputs.remove("cond") .ok_or_else(|| ExecError::MissingInput("cond".into()))?;
        let then  = inputs.remove("then") .ok_or_else(|| ExecError::MissingInput("then".into()))?;
        let else_ = inputs.remove("else_").ok_or_else(|| ExecError::MissingInput("else_".into()))?;
        match (cond, then, else_) {
            (Value::Bool(c), Value::Text(t), Value::Text(e)) =>
                Ok(HashMap::from([("out".into(), Value::Text(if c { t } else { e }))])),
            _ => Err(ExecError::Failed("if_text: type mismatch".into())),
        }
    }));

    registry.register("if_bool", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let cond  = inputs.remove("cond") .ok_or_else(|| ExecError::MissingInput("cond".into()))?;
        let then  = inputs.remove("then") .ok_or_else(|| ExecError::MissingInput("then".into()))?;
        let else_ = inputs.remove("else_").ok_or_else(|| ExecError::MissingInput("else_".into()))?;
        match (cond, then, else_) {
            (Value::Bool(c), Value::Bool(t), Value::Bool(e)) =>
                Ok(HashMap::from([("out".into(), Value::Bool(if c { t } else { e }))])),
            _ => Err(ExecError::Failed("if_bool: type mismatch".into())),
        }
    }));

    // --- float builtins ---

    registry.register("sub_float", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let a = inputs.remove("a").ok_or_else(|| ExecError::MissingInput("a".into()))?;
        let b = inputs.remove("b").ok_or_else(|| ExecError::MissingInput("b".into()))?;
        match (a, b) {
            (Value::Float(x), Value::Float(y)) => Ok(HashMap::from([("out".into(), Value::Float(x - y))])),
            _ => Err(ExecError::Failed("sub_float: expected Float inputs".into())),
        }
    }));

    registry.register("mul_float", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let a = inputs.remove("a").ok_or_else(|| ExecError::MissingInput("a".into()))?;
        let b = inputs.remove("b").ok_or_else(|| ExecError::MissingInput("b".into()))?;
        match (a, b) {
            (Value::Float(x), Value::Float(y)) => Ok(HashMap::from([("out".into(), Value::Float(x * y))])),
            _ => Err(ExecError::Failed("mul_float: expected Float inputs".into())),
        }
    }));

    registry.register("div_float", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let a = inputs.remove("a").ok_or_else(|| ExecError::MissingInput("a".into()))?;
        let b = inputs.remove("b").ok_or_else(|| ExecError::MissingInput("b".into()))?;
        match (a, b) {
            (Value::Float(x), Value::Float(y)) => Ok(HashMap::from([("out".into(), Value::Float(x / y))])),
            _ => Err(ExecError::Failed("div_float: expected Float inputs".into())),
        }
    }));

    registry.register("neg_float", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let a = inputs.remove("a").ok_or_else(|| ExecError::MissingInput("a".into()))?;
        match a {
            Value::Float(x) => Ok(HashMap::from([("out".into(), Value::Float(-x))])),
            _ => Err(ExecError::Failed("neg_float: expected Float".into())),
        }
    }));

    registry.register("abs_float", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let a = inputs.remove("a").ok_or_else(|| ExecError::MissingInput("a".into()))?;
        match a {
            Value::Float(x) => Ok(HashMap::from([("out".into(), Value::Float(x.abs()))])),
            _ => Err(ExecError::Failed("abs_float: expected Float".into())),
        }
    }));

    registry.register("floor_float", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let a = inputs.remove("a").ok_or_else(|| ExecError::MissingInput("a".into()))?;
        match a {
            Value::Float(x) => Ok(HashMap::from([("out".into(), Value::Int(x.floor() as i64))])),
            _ => Err(ExecError::Failed("floor_float: expected Float".into())),
        }
    }));

    registry.register("ceil_float", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let a = inputs.remove("a").ok_or_else(|| ExecError::MissingInput("a".into()))?;
        match a {
            Value::Float(x) => Ok(HashMap::from([("out".into(), Value::Int(x.ceil() as i64))])),
            _ => Err(ExecError::Failed("ceil_float: expected Float".into())),
        }
    }));

    registry.register("round_float", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let a = inputs.remove("a").ok_or_else(|| ExecError::MissingInput("a".into()))?;
        match a {
            Value::Float(x) => Ok(HashMap::from([("out".into(), Value::Int(x.round() as i64))])),
            _ => Err(ExecError::Failed("round_float: expected Float".into())),
        }
    }));

    registry.register("int_to_float", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let a = inputs.remove("a").ok_or_else(|| ExecError::MissingInput("a".into()))?;
        match a {
            Value::Int(x) => Ok(HashMap::from([("out".into(), Value::Float(x as f64))])),
            _ => Err(ExecError::Failed("int_to_float: expected Int".into())),
        }
    }));

    registry.register("float_to_int", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let a = inputs.remove("a").ok_or_else(|| ExecError::MissingInput("a".into()))?;
        match a {
            Value::Float(x) => Ok(HashMap::from([("out".into(), Value::Int(x as i64))])),
            _ => Err(ExecError::Failed("float_to_int: expected Float".into())),
        }
    }));

    registry.register("float_to_text", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let a = inputs.remove("a").ok_or_else(|| ExecError::MissingInput("a".into()))?;
        match a {
            Value::Float(x) => Ok(HashMap::from([("out".into(), Value::Text(x.to_string()))])),
            _ => Err(ExecError::Failed("float_to_text: expected Float".into())),
        }
    }));

    // --- string ops ---

    registry.register("trim_text", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let a = inputs.remove("a").ok_or_else(|| ExecError::MissingInput("a".into()))?;
        match a {
            Value::Text(s) => Ok(HashMap::from([("out".into(), Value::Text(s.trim().to_string()))])),
            _ => Err(ExecError::Failed("trim_text: expected Text".into())),
        }
    }));

    registry.register("to_upper_text", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let a = inputs.remove("a").ok_or_else(|| ExecError::MissingInput("a".into()))?;
        match a {
            Value::Text(s) => Ok(HashMap::from([("out".into(), Value::Text(s.to_uppercase()))])),
            _ => Err(ExecError::Failed("to_upper_text: expected Text".into())),
        }
    }));

    registry.register("to_lower_text", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let a = inputs.remove("a").ok_or_else(|| ExecError::MissingInput("a".into()))?;
        match a {
            Value::Text(s) => Ok(HashMap::from([("out".into(), Value::Text(s.to_lowercase()))])),
            _ => Err(ExecError::Failed("to_lower_text: expected Text".into())),
        }
    }));

    registry.register("contains_text", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let a = inputs.remove("a").ok_or_else(|| ExecError::MissingInput("a".into()))?;
        let b = inputs.remove("b").ok_or_else(|| ExecError::MissingInput("b".into()))?;
        match (a, b) {
            (Value::Text(s), Value::Text(needle)) => Ok(HashMap::from([("out".into(), Value::Bool(s.contains(needle.as_str())))])),
            _ => Err(ExecError::Failed("contains_text: expected Text inputs".into())),
        }
    }));

    registry.register("starts_with_text", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let a = inputs.remove("a").ok_or_else(|| ExecError::MissingInput("a".into()))?;
        let b = inputs.remove("b").ok_or_else(|| ExecError::MissingInput("b".into()))?;
        match (a, b) {
            (Value::Text(s), Value::Text(prefix)) => Ok(HashMap::from([("out".into(), Value::Bool(s.starts_with(prefix.as_str())))])),
            _ => Err(ExecError::Failed("starts_with_text: expected Text inputs".into())),
        }
    }));

    registry.register("ends_with_text", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let a = inputs.remove("a").ok_or_else(|| ExecError::MissingInput("a".into()))?;
        let b = inputs.remove("b").ok_or_else(|| ExecError::MissingInput("b".into()))?;
        match (a, b) {
            (Value::Text(s), Value::Text(suffix)) => Ok(HashMap::from([("out".into(), Value::Bool(s.ends_with(suffix.as_str())))])),
            _ => Err(ExecError::Failed("ends_with_text: expected Text inputs".into())),
        }
    }));

    registry.register("replace_text", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let a    = inputs.remove("a")   .ok_or_else(|| ExecError::MissingInput("a".into()))?;
        let from = inputs.remove("from").ok_or_else(|| ExecError::MissingInput("from".into()))?;
        let to   = inputs.remove("to")  .ok_or_else(|| ExecError::MissingInput("to".into()))?;
        match (a, from, to) {
            (Value::Text(s), Value::Text(f), Value::Text(t)) =>
                Ok(HashMap::from([("out".into(), Value::Text(s.replace(f.as_str(), t.as_str())))])),
            _ => Err(ExecError::Failed("replace_text: expected Text inputs".into())),
        }
    }));

    registry.register("split_text", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let a   = inputs.remove("a")  .ok_or_else(|| ExecError::MissingInput("a".into()))?;
        let sep = inputs.remove("sep").ok_or_else(|| ExecError::MissingInput("sep".into()))?;
        match (a, sep) {
            (Value::Text(s), Value::Text(d)) => {
                let parts = s.split(d.as_str()).map(|p| Value::Text(p.to_string())).collect();
                Ok(HashMap::from([("out".into(), Value::List(parts))]))
            }
            _ => Err(ExecError::Failed("split_text: expected Text inputs".into())),
        }
    }));

    registry.register("join_text", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let list = inputs.remove("list").ok_or_else(|| ExecError::MissingInput("list".into()))?;
        let sep  = inputs.remove("sep") .ok_or_else(|| ExecError::MissingInput("sep".into()))?;
        match (list, sep) {
            (Value::List(items), Value::Text(d)) => {
                let mut parts = Vec::with_capacity(items.len());
                for item in items {
                    match item {
                        Value::Text(s) => parts.push(s),
                        _ => return Err(ExecError::Failed("join_text: list contains non-Text item".into())),
                    }
                }
                Ok(HashMap::from([("out".into(), Value::Text(parts.join(d.as_str())))]))
            }
            _ => Err(ExecError::Failed("join_text: expected List and Text sep".into())),
        }
    }));

    registry.register("slice_text", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let a     = inputs.remove("a")    .ok_or_else(|| ExecError::MissingInput("a".into()))?;
        let start = inputs.remove("start").ok_or_else(|| ExecError::MissingInput("start".into()))?;
        let end   = inputs.remove("end")  .ok_or_else(|| ExecError::MissingInput("end".into()))?;
        match (a, start, end) {
            (Value::Text(s), Value::Int(lo), Value::Int(hi)) => {
                let lo = lo.max(0) as usize;
                let hi = (hi as usize).min(s.len());
                let slice = if lo <= hi { s[lo..hi].to_string() } else { String::new() };
                Ok(HashMap::from([("out".into(), Value::Text(slice))]))
            }
            _ => Err(ExecError::Failed("slice_text: expected Text, Int, Int".into())),
        }
    }));
}
