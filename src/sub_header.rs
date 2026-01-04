use alloc::vec::Vec;
use deku::{DekuRead, DekuWrite};

/// Radiotap sub header
///
/// See [www.radiotap.org/](https://www.radiotap.org/) for more information.
#[derive(Clone, Debug, Default, DekuRead, DekuWrite, PartialEq)]
#[deku(endian = "little")]
pub struct SubHeader {
  /// Indicates which major version of the radiotap header is in use. Currently, this is always `0`.
  /// Adding support for additional radiotap fields does not change the version number.
  #[deku(pad_bytes_after = "1")]
  pub version: u8,

  /// Indicates the entire length of the radiotap data, including the radiotap header. This is
  /// valuable for the developer so they can consistently locate the beginning of the 802.11 frame
  /// that follows the radiotap data, even if their parser doesn’t understand all of the data fields
  /// specified.
  #[deku(assert = "*length >= 8")]
  pub length: u16,

  /// Provided bit 31 of this field is not set, the data for fields specified in the this field
  /// bitmask immediately follow the radiotap header. If it is set, then more of these words follow
  /// and the radiotap data follows after the word that has bit 31 unset. Multiple namespaces may be
  /// present.
  #[deku(
    assert = "!present_bitmasks.is_empty()",
    until = "|word| word & 0x80000000 == 0"
  )]
  pub present_bitmasks: Vec<u32>,
}

#[cfg(test)]
mod tests {
  use deku::{DekuContainerRead, DekuContainerWrite};
  use pretty_assertions::assert_eq;
  use rstest::rstest;

  use super::*;

  /// Get encoding test cases
  fn get_encoding_cases() -> Vec<(Vec<u8>, SubHeader)> {
    vec![
      // Single present bitmask, no fields
      (
        vec![
          0x00, // Version
          0x00, // Padding
          0x08, 0x00, // Length
          0x00, 0x00, 0x00, 0x00, // Present bitmask
        ],
        SubHeader {
          version: 0,
          length: 8,
          present_bitmasks: vec![0x00000000],
        },
      ),
      // Multiple present bitmasks, no fields
      (
        vec![
          0x00, // Version
          0x00, // Padding
          0x10, 0x00, // Length
          0x01, 0x00, 0x00, 0x80, // Present bitmask 1
          0x02, 0x00, 0x00, 0x80, // Present bitmask 2
          0x03, 0x00, 0x00, 0x00, // Present bitmask 3
        ],
        SubHeader {
          version: 0,
          length: 16,
          present_bitmasks: vec![0x80000001, 0x80000002, 0x00000003],
        },
      ),
      // Single present bitmask, with fields
      (
        vec![
          0x00, // Version
          0x00, // Padding
          0x0C, 0x00, // Length
          0x00, 0x00, 0x00, 0x00, // Present bitmask
        ],
        SubHeader {
          version: 0,
          length: 12,
          present_bitmasks: vec![0x00000000],
        },
      ),
      // Multiple present bitmasks, with fields
      (
        vec![
          0x00, // Version
          0x00, // Padding
          0x14, 0x00, // Length
          0x01, 0x00, 0x00, 0x80, // Present bitmask 1
          0x02, 0x00, 0x00, 0x80, // Present bitmask 2
          0x03, 0x00, 0x00, 0x00, // Present bitmask 3
        ],
        SubHeader {
          version: 0,
          length: 20,
          present_bitmasks: vec![0x80000001, 0x80000002, 0x00000003],
        },
      ),
    ]
  }

  #[rstest]
  #[case::single_present_bitmask_no_fields(&get_encoding_cases()[0].0, &get_encoding_cases()[0].1)]
  #[case::multiple_present_bitmasks_no_fields(&get_encoding_cases()[1].0, &get_encoding_cases()[1].1)]
  #[case::single_present_bitmask_fields(&get_encoding_cases()[2].0, &get_encoding_cases()[2].1)]
  #[case::multiple_present_bitmasks_fields(&get_encoding_cases()[3].0, &get_encoding_cases()[3].1)]
  fn test_decode_radiotap_header(#[case] encoded: &[u8], #[case] expected: &SubHeader) {
    // Decode the bytes
    let decoded = SubHeader::from_bytes((encoded, 0)).unwrap().1;

    // Check the result
    assert_eq!(decoded, *expected);
  }

  #[rstest]
  #[case::single_present_bitmask_no_fields(get_encoding_cases()[0].1.clone(), &get_encoding_cases()[0].0)]
  #[case::multiple_present_bitmasks_no_fields(get_encoding_cases()[1].1.clone(), &get_encoding_cases()[1].0)]
  #[case::single_present_bitmask_fields(get_encoding_cases()[2].1.clone(), &get_encoding_cases()[2].0)]
  #[case::multiple_present_bitmasks_fields(get_encoding_cases()[3].1.clone(), &get_encoding_cases()[3].0)]
  fn test_encode_radiotap_header(#[case] header: SubHeader, #[case] decoded: &[u8]) {
    // Encode the struct
    let encoded = header.to_bytes().unwrap();

    // Check the result
    assert_eq!(encoded, decoded);
  }
}
