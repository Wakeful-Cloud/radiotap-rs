use deku::{DekuRead, DekuWrite, DekuWriter};
use pretty_assertions::assert_eq;
use rstest::rstest;

use crate::field_19_mcs::McsFlagsBw;

use super::*;

/// Test vendor field A
#[derive(Clone, Debug, Default, DekuRead, DekuWrite, PartialEq)]
#[deku(endian = "little", bit_order = "lsb")]
struct TestVendorFieldA {
  value: u8,
}

impl VendorFieldTrait<TestVendorFields> for TestVendorFieldA {
  fn get_identifiers(&self) -> VendorFieldTraitIdentifiers {
    VendorFieldTraitIdentifiers {
      oui: [0x02, 0x03, 0x04],
      sub_namespace: 0x05,
      bit_index: 42,
    }
  }

  fn get_alignment(&self) -> usize {
    1
  }

  fn decode(&self, reader: &mut FieldReader) -> Result<TestVendorFields, RadiotapError> {
    // Decode the field
    let decoded = TestVendorFieldA::from_reader((reader, 0))?.1;

    Ok(TestVendorFields::A(decoded))
  }

  fn encode(&self, writer: &mut Writer<&mut FieldWriter>) -> Result<(), RadiotapError> {
    // Encode the field
    self.to_writer(writer, ())?;

    Ok(())
  }
}
/// Test vendor field B
#[derive(Clone, Debug, Default, DekuRead, DekuWrite, PartialEq)]
#[deku(endian = "little", bit_order = "lsb")]
struct TestVendorFieldB {
  value: u8,
}

impl VendorFieldTrait<TestVendorFields> for TestVendorFieldB {
  fn get_identifiers(&self) -> VendorFieldTraitIdentifiers {
    VendorFieldTraitIdentifiers {
      oui: [0x02, 0x03, 0x04],
      sub_namespace: 0x05,
      bit_index: 43,
    }
  }

  fn get_alignment(&self) -> usize {
    2
  }

  fn decode(&self, reader: &mut FieldReader) -> Result<TestVendorFields, RadiotapError> {
    // Decode the field
    let decoded = TestVendorFieldB::from_reader((reader, 0))?.1;

    Ok(TestVendorFields::B(decoded))
  }

  fn encode(&self, writer: &mut Writer<&mut FieldWriter>) -> Result<(), RadiotapError> {
    // Encode the field
    self.to_writer(writer, ())?;

    Ok(())
  }
}

/// Test vendor fields
#[derive(Clone, Debug, PartialEq)]
enum TestVendorFields {
  A(TestVendorFieldA),
  B(TestVendorFieldB),
}

impl VendorFieldTrait<TestVendorFields> for TestVendorFields {
  fn get_identifiers(&self) -> VendorFieldTraitIdentifiers {
    match self {
      TestVendorFields::A(field) => field.get_identifiers(),
      TestVendorFields::B(field) => field.get_identifiers(),
    }
  }

  fn get_alignment(&self) -> usize {
    match self {
      TestVendorFields::A(field) => field.get_alignment(),
      TestVendorFields::B(field) => field.get_alignment(),
    }
  }

  fn decode(&self, reader: &mut FieldReader) -> Result<TestVendorFields, RadiotapError> {
    match self {
      TestVendorFields::A(field) => field.decode(reader),
      TestVendorFields::B(field) => field.decode(reader),
    }
  }

  fn encode(&self, writer: &mut Writer<&mut FieldWriter>) -> Result<(), RadiotapError> {
    match self {
      TestVendorFields::A(field) => field.encode(writer),
      TestVendorFields::B(field) => field.encode(writer),
    }
  }
}

