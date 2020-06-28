use mime::Mime;

use crate::de::{Deserializer, Remaining};
use crate::error::CborError;
use crate::types::IanaTag;

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
