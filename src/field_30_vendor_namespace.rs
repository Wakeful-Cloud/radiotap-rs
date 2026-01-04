use deku::{DekuRead, DekuWrite};

/// Vendor namespace field
///
/// See [www.radiotap.org/fields/Vendor%20Namespace.html](https://www.radiotap.org/fields/Vendor%20Namespace.html)
/// for more information.
#[derive(Clone, Debug, Default, DekuRead, DekuWrite, PartialEq)]
#[deku(endian = "little", bit_order = "lsb")]
pub struct VendorNamespaceField {
  /// Vendor Organizationally Unique Identifier (OUI)
  pub oui: [u8; 3],

  /// Sub-namespace
  pub sub_namespace: u8,

  /// Skip length
  pub skip_length: u16,
}

#[cfg(test)]
mod tests {
  use deku::{DekuContainerRead, DekuContainerWrite};
  use pretty_assertions::assert_eq;
  use rstest::rstest;

  use super::*;

  /// Encoding test cases
  const ENCODING_CASES: &[(&[u8], &VendorNamespaceField)] = &[
    // Empty data
    (
      &[
        0xAA, 0xBB, 0xCC, // OUI
        0x12, // Sub-namespace
        0x00, 0x00, // Skip length
      ],
      &VendorNamespaceField {
        oui: [0xAA, 0xBB, 0xCC],
        sub_namespace: 0x12,
        skip_length: 0,
      },
    ),
    // Non-empty data
    (
      &[
        0xAA, 0xBB, 0xCC, // OUI
        0x12, // Sub-namespace
        0x04, 0x00, // Skip length
      ],
      &VendorNamespaceField {
        oui: [0xAA, 0xBB, 0xCC],
        sub_namespace: 0x12,
        skip_length: 4,
      },
    ),
  ];

  #[rstest]
  #[case::empty(&ENCODING_CASES[0].0, &ENCODING_CASES[0].1)]
  #[case::non_empty(&ENCODING_CASES[1].0, &ENCODING_CASES[1].1)]
  fn test_decode_field(#[case] encoded: &[u8], #[case] expected: &VendorNamespaceField) {
    // Decode the bytes
    let decoded = VendorNamespaceField::from_bytes((encoded, 0)).unwrap().1;

    // Check the result
    assert_eq!(decoded, *expected);
  }

  #[rstest]
  #[case::empty(&ENCODING_CASES[0].1, &ENCODING_CASES[0].0)]
  #[case::non_empty(&ENCODING_CASES[1].1, &ENCODING_CASES[1].0)]
  fn test_encode_field(#[case] field: &VendorNamespaceField, #[case] expected: &[u8]) {
    // Encode the field
    let encoded = field.to_bytes().unwrap();

    // Check the result
    assert_eq!(encoded, expected);
  }
}
