use value::Value;

pub mod value;

#[derive(Debug, Clone)]
pub enum Filter {
    Eq(String, Value),
    Gt(String, Value),
    Lt(String, Value),
    Ge(String, Value),
    Le(String, Value),
    Like(String, String),
    And(Vec<Filter>),
    Or(Vec<Filter>),
    Not(Box<Filter>),
}

pub fn matches_filter(doc: &serde_json::Value, filter: &Filter) -> bool {
    use Filter::*;

    match filter {
        Eq(field, expected) => {
            doc.get(field).map(|val| json_val_eq(val, expected)).unwrap_or(false)
        }
        Gt(field, expected) => {
            doc.get(field).map(|val| json_val_gt(val, expected)).unwrap_or(false)
        }
        Lt(field, expected) => {
            doc.get(field).map(|val| json_val_lt(val, expected)).unwrap_or(false)
        }
        Like(field, pattern) => {
            doc.get(field)
                .and_then(|val| val.as_str())
                .map(|s| s.contains(pattern))
                .unwrap_or(false)
        }
        And(filters) => filters.iter().all(|f| matches_filter(doc, f)),
        Or(filters) => filters.iter().any(|f| matches_filter(doc, f)),
        Not(f) => !matches_filter(doc, f),
        _ => false,
    }
}

fn json_val_eq(val: &serde_json::Value, expected: &Value) -> bool {
    match expected {
        Value::String(s) => val.as_str().map(|v| v == s).unwrap_or(false),
        Value::Integer(i) => val.as_i64().map(|v| v == *i as i64).unwrap_or(false),
        Value::Float(f) => val.as_f64().map(|v| v == *f).unwrap_or(false),
        Value::Boolean(b) => val.as_bool().map(|v| v == *b).unwrap_or(false),
        Value::Null => val.is_null(),
    }
}

fn json_val_gt(val: &serde_json::Value, expected: &Value) -> bool {
    match expected {
        Value::Integer(i) => val.as_i64().map(|v| v > *i as i64).unwrap_or(false),
        Value::Float(f) => val.as_f64().map(|v| v > *f).unwrap_or(false),
        _ => false,
    }
}

fn json_val_lt(val: &serde_json::Value, expected: &Value) -> bool {
    match expected {
        Value::Integer(i) => val.as_i64().map(|v| v < *i as i64).unwrap_or(false),
        Value::Float(f) => val.as_f64().map(|v| v < *f).unwrap_or(false),
        _ => false,
    }
}
