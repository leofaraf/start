use value::Value;
use bson::{Bson, Document};

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

// The matches_filter function checks if the provided document matches the filter
pub fn matches_filter(doc: &Document, filter: &Filter) -> bool {
    use Filter::*;
    
    match filter {
        // Exact equality check for a field's value
        Eq(field, expected) => {
            if let Some(val) = doc.get(field) {
                json_val_eq(val, expected)
            } else {
                false
            }
        }
        // Greater-than check for a field's value
        Gt(field, expected) => {
            if let Some(val) = doc.get(field) {
                json_val_gt(val, expected)
            } else {
                false
            }
        }
        // Less-than check for a field's value
        Lt(field, expected) => {
            if let Some(val) = doc.get(field) {
                json_val_lt(val, expected)
            } else {
                false
            }
        }
        // Greater-than-or-equal check for a field's value
        Ge(field, expected) => {
            if let Some(val) = doc.get(field) {
                json_val_ge(val, expected)
            } else {
                false
            }
        }
        // Less-than-or-equal check for a field's value
        Le(field, expected) => {
            if let Some(val) = doc.get(field) {
                json_val_le(val, expected)
            } else {
                false
            }
        }
        // Check if a field matches a pattern (string contains check)
        Like(field, pattern) => {
            if let Some(Bson::String(val)) = doc.get(field) {
                val.contains(pattern)
            } else {
                false
            }
        }
        // Logical AND for multiple filters
        And(filters) => filters.iter().all(|f| matches_filter(doc, f)),
        // Logical OR for multiple filters
        Or(filters) => filters.iter().any(|f| matches_filter(doc, f)),
        // Logical NOT for a filter
        Not(f) => !matches_filter(doc, f),
    }
}

// Helper function to compare JSON values for equality
fn json_val_eq(val: &Bson, expected: &Value) -> bool {
    match expected {
        Value::String(s) => val.as_str().map(|v| v == s).unwrap_or(false),
        Value::Integer(i) => val.as_i64().map(|v| v == *i as i64).unwrap_or(false),
        Value::Float(f) => val.as_f64().map(|v| v == *f).unwrap_or(false),
        Value::Boolean(b) => val.as_bool().map(|v| v == *b).unwrap_or(false),
        Value::Null => val.as_null().is_some(),
    }
}

// Helper function to check if a value is greater than a given value
fn json_val_gt(val: &Bson, expected: &Value) -> bool {
    match expected {
        Value::Integer(i) => val.as_i32().map(|v| v > *i as i32).unwrap_or(false),
        Value::Float(f) => val.as_f64().map(|v| v > *f).unwrap_or(false),
        _ => false,
    }
}

// Helper function to check if a value is less than a given value
fn json_val_lt(val: &Bson, expected: &Value) -> bool {
    match expected {
        Value::Integer(i) => val.as_i32().map(|v| v < *i as i32).unwrap_or(false),
        Value::Float(f) => val.as_f64().map(|v| v < *f).unwrap_or(false),
        _ => false,
    }
}

// Helper function to check if a value is greater than or equal to a given value
fn json_val_ge(val: &Bson, expected: &Value) -> bool {
    match expected {
        Value::Integer(i) => val.as_i32().map(|v| v >= *i as i32).unwrap_or(false),
        Value::Float(f) => val.as_f64().map(|v| v >= *f).unwrap_or(false),
        _ => false,
    }
}

// Helper function to check if a value is less than or equal to a given value
fn json_val_le(val: &Bson, expected: &Value) -> bool {
    match expected {
        Value::Integer(i) => val.as_i32().map(|v| v <= *i as i32).unwrap_or(false),
        Value::Float(f) => val.as_f64().map(|v| v <= *f).unwrap_or(false),
        _ => false,
    }
}