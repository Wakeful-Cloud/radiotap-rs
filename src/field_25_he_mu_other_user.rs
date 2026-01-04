use deku::{DekuContainerRead, DekuRead, DekuWrite, DekuWriter, writer::Writer};

use crate::utils::{FieldReader, FieldWriter, RadiotapError, RadiotapField, RadiotapFieldTrait, RadiotapFieldTraitIdentifiers};

/// High-Efficiency Multi-User (HE-MU) other user field
///
/// See https://www.radiotap.org/fields/HE-MU-other-user.html for more information.
#[derive(Clone, Debug, Default, DekuRead, DekuWrite, PartialEq)]
#[deku(endian = "little", bit_order = "lsb")]
pub struct HeMuOtherUserField {
  /// Per user 1 station ID
  #[deku(bits = "11")]
  pub per_user1_sta_id: u16,

  /// Per user 1 bit 11 - bit 14
  pub per_user1_bits11to14: HeMuOtherUserBits11to14,

  /// Per user 1 reserved 0
  #[deku(bits = "1")]
  pub per_user1_reserved0: u8,

  /// Per user 2 Modulation and Coding Scheme (MCS)
  #[deku(bits = "4")]
  pub per_user2_mcs: u8,

  /// Per user 2 Dual Carrier Modulation (DCM)
  #[deku(bits = "1")]
  pub per_user2_dcm: bool,

  /// Per user 2 coding (0 = Binary Convolutional Coding (BCC), 1 = Low-Density Parity-Check (LDPC))
  #[deku(bits = "1")]
  pub per_user2_coding: bool,

  /// Per user 2 reserved 0
  #[deku(bits = "10")]
  pub per_user2_reserved0: u16,

  /// Per user position
  pub per_user_position: u8,

  /// Per user known user field position is known
  #[deku(bits = "1")]
  pub per_user_known_position: bool,

  /// Per user known station ID is known
  #[deku(bits = "1")]
  pub per_user_known_sta_id: bool,

  /// Per user known Number of Space-Time Streams (NSTS) is known
  #[deku(bits = "1")]
  pub per_user_known_nsts: bool,

  /// Per user known Transmit (TX) beamforming (BF) is known
  #[deku(bits = "1")]
  pub per_user_known_txbf: bool,

  /// Per user known spatial configuration is known
  #[deku(bits = "1")]
  pub per_user_known_spatial_cfg: bool,

  /// Per user known Modulation and Coding Scheme (MCS) is known
  #[deku(bits = "1")]
  pub per_user_known_mcs: bool,

  /// Per user known Dual Carrier Modulation (DCM) is known
  #[deku(bits = "1")]
  pub per_user_known_dcm: bool,

  /// Per user known coding is known
  #[deku(bits = "1")]
  pub per_user_known_coding: bool,
}

impl<V> RadiotapFieldTrait<V> for HeMuOtherUserField {
  fn get_identifiers(&self) -> RadiotapFieldTraitIdentifiers {
    RadiotapFieldTraitIdentifiers { bit_index: 25 }
  }

  fn get_alignment(&self) -> usize {
    2
  }

  fn decode(&self, reader: &mut FieldReader) -> Result<RadiotapField<V>, RadiotapError> {
    // Decode the field
    let decoded = HeMuOtherUserField::from_reader((reader, 0))?.1;

    Ok(RadiotapField::HeMuOtherUser(decoded))
  }

  fn encode(&self, writer: &mut Writer<&mut FieldWriter>) -> Result<(), RadiotapError> {
    // Encode the field
    self.to_writer(writer, ())?;

    Ok(())
  }
}

// HE-MU other user bit 11 - bit 14
#[derive(Clone, Debug, Default, DekuRead, DekuWrite, PartialEq)]
#[deku(
  bit_order = "order",
  endian = "endian",
  ctx = "endian: deku::ctx::Endian, order: deku::ctx::Order"
)]
pub struct HeMuOtherUserBits11to14 {
  /// HE-MU other user bit 11 - bit 14
  #[deku(bits = "4")]
  value: u8,
}

impl HeMuOtherUserBits11to14 {
  /// Get the Number of Space-Time Streams (NSTS); only for non-MU-MIMO (Multiple Input Multiple
  /// Output)
  pub fn get_nsts(&self) -> u8 {
    self.value & 0b0000_0111
  }

  /// Set the Number of Space-Time Streams (NSTS); only for non-MU-MIMO (Multiple Input Multiple
  /// Output)
  pub fn set_nsts(&mut self, nsts: u8) {
    self.value = (self.value & 0b1111_1000) | (nsts & 0b0000_0111);
  }

  /// Get the Transmit (TX) beamforming (BF); only for non-MU-MIMO (Multiple Input Multiple Output)
  pub fn get_txbf(&self) -> bool {
    (self.value & 0b0000_1000) != 0
  }

