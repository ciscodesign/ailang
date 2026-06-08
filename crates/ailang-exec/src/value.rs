use ailang_core::ty::Type;
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Text(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Bytes(Vec<u8>),
    Option(Option<Box<Value>>),
    Result(std::result::Result<Box<Value>, Box<Value>>),
}
impl Value {
    /// Returns true if this value is compatible with the given Type.
    /// Does not check recursively — top-level variant match only.
    pub fn matches_type(&self, ty: &Type) -> bool {
        match (self, ty) {
            (Value::Text(_), Type::Text) => true,
            (Value::Int(_), Type::Int) => true,
            (Value::Float(_), Type::Float) => true,
            (Value::Bool(_), Type::Bool) => true,
            (Value::Bytes(_), Type::Bytes) => true,
            (Value::Option(_), Type::Option(_)) => true,
            (Value::Result(_), Type::Result(_, _)) => true,
            (_, Type::Var(_)) => true, // any value matches Var
            _ => false,
        }
    }
}
