use alloc::vec::Vec;
use deku::{DekuRead, DekuWrite};

/// Type, Length, Value (TLV) field
///
/// See [www.radiotap.org/fields/TLV.html](https://www.radiotap.org/fields/TLV.html) for more
/// information.
#[derive(Clone, Debug, Default, DekuRead, DekuWrite, PartialEq)]
#[deku(endian = "little", bit_order = "lsb")]
pub struct TlvField {
  /// TLV field items
  #[deku(read_all)]
  pub items: Vec<TlvFieldItem>,
}

/// TLV field item
#[derive(Clone, Debug, DekuRead, DekuWrite, PartialEq)]
#[deku(
  id_type = "u16",
  bit_order = "order",
  endian = "endian",
  ctx = "endian: deku::ctx::Endian, order: deku::ctx::Order"
)]
pub enum TlvFieldItem {
  /// Vendor namespace
  #[deku(id = "30")]
  Vendor {
    /// Field length, excluding padding, but including the 8 bytes of the vendor header
    length: u16,

    /// Vendor Organizationally Unique Identifier (OUI)
    oui: [u8; 3],

    /// Sub-namespace
    sub_namespace: u8,

    /// Presence type index of the bit previously used in the presence bitmap, e.g. `0` for `b0`
    #[deku(pad_bytes_after = "2")]
    presence: u16,

    /// Data (Note that this might be shorter than expected due to implicit trailing `0x00` bytes)
    #[deku(
      count = "*length as usize - 8",
      pad_bytes_after = "((4 - ((*length as usize - 8) % 4)) % 4)"
    )]
    value: Vec<u8>,
  },

  /// Radiotap namespace
  #[deku(id_pat = "_")]
  Radiotap {
    /// Field type (From the regular radiotap type (bit) allocation, but the special values `29` and
    /// `31` are not valid)
    r#type: u16,

    /// Field length, excluding padding
    length: u16,

    /// Field data (Note that this might be shorter than expected due to implicit trailing `0x00`
    /// bytes)
    #[deku(
      count = "*length as usize",
      pad_bytes_after = "((4 - (*length as usize % 4)) % 4)"
    )]
    value: Vec<u8>,
  },
}

#[cfg(test)]
mod tests {
  use deku::{DekuContainerRead, DekuContainerWrite};
  use pretty_assertions::assert_eq;
  use rstest::rstest;

  use super::*;

