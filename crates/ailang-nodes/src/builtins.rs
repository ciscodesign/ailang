use std::collections::HashMap;
use ailang_exec::registry::{ExecError, Inputs, NodeRegistry, Outputs};
use ailang_exec::value::Value;

pub fn register_builtins(registry: &mut NodeRegistry) {
    // add_int: a + b
    registry.register("add_int", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let a = inputs.remove("a").ok_or_else(|| ExecError::MissingInput("a".into()))?;
        let b = inputs.remove("b").ok_or_else(|| ExecError::MissingInput("b".into()))?;
        match (a, b) {
            (Value::Int(x), Value::Int(y)) => Ok(HashMap::from([("out".into(), Value::Int(x + y))])),
            _ => Err(ExecError::Failed("add_int: expected Int inputs".into())),
        }
    }));

    // sub_int: a - b
    registry.register("sub_int", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let a = inputs.remove("a").ok_or_else(|| ExecError::MissingInput("a".into()))?;
        let b = inputs.remove("b").ok_or_else(|| ExecError::MissingInput("b".into()))?;
        match (a, b) {
            (Value::Int(x), Value::Int(y)) => Ok(HashMap::from([("out".into(), Value::Int(x - y))])),
            _ => Err(ExecError::Failed("sub_int: expected Int inputs".into())),
        }
    }));

    // mul_int: a * b
    registry.register("mul_int", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let a = inputs.remove("a").ok_or_else(|| ExecError::MissingInput("a".into()))?;
        let b = inputs.remove("b").ok_or_else(|| ExecError::MissingInput("b".into()))?;
        match (a, b) {
            (Value::Int(x), Value::Int(y)) => Ok(HashMap::from([("out".into(), Value::Int(x * y))])),
            _ => Err(ExecError::Failed("mul_int: expected Int inputs".into())),
        }
    }));

    // neg_int: -a
    registry.register("neg_int", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let a = inputs.remove("a").ok_or_else(|| ExecError::MissingInput("a".into()))?;
        match a {
            Value::Int(x) => Ok(HashMap::from([("out".into(), Value::Int(-x))])),
            _ => Err(ExecError::Failed("neg_int: expected Int input".into())),
        }
    }));

    // concat_text: a ++ b
    registry.register("concat_text", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let a = inputs.remove("a").ok_or_else(|| ExecError::MissingInput("a".into()))?;
        let b = inputs.remove("b").ok_or_else(|| ExecError::MissingInput("b".into()))?;
        match (a, b) {
            (Value::Text(x), Value::Text(y)) => Ok(HashMap::from([("out".into(), Value::Text(x + &y))])),
            _ => Err(ExecError::Failed("concat_text: expected Text inputs".into())),
        }
    }));

    // not_bool: !a
    registry.register("not_bool", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let a = inputs.remove("a").ok_or_else(|| ExecError::MissingInput("a".into()))?;
        match a {
            Value::Bool(x) => Ok(HashMap::from([("out".into(), Value::Bool(!x))])),
            _ => Err(ExecError::Failed("not_bool: expected Bool input".into())),
        }
    }));

    // and_bool: a && b
    registry.register("and_bool", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let a = inputs.remove("a").ok_or_else(|| ExecError::MissingInput("a".into()))?;
        let b = inputs.remove("b").ok_or_else(|| ExecError::MissingInput("b".into()))?;
        match (a, b) {
            (Value::Bool(x), Value::Bool(y)) => Ok(HashMap::from([("out".into(), Value::Bool(x && y))])),
            _ => Err(ExecError::Failed("and_bool: expected Bool inputs".into())),
        }
    }));

    // or_bool: a || b
    registry.register("or_bool", Box::new(|mut inputs: Inputs| -> Result<Outputs, ExecError> {
        let a = inputs.remove("a").ok_or_else(|| ExecError::MissingInput("a".into()))?;
        let b = inputs.remove("b").ok_or_else(|| ExecError::MissingInput("b".into()))?;
        match (a, b) {
            (Value::Bool(x), Value::Bool(y)) => Ok(HashMap::from([("out".into(), Value::Bool(x || y))])),
            _ => Err(ExecError::Failed("or_bool: expected Bool inputs".into())),
        }
    }));
}
