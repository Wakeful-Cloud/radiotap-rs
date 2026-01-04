use deku::{DekuContainerRead, DekuRead, DekuWrite, DekuWriter, writer::Writer};

use crate::utils::{FieldReader, FieldWriter, RadiotapError, RadiotapField, RadiotapFieldTrait, RadiotapFieldTraitIdentifiers};

/// Transmit (TX) flags field
///
/// The `NO_ACK` bit (`0x0008`) is used for uses of radiotap when frames are sent. Similarly, the
/// `NO_SEQ_NO` bit (`0x0010`) is used to allow userspace to send frames with a specific sequence
/// number (e.g. when transmitting fragments of a single frame one-by-one). When it is set, the
/// frame should be transmitted with the sequence number included in the 802.11 MAC header of the
/// frame as received from userspace, regardless of the state of the sequence counters in
/// driver/hardware.
///
/// When the `REORDER` bit (0x0020) is set, injected frames aren’t reordered relative to other
/// frames that also have this bit set (even when these frames have different QoS TID values).
///
/// See https://www.radiotap.org/fields/TX%20flags.html for more information.
#[derive(Clone, Debug, Default, DekuRead, DekuWrite, PartialEq)]
#[deku(endian = "little", bit_order = "lsb")]
pub struct TxFlagsField {
  /// Flags transmission failed due to excessive retries
  #[deku(bits = "1")]
  pub flags_tx_fail: bool,

  /// Flags transmission used Clear To Send (CTS)-to-self protection
  #[deku(bits = "1")]
  pub flags_cts: bool,

  /// Flags transmission used Request To Send (RTS)/Clear To Send (CTS) handshake
  #[deku(bits = "1")]
  pub flags_rts: bool,

  /// Flags transmission shall not expect an ACK frame and not retry when no ACK is received
  #[deku(bits = "1")]
  pub flags_no_ack: bool,

  /// Flags transmission includes a pre-configured sequence number that should not be changed by the
  /// driver's transmit handlers
  #[deku(bits = "1")]
  pub flags_no_seq_no: bool,

  /// Flags transmission should not be reordered relative to other frames that have this flag set
  #[deku(bits = "1")]
  pub flags_order: bool,

  /// Flags reserved 0
  #[deku(bits = "10")]
  pub flags_reserved0: u16,
}

impl<V> RadiotapFieldTrait<V> for TxFlagsField {
  fn get_identifiers(&self) -> RadiotapFieldTraitIdentifiers {
    RadiotapFieldTraitIdentifiers { bit_index: 15 }
  }

  fn get_alignment(&self) -> usize {
    2
  }

  fn decode(&self, reader: &mut FieldReader) -> Result<RadiotapField<V>, RadiotapError> {
    // Decode the field
    let decoded = TxFlagsField::from_reader((reader, 0))?.1;

    Ok(RadiotapField::TxFlags(decoded))
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
  const ENCODING_CASES: &[(&[u8], &TxFlagsField)] = &[
    // No flags
    (
      &[
        0x00, 0x00, // Flags
      ],
      &TxFlagsField {
        flags_tx_fail: false,
        flags_cts: false,
        flags_rts: false,
        flags_no_ack: false,
        flags_no_seq_no: false,
        flags_order: false,
        flags_reserved0: 0,
      },
    ),
    // Alternating flags (A)
    (
      &[
        0x15, 0x00, // Flags
      ],
      &TxFlagsField {
        flags_tx_fail: true,
        flags_cts: false,
        flags_rts: true,
        flags_no_ack: false,
        flags_no_seq_no: true,
        flags_order: false,
        flags_reserved0: 0,
      },
    ),
    // Alternating flags (B)
    (
      &[
        0x2A, 0x00, // Flags
      ],
      &TxFlagsField {
        flags_tx_fail: false,
        flags_cts: true,
        flags_rts: false,
        flags_no_ack: true,
        flags_no_seq_no: false,
        flags_order: true,
        flags_reserved0: 0,
      },
    ),
  ];

  #[rstest]
  #[case::no_flags(ENCODING_CASES[0].0, ENCODING_CASES[0].1)]
  #[case::alternating_flags_a(ENCODING_CASES[1].0, ENCODING_CASES[1].1)]
  #[case::alternating_flags_b(ENCODING_CASES[2].0, ENCODING_CASES[2].1)]
  fn test_decode_field(#[case] encoded: &[u8], #[case] expected: &TxFlagsField) {
    // Decode the bytes
    let decoded = TxFlagsField::from_bytes((encoded, 0)).unwrap().1;

    // Check the result
    assert_eq!(decoded, *expected);
  }

  #[rstest]
  #[case::no_flags(ENCODING_CASES[0].1, ENCODING_CASES[0].0)]
  #[case::alternating_flags_a(ENCODING_CASES[1].1, ENCODING_CASES[1].0)]
  #[case::alternating_flags_b(ENCODING_CASES[2].1, ENCODING_CASES[2].0)]
  fn test_encode_field(#[case] field: &TxFlagsField, #[case] expected: &[u8]) {
    // Encode the field
    let encoded = field.to_bytes().unwrap();

    // Check the result
    assert_eq!(encoded, expected);
  }
}