  /// Get encoding test cases
  fn get_encoding_cases() -> Vec<(Vec<u8>, TlvField)> {
    vec![
      // No items
      (vec![], TlvField { items: vec![] }),
      // Single empty aligned radiotap item
      (
        vec![
          0x01, 0x00, // Type
          0x00, 0x00, // Length
                // Value
        ],
        TlvField {
          items: vec![TlvFieldItem::Radiotap {
            r#type: 1,
            length: 0,
            value: vec![],
          }],
        },
      ),
      // Single non-empty aligned radiotap item
      (
        vec![
          0x01, 0x00, // Type
          0x04, 0x00, // Length
          0x01, 0x02, 0x03, 0x04, // Value
        ],
        TlvField {
          items: vec![TlvFieldItem::Radiotap {
            r#type: 1,
            length: 4,
            value: vec![0x01, 0x02, 0x03, 0x04],
          }],
        },
      ),
      // Single one-byte unaligned radiotap item
      (
        vec![
          0x01, 0x00, // Type
          0x01, 0x00, // Length
          0x01, // Value
          0x00, 0x00, 0x00, // Padding
        ],
        TlvField {
          items: vec![TlvFieldItem::Radiotap {
            r#type: 1,
            length: 1,
            value: vec![0x01],
          }],
        },
      ),
      // Single two-byte unaligned radiotap item
      (
        vec![
          0x01, 0x00, // Type
          0x02, 0x00, // Length
          0x01, 0x02, // Value
          0x00, 0x00, // Padding
        ],
        TlvField {
          items: vec![TlvFieldItem::Radiotap {
            r#type: 1,
            length: 2,
            value: vec![0x01, 0x02],
          }],
        },
      ),
      // Single three-byte unaligned radiotap item
      (
        vec![
          0x01, 0x00, // Type
          0x03, 0x00, // Length
          0x01, 0x02, 0x03, // Value
          0x00, // Padding
        ],
        TlvField {
          items: vec![TlvFieldItem::Radiotap {
            r#type: 1,
            length: 3,
            value: vec![0x01, 0x02, 0x03],
          }],
        },
      ),
      // Single empty vendor item
      (
        vec![
          0x1E, 0x00, // Type
          0x08, 0x00, // Length
          0xAA, 0xBB, 0xCC, // OUI
          0x01, // Sub-namespace
          0x02, 0x00, // Presence
          0x00, 0x00, // Padding
        ],
        TlvField {
          items: vec![TlvFieldItem::Vendor {
            length: 8,
            oui: [0xAA, 0xBB, 0xCC],
            sub_namespace: 1,
            presence: 2,
            value: vec![],
          }],
        },
      ),
      // Single non-empty aligned vendor item
      (
        vec![
          0x1E, 0x00, // Type
          0x0C, 0x00, // Length
          0xAA, 0xBB, 0xCC, // OUI
          0x01, // Sub-namespace
          0x02, 0x00, // Presence
          0x00, 0x00, // Padding
          0xDE, 0xAD, 0xBE, 0xEF, // Value
        ],
        TlvField {
          items: vec![TlvFieldItem::Vendor {
            length: 12,
            oui: [0xAA, 0xBB, 0xCC],
            sub_namespace: 1,
            presence: 2,
            value: vec![0xDE, 0xAD, 0xBE, 0xEF],
          }],
        },
      ),
      // Single one-byte unaligned vendor item
      (
        vec![
          0x1E, 0x00, // Type
          0x09, 0x00, // Length
          0xAA, 0xBB, 0xCC, // OUI
          0x01, // Sub-namespace
          0x02, 0x00, // Presence
          0x00, 0x00, // Padding
          0xDE, // Value
          0x00, 0x00, 0x00, // Padding
        ],
        TlvField {
          items: vec![TlvFieldItem::Vendor {
            length: 9,
            oui: [0xAA, 0xBB, 0xCC],
            sub_namespace: 1,
            presence: 2,
            value: vec![0xDE],
          }],
        },
      ),
      // Single two-byte unaligned vendor item
      (
        vec![
          0x1E, 0x00, // Type
          0x0A, 0x00, // Length
          0xAA, 0xBB, 0xCC, // OUI
          0x01, // Sub-namespace
          0x02, 0x00, // Presence
          0x00, 0x00, // Padding
          0xDE, 0xAD, // Value
          0x00, 0x00, // Padding
        ],
        TlvField {
          items: vec![TlvFieldItem::Vendor {
            length: 10,
            oui: [0xAA, 0xBB, 0xCC],
            sub_namespace: 1,
            presence: 2,
            value: vec![0xDE, 0xAD],
          }],
        },
      ),
      // Single three-byte unaligned vendor item
      (
        vec![
          0x1E, 0x00, // Type
          0x0B, 0x00, // Length
          0xAA, 0xBB, 0xCC, // OUI
          0x01, // Sub-namespace
          0x02, 0x00, // Presence
          0x00, 0x00, // Padding
          0xDE, 0xAD, 0xBE, // Value
          0x00, // Padding
        ],
        TlvField {
          items: vec![TlvFieldItem::Vendor {
            length: 11,
            oui: [0xAA, 0xBB, 0xCC],
            sub_namespace: 1,
            presence: 2,
            value: vec![0xDE, 0xAD, 0xBE],
          }],
        },
      ),
      // Non-empty aligned radiotap item, non-empty aligned vendor item
      (
        vec![
          // Item 1
          0x01, 0x00, // Type
          0x04, 0x00, // Length
          0x01, 0x02, 0x03, 0x04, // Value
          // Item 2
          0x1E, 0x00, // Type
          0x0C, 0x00, // Length
          0xAA, 0xBB, 0xCC, // OUI
          0x01, // Sub-namespace
          0x02, 0x00, // Presence
          0x00, 0x00, // Padding
          0xDE, 0xAD, 0xBE, 0xEF, // Value
        ],
        TlvField {
          items: vec![
            TlvFieldItem::Radiotap {
              r#type: 1,
              length: 4,
              value: vec![0x01, 0x02, 0x03, 0x04],
            },
            TlvFieldItem::Vendor {
              length: 12,
              oui: [0xAA, 0xBB, 0xCC],
              sub_namespace: 1,
              presence: 2,
              value: vec![0xDE, 0xAD, 0xBE, 0xEF],
            },
          ],
        },
      ),
    ]
  }

