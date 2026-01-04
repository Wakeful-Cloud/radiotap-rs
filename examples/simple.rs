use radiotap_rs::header::{RadiotapHeader, RadiotapHeaderDecodingConfig};

fn main() {
  // Copied from a real capture on Linux + Wireshark
  let bytes = &[
    0x00, // Version
    0x00, // Padding
    0x24, 0x00, // Length
    0x2F, 0x40, 0x00, 0xA0, // Present bitmask 1
    0x20, 0x08, 0x00, 0x00, // Present bitmask 2
    // TSFT field
    0x00, 0x00, 0x00, 0x00, // Padding
    0x01, 0x70, 0x30, 0x01, 0x00, 0x00, 0x00, 0x00, // MAC time
    // Flags field
    0x10, // Flags
    // Rate field
    0x02, // Rate (500 kbps increments)
    // Channel field
    0x9E, 0x09, // Frequency
    0xA0, 0x00, // Flags
    // Antenna signal field
    0xE7, // Signal
    // RX flags field
    0x00, // Padding
    0x00, 0x00, // Flags
    // Antenna signal field
    0xE7, // Signal
    // Antenna field
    0x00, // Antenna
  ];

  // Decode the header
  let config = RadiotapHeaderDecodingConfig::<()>::standard_config().unwrap();
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
