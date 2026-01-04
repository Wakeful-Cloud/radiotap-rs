use deku::{DekuContainerRead, DekuRead, DekuWrite, DekuWriter, writer::Writer};

use crate::utils::{
  FieldReader, FieldWriter, RadiotapError, RadiotapField, RadiotapFieldTrait,
  RadiotapFieldTraitIdentifiers,
};

/// Legacy Signal (L-SIG) field
///
/// See [www.radiotap.org/fields/L-SIG.html](https://www.radiotap.org/fields/L-SIG.html) for more
/// information.
#[derive(Clone, Debug, Default, DekuRead, DekuWrite, PartialEq)]
#[deku(endian = "little", bit_order = "lsb")]
pub struct LsigField {
  /// Data 1 rate known
  #[deku(bits = "1")]
  pub data1_rate_known: bool,

  /// Data 1 length known
  #[deku(bits = "1")]
  pub data1_length_known: bool,

  /// Data 1 reserved 0
  #[deku(bits = "14")]
  pub data1_reserved0: u16,

  /// Data 2 rate
  #[deku(bits = "4")]
  pub data2_rate: u8,

  /// Data 2 length
  #[deku(bits = "12")]
  pub data2_length: u16,
}

impl<V> RadiotapFieldTrait<V> for LsigField {
  fn get_identifiers(&self) -> RadiotapFieldTraitIdentifiers {
    RadiotapFieldTraitIdentifiers { bit_index: 27 }
  }

  fn get_alignment(&self) -> usize {
    2
  }

  fn decode(&self, reader: &mut FieldReader) -> Result<RadiotapField<V>, RadiotapError> {
    // Decode the field
    let decoded = LsigField::from_reader((reader, 0))?.1;

    Ok(RadiotapField::Lsig(decoded))
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
  const ENCODING_CASES: &[(&[u8], &LsigField)] = &[
    // Simple
    (
      &[
        0x00, 0x00, // Data 1
        0x8D, 0x1C, // Data 2
      ],
      &LsigField {
        data1_rate_known: false,
        data1_length_known: false,
        data1_reserved0: 0,
        data2_rate: 13,
        data2_length: 456,
      },
    ),
  ];

  #[rstest]
  #[case::simple(ENCODING_CASES[0].0, ENCODING_CASES[0].1)]
  fn test_decode_field(#[case] encoded: &[u8], #[case] expected: &LsigField) {
    // Decode the bytes
    let decoded = LsigField::from_bytes((encoded, 0)).unwrap().1;

    // Check the result
    assert_eq!(decoded, *expected);
  }

  #[rstest]
  #[case::simple(ENCODING_CASES[0].1, ENCODING_CASES[0].0)]
  fn test_encode_field(#[case] field: &LsigField, #[case] expected: &[u8]) {
    // Encode the field
    let encoded = field.to_bytes().unwrap();

    // Check the result
    assert_eq!(encoded, expected);
  }
}
