extern crate serde;

use std::fmt;
use self::serde::{de, Serialize, Serializer, Deserialize, Deserializer};

use timezones::Tz;

impl Serialize for Tz {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.name())
    }
}

impl<'de> Deserialize<'de> for Tz {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Tz;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "an IANA timezone string")
            }

            fn visit_str<E: de::Error>(self, value: &str) -> Result<Tz, E> {
                value.parse::<Tz>().map_err(|e| E::custom(e.to_string()))
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}

#[cfg(test)]
mod tests {
    extern crate serde_test;

    use self::serde_test::{Token, assert_tokens, assert_de_tokens_error};
    use timezones::Tz::{self, Europe__London, Etc__UTC, UTC};

    #[test]
    fn serde_ok_both_ways() {
        assert_tokens(&Europe__London, &[Token::String("Europe/London")]);
        assert_tokens(&Etc__UTC, &[Token::String("Etc/UTC")]);
        assert_tokens(&UTC, &[Token::String("UTC")]);
    }

    #[test]
    fn serde_de_error() {
        assert_de_tokens_error::<Tz>(&[Token::Str("Europe/L")], "'Europe/L' is not a valid timezone");
        assert_de_tokens_error::<Tz>(&[Token::Str("")], "'' is not a valid timezone");
    }
}
