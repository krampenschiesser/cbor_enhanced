use mime::Mime;

use crate::de::{Deserializer, Remaining};
use crate::error::CborError;
use crate::types::IanaTag;
use crate::Deserialize;

impl<'de> Deserializer {
    pub fn take_mime(&self, data: &'de [u8]) -> Result<(Mime, Remaining<'de>), CborError> {
        use std::str::FromStr;

        let remaining = self.expect_tag(data, IanaTag::MimeMessage)?;
        let (text, remaining) = self.take_text(remaining, true)?;
        let mime = Mime::from_str(text.as_ref())
            .map_err(|e| CborError::InvalidMimeString(text.to_string(), e.to_string()))?;
        Ok((mime, remaining))
    }
}

impl<'de> Deserialize<'de> for Mime {
    fn deserialize(
        deserializer: &mut Deserializer,
        data: &'de [u8],
    ) -> Result<(Self, &'de [u8]), CborError> {
        deserializer
            .take_mime(data)
            .map(|(v, remaining)| (v, remaining))
    }
}
