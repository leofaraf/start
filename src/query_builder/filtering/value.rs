#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Integer(i32),
    Float(f64),
    Boolean(bool),
    Null,
}

impl Value {
    // Converts from a String
    pub fn from_str(s: String) -> Self {
        Value::String(s)
    }

    // Converts from an integer
    pub fn from_int(i: i32) -> Self {
        Value::Integer(i)
    }

    // Converts from a float
    pub fn from_float(f: f64) -> Self {
        Value::Float(f)
    }

    // Converts from a boolean
    pub fn from_bool(b: bool) -> Self {
        Value::Boolean(b)
    }

    // Checks if the value is a String
    pub fn as_str(&self) -> Option<&str> {
        if let Value::String(s) = self {
            Some(s)
        } else {
            None
        }
    }

    // Checks if the value is an Integer
    pub fn as_int(&self) -> Option<i32> {
        if let Value::Integer(i) = self {
            Some(*i)
        } else {
            None
        }
    }

    // Checks if the value is a Float
    pub fn as_float(&self) -> Option<f64> {
        if let Value::Float(f) = self {
            Some(*f)
        } else {
            None
        }
    }

    // Checks if the value is a Boolean
    pub fn as_bool(&self) -> Option<bool> {
        if let Value::Boolean(b) = self {
            Some(*b)
        } else {
            None
        }
    }

    // Helper method to get a JSON-style value
    pub fn to_string_value(&self) -> String {
        match self {
            Value::String(s) => format!("\"{}\"", s),
            Value::Integer(i) => i.to_string(),
            Value::Float(f) => f.to_string(),
            Value::Boolean(b) => b.to_string(),
            Value::Null => "null".to_string(),
        }
    }
}