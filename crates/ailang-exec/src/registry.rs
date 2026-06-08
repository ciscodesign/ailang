use crate::value::Value;
use std::collections::HashMap;
pub type Inputs = HashMap<String, Value>;
pub type Outputs = HashMap<String, Value>;
#[derive(Debug, thiserror::Error)]
pub enum ExecError {
    #[error("unknown node kind: {0}")]
    UnknownKind(String),
    #[error("missing input: {0}")]
    MissingInput(String),
    #[error("execution failed: {0}")]
    Failed(String),
}
pub type ExecFn = Box<dyn Fn(Inputs) -> Result<Outputs, ExecError> + Send + Sync>;
pub struct NodeRegistry {
    fns: HashMap<String, ExecFn>,
}
impl Default for NodeRegistry {
    fn default() -> Self {
        Self::new()
    }
}
impl NodeRegistry {
    pub fn new() -> Self {
        Self { fns: HashMap::new() }
    }
    /// Register a node kind with its execution function.
    pub fn register(&mut self, kind: impl Into<String>, f: ExecFn) {
        self.fns.insert(kind.into(), f);
    }
    /// Look up and call a node's function.
    pub fn call(&self, kind: &str, inputs: Inputs) -> Result<Outputs, ExecError> {
        match self.fns.get(kind) {
            Some(f) => f(inputs),
            None => Err(ExecError::UnknownKind(kind.to_string())),
        }
    }
    /// Register the built-in Const node: kind "Const:<port_name>",
    /// takes no inputs, returns { port_name → value }.
    pub fn register_const(&mut self, port_name: impl Into<String>, value: Value) {
        let port_name = port_name.into();
        let const_kind = format!("Const:{}", port_name);
        let f = Box::new(move |_inputs: Inputs| {
            let mut outputs = HashMap::new();
            outputs.insert(port_name.clone(), value.clone());
            Ok(outputs)
        });
        self.register(const_kind, f);
    }
}
