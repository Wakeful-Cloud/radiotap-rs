use deku::{DekuContainerRead, DekuRead, DekuWrite, DekuWriter, writer::Writer};

use crate::utils::{
  FieldReader, FieldWriter, RadiotapError, RadiotapField, RadiotapFieldTrait,
  RadiotapFieldTraitIdentifiers,
};

/// Frame flags field
///
/// See [www.radiotap.org/fields/Flags.html](https://www.radiotap.org/fields/Flags.html) for more
/// information.
#[derive(Clone, Debug, Default, DekuRead, DekuWrite, PartialEq)]
#[deku(endian = "little", bit_order = "lsb")]
pub struct FlagsField {
  /// Flags frame was sent/received during Contention-Free Period (CFP)
  #[deku(bits = "1")]
  pub flags_cfp: bool,

  /// Flags frame was sent/received with short preamble
  #[deku(bits = "1")]
  pub flags_short_pre: bool,

  /// Flags frame was sent/received with WEP encryption
  #[deku(bits = "1")]
  pub flags_wep: bool,

  /// Flags frame was sent/received with fragmentation
  #[deku(bits = "1")]
  pub flags_frag: bool,

  /// Flags frame includes Frame Check Sequence (FCS)
  #[deku(bits = "1")]
  pub flags_fcs: bool,

  /// Flags frame has padding between 802.11 header and payload (to 32-bit boundary)
  #[deku(bits = "1")]
  pub flags_datapad: bool,

  /// Flags frame failed Frame Check Sequence (FCS) check
  #[deku(bits = "1")]
  pub flags_bad_fcs: bool,

  /// Flags frame used short guard interval (HT)
  #[deku(bits = "1")]
  pub flags_short_gi: bool,
}

impl<V> RadiotapFieldTrait<V> for FlagsField {
  fn get_identifiers(&self) -> RadiotapFieldTraitIdentifiers {
    RadiotapFieldTraitIdentifiers { bit_index: 1 }
  }

  fn get_alignment(&self) -> usize {
    1
  }

  fn decode(&self, reader: &mut FieldReader) -> Result<RadiotapField<V>, RadiotapError> {
    // Decode the field
    let decoded = FlagsField::from_reader((reader, 0))?.1;

    Ok(RadiotapField::Flags(decoded))
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
  const ENCODING_CASES: &[(&[u8], &FlagsField)] = &[
    // No flags
    (
      &[
        0x00, // Flags
      ],
      &FlagsField {
        flags_cfp: false,
        flags_short_pre: false,
        flags_wep: false,
        flags_frag: false,
        flags_fcs: false,
        flags_datapad: false,
        flags_bad_fcs: false,
        flags_short_gi: false,
      },
    ),
    // Alternating flags (A)
    (
      &[
        0x55, // Flags
      ],
      &FlagsField {
        flags_cfp: true,
        flags_short_pre: false,
        flags_wep: true,
        flags_frag: false,
        flags_fcs: true,
        flags_datapad: false,
        flags_bad_fcs: true,
        flags_short_gi: false,
      },
    ),
    // Alternating flags (B)
    (
      &[
        0xAA, // Flags
      ],
      &FlagsField {
        flags_cfp: false,
        flags_short_pre: true,
        flags_wep: false,
        flags_frag: true,
        flags_fcs: false,
        flags_datapad: true,
        flags_bad_fcs: false,
        flags_short_gi: true,
      },
    ),
  ];

  #[rstest]
  #[case::no_flags(ENCODING_CASES[0].0, ENCODING_CASES[0].1)]
  #[case::alternating_flags_a(ENCODING_CASES[1].0, ENCODING_CASES[1].1)]
  #[case::alternating_flags_b(ENCODING_CASES[2].0, ENCODING_CASES[2].1)]
  fn test_decode_field(#[case] encoded: &[u8], #[case] expected: &FlagsField) {
    // Decode the bytes
    let decoded = FlagsField::from_bytes((encoded, 0)).unwrap().1;

    // Check the result
    assert_eq!(decoded, *expected);
  }

  #[rstest]
  #[case::no_flags(ENCODING_CASES[0].1, ENCODING_CASES[0].0)]
  #[case::alternating_flags_a(ENCODING_CASES[1].1, ENCODING_CASES[1].0)]
  #[case::alternating_flags_b(ENCODING_CASES[2].1, ENCODING_CASES[2].0)]
  fn test_encode_field(#[case] field: &FlagsField, #[case] expected: &[u8]) {
    // Encode the field
    let encoded = field.to_bytes().unwrap();

    // Check the result
    assert_eq!(encoded, expected);
  }
}
