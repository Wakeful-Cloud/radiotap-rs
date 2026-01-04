use deku::{DekuContainerRead, DekuRead, DekuWrite, DekuWriter, writer::Writer};

use crate::utils::{
  FieldReader, FieldWriter, RadiotapError, RadiotapField, RadiotapFieldTrait,
  RadiotapFieldTraitIdentifiers,
};

/// Transmit (TX) attenuation field
///
/// See https://www.radiotap.org/fields/TX%20attenuation.html for more information.
#[derive(Clone, Debug, Default, DekuRead, DekuWrite, PartialEq)]
#[deku(endian = "little", bit_order = "lsb")]
pub struct TxAttenuationField {
  /// Transmit power expressed as unitless distance from max power set at factory calibration. 0
  /// is max power. Monotonically nondecreasing with lower power levels.
  pub attenuation: u16,
}

impl<V> RadiotapFieldTrait<V> for TxAttenuationField {
  fn get_identifiers(&self) -> RadiotapFieldTraitIdentifiers {
    RadiotapFieldTraitIdentifiers { bit_index: 8 }
  }

  fn get_alignment(&self) -> usize {
    2
  }

  fn decode(&self, reader: &mut FieldReader) -> Result<RadiotapField<V>, RadiotapError> {
    // Decode the field
    let decoded = TxAttenuationField::from_reader((reader, 0))?.1;

    Ok(RadiotapField::TxAttenuation(decoded))
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
  const ENCODING_CASES: &[(&[u8], &TxAttenuationField)] = &[
    // Simple
    (
      &[
        0x34, 0x12, // Attenuation
      ],
      &TxAttenuationField {
        attenuation: 0x1234,
      },
    ),
  ];

  #[rstest]
  #[case::simple(ENCODING_CASES[0].0, ENCODING_CASES[0].1)]
  fn test_decode_field(#[case] encoded: &[u8], #[case] expected: &TxAttenuationField) {
    // Decode the bytes
    let decoded = TxAttenuationField::from_bytes((encoded, 0)).unwrap().1;

    // Check the result
    assert_eq!(decoded, *expected);
  }

  #[rstest]
  #[case::simple(ENCODING_CASES[0].1, ENCODING_CASES[0].0)]
  fn test_encode_field(#[case] field: &TxAttenuationField, #[case] expected: &[u8]) {
    // Encode the field
    let encoded = field.to_bytes().unwrap();

    // Check the result
    assert_eq!(encoded, expected);
  }
}
