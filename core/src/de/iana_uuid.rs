use uuid::Uuid;

use crate::de::{Deserializer, Remaining};
use crate::error::CborError;
use crate::types::IanaTag;
use crate::Deserialize;

impl<'de> Deserializer {
    pub fn take_uuid(&self, data: &'de [u8]) -> Result<(Uuid, Remaining<'de>), CborError> {
        let remaining = self.expect_tag(data, IanaTag::Uuid)?;
        let (slice, remaining) = self.take_bytes(remaining, true)?;
        let uuid =
            Uuid::from_slice(slice.as_ref()).map_err(|_| CborError::InvalidUuid(slice.to_vec()))?;
        Ok((uuid, remaining))
    }
}

impl<'de> Deserialize<'de> for Uuid {
    fn deserialize(
        deserializer: &mut Deserializer,
        data: &'de [u8],
    ) -> Result<(Self, &'de [u8]), CborError> {
        deserializer
            .take_uuid(data)
            .map(|(v, remaining)| (v, remaining))
    }
}
