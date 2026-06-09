use std::collections::HashMap;
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
}
