use deku::{DekuContainerRead, DekuRead, DekuWrite, DekuWriter, writer::Writer};

use crate::utils::{
  FieldReader, FieldWriter, RadiotapError, RadiotapField, RadiotapFieldTrait,
  RadiotapFieldTraitIdentifiers,
};

/// dB antenna noise field
///
/// See [www.radiotap.org/fields/dB%20antenna%20noise.html](https://www.radiotap.org/fields/dB%20antenna%20noise.html)
/// for more information.
#[derive(Clone, Debug, Default, DekuRead, DekuWrite, PartialEq)]
#[deku(endian = "little", bit_order = "lsb")]
pub struct DbAntNoiseField {
  /// RF noise power at the antenna, decibel difference from an arbitrary, fixed reference.
  pub noise: u8,
}

impl<V> RadiotapFieldTrait<V> for DbAntNoiseField {
  fn get_identifiers(&self) -> RadiotapFieldTraitIdentifiers {
    RadiotapFieldTraitIdentifiers { bit_index: 13 }
  }

  fn get_alignment(&self) -> usize {
    1
  }

  fn decode(&self, reader: &mut FieldReader) -> Result<RadiotapField<V>, RadiotapError> {
    // Decode the field
    let decoded = DbAntNoiseField::from_reader((reader, 0))?.1;

    Ok(RadiotapField::DbAntNoise(decoded))
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
  const ENCODING_CASES: &[(&[u8], &DbAntNoiseField)] = &[
    // Simple
    (
      &[
        0x01, // Noise
      ],
      &DbAntNoiseField { noise: 0x01 },
    ),
  ];

  #[rstest]
  #[case::simple(ENCODING_CASES[0].0, ENCODING_CASES[0].1)]
  fn test_decode_field(#[case] encoded: &[u8], #[case] expected: &DbAntNoiseField) {
    // Decode the bytes
    let decoded = DbAntNoiseField::from_bytes((encoded, 0)).unwrap().1;

    // Check the result
    assert_eq!(decoded, *expected);
  }

  #[rstest]
  #[case::simple(ENCODING_CASES[0].1, ENCODING_CASES[0].0)]
  fn test_encode_field(#[case] field: &DbAntNoiseField, #[case] expected: &[u8]) {
    // Encode the field
    let encoded = field.to_bytes().unwrap();

    // Check the result
    assert_eq!(encoded, expected);
  }
}
