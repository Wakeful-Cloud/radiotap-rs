use deku::{DekuContainerRead, DekuRead, DekuWrite, DekuWriter, writer::Writer};

use crate::utils::{FieldReader, FieldWriter, RadiotapError, RadiotapField, RadiotapFieldTrait, RadiotapFieldTraitIdentifiers};

/// Timestamp field
///
/// See https://www.radiotap.org/fields/timestamp.html for more information.
#[derive(Clone, Debug, Default, DekuRead, DekuWrite, PartialEq)]
#[deku(endian = "little", bit_order = "lsb")]
pub struct TimestampField {
  /// The timestamp itself, in the unit defined in the `unit` field.
  pub timestamp: u64,

  /// Defines the expected accuracy of the timestamp, in the same units as the timestamp.
  pub accuracy: u16,

  /// The timestamp sampling positions.
  pub sampling_positions: TimestampSamplingPositions,

  /// The timestamp unit.
  pub unit: TimestampUnit,

  /// Flags 32-bit counter
  #[deku(bits = "1")]
  pub flags_32_bit: bool,

  /// Flags accuracy known
  #[deku(bits = "1")]
  pub flags_accuracy: bool,

  /// Flags reserved 0
  #[deku(bits = "6")]
  pub flags_reserved0: u8,
}

impl<V> RadiotapFieldTrait<V> for TimestampField {
  fn get_identifiers(&self) -> RadiotapFieldTraitIdentifiers {
    RadiotapFieldTraitIdentifiers { bit_index: 22 }
  }

  fn get_alignment(&self) -> usize {
    8
  }

  fn decode(&self, reader: &mut FieldReader) -> Result<RadiotapField<V>, RadiotapError> {
    // Decode the field
    let decoded = TimestampField::from_reader((reader, 0))?.1;

    Ok(RadiotapField::Timestamp(decoded))
  }

  fn encode(&self, writer: &mut Writer<&mut FieldWriter>) -> Result<(), RadiotapError> {
    // Encode the field
    self.to_writer(writer, ())?;

    Ok(())
  }
}

/// Timestamp unit
#[derive(Clone, Debug, Default, DekuRead, DekuWrite, PartialEq)]
#[deku(
  id_type = "u8",
  bits = "4",
  bit_order = "order",
  endian = "endian",
  ctx = "endian: deku::ctx::Endian, order: deku::ctx::Order"
)]
pub enum TimestampUnit {
  /// Milliseconds
  #[default]
  #[deku(id = "0")]
  MS,

  /// Microseconds
  #[deku(id = "1")]
  US,

  /// Nanoseconds
  #[deku(id = "2")]
  NS,
}

/// Timestamp sampling positions
#[derive(Clone, Debug, Default, DekuRead, DekuWrite, PartialEq)]
#[deku(
  id_type = "u8",
  bits = "4",
  bit_order = "order",
  endian = "endian",
  ctx = "endian: deku::ctx::Endian, order: deku::ctx::Order"
)]
pub enum TimestampSamplingPositions {
  /// First bit (or symbol containing it) of MAC Protocol Data Unit (MPDU) - matches
  /// Time Synchronization Function Timer (TSFT)
  #[default]
  #[deku(id = "0")]
  BeginMpdu,

  /// Signal acquisition at start of Physical Layer Convergence Procedure (PLCP) preamble
  #[deku(id = "1")]
  PlcpSigAcq,

  /// End of Physical Protocol Data Unit (PPDU)
  #[deku(id = "2")]
  EOPpdu,

  /// End of MAC Protocol Data Unit (MPDU) after the Frame Check Sequence (FCS)
  #[deku(id = "3")]
  EOMpdu,

  /// Unknown or vendor/OOB defined
  #[deku(id = "15")]
  Unknown,
}

#[cfg(test)]
mod tests {
  use deku::{DekuContainerRead, DekuContainerWrite};
  use pretty_assertions::assert_eq;
  use rstest::rstest;

  use super::*;

  /// Encoding test cases
  const ENCODING_CASES: &[(&[u8], &TimestampField)] = &[
    // No flags
    (
      &[
        0xF0, 0xDE, 0xBC, 0x9A, 0x78, 0x56, 0x34, 0x12, // Timestamp
        0x34, 0x12, // Accuracy
        0x02, // Sampling positions
        0x00, // Flags
      ],
      &TimestampField {
        timestamp: 0x123456789ABCDEF0,
        accuracy: 0x1234,
        unit: TimestampUnit::MS,
        sampling_positions: TimestampSamplingPositions::EOPpdu,
        flags_32_bit: false,
        flags_accuracy: false,
        flags_reserved0: 0,
      },
    ),
    // Alternating flags (A)
    (
      &[
        0xF0, 0xDE, 0xBC, 0x9A, 0x78, 0x56, 0x34, 0x12, // Timestamp
        0x34, 0x12, // Accuracy
        0x02, // Sampling positions
        0x01, // Flags
      ],
      &TimestampField {
        timestamp: 0x123456789ABCDEF0,
        accuracy: 0x1234,
        unit: TimestampUnit::MS,
        sampling_positions: TimestampSamplingPositions::EOPpdu,
        flags_32_bit: true,
        flags_accuracy: false,
        flags_reserved0: 0,
      },
    ),
    // Alternating flags (B)
    (
      &[
        0xF0, 0xDE, 0xBC, 0x9A, 0x78, 0x56, 0x34, 0x12, // Timestamp
        0x34, 0x12, // Accuracy
        0x02, // Sampling positions
        0x02, // Flags
      ],
      &TimestampField {
        timestamp: 0x123456789ABCDEF0,
        accuracy: 0x1234,
        unit: TimestampUnit::MS,
        sampling_positions: TimestampSamplingPositions::EOPpdu,
        flags_32_bit: false,
        flags_accuracy: true,
        flags_reserved0: 0,
      },
    ),
  ];

  #[rstest]
  #[case::no_flags(ENCODING_CASES[0].0, ENCODING_CASES[0].1)]
  #[case::alternating_flags_a(ENCODING_CASES[1].0, ENCODING_CASES[1].1)]
  #[case::alternating_flags_b(ENCODING_CASES[2].0, ENCODING_CASES[2].1)]
  fn test_decode_field(#[case] encoded: &[u8], #[case] expected: &TimestampField) {
    // Decode the bytes
    let decoded = TimestampField::from_bytes((encoded, 0)).unwrap().1;

    // Check the result
    assert_eq!(decoded, *expected);
  }

  #[rstest]
  #[case::no_flags(ENCODING_CASES[0].1, ENCODING_CASES[0].0)]
  #[case::alternating_flags_a(ENCODING_CASES[1].1, ENCODING_CASES[1].0)]
  #[case::alternating_flags_b(ENCODING_CASES[2].1, ENCODING_CASES[2].0)]
  fn test_encode_field(#[case] field: &TimestampField, #[case] expected: &[u8]) {
    // Encode the field
    let encoded = field.to_bytes().unwrap();

    // Check the result
    assert_eq!(encoded, expected);
  }
}
