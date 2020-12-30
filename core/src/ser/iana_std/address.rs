use std::net::IpAddr;

use crate::context::Context;
use crate::ser::Serializer;
use crate::types::IanaTag;
use crate::Serialize;

impl Serializer {
    pub fn write_ip_address(&mut self, address: &IpAddr) {
        self.write_tag(IanaTag::NetworkAddress);
        match address {
            IpAddr::V4(v4) => self.write_bytes(&v4.octets()),
            IpAddr::V6(v6) => self.write_bytes(&v6.octets()),
        }
    }
    pub fn write_ip_address_and_mask(&mut self, address: &IpAddr, mask: u8) {
        self.write_tag(IanaTag::NetworkAddressPlusMask);
        self.write_map_def(1);
        match address {
            IpAddr::V4(v4) => self.write_bytes(&v4.octets()),
            IpAddr::V6(v6) => self.write_bytes(&v6.octets()),
        }
        self.write_u64(mask as u64);
    }
}

impl Serialize for IpAddr {
    fn serialize(&self, serializer: &mut Serializer, _context: &Context) {
        serializer.write_ip_address(self);
    }
}
