use crate::ser::Serializer;
use crate::types::IanaTag;
use num_bigint::{BigInt, BigUint};
use num_traits::Signed;
use crate::de::iana_geo::GeoCoordinate;

impl Serializer {
    pub fn write_geo(&mut self, coordinate: GeoCoordinate) {
        self.write_tag(IanaTag::GeoCoordinate);

        let len = 2 + coordinate.uncertainty.map(|_| 1).unwrap_or(0) + coordinate.elevation.map(|_| 1).unwrap_or(0);
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