use deku::{DekuContainerRead, DekuRead, DekuWrite, DekuWriter, writer::Writer};
use radiotap_rs::{
  header::{RadiotapHeader, RadiotapHeaderDecodingConfig},
  utils::{FieldReader, FieldWriter, RadiotapError, VendorFieldTrait, VendorFieldTraitIdentifiers},
};

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

fn main() {
  // Made-up example
  let bytes = &[
    0x00, // Version
    0x00, // Padding
    0x2C, 0x00, // Length
    0x04, 0x00, 0x00, 0x10, // Present bitmask
    // Rate field
    0x13, // Rate
    // TLV field
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
  ];

  // Decode the header
  let mut config = RadiotapHeaderDecodingConfig::standard_config().unwrap();
  config
    .set_vendor_field(Box::new(TestVendorFieldA::default()))
    .unwrap();
  config
    .set_vendor_field(Box::new(TestVendorFieldB::default()))
    .unwrap();

  let (header, header_size) = RadiotapHeader::from_bytes(bytes, &config, false).unwrap();

  println!("Decoded header ({} bytes): {:#?}", header_size, header);

  // Re-encode the header
  let encoded_bytes = header.to_bytes().unwrap();

  println!(
    "Re-encoded header ({} bytes): {}",
    encoded_bytes.len(),
    encoded_bytes
      .iter()
      .map(|b| format!("{:02X}", b))
      .collect::<Vec<String>>()
      .join(" ")
  );

  // Note: in general, re-encoding may not produce the exact same byte sequence (see the README for details)
  assert_eq!(bytes, &encoded_bytes[..]);
}
