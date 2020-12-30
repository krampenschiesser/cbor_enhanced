use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use crate::context::Context;
use crate::de::{Deserializer, Remaining};
use crate::error::CborError;
use crate::types::IanaTag;
use crate::Deserialize;

impl<'de> Deserializer {
    pub fn take_ip_address(&self, data: &'de [u8]) -> Result<(IpAddr, Remaining<'de>), CborError> {
        let remaining = self.expect_tag(data, IanaTag::NetworkAddress)?;
        let (slice, remaining) = self.take_bytes(remaining, true)?;

        if slice.len() == 4 {
            let octets = [slice[0], slice[1], slice[2], slice[3]];
            let v4_addr = Ipv4Addr::from(octets);
            Ok((IpAddr::V4(v4_addr), remaining))
        } else if slice.len() == 16 {
            let octets = [
                slice[0], slice[1], slice[2], slice[3], slice[4], slice[5], slice[6], slice[7],
                slice[8], slice[9], slice[10], slice[11], slice[12], slice[13], slice[14],
                slice[15],
            ];
            let v6_addr = Ipv6Addr::from(octets);
            Ok((IpAddr::V6(v6_addr), remaining))
        } else {
            Err(CborError::InvalidArrayLength {
                expected: &[4, 16],
                got: slice.len(),
            })
        }
    }
    pub fn take_ip_address_and_mask(
        &self,
        data: &'de [u8],
    ) -> Result<((IpAddr, u8), Remaining<'de>), CborError> {
        let remaining = self.expect_tag(data, IanaTag::NetworkAddressPlusMask)?;
        let (length, remaining) = self.take_map_def(remaining, true)?;

        if length.unwrap_or(0) != 1 {
            return Err(CborError::InvalidArrayLength {
                expected: &[1],
                got: length.unwrap_or(0),
            });
        }

        let (slice, remaining) = self.take_bytes(remaining, true)?;
        let (prefix_length, remaining) = self.take_unsigned(remaining, true)?;

        if slice.len() == 4 {
            let octets = [slice[0], slice[1], slice[2], slice[3]];
            let v4_addr = Ipv4Addr::from(octets);
            Ok(((IpAddr::V4(v4_addr), prefix_length as u8), remaining))
        } else if slice.len() == 16 {
            let octets = [
                slice[0], slice[1], slice[2], slice[3], slice[4], slice[5], slice[6], slice[7],
                slice[8], slice[9], slice[10], slice[11], slice[12], slice[13], slice[14],
                slice[15],
            ];
            let v6_addr = Ipv6Addr::from(octets);
            Ok(((IpAddr::V6(v6_addr), prefix_length as u8), remaining))
        } else {
            Err(CborError::InvalidArrayLength {
                expected: &[4, 16],
                got: slice.len(),
            })
        }
    }
    pub fn take_mac_address(
        &self,
        data: &'de [u8],
    ) -> Result<([u8; 6], Remaining<'de>), CborError> {
        let remaining = self.expect_tag(data, IanaTag::NetworkAddress)?;
        let (slice, remaining) = self.take_bytes(remaining, true)?;

        if slice.len() == 6 {
            let octets = [slice[0], slice[1], slice[2], slice[3], slice[4], slice[5]];
            Ok((octets, remaining))
        } else {
            Err(CborError::InvalidArrayLength {
                expected: &[6],
                got: slice.len(),
            })
        }
    }
}
impl<'de> Deserialize<'de> for IpAddr {
    fn deserialize(
        deserializer: &mut Deserializer,
        data: &'de [u8],
        _context: &Context,
    ) -> Result<(Self, &'de [u8]), CborError> {
        deserializer
            .take_ip_address(data)
            .map(|(v, remaining)| (v, remaining))
    }
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

    use crate::de::Deserializer;

    #[test]
    fn test_take_ipv4() {
        let slice = b"\xd9\x01\x04\x44\xc0\x0a\x0a\x01";
        let (address, _) = Deserializer::new().take_ip_address(slice).unwrap();
        let expected: Ipv4Addr = "192.10.10.1".parse().unwrap();
        match address {
            IpAddr::V4(v4) => assert_eq!(expected, v4),
            IpAddr::V6(_) => panic!("expected v4"),
        }
    }

    #[test]
    fn test_take_mac() {
        let slice = b"\xd9\x01\x04\x46\x01\x23\x45\x67\x89\xab";
        let (address, _) = Deserializer::new().take_mac_address(slice).unwrap();
        let expected = b"\x01\x23\x45\x67\x89\xab";
        assert_eq!(address, *expected);
    }

    #[test]
    fn test_take_ipv6() {
        let slice =
            b"\xd9\x01\x04\x50\x20\x01\x0d\xb8\x85\xa3\x00\x00\x00\x00\x8a\x2e\x03\x70\x73\x34";
        let (address, _) = Deserializer::new().take_ip_address(slice).unwrap();
        let expected: Ipv6Addr = "2001:db8:85a3::8a2e:370:7334".parse().unwrap();
        match address {
            IpAddr::V6(v6) => assert_eq!(expected, v6),
            IpAddr::V4(_) => panic!("expected v6"),
        }
    }

    #[test]
    fn test_take_ipv4_and_mask() {
        let slice = b"\xd9\x01\x05\xa1\x44\xc0\xa8\x00\x64\x18\x18";
        let ((address, length), _) = Deserializer::new().take_ip_address_and_mask(slice).unwrap();
        let expected: Ipv4Addr = "192.168.0.100".parse().unwrap();
        match address {
            IpAddr::V4(v4) => assert_eq!(expected, v4),
            IpAddr::V6(_) => panic!("expected v4"),
        }
        assert_eq!(24, length);
    }
}
