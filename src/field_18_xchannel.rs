use deku::{DekuContainerRead, DekuRead, DekuWrite, DekuWriter, writer::Writer};

use crate::utils::{
  FieldReader, FieldWriter, RadiotapError, RadiotapField, RadiotapFieldTrait,
  RadiotapFieldTraitIdentifiers,
};

/// Extended channel (XChannel) information field
///
/// This field is parsed by Wireshark, but only partially (it ignores `maxpower`). Origin of the
/// field is unknown. Used by FreeBSD and OS X.
///
/// Channel numbers are problematic - using the channel’s center frequency would be much better.
///
/// The flags define some things that can be inferred (2 vs. 5 GHz).
///
/// Things like the "Channel Type Passive" don’t make sense per packet. As used, this field
/// conflates channel properties (which need not be stored per packet but are more or less fixed)
/// with packet properties (like the modulation).
///
/// See https://www.radiotap.org/fields/XChannel.html for more information.
#[derive(Clone, Debug, Default, DekuRead, DekuWrite, PartialEq)]
#[deku(endian = "little", bit_order = "lsb")]
pub struct XChannelField {
  /// Flags reserved 0
  #[deku(bits = "4")]
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

  /// Flags Dynamic Complementary Code Keying (CCK)-Orthogonal Frequency-Division Multiplexing
  /// (OFDM) channel
  #[deku(bits = "1")]
  pub flags_dyn: bool,

  /// Flags Gaussian Frequency-Shift Keying (GFSK) channel
  #[deku(bits = "1")]
  pub flags_gfsk: bool,

  /// Flags Global System for Mobile Communications (GSM) channel
  #[deku(bits = "1")]
  pub flags_gsm: bool,

  /// Flags status turbo
  #[deku(bits = "1")]
  pub flags_status_turbo: bool,

  /// Flags half-rate channel
  #[deku(bits = "1")]
  pub flags_half_rate: bool,

  /// Flags quarter-rate channel
  #[deku(bits = "1")]
  pub flags_quarter_rate: bool,

  /// Flags HT/20 channel
  #[deku(bits = "1")]
  pub flags_ht_20: bool,

  /// Flags HT/40+ channel
  #[deku(bits = "1")]
  pub flags_ht_40_above: bool,

  /// Flags HT/40- channel
  #[deku(bits = "1")]
  pub flags_ht_40_below: bool,

  /// Flags reserved 1
  #[deku(bits = "13")]
  pub flags_reserved1: u16,

  /// Frequency in MHz
  pub frequency: u16,

  /// Channel number
  pub channel: u8,

  /// Maximum transmit power
  pub maxpower: u8,
}

impl<V> RadiotapFieldTrait<V> for XChannelField {
  fn get_identifiers(&self) -> RadiotapFieldTraitIdentifiers {
    RadiotapFieldTraitIdentifiers { bit_index: 18 }
  }

  fn get_alignment(&self) -> usize {
    4
  }

  fn decode(&self, reader: &mut FieldReader) -> Result<RadiotapField<V>, RadiotapError> {
    // Decode the field
    let decoded = XChannelField::from_reader((reader, 0))?.1;

    Ok(RadiotapField::XChannel(decoded))
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
  const ENCODING_CASES: &[(&[u8], &XChannelField)] = &[
    // No flags
    (
      &[
        0x00, 0x00, 0x00, 0x00, // Flags
        0x6C, 0x09, // Frequency
        0x01, // Channel
        0x1E, // Max power
      ],
      &XChannelField {
        flags_reserved0: 0,
        flags_turbo: false,
        flags_cck: false,
        flags_ofdm: false,
        flags_spectrum_2ghz: false,
        flags_spectrum_5ghz: false,
        flags_passive: false,
        flags_dyn: false,
        flags_gfsk: false,
        flags_gsm: false,
        flags_status_turbo: false,
        flags_half_rate: false,
        flags_quarter_rate: false,
        flags_ht_20: false,
        flags_ht_40_above: false,
        flags_ht_40_below: false,
        flags_reserved1: 0,
        frequency: 2412,
        channel: 1,
        maxpower: 30,
      },
    ),
    // Alternating flags (A)
    (
      &[
        0x50, 0x55, 0x05, 0x00, // Flags
        0x6C, 0x09, // Frequency
        0x01, // Channel
        0x1E, // Max power
      ],
      &XChannelField {
        flags_reserved0: 0,
        flags_turbo: true,
        flags_cck: false,
        flags_ofdm: true,
        flags_spectrum_2ghz: false,
        flags_spectrum_5ghz: true,
        flags_passive: false,
        flags_dyn: true,
        flags_gfsk: false,
        flags_gsm: true,
        flags_status_turbo: false,
        flags_half_rate: true,
        flags_quarter_rate: false,
        flags_ht_20: true,
        flags_ht_40_above: false,
        flags_ht_40_below: true,
        flags_reserved1: 0,
        frequency: 2412,
        channel: 1,
        maxpower: 30,
      },
    ),
    // Alternating flags (B)
    (
      &[
        0xA0, 0x0AA, 0x02, 0x00, // Flags
        0x6C, 0x09, // Frequency
        0x01, // Channel
        0x1E, // Max power
      ],
      &XChannelField {
        flags_reserved0: 0,
        flags_turbo: false,
        flags_cck: true,
        flags_ofdm: false,
        flags_spectrum_2ghz: true,
        flags_spectrum_5ghz: false,
        flags_passive: true,
        flags_dyn: false,
        flags_gfsk: true,
        flags_gsm: false,
        flags_status_turbo: true,
        flags_half_rate: false,
        flags_quarter_rate: true,
        flags_ht_20: false,
        flags_ht_40_above: true,
        flags_ht_40_below: false,
        flags_reserved1: 0,
        frequency: 2412,
        channel: 1,
        maxpower: 30,
      },
    ),
  ];

  #[rstest]
  #[case::no_flags(ENCODING_CASES[0].0, ENCODING_CASES[0].1)]
  #[case::alternating_flags_a(ENCODING_CASES[1].0, ENCODING_CASES[1].1)]
  #[case::alternating_flags_b(ENCODING_CASES[2].0, ENCODING_CASES[2].1)]
  fn test_decode_field(#[case] encoded: &[u8], #[case] expected: &XChannelField) {
    // Decode the bytes
    let decoded = XChannelField::from_bytes((encoded, 0)).unwrap().1;

    // Check the result
    assert_eq!(decoded, *expected);
  }

  #[rstest]
  #[case::no_flags(ENCODING_CASES[0].1, ENCODING_CASES[0].0)]
  #[case::alternating_flags_a(ENCODING_CASES[1].1, ENCODING_CASES[1].0)]
  #[case::alternating_flags_b(ENCODING_CASES[2].1, ENCODING_CASES[2].0)]
  fn test_encode_field(#[case] field: &XChannelField, #[case] expected: &[u8]) {
    // Encode the field
    let encoded = field.to_bytes().unwrap();

    // Check the result
    assert_eq!(encoded, expected);
  }
}
