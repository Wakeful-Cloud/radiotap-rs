use deku::{DekuContainerRead, DekuRead, DekuWrite, DekuWriter, writer::Writer};

use crate::utils::{
  FieldReader, FieldWriter, RadiotapError, RadiotapField, RadiotapFieldTrait,
  RadiotapFieldTraitIdentifiers,
};

/// Receive (RX) flags field
///
/// See [www.radiotap.org/fields/RX%20flags.html](https://www.radiotap.org/fields/RX%20flags.html)
/// for more information.
#[derive(Clone, Debug, Default, DekuRead, DekuWrite, PartialEq)]
#[deku(endian = "little", bit_order = "lsb")]
pub struct RxFlagsField {
  /// Flags reserved 0
  #[deku(bits = "1")]
  pub flags_reserved0: u8,

  /// Flags Physical Layer Convergence Procedure (PLCP) Cyclic Redundancy Check (CRC) failed
  #[deku(bits = "1")]
  pub flags_bad_plcp: bool,

  /// Flags reserved 1
  #[deku(bits = "14")]
  pub flags_reserved1: u16,
}

impl<V> RadiotapFieldTrait<V> for RxFlagsField {
  fn get_identifiers(&self) -> RadiotapFieldTraitIdentifiers {
    RadiotapFieldTraitIdentifiers { bit_index: 14 }
  }

  fn get_alignment(&self) -> usize {
    2
  }

  fn decode(&self, reader: &mut FieldReader) -> Result<RadiotapField<V>, RadiotapError> {
    // Decode the field
    let decoded = RxFlagsField::from_reader((reader, 0))?.1;

    Ok(RadiotapField::RxFlags(decoded))
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
  const ENCODING_CASES: &[(&[u8], &RxFlagsField)] = &[
    // No flags
    (
      &[
        0x00, 0x00, // Flags
      ],
      &RxFlagsField {
        flags_reserved0: 0,
        flags_bad_plcp: false,
        flags_reserved1: 0,
      },
    ),
    // Alternating flags (A)
    (
      &[
        0x02, 0x00, // Flags
      ],
      &RxFlagsField {
        flags_reserved0: 0,
        flags_bad_plcp: true,
        flags_reserved1: 0,
      },
    ),
    // Alternating flags (B)
    (
      &[
        0x00, 0x00, // Flags
      ],
      &RxFlagsField {
        flags_reserved0: 0,
        flags_bad_plcp: false,
        flags_reserved1: 0,
      },
    ),
  ];

  #[rstest]
  #[case::no_flags(ENCODING_CASES[0].0, ENCODING_CASES[0].1)]
  #[case::alternating_flags_a(ENCODING_CASES[1].0, ENCODING_CASES[1].1)]
  #[case::alternating_flags_b(ENCODING_CASES[2].0, ENCODING_CASES[2].1)]
  fn test_decode_field(#[case] encoded: &[u8], #[case] expected: &RxFlagsField) {
    // Decode the bytes
    let decoded = RxFlagsField::from_bytes((encoded, 0)).unwrap().1;

    // Check the result
    assert_eq!(decoded, *expected);
  }

  #[rstest]
  #[case::no_flags(ENCODING_CASES[0].1, ENCODING_CASES[0].0)]
  #[case::alternating_flags_a(ENCODING_CASES[1].1, ENCODING_CASES[1].0)]
  #[case::alternating_flags_b(ENCODING_CASES[2].1, ENCODING_CASES[2].0)]
  fn test_encode_field(#[case] field: &RxFlagsField, #[case] expected: &[u8]) {
    // Encode the field
    let encoded = field.to_bytes().unwrap();

    // Check the result
    assert_eq!(encoded, expected);
  }
}
