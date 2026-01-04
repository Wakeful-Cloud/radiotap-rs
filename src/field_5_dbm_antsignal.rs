use deku::{DekuContainerRead, DekuRead, DekuWrite, DekuWriter, writer::Writer};

use crate::utils::{FieldReader, FieldWriter, RadiotapError, RadiotapField, RadiotapFieldTrait, RadiotapFieldTraitIdentifiers};

/// dBm antenna signal field
///
/// See https://www.radiotap.org/fields/Antenna%20signal.html for more information.
#[derive(Clone, Debug, Default, DekuRead, DekuWrite, PartialEq)]
#[deku(endian = "little", bit_order = "lsb")]
pub struct DbmAntSignalField {
  /// RF signal power at the antenna. This field contains a single signed 8-bit value, which
  /// indicates the RF signal power at the antenna, in decibels difference from 1 mW.
  pub signal: i8,
}

impl<V> RadiotapFieldTrait<V> for DbmAntSignalField {
  fn get_identifiers(&self) -> RadiotapFieldTraitIdentifiers {
    RadiotapFieldTraitIdentifiers { bit_index: 5 }
  }

  fn get_alignment(&self) -> usize {
    1
  }

  fn decode(&self, reader: &mut FieldReader) -> Result<RadiotapField<V>, RadiotapError> {
    // Decode the field
    let decoded = DbmAntSignalField::from_reader((reader, 0))?.1;

    Ok(RadiotapField::DbmAntSignal(decoded))
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
  const ENCODING_CASES: &[(&[u8], &DbmAntSignalField)] = &[
    // Simple
    (
      &[
        0x02, // Signal
      ],
      &DbmAntSignalField { signal: 0x02 },
    ),
  ];

  #[rstest]
  #[case::simple(ENCODING_CASES[0].0, ENCODING_CASES[0].1)]
  fn test_decode_field(#[case] encoded: &[u8], #[case] expected: &DbmAntSignalField) {
    // Decode the bytes
    let decoded = DbmAntSignalField::from_bytes((encoded, 0)).unwrap().1;

    // Check the result
    assert_eq!(decoded, *expected);
  }

  #[rstest]
  #[case::simple(ENCODING_CASES[0].1, ENCODING_CASES[0].0)]
  fn test_encode_field(#[case] field: &DbmAntSignalField, #[case] expected: &[u8]) {
    // Encode the field
    let encoded = field.to_bytes().unwrap();

    // Check the result
    assert_eq!(encoded, expected);
  }
}
