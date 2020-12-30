use crate::context::Context;
use crate::de::iana_geo::GeoCoordinate;
use crate::ser::Serializer;
use crate::types::IanaTag;
use crate::Serialize;

impl Serializer {
    pub fn write_geo(&mut self, coordinate: &GeoCoordinate) {
        self.write_tag(IanaTag::GeoCoordinate);

        let len = 2
            + coordinate.uncertainty.map(|_| 1).unwrap_or(0)
            + coordinate.elevation.map(|_| 1).unwrap_or(0);
        self.write_array_def(len);

        self.write_f64(coordinate.latitude);
        self.write_f64(coordinate.longitude);
        if let Some(val) = coordinate.elevation {
            self.write_f64(val);
        }
        if let Some(val) = coordinate.uncertainty {
            self.write_f64(val);
        }
    }
}

impl Serialize for GeoCoordinate {
    fn serialize(&self, serializer: &mut Serializer, _context: &Context) {
        serializer.write_geo(self);
    }
}
