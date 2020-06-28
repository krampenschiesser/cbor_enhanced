use uuid::Uuid;

use crate::ser::Serializer;
use crate::types::IanaTag;
use crate::Serialize;

impl Serializer {
    pub fn write_uuid(&mut self, uuid: &Uuid) {
        self.write_tag(IanaTag::Uuid);
        self.write_bytes(uuid.as_bytes());
    }
}

impl Serialize for Uuid {
    fn serialize(&self, serializer: &mut Serializer) {
        serializer.write_uuid(self);
    }
}
