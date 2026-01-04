use deku::{DekuContainerRead, DekuRead, DekuWrite, DekuWriter, writer::Writer};

use crate::utils::{
  FieldReader, FieldWriter, RadiotapError, RadiotapField, RadiotapFieldTrait,
  RadiotapFieldTraitIdentifiers,
};

/// Time Synchronization Function Timer (TSFT) field
///
/// See [www.radiotap.org/fields/TSFT.html](https://www.radiotap.org/fields/TSFT.html) for more
/// information.
#[derive(Clone, Debug, Default, DekuRead, DekuWrite, PartialEq)]
#[deku(endian = "little", bit_order = "lsb")]
pub struct TsftField {
  /// Value in microseconds of the MAC’s 64-bit 802.11 TSFT when the first bit of the MAC Protocol
  /// Data Unit (MPDU) arrived at the MAC. For received frames only.
  pub mactime: u64,
}

impl<V> RadiotapFieldTrait<V> for TsftField {
  fn get_identifiers(&self) -> RadiotapFieldTraitIdentifiers {
    RadiotapFieldTraitIdentifiers { bit_index: 0 }
  }

  fn get_alignment(&self) -> usize {
    8
  }

  fn decode(&self, reader: &mut FieldReader) -> Result<RadiotapField<V>, RadiotapError> {
    // Decode the field
    let decoded = TsftField::from_reader((reader, 0))?.1;

    Ok(RadiotapField::Tsft(decoded))
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
  const ENCODING_CASES: &[(&[u8], &TsftField)] = &[
    // Simple
    (
      &[
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, // MAC time
      ],
      &TsftField {
        mactime: 0x0807060504030201,
      },
    ),
  ];

  #[rstest]
  #[case::simple(ENCODING_CASES[0].0, ENCODING_CASES[0].1)]
  fn test_decode_field(#[case] encoded: &[u8], #[case] expected: &TsftField) {
    // Decode the bytes
    let decoded = TsftField::from_bytes((encoded, 0)).unwrap().1;

    // Check the result
    assert_eq!(decoded, *expected);
  }

  #[rstest]
  #[case::simple(ENCODING_CASES[0].1, ENCODING_CASES[0].0)]
  fn test_encode_field(#[case] field: &TsftField, #[case] expected: &[u8]) {
    // Encode the field
    let encoded = field.to_bytes().unwrap();

    // Check the result
    assert_eq!(encoded, expected);
  }
}
