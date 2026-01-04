use alloc::{string::String, vec::Vec};
use deku::{
  no_std_io::{Read, Seek, Write},
  writer::Writer,
};
use thiserror::Error;

use crate::{
  field_0_tsft::TsftField, field_1_flags::FlagsField, field_2_rate::RateField,
  field_3_channel::ChannelField, field_4_fhss::FhssField, field_5_dbm_antsignal::DbmAntSignalField,
  field_6_dbm_antnoise::DbmAntNoiseField, field_7_lock_quality::LockQualityField,
  field_8_tx_attenuation::TxAttenuationField, field_9_db_tx_attenuation::DbTxAttenuationField,
  field_10_dbm_tx_power::DbmTxPowerField, field_11_antenna::AntennaField,
  field_12_db_antsignal::DbAntSignalField, field_13_db_antnoise::DbAntNoiseField,
  field_14_fcs_in_header::FcsInHeaderField, field_14_rx_flags::RxFlagsField,
  field_15_hardware_queue::HardwareQueueField, field_15_tx_flags::TxFlagsField,
  field_16_rssi::RssiField, field_16_rts_retries::RtsRetriesField,
  field_17_data_retries::DataRetriesField, field_18_xchannel::XChannelField,
  field_19_mcs::McsField, field_20_ampdu_status::AmpduStatusField, field_21_vht::VhtField,
  field_22_extended_flags::ExtendedFlagsField, field_22_timestamp::TimestampField,
  field_23_he::HeField, field_24_he_mu::HeMuField, field_25_he_mu_other_user::HeMuOtherUserField,
  field_26_psdu::PsduField, field_27_lsig::LsigField,
};

/// Radiotap field trait identifiers
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RadiotapFieldTraitIdentifiers {
  /// Bit index
  pub bit_index: u16,
}

/// Radiotap field trait
pub trait RadiotapFieldTrait<V> {
  /// Get the field's identifiers
  fn get_identifiers(&self) -> RadiotapFieldTraitIdentifiers;

  /// Get the field's alignment
  fn get_alignment(&self) -> usize;

  /// Decode another field
  fn decode(&self, reader: &mut FieldReader) -> Result<RadiotapField<V>, RadiotapError>;

  /// Encode the field
  fn encode(&self, writer: &mut Writer<&mut FieldWriter>) -> Result<(), RadiotapError>;
}

/// Vendor field trait identifiers
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct VendorFieldTraitIdentifiers {
  /// Organizationally Unique Identifier (OUI)
  pub oui: [u8; 3],

  /// Sub-namespace
  pub sub_namespace: u8,

  /// Bit index
  pub bit_index: u16,
}

/// Vendor field trait
pub trait VendorFieldTrait<V> {
  /// Get the field's identifiers
  fn get_identifiers(&self) -> VendorFieldTraitIdentifiers;

  /// Get the field's alignment
  fn get_alignment(&self) -> usize;

  /// Decode another field
  fn decode(&self, reader: &mut FieldReader) -> Result<V, RadiotapError>;

  /// Encode the field
  fn encode(&self, writer: &mut Writer<&mut FieldWriter>) -> Result<(), RadiotapError>;
}

impl VendorFieldTrait<()> for () {
  fn get_identifiers(&self) -> VendorFieldTraitIdentifiers {
    VendorFieldTraitIdentifiers {
      oui: [0, 0, 0],
      sub_namespace: 0,
      bit_index: 0,
    }
  }

  fn get_alignment(&self) -> usize {
    1
  }

  fn decode(&self, _: &mut FieldReader) -> Result<(), RadiotapError> {
    panic!("Cannot decode vendor field with unit type")
  }

  fn encode(&self, _: &mut Writer<&mut FieldWriter>) -> Result<(), RadiotapError> {
    panic!("Cannot encode vendor field with unit type")
  }
}

/// Radiotap error
#[derive(Clone, Debug, Error, PartialEq)]
pub enum RadiotapError {
  /// Invalid field bit index
  #[error("Invalid field bit index")]
  InvalidFieldBitIndex,

  /// Deku error
  #[error("Deku error: {error}")]
  DekuError {
    #[from]
    error: deku::DekuError,
  },

  /// Field type/value mismatch
  #[error("Field type/value mismatch")]
  FieldTypeValueMismatch,

  /// Unknown field
  #[error("Unknown field")]
  UnknownField,

  /// Unexpected header length
  #[error("Unexpected header length")]
  UnexpectedHeaderLength,

