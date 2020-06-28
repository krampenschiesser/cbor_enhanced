use regex::Regex;

use crate::de::{Deserializer, Remaining};
use crate::error::CborError;
use crate::types::IanaTag;
use crate::Deserialize;

impl<'de> Deserializer {
    pub fn take_regex(&self, data: &'de [u8]) -> Result<(Regex, Remaining<'de>), CborError> {
        use std::str::FromStr;

        let remaining = self.expect_tag(data, IanaTag::Regex)?;
        let (text, remaining) = self.take_text(remaining, true)?;
        let regex = Regex::from_str(text.as_ref())
            .map_err(|e| CborError::InvalidRegex(text.to_string(), e.to_string()))?;
        Ok((regex, remaining))
    }
}


impl<'de> Deserialize<'de> for Regex {
    fn deserialize(
        deserializer: &mut Deserializer,
        data: &'de [u8],
    ) -> Result<(Self, &'de [u8]), CborError> {
        deserializer
            .take_regex(data)
            .map(|(v, remaining)| (v, remaining))
    }
}