/// Get radiotap header from bytes test cases
fn get_radiotap_header_from_bytes_cases() -> Vec<(Vec<u8>, RadiotapHeader<TestVendorFields>, usize)>
{
  vec![
    // Single present bitmask, no fields, without leftovers
    (
      vec![
        0x00, // Version
        0x00, // Padding
        0x08, 0x00, // Length
        0x00, 0x00, 0x00, 0x00, // Present bitmask
      ],
      RadiotapHeader {
        version: 0,
        fields: vec![],
      },
      8,
    ),
    // Single present bitmask, no fields, with leftovers
    (
      vec![
        0x00, // Version
        0x00, // Padding
        0x08, 0x00, // Length
        0x00, 0x00, 0x00, 0x00, // Present bitmask
        0xFF, 0xFF, // Leftovers
      ],
      RadiotapHeader {
        version: 0,
        fields: vec![],
      },
      8,
    ),
    // Multiple present bitmasks, no fields, without leftovers
    (
      vec![
        0x00, // Version
        0x00, // Padding
        0x0C, 0x00, // Length
        0x00, 0x00, 0x00, 0x80, // Present bitmask 1
        0x00, 0x00, 0x00, 0x00, // Present bitmask 2
      ],
      RadiotapHeader {
        version: 0,
        fields: vec![],
      },
      12,
    ),
    // Multiple present bitmasks, no fields, with leftovers
    (
      vec![
        0x00, // Version
        0x00, // Padding
        0x0C, 0x00, // Length
        0x00, 0x00, 0x00, 0x80, // Present bitmask 1
        0x00, 0x00, 0x00, 0x00, // Present bitmask 2
        0xFF, 0xFF, // Leftovers
      ],
      RadiotapHeader {
        version: 0,
        fields: vec![],
      },
      12,
    ),
    // Single present bitmask, with single implicit-length field, without leftovers
    (
      vec![
        0x00, // Version
        0x00, // Padding
        0x09, 0x00, // Length
        0x04, 0x00, 0x00, 0x00, // Present bitmask
        // Rate field
        0x42, // Rate
      ],
      RadiotapHeader {
        version: 0,
        fields: vec![RadiotapField::Rate(RateField { rate: 0x42 })],
      },
      9,
    ),
    // Single present bitmask, with single implicit-length field, with leftovers
    (
      vec![
        0x00, // Version
        0x00, // Padding
        0x09, 0x00, // Length
        0x04, 0x00, 0x00, 0x00, // Present bitmask
        // Rate field
        0x42, // Rate
        0xFF, 0xFF, // Leftovers
      ],
      RadiotapHeader {
        version: 0,
        fields: vec![RadiotapField::Rate(RateField { rate: 0x42 })],
      },
      9,
    ),
    // Single present bitmask, with multiple aligned implicit-length fields, without leftovers
    (
      vec![
        0x00, // Version
        0x00, // Padding
        0x0A, 0x00, // Length
        0x24, 0x00, 0x00, 0x00, // Present bitmask
        // Rate field
        0x42, // Rate
        // Antenna signal field
        0x13, // Signal
      ],
      RadiotapHeader {
        version: 0,
        fields: vec![
          RadiotapField::Rate(RateField { rate: 0x42 }),
          RadiotapField::DbmAntSignal(DbmAntSignalField { signal: 0x13 }),
        ],
      },
      10,
    ),
    // Single present bitmask, with multiple aligned implicit-length fields, with leftovers
    (
      vec![
        0x00, // Version
        0x00, // Padding
        0x0A, 0x00, // Length
        0x24, 0x00, 0x00, 0x00, // Present bitmask
        // Rate field
        0x42, // Rate
        // Antenna signal field
        0x13, // Signal
        0xFF, 0xFF, // Leftovers
      ],
      RadiotapHeader {
        version: 0,
        fields: vec![
          RadiotapField::Rate(RateField { rate: 0x42 }),
          RadiotapField::DbmAntSignal(DbmAntSignalField { signal: 0x13 }),
        ],
      },
      10,
    ),
    // Single present bitmask, with multiple unaligned implicit-length fields, without leftovers
    (
      vec![
        0x00, // Version
        0x00, // Padding
        0x10, 0x00, // Length
        0x00, 0x00, 0x08, 0x08, // Present bitmask
        // MCS field
        0x01, // Known
        0x02, // Flags
        0x03, // MCS
        // L-SIG field
        0x00, // Padding
        0x01, 0x00, // Data 1
        0xCE, 0xAB, // Data 2
      ],
      RadiotapHeader {
        version: 0,
        fields: vec![
          RadiotapField::Mcs(McsField {
            known_bw: true,
            known_mcs: false,
            known_gi: false,
            known_fmt: false,
            known_fec: false,
            known_stbc: false,
            known_ness: false,
            known_ness_bit1: false,
            flags_bw: McsFlagsBw::Bw20L,
            flags_gi: false,
            flags_fmt: false,
            flags_fec: false,
            flags_stbc: 0,
            flags_ness_bit0: false,
            mcs: 3,
          }),
          RadiotapField::Lsig(LsigField {
            data1_rate_known: true,
            data1_length_known: false,
            data1_reserved0: 0,
            data2_rate: 0xE,
            data2_length: 0xABC,
          }),
        ],
      },
      16,
    ),
    // Single present bitmask, with multiple unaligned implicit-length fields, with leftovers
    (
      vec![
        0x00, // Version
        0x00, // Padding
        0x10, 0x00, // Length
        0x00, 0x00, 0x08, 0x08, // Present bitmask
        // MCS field
        0x01, // Known
        0x02, // Flags
        0x03, // MCS
        // L-SIG field
        0x00, // Padding
        0x01, 0x00, // Data 1
        0xCE, 0xAB, // Data 2
        0xFF, 0xFF, // Leftovers
      ],
      RadiotapHeader {
        version: 0,
        fields: vec![
          RadiotapField::Mcs(McsField {
            known_bw: true,
            known_mcs: false,
            known_gi: false,
            known_fmt: false,
            known_fec: false,
            known_stbc: false,
            known_ness: false,
            known_ness_bit1: false,
            flags_bw: McsFlagsBw::Bw20L,
            flags_gi: false,
            flags_fmt: false,
            flags_fec: false,
            flags_stbc: 0,
            flags_ness_bit0: false,
            mcs: 3,
          }),
          RadiotapField::Lsig(LsigField {
            data1_rate_known: true,
            data1_length_known: false,
            data1_reserved0: 0,
            data2_rate: 0xE,
            data2_length: 0xABC,
          }),
        ],
      },
      16,
    ),
    // Multiple present bitmasks, with multiple aligned implicit-length fields, without leftovers
    (
      vec![
        0x00, // Version
        0x00, // Padding
        0x0E, 0x00, // Length
        0x04, 0x00, 0x00, 0xA0, // Present bitmask 1
        0x20, 0x00, 0x00, 0x00, // Present bitmask 2
        // Rate field
        0x42, // Rate
        // Antenna signal field
        0x13, // Signal
      ],
      RadiotapHeader {
        version: 0,
        fields: vec![
          RadiotapField::Rate(RateField { rate: 0x42 }),
          RadiotapField::DbmAntSignal(DbmAntSignalField { signal: 0x13 }),
        ],
      },
      14,
    ),
    // Multiple present bitmasks, with multiple aligned implicit-length fields, with leftovers
    (
      vec![
        0x00, // Version
        0x00, // Padding
        0x0E, 0x00, // Length
        0x04, 0x00, 0x00, 0xA0, // Present bitmask 1
        0x20, 0x00, 0x00, 0x00, // Present bitmask 2
        // Rate field
        0x42, // Rate
        // Antenna signal field
        0x13, // Signal
        0xFF, 0xFF, // Leftovers
      ],
      RadiotapHeader {
        version: 0,
        fields: vec![
          RadiotapField::Rate(RateField { rate: 0x42 }),
          RadiotapField::DbmAntSignal(DbmAntSignalField { signal: 0x13 }),
        ],
      },
      14,
    ),
    // Multiple present bitmasks, with repeated implicit-length fields, without leftovers
    (
      vec![
        0x00, // Version
        0x00, // Padding
        0x0E, 0x00, // Length
        0x04, 0x00, 0x00, 0xA0, // Present bitmask 1
        0x04, 0x00, 0x00, 0x00, // Present bitmask 2
        // Rate field 1
        0x42, // Rate
        // Rate field 2
        0x13, // Rate
      ],
      RadiotapHeader {
        version: 0,
        fields: vec![
          RadiotapField::Rate(RateField { rate: 0x42 }),
          RadiotapField::Rate(RateField { rate: 0x13 }),
        ],
      },
      14,
    ),
    // Multiple present bitmasks, with repeated implicit-length fields, with leftovers
    (
      vec![
        0x00, // Version
        0x00, // Padding
        0x0E, 0x00, // Length
        0x04, 0x00, 0x00, 0xA0, // Present bitmask 1
        0x04, 0x00, 0x00, 0x00, // Present bitmask 2
        // Rate field 1
        0x42, // Rate
        // Rate field 2
        0x13, // Rate
        0xFF, 0xFF, // Leftovers
      ],
      RadiotapHeader {
        version: 0,
        fields: vec![
          RadiotapField::Rate(RateField { rate: 0x42 }),
          RadiotapField::Rate(RateField { rate: 0x13 }),
        ],
      },
      14,
    ),
    // Single present bitmask, with single radiotap TLV field, without leftovers
    (
      vec![
        0x00, // Version
        0x00, // Padding
        0x10, 0x00, // Length
        0x00, 0x00, 0x00, 0x10, // Present bitmask
        // TLV field
        // Rate subfield
        0x02, 0x00, // Type
        0x01, 0x00, // Length
        0x42, // Rate
        0x00, 0x00, 0x00, // Padding
      ],
      RadiotapHeader {
        version: 0,
        fields: vec![RadiotapField::Rate(RateField { rate: 0x42 })],
      },
      16,
    ),
    // Single present bitmask, with single radiotap TLV field, with leftovers
    (
      vec![
        0x00, // Version
        0x00, // Padding
        0x10, 0x00, // Length
        0x00, 0x00, 0x00, 0x10, // Present bitmask
        // TLV field
        // Rate subfield
        0x02, 0x00, // Type
        0x01, 0x00, // Length
        0x42, // Rate
        0x00, 0x00, 0x00, // Padding
        0xFF, 0xFF, // Leftovers
      ],
      RadiotapHeader {
        version: 0,
        fields: vec![RadiotapField::Rate(RateField { rate: 0x42 })],
      },
      16,
    ),
    // Single present bitmask, with single radiotap TLV field with implicit trailing zeros, without leftovers
    (
      vec![
        0x00, // Version
        0x00, // Padding
        0x0C, 0x00, // Length
        0x00, 0x00, 0x00, 0x10, // Present bitmask
        // TLV field
        // Rate subfield
        0x02, 0x00, // Type
        0x00, 0x00, // Length
              // Rate (Implicit trailing zeros)
      ],
      RadiotapHeader {
        version: 0,
        fields: vec![RadiotapField::Rate(RateField { rate: 0x00 })],
      },
      12,
    ),
    // Single present bitmask, with single radiotap TLV field with implicit trailing zeros, with leftovers
    (
      vec![
        0x00, // Version
        0x00, // Padding
        0x0C, 0x00, // Length
        0x00, 0x00, 0x00, 0x10, // Present bitmask
        // TLV field
        // Rate subfield
        0x02, 0x00, // Type
        0x00, 0x00, // Length
        // Rate (Implicit trailing zeros)
        0xFF, 0xFF, // Leftovers
      ],
      RadiotapHeader {
        version: 0,
        fields: vec![RadiotapField::Rate(RateField { rate: 0x00 })],
      },
      12,
    ),
    // Single present bitmask, with single 4-byte aligned vendor TLV field, without leftovers
    (
      vec![
        0x00, // Version
        0x00, // Padding
        0x18, 0x00, // Length
        0x00, 0x00, 0x00, 0x10, // Present bitmask
        // TLV field
        // Vendor subfield A
        0x1E, 0x00, // Type
        0x09, 0x00, // Length
        0x02, 0x03, 0x04, // OUI
        0x05, // Sub-namespace
        0x2A, 0x00, // Vendor-specific presence type
        0x00, 0x00, // Padding
        0x99, // Value
        0x00, 0x00, 0x00, // Padding
      ],
      RadiotapHeader {
        version: 0,
        fields: vec![RadiotapField::Vendor(TestVendorFields::A(
          TestVendorFieldA { value: 0x99 },
        ))],
      },
      24,
    ),
    // Single present bitmask, with single 4-byte aligned vendor TLV field, with leftovers
    (
      vec![
        0x00, // Version
        0x00, // Padding
        0x18, 0x00, // Length
        0x00, 0x00, 0x00, 0x10, // Present bitmask
        // TLV field
        // Vendor subfield A
        0x1E, 0x00, // Type
        0x09, 0x00, // Length
        0x02, 0x03, 0x04, // OUI
        0x05, // Sub-namespace
        0x2A, 0x00, // Vendor-specific presence type
        0x00, 0x00, // Padding
        0x99, // Value
        0x00, 0x00, 0x00, // Padding
        0xFF, 0xFF, // Leftovers
      ],
      RadiotapHeader {
        version: 0,
        fields: vec![RadiotapField::Vendor(TestVendorFields::A(
          TestVendorFieldA { value: 0x99 },
        ))],
      },
      24,
    ),
    // Single present bitmask, with single 8-byte aligned vendor TLV field, without leftovers
    (
      vec![
        0x00, // Version
        0x00, // Padding
        0x24, 0x00, // Length
        0x00, 0x00, 0x00, 0x10, // Present bitmask
        // TLV field
        // Padding subfield
        0x1C, 0x00, // Type
        0x08, 0x00, // Length
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Padding bytes
        // Vendor subfield A
        0x1E, 0x00, // Type
        0x09, 0x00, // Length
        0x02, 0x03, 0x04, // OUI
        0x05, // Sub-namespace
        0x2A, 0x00, // Vendor-specific presence type
        0x00, 0x00, // Padding
        0x99, // Value
        0x00, 0x00, 0x00, // Padding
      ],
      RadiotapHeader {
        version: 0,
        fields: vec![RadiotapField::Vendor(TestVendorFields::A(
          TestVendorFieldA { value: 0x99 },
        ))],
      },
      36,
    ),
    // Single present bitmask, with single 8-byte aligned vendor TLV field, with leftovers
    (
      vec![
        0x00, // Version
        0x00, // Padding
        0x24, 0x00, // Length
        0x00, 0x00, 0x00, 0x10, // Present bitmask
        // TLV field
        // Padding subfield
        0x1C, 0x00, // Type
        0x08, 0x00, // Length
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Padding bytes
        // Vendor subfield A
        0x1E, 0x00, // Type
        0x09, 0x00, // Length
        0x02, 0x03, 0x04, // OUI
        0x05, // Sub-namespace
        0x2A, 0x00, // Vendor-specific presence type
        0x00, 0x00, // Padding
        0x99, // Value
        0x00, 0x00, 0x00, // Padding
      ],
      RadiotapHeader {
        version: 0,
        fields: vec![RadiotapField::Vendor(TestVendorFields::A(
          TestVendorFieldA { value: 0x99 },
        ))],
      },
      36,
    ),
    // Multiple present bitmasks, with single vendor namespace field, without leftovers
    (
      vec![
        0x00, // Version
        0x00, // Padding
        0x17, 0x00, // Length
        0x00, 0x00, 0x00, 0xC0, // Present bitmask 1
        0x00, 0x00, 0x00, 0x80, // Present bitmask 2
        0x00, 0x04, 0x00, 0x00, // Present bitmask 3
        // Vendor namespace
        0x02, 0x03, 0x04, // OUI
        0x05, // Sub-namespace
        0x01, 0x00, // Skip length
        0x99, // Value
      ],
      RadiotapHeader {
        version: 0,
        fields: vec![RadiotapField::Vendor(TestVendorFields::A(
          TestVendorFieldA { value: 0x99 },
        ))],
      },
      23,
    ),
    // Multiple present bitmasks, with single vendor namespace field, with leftovers
    (
      vec![
        0x00, // Version
        0x00, // Padding
        0x17, 0x00, // Length
        0x00, 0x00, 0x00, 0xC0, // Present bitmask 1
        0x00, 0x00, 0x00, 0x80, // Present bitmask 2
        0x00, 0x04, 0x00, 0x00, // Present bitmask 3
        // Vendor namespace
        0x02, 0x03, 0x04, // OUI
        0x05, // Sub-namespace
        0x01, 0x00, // Skip length
        0x99, // Value
        0xFF, 0xFF, // Leftovers
      ],
      RadiotapHeader {
        version: 0,
        fields: vec![RadiotapField::Vendor(TestVendorFields::A(
          TestVendorFieldA { value: 0x99 },
        ))],
      },
      23,
    ),
    // Single present bitmask, with multiple TLV fields, without leftovers
    (
      vec![
        0x00, // Version
        0x00, // Padding
        0x20, 0x00, // Length
        0x00, 0x00, 0x00, 0x10, // Present bitmask
        // TLV field
        // Rate subfield
        0x02, 0x00, // Type
        0x01, 0x00, // Length
        0x42, // Value
        0x00, 0x00, 0x00, // Padding
        // Vendor subfield A
        0x1E, 0x00, // Type
        0x09, 0x00, // Length
        0x02, 0x03, 0x04, // OUI
        0x05, // Sub-namespace
        0x2A, 0x00, // Vendor-specific presence type
        0x00, 0x00, // Padding
        0x99, // Value
        0x00, 0x00, 0x00, // Padding
      ],
      RadiotapHeader {
        version: 0,
        fields: vec![
          RadiotapField::Rate(RateField { rate: 0x42 }),
          RadiotapField::Vendor(TestVendorFields::A(TestVendorFieldA { value: 0x99 })),
        ],
      },
      32,
    ),
    // Single present bitmask, with multiple TLV fields, with leftovers
    (
      vec![
        0x00, // Version
        0x00, // Padding
        0x20, 0x00, // Length
        0x00, 0x00, 0x00, 0x10, // Present bitmask
        // TLV field
        // Rate subfield
        0x02, 0x00, // Type
        0x01, 0x00, // Length
        0x42, // Rate
        0x00, 0x00, 0x00, // Padding
        // Vendor subfield A
        0x1E, 0x00, // Type
        0x09, 0x00, // Length
        0x02, 0x03, 0x04, // OUI
        0x05, // Sub-namespace
        0x2A, 0x00, // Vendor-specific presence type
        0x00, 0x00, // Padding
        0x99, // Value
        0x00, 0x00, 0x00, // Padding
        0xFF, 0xFF, // Leftovers
      ],
      RadiotapHeader {
        version: 0,
        fields: vec![
          RadiotapField::Rate(RateField { rate: 0x42 }),
          RadiotapField::Vendor(TestVendorFields::A(TestVendorFieldA { value: 0x99 })),
        ],
      },
      32,
    ),
    // Single present bitmask, with repeated TLV fields, without leftovers
    (
      vec![
        0x00, // Version
        0x00, // Padding
        0x18, 0x00, // Length
        0x00, 0x00, 0x00, 0x10, // Present bitmask
        // TLV field
        // Rate subfield 1
        0x02, 0x00, // Type
        0x01, 0x00, // Length
        0x42, // Rate
        0x00, 0x00, 0x00, // Padding
        // Rate subfield 2
        0x02, 0x00, // Type
        0x01, 0x00, // Length
        0x13, // Rate
        0x00, 0x00, 0x00, // Padding
      ],
      RadiotapHeader {
        version: 0,
        fields: vec![
          RadiotapField::Rate(RateField { rate: 0x42 }),
          RadiotapField::Rate(RateField { rate: 0x13 }),
        ],
      },
      24,
    ),
    // Single present bitmask, with repeated TLV fields, with leftovers
    (
      vec![
        0x00, // Version
        0x00, // Padding
        0x18, 0x00, // Length
        0x00, 0x00, 0x00, 0x10, // Present bitmask
        // TLV field
        // Rate subfield 1
        0x02, 0x00, // Type
        0x01, 0x00, // Length
        0x42, // Rate
        0x00, 0x00, 0x00, // Padding
        // Rate subfield 2
        0x02, 0x00, // Type
        0x01, 0x00, // Length
        0x13, // Rate
        0x00, 0x00, 0x00, // Padding
        0xFF, 0xFF, // Leftovers
      ],
      RadiotapHeader {
        version: 0,
        fields: vec![
          RadiotapField::Rate(RateField { rate: 0x42 }),
          RadiotapField::Rate(RateField { rate: 0x13 }),
        ],
      },
      24,
    ),
    // Single present bitmask, with mixed fields, without leftovers
    (
      vec![
        0x00, // Version
        0x00, // Padding
        0x24, 0x00, // Length
        0x04, 0x00, 0x00, 0x10, // Present bitmask
        // Rate field
        0x13, // Rate
        // TLV field
        0x00, 0x00, 0x00, // Padding
        // Antenna signal subfield
        0x05, 0x00, // Type
        0x01, 0x00, // Length
        0x42, // Signal
        // Vendor subfield A
        0x00, 0x00, 0x00, // Padding
        0x1E, 0x00, // Type
        0x09, 0x00, // Length
        0x02, 0x03, 0x04, // OUI
        0x05, // Sub-namespace
        0x2A, 0x00, // Vendor-specific presence type
        0x00, 0x00, // Padding
        0x99, // Value
        0x00, 0x00, 0x00, // Padding
      ],
      RadiotapHeader {
        version: 0,
        fields: vec![
          RadiotapField::Rate(RateField { rate: 0x13 }),
          RadiotapField::DbmAntSignal(DbmAntSignalField { signal: 0x42 }),
          RadiotapField::Vendor(TestVendorFields::A(TestVendorFieldA { value: 0x99 })),
        ],
      },
      36,
    ),
    // Single present bitmask, with mixed fields, with leftovers
    (
      vec![
        0x00, // Version
        0x00, // Padding
        0x24, 0x00, // Length
        0x04, 0x00, 0x00, 0x10, // Present bitmask
        // Rate field
        0x13, // Rate
        // TLV field
        0x00, 0x00, 0x00, // Padding
        // Antenna signal subfield
        0x05, 0x00, // Type
        0x01, 0x00, // Length
        0x42, // Signal
        0x00, 0x00, 0x00, // Padding
        // Vendor subfield A
        0x1E, 0x00, // Type
        0x09, 0x00, // Length
        0x02, 0x03, 0x04, // OUI
        0x05, // Sub-namespace
        0x2A, 0x00, // Vendor-specific presence type
        0x00, 0x00, // Padding
        0x99, // Value
        0x00, 0x00, 0x00, // Padding
        0xFF, 0xFF, // Leftovers
      ],
      RadiotapHeader {
        version: 0,
        fields: vec![
          RadiotapField::Rate(RateField { rate: 0x13 }),
          RadiotapField::DbmAntSignal(DbmAntSignalField { signal: 0x42 }),
          RadiotapField::Vendor(TestVendorFields::A(TestVendorFieldA { value: 0x99 })),
        ],
      },
      36,
    ),
    // Multiple present bitmask, with mixed fields, without leftovers
    (
      vec![
        0x00, // Version
        0x00, // Padding
        0x38, 0x00, // Length
        0x04, 0x00, 0x00, 0xA0, // Present bitmask 1
        0x00, 0x00, 0x00, 0x10, // Present bitmask 2
        // Rate field
        0x13, // Rate
        // TLV field
        0x00, 0x00, 0x00, // Padding
        // Antenna signal subfield
        0x05, 0x00, // Type
        0x01, 0x00, // Length
        0x42, // Signal
        0x00, 0x00, 0x00, // Padding
        // Vendor subfield A
        0x1E, 0x00, // Type
        0x09, 0x00, // Length
        0x02, 0x03, 0x04, // OUI
        0x05, // Sub-namespace
        0x2A, 0x00, // Vendor-specific presence type
        0x00, 0x00, // Padding
        0x99, // Value
        0x00, 0x00, 0x00, // Padding
        // Vendor subfield B
        0x1E, 0x00, // Type
        0x09, 0x00, // Length
        0x02, 0x03, 0x04, // OUI
        0x05, // Sub-namespace
        0x2B, 0x00, // Vendor-specific presence type
        0x00, 0x00, // Padding
        0x77, // Value
        0x00, 0x00, 0x00, // Padding
      ],
      RadiotapHeader {
        version: 0,
        fields: vec![
          RadiotapField::Rate(RateField { rate: 0x13 }),
          RadiotapField::DbmAntSignal(DbmAntSignalField { signal: 0x42 }),
          RadiotapField::Vendor(TestVendorFields::A(TestVendorFieldA { value: 0x99 })),
          RadiotapField::Vendor(TestVendorFields::B(TestVendorFieldB { value: 0x77 })),
        ],
      },
      56,
    ),
    // Multiple present bitmask, with mixed fields, with leftovers
    (
      vec![
        0x00, // Version
        0x00, // Padding
        0x38, 0x00, // Length
        0x04, 0x00, 0x00, 0xA0, // Present bitmask 1
        0x00, 0x00, 0x00, 0x10, // Present bitmask 2
        // Rate field
        0x13, // Rate
        // TLV field
        0x00, 0x00, 0x00, // Padding
        // Antenna signal subfield
        0x05, 0x00, // Type
        0x01, 0x00, // Length
        0x42, // Signal
        0x00, 0x00, 0x00, // Padding
        // Vendor subfield A
        0x1E, 0x00, // Type
        0x09, 0x00, // Length
        0x02, 0x03, 0x04, // OUI
        0x05, // Sub-namespace
        0x2A, 0x00, // Vendor-specific presence type
        0x00, 0x00, // Padding
        0x99, // Value
        0x00, 0x00, 0x00, // Padding
        // Vendor subfield B
        0x1E, 0x00, // Type
        0x09, 0x00, // Length
        0x02, 0x03, 0x04, // OUI
        0x05, // Sub-namespace
        0x2B, 0x00, // Vendor-specific presence type
        0x00, 0x00, // Padding
        0x77, // Value
        0x00, 0x00, 0x00, // Padding
        0xFF, 0xFF, // Leftovers
      ],
      RadiotapHeader {
        version: 0,
        fields: vec![
          RadiotapField::Rate(RateField { rate: 0x13 }),
          RadiotapField::DbmAntSignal(DbmAntSignalField { signal: 0x42 }),
          RadiotapField::Vendor(TestVendorFields::A(TestVendorFieldA { value: 0x99 })),
          RadiotapField::Vendor(TestVendorFields::B(TestVendorFieldB { value: 0x77 })),
        ],
      },
      56,
    ),
  ]
}

