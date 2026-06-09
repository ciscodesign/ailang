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
    List(Vec<Value>),
}
impl Value {
    pub fn matches_type(&self, ty: &Type) -> bool {
        matches!((self, ty),
            (Value::Text(_), Type::Text)
            | (Value::Int(_), Type::Int)
            | (Value::Float(_), Type::Float)
            | (Value::Bool(_), Type::Bool)
            | (Value::Bytes(_), Type::Bytes)
            | (Value::Option(_), Type::Option(_))
            | (Value::Result(_), Type::Result(_, _))
            | (Value::List(_), Type::List(_))
            | (_, Type::Var(_))
        )
    }
}