  /// Other error
  #[error("Other error: {message}")]
  Other { message: String },
}
/// Radiotap header field
#[derive(Clone, Debug, PartialEq)]
pub enum RadiotapField<V> {
  /// Time Synchronization Function Timer (TSFT) field
  ///
  /// Bit index: `0`
  Tsft(TsftField),

  /// Frame flags field
  ///
  /// Bit index: `1`
  Flags(FlagsField),

  /// Transmit/received data rate field
  ///
  /// Bit index: `2`
  Rate(RateField),

  /// Channel field
  ///
  /// Bit index: `3`
  Channel(ChannelField),

  /// Frequency Hopping Spread Spectrum (FHSS) field
  ///
  /// Bit index: `4`
  Fhss(FhssField),

  /// dBm antenna signal field
  ///
  /// Bit index: `5`
  DbmAntSignal(DbmAntSignalField),

  /// dBm antenna noise field
  ///
  /// Bit index: `6`
  DbmAntNoise(DbmAntNoiseField),

  /// Lock quality field
  ///
  /// Bit index: `7`
  LockQuality(LockQualityField),

  /// Transmit (TX) attenuation field
  ///
  /// Bit index: `8`
  TxAttenuation(TxAttenuationField),

  /// dB transmit (TX) attenuation field
  ///
  /// Bit index: `9`
  DbTxAttenuation(DbTxAttenuationField),

  /// dBm transmit (TX) power field
  ///
  /// Bit index: `10`
  DbmTxPower(DbmTxPowerField),

  /// Antenna field
  ///
  /// Bit index: `11`
  Antenna(AntennaField),

  /// dB antenna signal field
  ///
  /// Bit index: `12`
  DbAntSignal(DbAntSignalField),

  /// dB antenna noise field
  ///
  /// Bit index: `13`
  DbAntNoise(DbAntNoiseField),

  /// Frame Check Sequence (FCS) in header field
  ///
  /// Bit index: `14`
  FcsInHeader(FcsInHeaderField),

  /// Receive (RX) flags field
  ///
  /// Bit index: `14`
  RxFlags(RxFlagsField),

  /// Hardware queue field
  ///
  /// Bit index: `15`
  HardwareQueue(HardwareQueueField),

  /// Transmit (TX) flags field
  ///
  /// Bit index: `15`
  TxFlags(TxFlagsField),

  /// Received Signal Strength Indicator (RSSI) field
  ///
  /// Bit index: `16`
  Rssi(RssiField),

  /// Requests To Send (RTS) retries field
  ///
  /// Bit index: `16`
  RtsRetries(RtsRetriesField),

  /// Data retries field
  ///
  /// Bit index: `17`
  DataRetries(DataRetriesField),

  /// Extended channel (XChannel) information field
  ///
  /// Bit index: `18`
  XChannel(XChannelField),

  /// Modulation and Coding Scheme (MCS) field
  ///
  /// Bit index: `19`
  Mcs(McsField),

  /// Aggregated MAC Protocol Data Unit (A-MPDU) status field
  ///
  /// Bit index: `20`
  AmpduStatus(AmpduStatusField),

  /// Very High Throughput (VHT) field
  ///
  /// Bit index: `21`
  Vht(VhtField),

  /// Extended flags field
  ///
  /// Bit index: `22`
  ExtendedFlags(ExtendedFlagsField),

  /// Timestamp field
  ///
  /// Bit index: `22`
  Timestamp(TimestampField),

  /// High-Efficiency (HE) field
  ///
  /// Bit index: `23`
  He(HeField),

  /// High-Efficiency Multi-User (HE-MU) field
  ///
  /// Bit index: `24`
  HeMu(HeMuField),

  /// High-Efficiency Multi-User (HE-MU) other user field
  ///
  /// Bit index: `25`
  HeMuOtherUser(HeMuOtherUserField),

  /// 0-length Physical Service Data Unit (PSDU) Field
  ///
  /// Bit index: `26`
  Psdu(PsduField),

  /// Legacy Signal (L-SIG) field
  ///
  /// Bit index: `27`
  Lsig(LsigField),

  /// Vendor field(s)
  Vendor(V),
}

/// Radiotap field reader with optional support for implicit trailing zero bytes
pub struct FieldReader<'a> {
  /// Whether or not implicit trailing zeros are enabled
  implicit_trailing_zeros: bool,

  /// Underlying bytes
  bytes: &'a [u8],

  /// Reader position
  position: usize,
}

