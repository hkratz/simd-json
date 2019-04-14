use super::Value;
use crate::numberparse::Number;
use crate::BorrowedValue;

impl From<Number> for Value {
    #[inline]
    fn from(n: Number) -> Self {
        match n {
            Number::F64(n) => Value::F64(n),
            Number::I64(n) => Value::I64(n),
        }
    }
}

impl From<crate::BorrowedValue<'_>> for Value {
    fn from(b: BorrowedValue<'_>) -> Self {
        match b {
            BorrowedValue::Null => Value::Null,
            BorrowedValue::Bool(b) => Value::Bool(b),
            BorrowedValue::F64(f) => Value::F64(f),
            BorrowedValue::I64(i) => Value::I64(i),
            BorrowedValue::String(s) => Value::from(s.to_string()),
            BorrowedValue::Array(a) => {
                Value::Array(a.into_iter().map(|v| v.into()).collect::<Vec<Value>>())
            }
            BorrowedValue::Object(m) => Value::Object(
                m.into_iter()
                    .map(|(k, v)| (k.to_string(), v.into()))
                    .collect(),
            ),
        }
    }
}

/********* str_ **********/

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value::String(s.to_owned())
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::String(s)
    }
}

impl From<&String> for Value {
    fn from(s: &String) -> Self {
        Value::String(s.to_owned())
    }
}

/********* atoms **********/
impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Bool(b)
    }
}

/********* i_ **********/
impl From<i8> for Value {
    fn from(i: i8) -> Self {
        Value::I64(i as i64)
    }
}

impl From<i16> for Value {
    fn from(i: i16) -> Self {
        Value::I64(i as i64)
    }
}

impl From<i32> for Value {
    fn from(i: i32) -> Self {
        Value::I64(i as i64)
    }
}

impl From<i64> for Value {
    fn from(i: i64) -> Self {
        Value::I64(i as i64)
    }
}

impl From<&i8> for Value {
    fn from(i: &i8) -> Self {
        Value::I64(*i as i64)
    }
}

impl From<&i16> for Value {
    fn from(i: &i16) -> Self {
        Value::I64(*i as i64)
    }
}

impl From<&i32> for Value {
    fn from(i: &i32) -> Self {
        Value::I64(*i as i64)
    }
}

impl From<&i64> for Value {
    fn from(i: &i64) -> Self {
        Value::I64(*i as i64)
    }
}

/********* u_ **********/
impl From<u8> for Value {
    fn from(i: u8) -> Self {
        Value::I64(i as i64)
    }
}

impl From<u16> for Value {
    fn from(i: u16) -> Self {
        Value::I64(i as i64)
    }
}

impl From<u32> for Value {
    fn from(i: u32) -> Self {
        Value::I64(i as i64)
    }
}

impl From<u64> for Value {
    fn from(i: u64) -> Self {
        Value::I64(i as i64)
    }
}

impl From<&u8> for Value {
    fn from(i: &u8) -> Self {
        Value::I64(*i as i64)
    }
}

impl From<&u16> for Value {
    fn from(i: &u16) -> Self {
        Value::I64(*i as i64)
    }
}

impl From<&u32> for Value {
    fn from(i: &u32) -> Self {
        Value::I64(*i as i64)
    }
}

impl From<&u64> for Value {
    fn from(i: &u64) -> Self {
        Value::I64(*i as i64)
    }
}

/********* f_ **********/
impl From<f32> for Value {
    fn from(f: f32) -> Self {
        Value::F64(f as f64)
    }
}

impl From<f64> for Value {
    fn from(f: f64) -> Self {
        Value::F64(f as f64)
    }
}

impl From<&f32> for Value {
    fn from(f: &f32) -> Self {
        Value::F64(*f as f64)
    }
}

impl From<&f64> for Value {
    fn from(f: &f64) -> Self {
        Value::F64(*f as f64)
    }
}
