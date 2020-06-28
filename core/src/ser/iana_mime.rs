use mime::Mime;

use crate::ser::Serializer;
use crate::types::IanaTag;
use crate::Serialize;

impl Serializer {
    pub fn write_mime_as_string(&mut self, mime: &Mime) {
        self.write_tag(IanaTag::MimeMessage);
        self.write_text(mime.as_ref());
    }
}

impl Serialize for Mime {
    fn serialize(&self, serializer: &mut Serializer) {
        serializer.write_mime_as_string(self);
    }
}