impl<'a> FieldReader<'a> {
  /// Create a new field reader
  pub fn new(bytes: &'a [u8], implicit_trailing_zeros: bool) -> Self {
    Self {
      bytes,
      implicit_trailing_zeros,
      position: 0,
    }
  }

  /// Get the current position of the reader
  pub fn position(&self) -> usize {
    self.position
  }
}

impl Read for FieldReader<'_> {
  fn read(&mut self, buf: &mut [u8]) -> deku::no_std_io::Result<usize> {
    let mut bytes_read = 0;

    for byte in buf.iter_mut() {
      if self.position < self.bytes.len() {
        *byte = self.bytes[self.position];
      } else if self.implicit_trailing_zeros {
        *byte = 0;
      } else {
        break;
      }

      self.position += 1;
      bytes_read += 1;
    }

    Ok(bytes_read)
  }
}

impl Seek for FieldReader<'_> {
  fn seek(&mut self, pos: deku::no_std_io::SeekFrom) -> deku::no_std_io::Result<u64> {
    let new_position = match pos {
      deku::no_std_io::SeekFrom::Start(offset) => offset as isize,
      deku::no_std_io::SeekFrom::End(offset) => self.bytes.len() as isize + offset as isize,
      deku::no_std_io::SeekFrom::Current(offset) => self.position as isize + offset as isize,
    };

    if new_position < 0 {
      return Err(deku::no_std_io::Error::new(
        deku::no_std_io::ErrorKind::InvalidInput,
        "Invalid seek to a negative position",
      ));
    }

    self.position = new_position as usize;
    Ok(self.position as u64)
  }
}

/// Radiotap field writer
pub struct FieldWriter<'a> {
  /// Underlying bytes
  bytes: &'a mut Vec<u8>,

  /// Writer position
  position: usize,
}

impl<'a> FieldWriter<'a> {
  /// Create a new field writer
  pub fn new(bytes: &'a mut Vec<u8>) -> Self {
    Self { bytes, position: 0 }
  }
}

impl Write for FieldWriter<'_> {
  fn write(&mut self, buf: &[u8]) -> deku::no_std_io::Result<usize> {
    for &byte in buf {
      if self.position < self.bytes.len() {
        self.bytes[self.position] = byte;
      } else {
        self.bytes.push(byte);
      }

      self.position += 1;
    }

    Ok(buf.len())
  }

  fn flush(&mut self) -> deku::no_std_io::Result<()> {
    Ok(())
  }
}

impl Seek for FieldWriter<'_> {
  fn seek(&mut self, pos: deku::no_std_io::SeekFrom) -> deku::no_std_io::Result<u64> {
    let new_position = match pos {
      deku::no_std_io::SeekFrom::Start(offset) => offset as isize,
      deku::no_std_io::SeekFrom::End(offset) => self.bytes.len() as isize + offset as isize,
      deku::no_std_io::SeekFrom::Current(offset) => self.position as isize + offset as isize,
    };

    if new_position < 0 {
      return Err(deku::no_std_io::Error::new(
        deku::no_std_io::ErrorKind::InvalidInput,
        "Invalid seek to a negative position",
      ));
    }

    self.position = new_position as usize;
    Ok(self.position as u64)
  }
}

