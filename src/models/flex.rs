use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use std::fmt;
use std::ops::Deref;

/// J-Quants API は同じ項目を string / number / null のいずれでも返すことがあるため、
/// どれで来ても文字列として受け取る newtype。null は空文字列になる。
#[derive(Debug, Clone, PartialEq, Default)]
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

/// `null` を既定値として受ける。`#[serde(default)]` は「項目が無い」場合しか効かず、
/// 明示的な `null` では失敗するため、配列項目にはこちらを使う。
/// EDINET の応答は同じ項目を配列でも null でも返してくる
pub fn null_as_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + Default,
{
    Ok(Option::<T>::deserialize(deserializer)?.unwrap_or_default())
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
    fn null_becomes_empty_vec() {
        #[derive(Deserialize)]
        struct T {
            #[serde(default, deserialize_with = "null_as_default")]
            xs: Vec<i32>,
        }
        // 項目が無い場合も、明示的な null の場合も空になる
        assert!(serde_json::from_str::<T>(r#"{}"#).unwrap().xs.is_empty());
        assert!(serde_json::from_str::<T>(r#"{"xs":null}"#)
            .unwrap()
            .xs
            .is_empty());
        assert_eq!(
            serde_json::from_str::<T>(r#"{"xs":[1,2]}"#).unwrap().xs,
            vec![1, 2]
        );
    }

    #[test]
    fn as_f64_parses_numeric_only() {
        assert_eq!(FlexString("123.5".into()).as_f64(), Some(123.5));
        assert_eq!(FlexString(String::new()).as_f64(), None);
        assert_eq!(FlexString("-".into()).as_f64(), None);
    }
}
