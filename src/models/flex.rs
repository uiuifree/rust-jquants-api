use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use std::fmt;
use std::ops::Deref;

/// J-Quants API は同じ項目を string / number / null のいずれでも返すことがあるため、
/// どれで来ても文字列として受け取る newtype。null は空文字列になる。
#[derive(Debug, Clone, PartialEq)]
pub struct FlexString(pub String);

impl FlexString {
    /// 数値として解釈できれば `f64` を返す。null（空文字列）や非数値は `None`
    pub fn as_f64(&self) -> Option<f64> {
        self.0.parse().ok()
    }

    /// 値が空（API 応答が null または空文字列）かどうか
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<'de> Deserialize<'de> for FlexString {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = Value::deserialize(deserializer)?;
        let s = match value {
            Value::String(s) => s,
            Value::Number(n) => n.to_string(),
            Value::Null => String::new(),
            other => other.to_string(),
        };
        Ok(FlexString(s))
    }
}

impl Serialize for FlexString {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.0)
    }
}

impl fmt::Display for FlexString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl Deref for FlexString {
    type Target = str;
    fn deref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_string_number_null() {
        let v: Vec<FlexString> = serde_json::from_str(r#"["123.5", 42, null]"#).unwrap();
        assert_eq!(v[0].0, "123.5");
        assert_eq!(v[1].0, "42");
        assert_eq!(v[2].0, "");
    }

    #[test]
    fn as_f64_parses_numeric_only() {
        assert_eq!(FlexString("123.5".into()).as_f64(), Some(123.5));
        assert_eq!(FlexString(String::new()).as_f64(), None);
        assert_eq!(FlexString("-".into()).as_f64(), None);
    }
}