  #[rstest]
  #[case::no_items(&get_encoding_cases()[0].0, &get_encoding_cases()[0].1)]
  #[case::single_empty_aligned_radiotap_item(&get_encoding_cases()[1].0, &get_encoding_cases()[1].1)]
  #[case::single_non_empty_aligned_radiotap_item(&get_encoding_cases()[2].0, &get_encoding_cases()[2].1)]
  #[case::single_one_byte_unaligned_radiotap_item(&get_encoding_cases()[3].0, &get_encoding_cases()[3].1)]
  #[case::single_two_byte_unaligned_radiotap_item(&get_encoding_cases()[4].0, &get_encoding_cases()[4].1)]
  #[case::single_three_byte_unaligned_radiotap_item(&get_encoding_cases()[5].0, &get_encoding_cases()[5].1)]
  #[case::single_empty_vendor_item(&get_encoding_cases()[6].0, &get_encoding_cases()[6].1)]
  #[case::single_non_empty_aligned_vendor_item(&get_encoding_cases()[7].0, &get_encoding_cases()[7].1)]
  #[case::single_one_byte_unaligned_vendor_item(&get_encoding_cases()[8].0, &get_encoding_cases()[8].1)]
  #[case::single_two_byte_unaligned_vendor_item(&get_encoding_cases()[9].0, &get_encoding_cases()[9].1)]
  #[case::single_three_byte_unaligned_vendor_item(&get_encoding_cases()[10].0, &get_encoding_cases()[10].1)]
  #[case::non_empty_aligned_radiotap_item_non_empty_aligned_vendor_item(&get_encoding_cases()[11].0, &get_encoding_cases()[11].1)]
  fn test_decode_field(#[case] encoded: &[u8], #[case] expected: &TlvField) {
    // Decode the bytes
    let decoded = TlvField::from_bytes((encoded, 0)).unwrap().1;

    // Check the result
    assert_eq!(decoded, *expected);
  }

  #[rstest]
  #[case::no_items(&get_encoding_cases()[0].1, &get_encoding_cases()[0].0)]
  #[case::single_empty_aligned_radiotap_item(&get_encoding_cases()[1].1, &get_encoding_cases()[1].0)]
  #[case::single_non_empty_aligned_radiotap_item(&get_encoding_cases()[2].1, &get_encoding_cases()[2].0)]
  #[case::single_one_byte_unaligned_radiotap_item(&get_encoding_cases()[3].1, &get_encoding_cases()[3].0)]
  #[case::single_two_byte_unaligned_radiotap_item(&get_encoding_cases()[4].1, &get_encoding_cases()[4].0)]
  #[case::single_three_byte_unaligned_radiotap_item(&get_encoding_cases()[5].1, &get_encoding_cases()[5].0)]
  #[case::single_empty_vendor_item(&get_encoding_cases()[6].1, &get_encoding_cases()[6].0)]
  #[case::single_non_empty_aligned_vendor_item(&get_encoding_cases()[7].1, &get_encoding_cases()[7].0)]
  #[case::single_one_byte_unaligned_vendor_item(&get_encoding_cases()[8].1, &get_encoding_cases()[8].0)]
  #[case::single_two_byte_unaligned_vendor_item(&get_encoding_cases()[9].1, &get_encoding_cases()[9].0)]
  #[case::single_three_byte_unaligned_vendor_item(&get_encoding_cases()[10].1, &get_encoding_cases()[10].0)]
  #[case::non_empty_aligned_radiotap_item_non_empty_aligned_vendor_item(&get_encoding_cases()[11].1, &get_encoding_cases()[11].0)]
  fn test_encode_field(#[case] field: &TlvField, #[case] expected: &[u8]) {
    // Encode the field
    let encoded = field.to_bytes().unwrap();

    // Check the result
    assert_eq!(encoded, expected);
  }
}
