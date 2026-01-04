use deku::{DekuContainerRead, DekuRead, DekuWrite, DekuWriter, writer::Writer};

use crate::utils::{
  FieldReader, FieldWriter, RadiotapError, RadiotapField, RadiotapFieldTrait,
  RadiotapFieldTraitIdentifiers,
};

/// 0-length Physical Service Data Unit (PSDU) Field
///
/// The presence of this field indicates that there was no PSDU in or captured for this PPDU, only
/// the PHY data is valid and the radiotap header is not followed by an 802.11 header.
///
/// See [www.radiotap.org/fields/0-length-PSDU.html](https://www.radiotap.org/fields/0-length-PSDU.html)
/// for more information.
#[derive(Clone, Debug, Default, DekuRead, DekuWrite, PartialEq)]
#[deku(endian = "little", bit_order = "lsb")]
pub struct PsduField {
  /// Indicates the type of the Physical Protocol Data Unit (PPDU)
  pub r#type: PsduType,
}

impl<V> RadiotapFieldTrait<V> for PsduField {
  fn get_identifiers(&self) -> RadiotapFieldTraitIdentifiers {
    RadiotapFieldTraitIdentifiers { bit_index: 26 }
  }

  fn get_alignment(&self) -> usize {
    1
  }

  fn decode(&self, reader: &mut FieldReader) -> Result<RadiotapField<V>, RadiotapError> {
    // Decode the field
    let decoded = PsduField::from_reader((reader, 0))?.1;

    Ok(RadiotapField::Psdu(decoded))
  }

  fn encode(&self, writer: &mut Writer<&mut FieldWriter>) -> Result<(), RadiotapError> {
    // Encode the field
    self.to_writer(writer, ())?;

    Ok(())
  }
}

/// Physical Service Data Unit (PSDU) Type
#[derive(Clone, Debug, Default, DekuRead, DekuWrite, PartialEq)]
#[deku(
  id_type = "u8",
  bits = "8",
  bit_order = "order",
  endian = "endian",
  ctx = "endian: deku::ctx::Endian, order: deku::ctx::Order"
)]
pub enum PsduType {
  /// Sounding PPDU
  #[default]
  #[deku(id = "0x00")]
  Sounding,

  /// Data not captured (e.g., multi-user PPDU)
  #[deku(id = "0x01")]
  NotCaptured,

  /// Vendor-specific
  #[deku(id = "0xFF")]
  Vendor,
}

#[cfg(test)]
mod tests {
  use deku::{DekuContainerRead, DekuContainerWrite};
  use pretty_assertions::assert_eq;
  use rstest::rstest;

  use super::*;

  /// Encoding test cases
  const ENCODING_CASES: &[(&[u8], &PsduField)] = &[
    // Simple
    (
      &[
        0x01, // Type
      ],
      &PsduField {
        r#type: PsduType::NotCaptured,
      },
    ),
  ];

  #[rstest]
  #[case::simple(ENCODING_CASES[0].0, ENCODING_CASES[0].1)]
  fn test_decode_field(#[case] encoded: &[u8], #[case] expected: &PsduField) {
    // Decode the bytes
    let decoded = PsduField::from_bytes((encoded, 0)).unwrap().1;

    // Check the result
    assert_eq!(decoded, *expected);
  }

  #[rstest]
  #[case::simple(ENCODING_CASES[0].1, ENCODING_CASES[0].0)]
  fn test_encode_field(#[case] field: &PsduField, #[case] expected: &[u8]) {
    // Encode the field
    let encoded = field.to_bytes().unwrap();

    // Check the result
    assert_eq!(encoded, expected);
  }
}