#[rstest]
#[case::single_present_bitmask_no_fields_without_leftovers(
  &get_radiotap_header_from_bytes_cases()[0].0,
  &get_radiotap_header_from_bytes_cases()[0].1,
  get_radiotap_header_from_bytes_cases()[0].2
)]
#[case::single_present_bitmask_no_fields_with_leftovers(
  &get_radiotap_header_from_bytes_cases()[1].0,
  &get_radiotap_header_from_bytes_cases()[1].1,
  get_radiotap_header_from_bytes_cases()[1].2
)]
#[case::multiple_present_bitmasks_no_fields_without_leftovers(
  &get_radiotap_header_from_bytes_cases()[2].0,
  &get_radiotap_header_from_bytes_cases()[2].1,
  get_radiotap_header_from_bytes_cases()[2].2
)]
#[case::multiple_present_bitmasks_no_fields_with_leftovers(
  &get_radiotap_header_from_bytes_cases()[3].0,
  &get_radiotap_header_from_bytes_cases()[3].1,
  get_radiotap_header_from_bytes_cases()[3].2
)]
#[case::single_present_bitmask_with_single_implicit_length_field_without_leftovers(
  &get_radiotap_header_from_bytes_cases()[4].0,
  &get_radiotap_header_from_bytes_cases()[4].1,
  get_radiotap_header_from_bytes_cases()[4].2
)]
#[case::single_present_bitmask_with_single_implicit_length_field_with_leftovers(
  &get_radiotap_header_from_bytes_cases()[5].0,
  &get_radiotap_header_from_bytes_cases()[5].1,
  get_radiotap_header_from_bytes_cases()[5].2
)]
#[case::single_present_bitmask_with_multiple_aligned_implicit_length_fields_without_leftovers(
  &get_radiotap_header_from_bytes_cases()[6].0,
  &get_radiotap_header_from_bytes_cases()[6].1,
  get_radiotap_header_from_bytes_cases()[6].2
)]
#[case::single_present_bitmask_with_multiple_aligned_implicit_length_fields_with_leftovers(
  &get_radiotap_header_from_bytes_cases()[7].0,
  &get_radiotap_header_from_bytes_cases()[7].1,
  get_radiotap_header_from_bytes_cases()[7].2
)]
#[case::single_present_bitmask_with_multiple_unaligned_implicit_length_fields_without_leftovers(
  &get_radiotap_header_from_bytes_cases()[8].0,
  &get_radiotap_header_from_bytes_cases()[8].1,
  get_radiotap_header_from_bytes_cases()[8].2
)]
#[case::single_present_bitmask_with_multiple_unaligned_implicit_length_fields_with_leftovers(
  &get_radiotap_header_from_bytes_cases()[9].0,
  &get_radiotap_header_from_bytes_cases()[9].1,
  get_radiotap_header_from_bytes_cases()[9].2
)]
#[case::multiple_present_bitmasks_with_multiple_aligned_implicit_length_fields_without_leftovers(
  &get_radiotap_header_from_bytes_cases()[10].0,
  &get_radiotap_header_from_bytes_cases()[10].1,
  get_radiotap_header_from_bytes_cases()[10].2
)]
#[case::multiple_present_bitmasks_with_multiple_aligned_implicit_length_fields_with_leftovers(
  &get_radiotap_header_from_bytes_cases()[11].0,
  &get_radiotap_header_from_bytes_cases()[11].1,
  get_radiotap_header_from_bytes_cases()[11].2
)]
#[case::multiple_present_bitmasks_with_repeated_implicit_length_fields_without_leftovers(
  &get_radiotap_header_from_bytes_cases()[12].0,
  &get_radiotap_header_from_bytes_cases()[12].1,
  get_radiotap_header_from_bytes_cases()[12].2
)]
#[case::multiple_present_bitmasks_with_repeated_implicit_length_fields_with_leftovers(
  &get_radiotap_header_from_bytes_cases()[13].0,
  &get_radiotap_header_from_bytes_cases()[13].1,
  get_radiotap_header_from_bytes_cases()[13].2
)]
#[case::single_present_bitmask_with_single_radiotap_tlv_field_without_leftovers(
  &get_radiotap_header_from_bytes_cases()[14].0,
  &get_radiotap_header_from_bytes_cases()[14].1,
  get_radiotap_header_from_bytes_cases()[14].2
)]
#[case::single_present_bitmask_with_single_radiotap_tlv_field_with_leftovers(
  &get_radiotap_header_from_bytes_cases()[15].0,
  &get_radiotap_header_from_bytes_cases()[15].1,
  get_radiotap_header_from_bytes_cases()[15].2
)]
#[case::single_present_bitmask_with_single_radiotap_tlv_field_with_implicit_trailing_zeros_without_leftovers(
  &get_radiotap_header_from_bytes_cases()[16].0,
  &get_radiotap_header_from_bytes_cases()[16].1,
  get_radiotap_header_from_bytes_cases()[16].2
)]
#[case::single_present_bitmask_with_single_radiotap_tlv_field_with_implicit_trailing_zeros_with_leftovers(
  &get_radiotap_header_from_bytes_cases()[17].0,
  &get_radiotap_header_from_bytes_cases()[17].1,
  get_radiotap_header_from_bytes_cases()[17].2
)]
#[case::single_present_bitmask_with_single_4_byte_aligned_vendor_tlv_field_without_leftovers(
  &get_radiotap_header_from_bytes_cases()[18].0,
  &get_radiotap_header_from_bytes_cases()[18].1,
  get_radiotap_header_from_bytes_cases()[18].2
)]
#[case::single_present_bitmask_with_single_4_byte_aligned_vendor_tlv_field_with_leftovers(
  &get_radiotap_header_from_bytes_cases()[19].0,
  &get_radiotap_header_from_bytes_cases()[19].1,
  get_radiotap_header_from_bytes_cases()[19].2
)]
#[case::single_present_bitmask_with_single_8_byte_aligned_vendor_tlv_field_without_leftovers(
  &get_radiotap_header_from_bytes_cases()[20].0,
  &get_radiotap_header_from_bytes_cases()[20].1,
  get_radiotap_header_from_bytes_cases()[20].2
)]
#[case::single_present_bitmask_with_single_8_byte_aligned_vendor_tlv_field_with_leftovers(
  &get_radiotap_header_from_bytes_cases()[21].0,
  &get_radiotap_header_from_bytes_cases()[21].1,
  get_radiotap_header_from_bytes_cases()[21].2
)]
#[case::multiple_present_bitmasks_with_single_vendor_namespace_field_without_leftovers(
  &get_radiotap_header_from_bytes_cases()[22].0,
  &get_radiotap_header_from_bytes_cases()[22].1,
  get_radiotap_header_from_bytes_cases()[22].2
)]
#[case::multiple_present_bitmasks_with_single_vendor_namespace_field_with_leftovers(
  &get_radiotap_header_from_bytes_cases()[23].0,
  &get_radiotap_header_from_bytes_cases()[23].1,
  get_radiotap_header_from_bytes_cases()[23].2
)]
#[case::single_present_bitmask_with_multiple_tlv_fields_without_leftovers(
  &get_radiotap_header_from_bytes_cases()[24].0,
  &get_radiotap_header_from_bytes_cases()[24].1,
  get_radiotap_header_from_bytes_cases()[24].2
)]
#[case::single_present_bitmask_with_multiple_tlv_fields_with_leftovers(
  &get_radiotap_header_from_bytes_cases()[25].0,
  &get_radiotap_header_from_bytes_cases()[25].1,
  get_radiotap_header_from_bytes_cases()[25].2
)]
#[case::single_present_bitmask_with_repeated_tlv_fields_without_leftovers(
  &get_radiotap_header_from_bytes_cases()[26].0,
  &get_radiotap_header_from_bytes_cases()[26].1,
  get_radiotap_header_from_bytes_cases()[26].2
)]
#[case::single_present_bitmask_with_repeated_tlv_fields_with_leftovers(
  &get_radiotap_header_from_bytes_cases()[27].0,
  &get_radiotap_header_from_bytes_cases()[27].1,
  get_radiotap_header_from_bytes_cases()[27].2
)]
#[case::single_present_bitmask_with_mixed_fields_without_leftovers(
  &get_radiotap_header_from_bytes_cases()[28].0,
  &get_radiotap_header_from_bytes_cases()[28].1,
  get_radiotap_header_from_bytes_cases()[28].2
)]
#[case::single_present_bitmask_with_mixed_fields_with_leftovers(
  &get_radiotap_header_from_bytes_cases()[29].0,
  &get_radiotap_header_from_bytes_cases()[29].1,
  get_radiotap_header_from_bytes_cases()[29].2
)]
#[case::multiple_present_bitmasks_with_mixed_fields_without_leftovers(
  &get_radiotap_header_from_bytes_cases()[30].0,
  &get_radiotap_header_from_bytes_cases()[30].1,
  get_radiotap_header_from_bytes_cases()[30].2
)]
#[case::multiple_present_bitmasks_with_mixed_fields_with_leftovers(
  &get_radiotap_header_from_bytes_cases()[31].0,
  &get_radiotap_header_from_bytes_cases()[31].1,
  get_radiotap_header_from_bytes_cases()[31].2
)]
fn test_radiotap_header_from_bytes(
  #[case] bytes: &[u8],
  #[case] expected_header: &RadiotapHeader<TestVendorFields>,
  #[case] expected_size: usize,
) {
  // Build the decoding config
  let mut config = RadiotapHeaderDecodingConfig::standard_config().unwrap();
  config
    .set_vendor_field(Box::new(TestVendorFieldA::default()))
    .unwrap();
  config
    .set_vendor_field(Box::new(TestVendorFieldB::default()))
    .unwrap();

  // Decode the header
  let (decoded_header, header_size) = RadiotapHeader::from_bytes(bytes, &config, true).unwrap();

  // Check the result
  assert_eq!(header_size, expected_size);
  assert_eq!(decoded_header, *expected_header);
}

