use std::collections::HashMap;

#[derive(Debug)]
pub enum Value {
    Str(String),
    Number(i32),
    Float(f32),
    Bool(bool),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
    Null
}
