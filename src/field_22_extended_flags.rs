use deku::{DekuContainerRead, DekuRead, DekuWrite, DekuWriter, writer::Writer};

use crate::utils::{FieldReader, FieldWriter, RadiotapError, RadiotapField, RadiotapFieldTrait, RadiotapFieldTraitIdentifiers};

/// Extended flags field
///
/// See https://www.radiotap.org/fields/extended%20flags.html for more information.
#[derive(Clone, Debug, Default, DekuRead, DekuWrite, PartialEq)]
#[deku(endian = "little", bit_order = "lsb")]
pub struct ExtendedFlagsField {
  /// Flags frame is decrypted (but FC protected bit is set)
  #[deku(bits = "1")]
  pub flags_encrypted_fc_set: bool,

  /// Flags frame is encrypted (but FC protected bit is also set)
  #[deku(bits = "1")]
  pub flags_decrypted_fc_set: bool,

  /// Flags IV is still present (Reserved if `flags_encrypted_fc_set` isn't set)
  #[deku(bits = "1")]
  pub flags_iv_present: bool,

  /// Flags MIC is not present (Should only be used if FCS is also not present, reserved if
  /// `flags_encrypted_fc_set` isn't set)
  #[deku(bits = "1")]
  pub flags_mic_not_present: bool,

  /// Flags reserved 0
  #[deku(bits = "28")]
  pub flags_reserved0: u32,
}

impl<V> RadiotapFieldTrait<V> for ExtendedFlagsField {
  fn get_identifiers(&self) -> RadiotapFieldTraitIdentifiers {
    RadiotapFieldTraitIdentifiers { bit_index: 22 }
  }

  fn get_alignment(&self) -> usize {
    4
  }

  fn decode(&self, reader: &mut FieldReader) -> Result<RadiotapField<V>, RadiotapError> {
    // Decode the field
    let decoded = ExtendedFlagsField::from_reader((reader, 0))?.1;

    Ok(RadiotapField::ExtendedFlags(decoded))
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
  const ENCODING_CASES: &[(&[u8], &ExtendedFlagsField)] = &[
    // No flags
    (
      &[
        0x00, 0x00, 0x00, 0x00, // Flags
      ],
      &ExtendedFlagsField {
        flags_encrypted_fc_set: false,
        flags_decrypted_fc_set: false,
        flags_iv_present: false,
        flags_mic_not_present: false,
        flags_reserved0: 0,
      },
    ),
    // Alternating flags (A)
    (
      &[
        0x05, 0x00, 0x00, 0x00, // Flags
      ],
      &ExtendedFlagsField {
        flags_encrypted_fc_set: true,
        flags_decrypted_fc_set: false,
        flags_iv_present: true,
        flags_mic_not_present: false,
        flags_reserved0: 0,
      },
    ),
    // Alternating flags (B)
    (
      &[
        0x0A, 0x00, 0x00, 0x00, // Flags
      ],
      &ExtendedFlagsField {
        flags_encrypted_fc_set: false,
        flags_decrypted_fc_set: true,
        flags_iv_present: false,
        flags_mic_not_present: true,
        flags_reserved0: 0,
      },
    ),
  ];

  #[rstest]
  #[case::no_flags(ENCODING_CASES[0].0, ENCODING_CASES[0].1)]
  #[case::alternating_flags_a(ENCODING_CASES[1].0, ENCODING_CASES[1].1)]
  #[case::alternating_flags_b(ENCODING_CASES[2].0, ENCODING_CASES[2].1)]
  fn test_decode_field(#[case] encoded: &[u8], #[case] expected: &ExtendedFlagsField) {
    // Decode the bytes
    let decoded = ExtendedFlagsField::from_bytes((encoded, 0)).unwrap().1;

    // Check the result
    assert_eq!(decoded, *expected);
  }

  #[rstest]
  #[case::no_flags(ENCODING_CASES[0].1, ENCODING_CASES[0].0)]
  #[case::alternating_flags_a(ENCODING_CASES[1].1, ENCODING_CASES[1].0)]
  #[case::alternating_flags_b(ENCODING_CASES[2].1, ENCODING_CASES[2].0)]
  fn test_encode_field(#[case] field: &ExtendedFlagsField, #[case] expected: &[u8]) {
    // Encode the field
    let encoded = field.to_bytes().unwrap();

    // Check the result
    assert_eq!(encoded, expected);
  }
}