/// Get radiotap header to bytes test cases
fn get_radiotap_header_to_bytes_cases() -> Vec<(RadiotapHeader<TestVendorFields>, Vec<u8>)> {
  vec![
    // No fields
    (
      RadiotapHeader {
        version: 0,
        fields: vec![],
      },
      vec![
        0x00, // Version
        0x00, // Padding
        0x08, 0x00, // Length
        0x00, 0x00, 0x00, 0x00, // Present bitmask
      ],
    ),
    // Single implicit-length field
    (
      RadiotapHeader {
        version: 0,
        fields: vec![RadiotapField::Rate(RateField { rate: 0x42 })],
      },
      vec![
        0x00, // Version
        0x00, // Padding
        0x09, 0x00, // Length
        0x04, 0x00, 0x00, 0x00, // Present bitmask
        // Rate field
        0x42, // Value
      ],
    ),
    // Multiple aligned implicit-length fields
    (
      RadiotapHeader {
        version: 0,
        fields: vec![
          RadiotapField::Rate(RateField { rate: 0x42 }),
          RadiotapField::DbmAntSignal(DbmAntSignalField { signal: 0x13 }),
        ],
      },
      vec![
        0x00, // Version
        0x00, // Padding
        0x0A, 0x00, // Length
        0x24, 0x00, 0x00, 0x00, // Present bitmask
        // Rate field
        0x42, // Value
        // Antenna signal field
        0x13, // Value
      ],
    ),
    // Multiple unaligned implicit-length fields
    (
      RadiotapHeader {
        version: 0,
        fields: vec![
          RadiotapField::Mcs(McsField {
            known_bw: true,
            known_mcs: false,
            known_gi: false,
            known_fmt: false,
            known_fec: false,
            known_stbc: false,
            known_ness: false,
            known_ness_bit1: false,
            flags_bw: McsFlagsBw::Bw20L,
            flags_gi: false,
            flags_fmt: false,
            flags_fec: false,
            flags_stbc: 0,
            flags_ness_bit0: false,
            mcs: 3,
          }),
          RadiotapField::Lsig(LsigField {
            data1_rate_known: true,
            data1_length_known: false,
            data1_reserved0: 0,
            data2_rate: 0xE,
            data2_length: 0xABC,
          }),
        ],
      },
      vec![
        0x00, // Version
        0x00, // Padding
        0x10, 0x00, // Length
        0x00, 0x00, 0x08, 0x08, // Present bitmask
        // MCS field
        0x01, // Known
        0x02, // Flags
        0x03, // MCS
        // L-SIG field
        0x00, // Padding
        0x01, 0x00, // Data 1
        0xCE, 0xAB, // Data 2
      ],
    ),
    // Repeated implicit-length fields
    (
      RadiotapHeader {
        version: 0,
        fields: vec![
          RadiotapField::Rate(RateField { rate: 0x42 }),
          RadiotapField::Rate(RateField { rate: 0x13 }),
        ],
      },
      vec![
        0x00, // Version
        0x00, // Padding
        0x0E, 0x00, // Length
        0x04, 0x00, 0x00, 0xA0, // Present bitmask 1
        0x04, 0x00, 0x00, 0x00, // Present bitmask 2
        // Rate field 1
        0x42, // Value
        // Rate field 2
        0x13, // Value
      ],
    ),
    // Single vendor TLV field
    (
      RadiotapHeader {
        version: 0,
        fields: vec![RadiotapField::Vendor(TestVendorFields::A(
          TestVendorFieldA { value: 0x99 },
        ))],
      },
      vec![
        0x00, // Version
        0x00, // Padding
        0x18, 0x00, // Length
        0x00, 0x00, 0x00, 0x10, // Present bitmask
        // TLV field
        // Vendor subfield A
        0x1E, 0x00, // Type
        0x09, 0x00, // Length
        0x02, 0x03, 0x04, // OUI
        0x05, // Sub-namespace
        0x2A, 0x00, // Vendor-specific presence type
        0x00, 0x00, // Padding
        0x99, // Value
        0x00, 0x00, 0x00, // Padding
      ],
    ),
    // Multiple vendor TLV fields
    (
      RadiotapHeader {
        version: 0,
        fields: vec![
          RadiotapField::Vendor(TestVendorFields::A(TestVendorFieldA { value: 0x99 })),
          RadiotapField::Vendor(TestVendorFields::B(TestVendorFieldB { value: 0x77 })),
        ],
      },
      vec![
        0x00, // Version
        0x00, // Padding
        0x28, 0x00, // Length
        0x00, 0x00, 0x00, 0x10, // Present bitmask
        // TLV field
        // Vendor subfield A
        0x1E, 0x00, // Type
        0x09, 0x00, // Length
        0x02, 0x03, 0x04, // OUI
        0x05, // Sub-namespace
        0x2A, 0x00, // Vendor-specific presence type
        0x00, 0x00, // Padding
        0x99, // Value
        0x00, 0x00, 0x00, // Padding
        // Vendor subfield B
        0x1E, 0x00, // Type
        0x09, 0x00, // Length
        0x02, 0x03, 0x04, // OUI
        0x05, // Sub-namespace
        0x2B, 0x00, // Vendor-specific presence type
        0x00, 0x00, // Padding
        0x77, // Value
        0x00, 0x00, 0x00, // Padding
      ],
    ),
    // Mixed fields
    (
      RadiotapHeader {
        version: 0,
        fields: vec![
          RadiotapField::Rate(RateField { rate: 0x13 }),
          RadiotapField::DbmAntSignal(DbmAntSignalField { signal: 0x42 }),
          RadiotapField::Vendor(TestVendorFields::A(TestVendorFieldA { value: 0x99 })),
          RadiotapField::Vendor(TestVendorFields::B(TestVendorFieldB { value: 0x77 })),
        ],
      },
      vec![
        0x00, // Version
        0x00, // Padding
        0x2C, 0x00, // Length
        0x24, 0x00, 0x00, 0x10, // Present bitmask
        // Rate field
        0x13, // Rate
        // Antenna signal field
        0x42, // Signal
        // TLV field
        0x00, 0x00, // Padding
        // Vendor subfield A
        0x1E, 0x00, // Type
        0x09, 0x00, // Length
        0x02, 0x03, 0x04, // OUI
        0x05, // Sub-namespace
        0x2A, 0x00, // Vendor-specific presence type
        0x00, 0x00, // Padding
        0x99, // Value
        0x00, 0x00, 0x00, // Padding
        // Vendor subfield B
        0x1E, 0x00, // Type
        0x09, 0x00, // Length
        0x02, 0x03, 0x04, // OUI
        0x05, // Sub-namespace
        0x2B, 0x00, // Vendor-specific presence type
        0x00, 0x00, // Padding
        0x77, // Value
        0x00, 0x00, 0x00, // Padding
      ],
    ),
  ]
}

