use deku::{DekuContainerRead, DekuRead, DekuWrite, DekuWriter, writer::Writer};

use crate::utils::{FieldReader, FieldWriter, RadiotapError, RadiotapField, RadiotapFieldTrait, RadiotapFieldTraitIdentifiers};

/// Antenna field
///
/// See https://www.radiotap.org/fields/Antenna.html for more information.
#[derive(Clone, Debug, Default, DekuRead, DekuWrite, PartialEq)]
#[deku(endian = "little", bit_order = "lsb")]
pub struct AntennaField {
  /// Unitless indication of the receive/transmit antenna for this packet. The first antenna is
  /// antenna `0`.
  pub index: u8,
}

impl<V> RadiotapFieldTrait<V> for AntennaField {
  fn get_identifiers(&self) -> RadiotapFieldTraitIdentifiers {
    RadiotapFieldTraitIdentifiers { bit_index: 11 }
  }

  fn get_alignment(&self) -> usize {
    1
  }

  fn decode(&self, reader: &mut FieldReader) -> Result<RadiotapField<V>, RadiotapError> {
    // Decode the field
    let decoded = AntennaField::from_reader((reader, 0))?.1;

    Ok(RadiotapField::Antenna(decoded))
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
  const ENCODING_CASES: &[(&[u8], &AntennaField)] = &[
    // Simple
    (
      &[
        0x01, // Antenna index
      ],
      &AntennaField { index: 0x01 },
    ),
  ];

  #[rstest]
  #[case::simple(ENCODING_CASES[0].0, ENCODING_CASES[0].1)]
  fn test_decode_field(#[case] encoded: &[u8], #[case] expected: &AntennaField) {
    // Decode the bytes
    let decoded = AntennaField::from_bytes((encoded, 0)).unwrap().1;

    // Check the result
    assert_eq!(decoded, *expected);
  }

  #[rstest]
  #[case::simple(ENCODING_CASES[0].1, ENCODING_CASES[0].0)]
  fn test_encode_field(#[case] field: &AntennaField, #[case] expected: &[u8]) {
    // Encode the field
    let encoded = field.to_bytes().unwrap();

    // Check the result
    assert_eq!(encoded, expected);
  }
}