#[cfg(test)]
mod tests {
  use deku::no_std_io::Seek;
  use pretty_assertions::assert_eq;
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case(
    vec![],
    false,
    deku::no_std_io::SeekFrom::Start(0),
    0,
    0,
    vec![],
  )]
  #[case(
    vec![0x01, 0x02, 0x03],
    false,
    deku::no_std_io::SeekFrom::Start(0),
    3,
    3,
    vec![0x01, 0x02, 0x03],
  )]
  #[case(
    vec![],
    false,
    deku::no_std_io::SeekFrom::Start(0),
    5,
    0,
    vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF],
  )]
  #[case(
    vec![0x01, 0x02, 0x03],
    false,
    deku::no_std_io::SeekFrom::Start(0),
    5,
    3,
    vec![0x01, 0x02, 0x03, 0xFF, 0xFF],
  )]
  #[case(
    vec![0x01, 0x02, 0x03],
    false,
    deku::no_std_io::SeekFrom::Start(1),
    5,
    2,
    vec![0x02, 0x03, 0xFF, 0xFF, 0xFF],
  )]
  #[case(
    vec![0x01, 0x02, 0x03],
    false,
    deku::no_std_io::SeekFrom::Current(1),
    5,
    2,
    vec![0x02, 0x03, 0xFF, 0xFF, 0xFF],
  )]
  #[case(
    vec![0x01, 0x02, 0x03],
    false,
    deku::no_std_io::SeekFrom::End(-1),
    5,
    1,
    vec![0x03, 0xFF, 0xFF, 0xFF, 0xFF],
  )]
  #[case(
    vec![],
    true,
    deku::no_std_io::SeekFrom::Start(0),
    0,
    0,
    vec![],
  )]
  #[case(
    vec![0x01, 0x02, 0x03],
    true,
    deku::no_std_io::SeekFrom::Start(0),
    3,
    3,
    vec![0x01, 0x02, 0x03],
  )]
  #[case(
    vec![],
    true,
    deku::no_std_io::SeekFrom::Start(0),
    5,
    5,
    vec![0x00, 0x00, 0x00, 0x00, 0x00],
  )]
  #[case(
    vec![0x01, 0x02, 0x03],
    true,
    deku::no_std_io::SeekFrom::Start(0),
    5,
    5,
    vec![0x01, 0x02, 0x03, 0x00, 0x00],
  )]
  #[case(
    vec![0x01, 0x02, 0x03],
    true,
    deku::no_std_io::SeekFrom::Start(1),
    5,
    5,
    vec![0x02, 0x03, 0x00, 0x00, 0x00],
  )]
  #[case(
    vec![0x01, 0x02, 0x03],
    true,
    deku::no_std_io::SeekFrom::Current(1),
    5,
    5,
    vec![0x02, 0x03, 0x00, 0x00, 0x00],
  )]
  #[case(
    vec![0x01, 0x02, 0x03],
    true,
    deku::no_std_io::SeekFrom::End(-1),
    5,
    5,
    vec![0x03, 0x00, 0x00, 0x00, 0x00],
  )]
  fn test_field_reader(
    #[case] input: Vec<u8>,
    #[case] implicit_trailing_zeros: bool,
    #[case] seek: deku::no_std_io::SeekFrom,
    #[case] read_size: usize,
    #[case] expected_bytes_read: usize,
    #[case] expected_output: Vec<u8>,
  ) {
    // Initialize the reader
    let mut reader = FieldReader::new(&input, implicit_trailing_zeros);
    reader.seek(seek).unwrap();

    // Read from the reader
    let mut buffer = vec![0xFF; read_size];
    let bytes_read = reader.read(&mut buffer).unwrap();

    // Check the result
    assert_eq!(bytes_read, expected_bytes_read);
    assert_eq!(buffer, expected_output);
  }

  #[rstest]
  #[case(
    vec![],
    deku::no_std_io::SeekFrom::Start(0),
    vec![],
    vec![],
  )]
  #[case(
    vec![],
    deku::no_std_io::SeekFrom::Start(0),
    vec![0x01, 0x02, 0x03],
    vec![0x01, 0x02, 0x03],
  )]
  #[case(
    vec![0x01, 0x02, 0x03],
    deku::no_std_io::SeekFrom::Start(0),
    vec![0x04, 0x05, 0x06],
    vec![0x04, 0x05, 0x06],
  )]
  #[case(
    vec![0x01, 0x02, 0x03],
    deku::no_std_io::SeekFrom::Start(1),
    vec![0x04, 0x05, 0x06],
    vec![0x01, 0x04, 0x05, 0x06],
  )]
  #[case(
    vec![0x01, 0x02, 0x03],
    deku::no_std_io::SeekFrom::Current(1),
    vec![0x04, 0x05, 0x06],
    vec![0x01, 0x04, 0x05, 0x06],
  )]
  #[case(
    vec![0x01, 0x02, 0x03],
    deku::no_std_io::SeekFrom::End(-1),
    vec![0x04, 0x05, 0x06],
    vec![0x01, 0x02, 0x04, 0x05, 0x06],
  )]
  fn test_field_writer(
    #[case] initial: Vec<u8>,
    #[case] seek: deku::no_std_io::SeekFrom,
    #[case] input: Vec<u8>,
    #[case] expected_output: Vec<u8>,
  ) {
    // Initialize the writer
    let mut bytes = initial;
    let mut writer = FieldWriter::new(&mut bytes);
    writer.seek(seek).unwrap();

    // Write to the writer
    writer.write(&input).unwrap();

    // Check the result
    assert_eq!(bytes, expected_output);
  }
}