#[rstest]
#[case::no_fields(
  &get_radiotap_header_to_bytes_cases()[0].0,
  &get_radiotap_header_to_bytes_cases()[0].1,
)]
#[case::single_implicit_length_field(
  &get_radiotap_header_to_bytes_cases()[1].0,
  &get_radiotap_header_to_bytes_cases()[1].1,
)]
#[case::multiple_aligned_implicit_length_fields(
  &get_radiotap_header_to_bytes_cases()[2].0,
  &get_radiotap_header_to_bytes_cases()[2].1,
)]
#[case::multiple_unaligned_implicit_length_fields(
  &get_radiotap_header_to_bytes_cases()[3].0,
  &get_radiotap_header_to_bytes_cases()[3].1,
)]
#[case::repeated_implicit_length_fields(
  &get_radiotap_header_to_bytes_cases()[4].0,
  &get_radiotap_header_to_bytes_cases()[4].1,
)]
#[case::single_vendor_tlv_field(
  &get_radiotap_header_to_bytes_cases()[5].0,
  &get_radiotap_header_to_bytes_cases()[5].1,
)]
#[case::multiple_vendor_tlv_fields(
  &get_radiotap_header_to_bytes_cases()[6].0,
  &get_radiotap_header_to_bytes_cases()[6].1,
)]
#[case::mixed_fields(
  &get_radiotap_header_to_bytes_cases()[7].0,
  &get_radiotap_header_to_bytes_cases()[7].1,
)]
fn test_radiotap_header_to_bytes(
  #[case] header: &RadiotapHeader<TestVendorFields>,
  #[case] expected_bytes: &Vec<u8>,
) {
  // Encode the header
  let encoded_bytes = header.to_bytes().unwrap();

  // Check the result
  assert_eq!(encoded_bytes, *expected_bytes);
}
