/*
 *
 * Copyright 2026 gRPC authors.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to
 * deal in the Software without restriction, including without limitation the
 * rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
 * sell copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
 * IN THE SOFTWARE.
 *
 */

use serde::Deserialize;
use std::time::Duration;

pub(crate) fn parse_duration(s: &str) -> Result<Duration, String> {
    if !s.ends_with('s') {
        return Err("duration string must end with 's'".to_string());
    }
    let s = &s[..s.len() - 1]; // strip 's'.
    let mut parts = s.splitn(2, '.');
    let secs_str = parts
        .next()
        .ok_or_else(|| "empty duration string".to_string())?;
    let secs: u64 = secs_str
        .parse()
        .map_err(|e| format!("failed to parse seconds: {}", e))?;

    let nanos = if let Some(fraction_str) = parts.next() {
        if fraction_str.is_empty() {
            return Err("empty fraction part".to_string());
        }
        if fraction_str.len() > 9 {
            return Err("fraction part has more than 9 digits".to_string());
        }
        let fraction_val: u32 = fraction_str
            .parse()
            .map_err(|e| format!("failed to parse fraction: {}", e))?;
        let pad = 9 - fraction_str.len();
        fraction_val * 10u32.pow(pad as u32)
    } else {
        0
    };

    Ok(Duration::new(secs, nanos))
}

pub(crate) fn deserialize_duration<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    parse_duration(&s).map_err(serde::de::Error::custom)
}

pub(crate) fn deserialize_duration_opt<'de, D>(
    deserializer: D,
) -> Result<Option<Duration>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    struct OptVisitor;
    impl<'de> serde::de::Visitor<'de> for OptVisitor {
        type Value = Option<Duration>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str("a string representing a duration or null")
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(None)
        }

        fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            parse_duration(&s)
                .map(Some)
                .map_err(serde::de::Error::custom)
        }
    }
    deserializer.deserialize_option(OptVisitor)
}

pub(crate) fn deserialize_int64<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    struct Visitor;
    impl<'de> serde::de::Visitor<'de> for Visitor {
        type Value = i64;
        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str("an i64 or a string representing an i64")
        }
        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(v)
        }
        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            v.try_into().map_err(serde::de::Error::custom)
        }
        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            v.parse().map_err(serde::de::Error::custom)
        }
    }
    deserializer.deserialize_any(Visitor)
}

pub(crate) fn deserialize_uint32_opt<'de, D>(deserializer: D) -> Result<Option<u32>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    struct OptVisitor;
    impl<'de> serde::de::Visitor<'de> for OptVisitor {
        type Value = Option<u32>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str("a u32 or a string representing a u32 or null")
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(None)
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            v.try_into().map(Some).map_err(serde::de::Error::custom)
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            v.parse().map(Some).map_err(serde::de::Error::custom)
        }

        fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            deserializer.deserialize_any(self)
        }
    }
    deserializer.deserialize_option(OptVisitor)
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_parse_duration() {
        assert_eq!(parse_duration("1s").unwrap(), Duration::from_secs(1));
        assert_eq!(parse_duration("0s").unwrap(), Duration::from_secs(0));
        assert_eq!(
            parse_duration("1.5s").unwrap(),
            Duration::new(1, 500_000_000)
        );
        assert_eq!(parse_duration("0.000000001s").unwrap(), Duration::new(0, 1));
        assert_eq!(parse_duration("1.0000001s").unwrap(), Duration::new(1, 100));
        assert_eq!(
            parse_duration("1.0001s").unwrap(),
            Duration::new(1, 100_000)
        );

        assert!(parse_duration("").is_err());
        assert!(parse_duration("1").is_err());
        assert!(parse_duration("1.s").is_err());
        assert!(parse_duration("1.0000000001s").is_err());
        assert!(parse_duration("as").is_err());
    }

    #[test]
    fn test_deserialize_duration() {
        #[derive(Deserialize, Debug, PartialEq)]
        struct TestStruct {
            #[serde(deserialize_with = "deserialize_duration")]
            d: Duration,
        }

        let val: TestStruct = serde_json::from_value(json!({ "d": "1.5s" })).unwrap();
        assert_eq!(val.d, Duration::new(1, 500_000_000));

        let res: Result<TestStruct, _> = serde_json::from_value(json!({ "d": "1" }));
        assert!(res.is_err());
    }

    #[test]
    fn test_deserialize_duration_opt() {
        #[derive(Deserialize, Debug, PartialEq)]
        struct TestStruct {
            #[serde(default, deserialize_with = "deserialize_duration_opt")]
            d: Option<Duration>,
        }

        let val: TestStruct = serde_json::from_value(json!({ "d": "1.5s" })).unwrap();
        assert_eq!(val.d, Some(Duration::new(1, 500_000_000)));

        let val: TestStruct = serde_json::from_value(json!({ "d": null })).unwrap();
        assert_eq!(val.d, None);

        let val: TestStruct = serde_json::from_value(json!({})).unwrap();
        assert_eq!(val.d, None);

        let res: Result<TestStruct, _> = serde_json::from_value(json!({ "d": "invalid" }));
        assert!(res.is_err());
    }

    #[test]
    fn test_deserialize_uint32_opt() {
        #[derive(Deserialize, Debug, PartialEq)]
        struct TestStruct {
            #[serde(default, deserialize_with = "deserialize_uint32_opt")]
            val: Option<u32>,
        }

        let val: TestStruct = serde_json::from_value(json!({ "val": 123 })).unwrap();
        assert_eq!(val.val, Some(123));

        let val: TestStruct = serde_json::from_value(json!({ "val": "456" })).unwrap();
        assert_eq!(val.val, Some(456));

        let val: TestStruct = serde_json::from_value(json!({ "val": null })).unwrap();
        assert_eq!(val.val, None);

        let val: TestStruct = serde_json::from_value(json!({})).unwrap();
        assert_eq!(val.val, None);

        let res: Result<TestStruct, _> = serde_json::from_value(json!({ "val": "invalid" }));
        assert!(res.is_err());

        let res: Result<TestStruct, _> = serde_json::from_value(json!({ "val": -1 }));
        assert!(res.is_err());
    }
}
