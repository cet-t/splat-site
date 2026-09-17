#[derive(Debug, Default, Clone, Copy)]
pub struct Rgb(pub [u8; 3]);

impl From<Rgb> for String {
    fn from(value: Rgb) -> Self {
        format!("{value}")
    }
}

impl std::fmt::Display for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let [r, g, b] = self.0;
        write!(f, "#{r:02X}{g:02X}{b:02X}")
    }
}

#[allow(dead_code)]
#[derive(Default, serde::Deserialize)]
struct RawRgb {
    #[serde(alias = "color", alias = "c")]
    colour: String,
}

impl<'de> serde::de::Visitor<'de> for RawRgb {
    type Value = [u8; 3];

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a hexadecimal string of length 6")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match v.len() {
            6 => {
                let s = [&v[..2], &v[2..4], &v[4..6]].map(|s| u8::from_str_radix(s, 16));
                if s.iter().any(Result::is_err) {
                    Err(serde::de::Error::invalid_value(
                        serde::de::Unexpected::Str(v),
                        &self,
                    ))
                } else {
                    Ok(s.map(Result::unwrap))
                }
            }
            _ => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(v),
                &self,
            )),
        }
    }
}

impl<'de> serde::Deserialize<'de> for Rgb {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let r =
            deserializer.deserialize_struct(stringify!(RawRgb), &["colour"], RawRgb::default())?;
        Ok(Rgb(r))
    }
}
