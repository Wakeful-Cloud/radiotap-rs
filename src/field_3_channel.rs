use deku::{DekuContainerRead, DekuRead, DekuWrite, DekuWriter, writer::Writer};

use crate::utils::{FieldReader, FieldWriter, RadiotapError, RadiotapField, RadiotapFieldTrait, RadiotapFieldTraitIdentifiers};

/// Channel field
///
/// See https://www.radiotap.org/fields/Channel.html for more information.
#[derive(Clone, Debug, Default, DekuRead, DekuWrite, PartialEq)]
#[deku(endian = "little", bit_order = "lsb")]
pub struct ChannelField {
  /// Channel frequency in KHz
  pub frequency: u16,

  /// Flags 700 MHz spectrum channel
  #[deku(bits = "1")]
  pub flags_spectrum_700_mhz: bool,

  /// Flags 800 MHz spectrum channel
  #[deku(bits = "1")]
  pub flags_spectrum_800_mhz: bool,

  /// Flags 900 MHz spectrum channel
  #[deku(bits = "1")]
  pub flags_spectrum_900_mhz: bool,

  /// Flags reserved 0
  #[deku(bits = "1")]
  pub flags_reserved0: u8,

  /// Flags turbo channel
  #[deku(bits = "1")]
  pub flags_turbo: bool,

  /// Flags Complementary Code Keying (CCK) channel
  #[deku(bits = "1")]
  pub flags_cck: bool,

  /// Flags Orthogonal Frequency-Division Multiplexing (OFDM) channel
  #[deku(bits = "1")]
  pub flags_ofdm: bool,

  /// Flags 2 GHz spectrum channel
  #[deku(bits = "1")]
  pub flags_spectrum_2ghz: bool,

  /// Flags 5 GHz spectrum channel
  #[deku(bits = "1")]
  pub flags_spectrum_5ghz: bool,

  /// Flags only passive scanning allowed
  #[deku(bits = "1")]
  pub flags_passive: bool,

  /// Flags dynamic Complementary Code Keying (CCK)-Orthogonal Frequency-Division Multiplexing
  /// (OFDM) channel
  #[deku(bits = "1")]
  pub flags_dyn: bool,

  /// Flags Gaussian Frequency-Shift Keying (GFSK) channel (FHSS PHY)
  #[deku(bits = "1")]
  pub flags_gfsk: bool,

  /// Flags reserved 1
  #[deku(bits = "4")]
  pub flags_reserved1: u8,
}

impl<V> RadiotapFieldTrait<V> for ChannelField {
  fn get_identifiers(&self) -> RadiotapFieldTraitIdentifiers {
    RadiotapFieldTraitIdentifiers { bit_index: 3 }
  }

  fn get_alignment(&self) -> usize {
    2
  }

  fn decode(&self, reader: &mut FieldReader) -> Result<RadiotapField<V>, RadiotapError> {
    // Decode the field
    let decoded = ChannelField::from_reader((reader, 0))?.1;

    Ok(RadiotapField::Channel(decoded))
  }

  fn encode(&self, writer: &mut Writer<&mut FieldWriter>) -> Result<(), RadiotapError> {
    // Encode the field
    self.to_writer(writer, ())?;

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use deku::{DekuContainerRead, DekuContainerWrite};
  use pretty_assertions::assert_eq;
  use rstest::rstest;

  use super::*;

  /// Encoding test cases
  const ENCODING_CASES: &[(&[u8], &ChannelField)] = &[
    // No flags
    (
      &[
        0x6C, 0x09, // Frequency
        0x00, 0x00, // Flags
      ],
      &ChannelField {
        frequency: 2412,
        flags_spectrum_700_mhz: false,
        flags_spectrum_800_mhz: false,
        flags_spectrum_900_mhz: false,
        flags_reserved0: 0,
        flags_turbo: false,
        flags_cck: false,
        flags_ofdm: false,
        flags_spectrum_2ghz: false,
        flags_spectrum_5ghz: false,
        flags_passive: false,
        flags_dyn: false,
        flags_gfsk: false,
        flags_reserved1: 0,
      },
    ),
    // Alternating flags (A)
    (
      &[
        0x6C, 0x09, // Frequency
        0xA5, 0x0A, // Flags
      ],
      &ChannelField {
        frequency: 2412,
        flags_spectrum_700_mhz: true,
        flags_spectrum_800_mhz: false,
        flags_spectrum_900_mhz: true,
        flags_reserved0: 0,
        flags_turbo: false,
        flags_cck: true,
        flags_ofdm: false,
        flags_spectrum_2ghz: true,
        flags_spectrum_5ghz: false,
        flags_passive: true,
        flags_dyn: false,
        flags_gfsk: true,
        flags_reserved1: 0,
      },
    ),
    // Alternating flags (B)
    (
      &[
        0x6C, 0x09, // Frequency
        0x52, 0x05, // Flags
      ],
      &ChannelField {
        frequency: 2412,
        flags_spectrum_700_mhz: false,
        flags_spectrum_800_mhz: true,
        flags_spectrum_900_mhz: false,
        flags_turbo: true,
        flags_reserved0: 0,
        flags_cck: false,
        flags_ofdm: true,
        flags_spectrum_2ghz: false,
        flags_spectrum_5ghz: true,
        flags_passive: false,
        flags_dyn: true,
        flags_gfsk: false,
        flags_reserved1: 0,
      },
    ),
  ];

  #[rstest]
  #[case::no_flags(ENCODING_CASES[0].0, ENCODING_CASES[0].1)]
  #[case::alternating_flags_a(ENCODING_CASES[1].0, ENCODING_CASES[1].1)]
  #[case::alternating_flags_b(ENCODING_CASES[2].0, ENCODING_CASES[2].1)]
  fn test_decode_field(#[case] encoded: &[u8], #[case] expected: &ChannelField) {
    // Decode the bytes
    let decoded = ChannelField::from_bytes((encoded, 0)).unwrap().1;

    // Check the result
    assert_eq!(decoded, *expected);
  }

  #[rstest]
  #[case::no_flags(ENCODING_CASES[0].1, ENCODING_CASES[0].0)]
  #[case::alternating_flags_a(ENCODING_CASES[1].1, ENCODING_CASES[1].0)]
  #[case::alternating_flags_b(ENCODING_CASES[2].1, ENCODING_CASES[2].0)]
  fn test_encode_field(#[case] field: &ChannelField, #[case] expected: &[u8]) {
    // Encode the field
    let encoded = field.to_bytes().unwrap();

    // Check the result
    assert_eq!(encoded, expected);
  }
}
