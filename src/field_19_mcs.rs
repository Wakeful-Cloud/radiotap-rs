use deku::{DekuContainerRead, DekuRead, DekuWrite, DekuWriter, writer::Writer};

use crate::utils::{FieldReader, FieldWriter, RadiotapError, RadiotapField, RadiotapFieldTrait, RadiotapFieldTraitIdentifiers};

/// Modulation and Coding Scheme (MCS) field
///
/// See https://www.radiotap.org/fields/MCS.html for more information.
#[derive(Clone, Debug, Default, DekuRead, DekuWrite, PartialEq)]
#[deku(endian = "little", bit_order = "lsb")]
pub struct McsField {
  /// Known bandwidth known
  #[deku(bits = "1")]
  pub known_bw: bool,

  /// Known MCS index known
  #[deku(bits = "1")]
  pub known_mcs: bool,

  /// Known guard interval known
  #[deku(bits = "1")]
  pub known_gi: bool,

  /// Known High Throughput (HT) format known
  #[deku(bits = "1")]
  pub known_fmt: bool,

  /// Known Forward Error Correction (FEC) type known
  #[deku(bits = "1")]
  pub known_fec: bool,

  /// Known Space-Time Block Coding (STBC) known
  #[deku(bits = "1")]
  pub known_stbc: bool,

  /// Known Number of Extension Spatial Stream (NESS) known
  #[deku(bits = "1")]
  pub known_ness: bool,

  /// Known Number of Extension Spatial Stream (NESS) bit 1 (MSB) known
  #[deku(bits = "1")]
  pub known_ness_bit1: bool,

  /// Flags bandwidth
  pub flags_bw: McsFlagsBw,

  /// Flags guard interval (0 = long, 1 = short)
  #[deku(bits = "1")]
  pub flags_gi: bool,

  /// Flags High Throughput (HT) format (0 = mixed, 1 = greenfield)
  #[deku(bits = "1")]
  pub flags_fmt: bool,

  /// Flags FEC type (0 = Binary Convolutional Code (BCC), 1 = Low-Density Parity-Check (LDPC))
  #[deku(bits = "1")]
  pub flags_fec: bool,

  /// Flags Space-Time Block Coding (STBC) streams
  #[deku(bits = "2")]
  pub flags_stbc: u8,

  /// Flags Number of Extension Spatial Stream (NESS) bit 0 (LSB)
  #[deku(bits = "1")]
  pub flags_ness_bit0: bool,

  /// MCS rate index
  ///
  /// See https://en.wikipedia.org/wiki/IEEE_802.11n-2009#Data_rates for more information.
  pub mcs: u8,
}

impl<V> RadiotapFieldTrait<V> for McsField {
  fn get_identifiers(&self) -> RadiotapFieldTraitIdentifiers {
    RadiotapFieldTraitIdentifiers { bit_index: 19 }
  }

  fn get_alignment(&self) -> usize {
    1
  }

  fn decode(&self, reader: &mut FieldReader) -> Result<RadiotapField<V>, RadiotapError> {
    // Decode the field
    let decoded = McsField::from_reader((reader, 0))?.1;

    Ok(RadiotapField::Mcs(decoded))
  }

  fn encode(&self, writer: &mut Writer<&mut FieldWriter>) -> Result<(), RadiotapError> {
    // Encode the field
    self.to_writer(writer, ())?;

    Ok(())
  }
}

/// MCS flags bandwidth
#[derive(Clone, Debug, Default, DekuRead, DekuWrite, PartialEq)]
#[deku(
  id_type = "u8",
  bits = "2",
  bit_order = "order",
  endian = "endian",
  ctx = "endian: deku::ctx::Endian, order: deku::ctx::Order"
)]
pub enum McsFlagsBw {
  /// 20 MHz
  #[default]
  #[deku(id = "0")]
  Bw20,

  /// 40 MHz
  #[deku(id = "1")]
  Bw40,

  /// 20 MHz lower
  #[deku(id = "2")]
  Bw20L,

  /// 20 MHz upper
  #[deku(id = "3")]
  Bw20U,
}

#[cfg(test)]
mod tests {
  use deku::{DekuContainerRead, DekuContainerWrite};
  use pretty_assertions::assert_eq;
  use rstest::rstest;

  use super::*;

  /// Encoding test cases
  const ENCODING_CASES: &[(&[u8], &McsField)] = &[
    // No flags
    (
      &[
        0x00, // Known
        0x43, // Flags
        0x05, // MCS
      ],
      &McsField {
        known_bw: false,
        known_mcs: false,
        known_gi: false,
        known_fmt: false,
        known_fec: false,
        known_stbc: false,
        known_ness: false,
        known_ness_bit1: false,
        flags_bw: McsFlagsBw::Bw20U,
        flags_gi: false,
        flags_fmt: false,
        flags_fec: false,
        flags_stbc: 2,
        flags_ness_bit0: false,
        mcs: 0x05,
      },
    ),
    // Alternating flags (A)
    (
      &[
        0x55, // Known
        0x57, // Flags
        0x05, // MCS
      ],
      &McsField {
        known_bw: true,
        known_mcs: false,
        known_gi: true,
        known_fmt: false,
        known_fec: true,
        known_stbc: false,
        known_ness: true,
        known_ness_bit1: false,
        flags_bw: McsFlagsBw::Bw20U,
        flags_gi: true,
        flags_fmt: false,
        flags_fec: true,
        flags_stbc: 2,
        flags_ness_bit0: false,
        mcs: 0x05,
      },
    ),
    // Alternating flags (B)
    (
      &[
        0xAA, // Known
        0xCB, // Flags
        0x05, // MCS
      ],
      &McsField {
        known_bw: false,
        known_mcs: true,
        known_gi: false,
        known_fmt: true,
        known_fec: false,
        known_stbc: true,
        known_ness: false,
        known_ness_bit1: true,
        flags_bw: McsFlagsBw::Bw20U,
        flags_gi: false,
        flags_fmt: true,
        flags_fec: false,
        flags_stbc: 2,
        flags_ness_bit0: true,
        mcs: 0x05,
      },
    ),
  ];

  #[rstest]
  #[case::no_flags(ENCODING_CASES[0].0, ENCODING_CASES[0].1)]
  #[case::alternating_flags_a(ENCODING_CASES[1].0, ENCODING_CASES[1].1)]
  #[case::alternating_flags_b(ENCODING_CASES[2].0, ENCODING_CASES[2].1)]
  fn test_decode_field(#[case] encoded: &[u8], #[case] expected: &McsField) {
    // Decode the bytes
    let decoded = McsField::from_bytes((encoded, 0)).unwrap().1;

    // Check the result
    assert_eq!(decoded, *expected);
  }

  #[rstest]
  #[case::no_flags(ENCODING_CASES[0].1, ENCODING_CASES[0].0)]
  #[case::alternating_flags_a(ENCODING_CASES[1].1, ENCODING_CASES[1].0)]
  #[case::alternating_flags_b(ENCODING_CASES[2].1, ENCODING_CASES[2].0)]
  fn test_encode_field(#[case] field: &McsField, #[case] expected: &[u8]) {
    // Encode the field
    let encoded = field.to_bytes().unwrap();

    // Check the result
    assert_eq!(encoded, expected);
  }
}
