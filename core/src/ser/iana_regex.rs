use regex::Regex;

use crate::context::Context;
use crate::ser::Serializer;
use crate::types::IanaTag;
use crate::Serialize;

impl Serializer {
    pub fn write_regex_as_string(&mut self, regex: &Regex) {
        self.write_tag(IanaTag::Regex);
        self.write_text(regex.as_str());
    }
}

impl Serialize for Regex {
    fn serialize(&self, serializer: &mut Serializer, _context: &Context) {
        serializer.write_regex_as_string(self);
    }
}