  /// Set the Transmit (TX) beamforming (BF); only for non-MU-MIMO (Multiple Input Multiple Output)
  pub fn set_txbf(&mut self, txbf: bool) {
    if txbf {
      self.value |= 0b0000_1000;
    } else {
      self.value &= 0b1111_0111;
    }
  }

  /// Get the spatial configuration; only for MU-MIMO (Multiple Input Multiple Output)
  pub fn get_spatial_cfg(&self) -> u8 {
    (self.value & 0b1111_0000) >> 4
  }

  /// Set the spatial configuration; only for MU-MIMO (Multiple Input Multiple Output)
  pub fn set_spatial_cfg(&mut self, spatial_cfg: u8) {
    self.value = (self.value & 0b0000_1111) | ((spatial_cfg & 0b0000_1111) << 4);
  }
}

#[cfg(test)]
mod tests {
  use deku::{DekuContainerRead, DekuContainerWrite};
  use pretty_assertions::assert_eq;
  use rstest::rstest;

  use super::*;

  /// Encoding test cases
  const ENCODING_CASES: &[(&[u8], &HeMuOtherUserField)] = &[
    // No flags
    (
      &[
        0x7B, 0x50, // Per user 1
        0x0C, 0x00, // Per user 2
        0x05, // Per user position
        0x00, // Per user known
      ],
      &HeMuOtherUserField {
        per_user1_sta_id: 123,
        per_user1_bits11to14: HeMuOtherUserBits11to14 { value: 0x0A },
        per_user1_reserved0: 0x00,
        per_user2_mcs: 0x0C,
        per_user2_dcm: false,
        per_user2_coding: false,
        per_user2_reserved0: 0x00,
        per_user_position: 5,
        per_user_known_position: false,
        per_user_known_sta_id: false,
        per_user_known_nsts: false,
        per_user_known_txbf: false,
        per_user_known_spatial_cfg: false,
        per_user_known_mcs: false,
        per_user_known_dcm: false,
        per_user_known_coding: false,
      },
    ),
    // Alternating flags (A)
    (
      &[
        0x7B, 0x50, // Per user 1
        0x1C, 0x00, // Per user 2
        0x05, // Per user position
        0x55, // Per user known
      ],
      &HeMuOtherUserField {
        per_user1_sta_id: 123,
        per_user1_bits11to14: HeMuOtherUserBits11to14 { value: 0x0A },
        per_user1_reserved0: 0x00,
        per_user2_mcs: 0x0C,
        per_user2_dcm: true,
        per_user2_coding: false,
        per_user2_reserved0: 0x00,
        per_user_position: 5,
        per_user_known_position: true,
        per_user_known_sta_id: false,
        per_user_known_nsts: true,
        per_user_known_txbf: false,
        per_user_known_spatial_cfg: true,
        per_user_known_mcs: false,
        per_user_known_dcm: true,
        per_user_known_coding: false,
      },
    ),
    // Alternating flags (B)
    (
      &[
        0x7B, 0x50, // Per user 1
        0x2C, 0x00, // Per user 2
        0x05, // Per user position
        0xAA, // Per user known
      ],
      &HeMuOtherUserField {
        per_user1_sta_id: 123,
        per_user1_bits11to14: HeMuOtherUserBits11to14 { value: 0x0A },
        per_user1_reserved0: 0x00,
        per_user2_mcs: 0x0C,
        per_user2_dcm: false,
        per_user2_coding: true,
        per_user2_reserved0: 0x00,
        per_user_position: 5,
        per_user_known_position: false,
        per_user_known_sta_id: true,
        per_user_known_nsts: false,
        per_user_known_txbf: true,
        per_user_known_spatial_cfg: false,
        per_user_known_mcs: true,
        per_user_known_dcm: false,
        per_user_known_coding: true,
      },
    ),
  ];

  #[rstest]
  #[case::no_flags(ENCODING_CASES[0].0, ENCODING_CASES[0].1)]
  #[case::alternating_flags_a(ENCODING_CASES[1].0, ENCODING_CASES[1].1)]
  #[case::alternating_flags_b(ENCODING_CASES[2].0, ENCODING_CASES[2].1)]
  fn test_decode_field(#[case] encoded: &[u8], #[case] expected: &HeMuOtherUserField) {
    // Decode the bytes
    let decoded = HeMuOtherUserField::from_bytes((encoded, 0)).unwrap().1;

    // Check the result
    assert_eq!(decoded, *expected);
  }

  #[rstest]
  #[case::no_flags(ENCODING_CASES[0].1, ENCODING_CASES[0].0)]
  #[case::alternating_flags_a(ENCODING_CASES[1].1, ENCODING_CASES[1].0)]
  #[case::alternating_flags_b(ENCODING_CASES[2].1, ENCODING_CASES[2].0)]
  fn test_encode_field(#[case] field: &HeMuOtherUserField, #[case] expected: &[u8]) {
    // Encode the field
    let encoded = field.to_bytes().unwrap();

    // Check the result
    assert_eq!(encoded, expected);
  }
}
