use regex::Regex;

use crate::ser::Serializer;
use crate::types::IanaTag;

impl Serializer {
    pub fn write_regex_as_string(&mut self, regex: &Regex) {
        self.write_tag(IanaTag::Regex);
        self.write_text(regex.as_str());
    }
}
