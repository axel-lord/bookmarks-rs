use std::{fmt::Display, ops::Deref, str::FromStr};

/// Type representing a string that can be parsed
#[derive(Debug, Clone)]
pub struct ParsedStr<V> {
    string: String,
    val: Option<V>,
}

impl<V> Default for ParsedStr<V> {
    fn default() -> Self {
        Self {
            string: "".into(),
            val: None,
        }
    }
}

impl<V> AsRef<str> for ParsedStr<V> {
    fn as_ref(&self) -> &str {
        &self.string
    }
}

impl<V> Deref for ParsedStr<V> {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<V> From<V> for ParsedStr<V>
where
    V: ToString,
{
    fn from(value: V) -> Self {
        Self {
            string: value.to_string(),
            val: Some(value),
        }
    }
}

impl<V> FromStr for ParsedStr<V>
where
    V: FromStr,
{
    type Err = <V as FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Ok(Self {
                string: "".into(),
                val: None,
            })
        } else {
            let val = s.parse()?;
            Ok(Self {
                string: s.into(),
                val: Some(val),
            })
        }
    }
}

impl<V> ParsedStr<V>
where
    V: Copy + Default,
{
    /// Type represented by a tuple containing value and it's string representation.
    pub fn as_tuple(&self) -> (V, &str) {
        (self.val.unwrap_or_default(), &self.string)
    }
}

impl<V> ParsedStr<V>
where
    V: ToString,
{
    /// If any get value contained.
    pub fn value(&self) -> &Option<V> {
        &self.val
    }

    /// Set value contained.
    pub fn set_value(&mut self, val: Option<V>) {
        self.string = val
            .as_ref()
            .map(|v| v.to_string())
            .unwrap_or_else(|| "".into());
        self.val = val;
    }
}

impl<V> ParsedStr<V>
where
    V: FromStr + Display,
{
    /// Parse a value and get a message depending on success.
    ///
    /// # Errors
    /// If and how the parsing failed.
    pub fn parse_with_message(
        &mut self,
        from: impl ToString,
        msg: &str,
    ) -> Result<String, <V as FromStr>::Err> {
        let string = from.to_string();
        let out_msg;

        (self.val, out_msg) = if string.is_empty() {
            (None, format!("changed {msg} to none"))
        } else {
            let val = string.parse()?;
            let out_msg = format!("changed {msg} to {val}");
            (Some(val), out_msg)
        };

        self.string = string;

        Ok(out_msg)
    }
}
