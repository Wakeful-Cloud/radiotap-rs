use deku::{DekuContainerRead, DekuRead, DekuWrite, DekuWriter, writer::Writer};

use crate::utils::{FieldReader, FieldWriter, RadiotapError, RadiotapField, RadiotapFieldTrait, RadiotapFieldTraitIdentifiers};

/// Very High Throughput (VHT) field
///
/// See https://www.radiotap.org/fields/VHT.html for more information.
#[derive(Clone, Debug, Default, DekuRead, DekuWrite, PartialEq)]
#[deku(endian = "little", bit_order = "lsb")]
pub struct VhtField {
  /// Known Space-Time Block Coding (STBC) known
  #[deku(bits = "1")]
  pub known_stbc: bool,

  /// Known Transmission Opportunity (TXOP) power save not allowed known
  #[deku(bits = "1")]
  pub known_txop_ps_na: bool,

  /// Known guard interval known
  #[deku(bits = "1")]
  pub known_gi: bool,

  /// Known short guard interval number of symbols (Nsym) disambiguation known
  #[deku(bits = "1")]
  pub known_sgi_nsym_dis: bool,

  /// Known Low-Density Parity-Check (LDPC) extra Orthogonal Frequency-Division Multiplexing (OFDM)
  /// symbol known
  #[deku(bits = "1")]
  pub known_ldpc_extra_ofdm_sym: bool,

  /// Known beamformed known/applicable
  ///
  /// This flag should be set to zero for Multi-User (UM) Physical Protocol Data Units (PPDUs)
  #[deku(bits = "1")]
  pub known_beamformed: bool,

  /// Known bandwidth known
  #[deku(bits = "1")]
  pub known_bandwidth: bool,

  /// Known group ID known
  #[deku(bits = "1")]
  pub known_group_id: bool,

  /// Known partial AID known/applicable
  ///
  /// This flag should be set to zero for Multi-User (UM) Physical Protocol Data Units (PPDUs)
  #[deku(bits = "1")]
  pub known_partial_aid: bool,

  /// Known reserved 0
  #[deku(bits = "7")]
  pub known_reserved_0: u8,

  /// Flags all spatial streams of all users are using Space-Time Block Coding (STBC)
  #[deku(bits = "1")]
  pub flags_stbc: bool,

  /// Flags stations may not doze during Transmission Opportunity (TXOP)
  #[deku(bits = "1")]
  pub flags_txop_ps_na: bool,

  /// Flags short guard interval
  #[deku(bits = "1")]
  pub flags_sgi: bool,

  /// Flags short guard interval number of symbols (Nsym) disambiguation used (Nsym mod 10 = 9)
  #[deku(bits = "1")]
  pub flags_sgi_nsym_dis: bool,

  /// Flags one or more users are using Low-Density Parity-Check (LDPC) with extra Orthogonal
  /// Frequency-Division Multiplexing (OFDM) symbol(s)
  #[deku(bits = "1")]
  pub flags_ldpc_extra_ofdm_sym: bool,

  /// Flags beamformed
  ///
  /// Valid for Single-User (SU) Physical Protocol Data Units (PPDUs) only
  #[deku(bits = "1")]
  pub flags_beamformed: bool,

  /// Flags reserved 0
  #[deku(bits = "2")]
  pub flags_reserved_0: u8,

  /// Bandwidth
  ///
  /// Note: for receive capture, the total bandwidth of the transmitter is not generally known, so
  /// the bandwidth will only be recorded as `20`, `40`, `80`, or `160`.
  ///
  /// Note: for transmit injection, the sub-band choice for wide channels may be constrained by the
  /// current primary sub-band channels. IEEE 802.11 prohibits transmissions on sub-bands other than
  /// the designated sub-bands. For example, if the device is currently operating on the 40MHz
  /// channel pair `{36, 40}` with `36` designated as the primary 20MHz channel, the transmitter
  /// will naturally send 20MHz transmissions on channel `36`, and may not be able to force a 20MHz
  /// transmission on channel `40`.
  pub bandwidth: VhtBandwidth,

  /// User Number of Spatial Streams (NSS) and Modulation and Coding Scheme (MCS) fields
  ///
  /// If this field is zero, the user is not present and the MCS and coding (in the `coding` field)
  /// associated with that user are not valid. If the `nss` field is non-zero, but the `mcs` is not
  /// known (for example due to receiving only data for a single user), the `mcs` shall be set to
  /// `15`. For Single-User (SU) Physical Protocol Data Units (PPDUs), only the first user will have
  /// a nonzero `nss` field.
  ///
  /// Note: the Number of Space-Time Streams (NSTS) for a user can be calculated from the NSS for
  /// that user and the Space-Time Block Coding (STBC) flag:
  ///
  /// STBC not in use: NSTS = NSS
  /// STBC in use: NSTS = 2*NSS
  pub mcs_ncss: [VhtUserMcsNssField; 4],

  /// Coding user 0 coding (0 = Binary Convolutional Code (BCC), 1 = Low-Density Parity-Check
  /// (LDPC))
  #[deku(bits = "1")]
  pub coding_ldpc_user0: bool,

  /// Coding user 1 coding (0 = Binary Convolutional Code (BCC), 1 = Low-Density Parity-Check
  /// (LDPC))
  #[deku(bits = "1")]
  pub coding_ldpc_user1: bool,

  /// Coding user 2 coding (0 = Binary Convolutional Code (BCC), 1 = Low-Density Parity-Check
  /// (LDPC))
  #[deku(bits = "1")]
  pub coding_ldpc_user2: bool,

  /// Coding user 3 coding (0 = Binary Convolutional Code (BCC), 1 = Low-Density Parity-Check
  /// (LDPC))
  #[deku(bits = "1")]
  pub coding_ldpc_user3: bool,

  /// Coding reserved 0
  #[deku(bits = "4")]
  pub coding_reserved_0: u8,

  /// Group ID
  ///
  /// Note: the group ID can be used to differentiate between Single-User Physical Protocol Data
  /// Units (PPDUs) (Group ID is `0` or `63`) and Multi-User (MU) PPDUs (Group ID is `1` through
  /// `62`).
  pub group_id: u8,

  /// Partial Association IDentifier (AID)
  ///
  /// Note: only applicable to Single-User Physical Protocol Data Units (PPDUs)
  pub partial_aid: u16,
}

impl<V> RadiotapFieldTrait<V> for VhtField {
  fn get_identifiers(&self) -> RadiotapFieldTraitIdentifiers {
    RadiotapFieldTraitIdentifiers { bit_index: 21 }
  }

  fn get_alignment(&self) -> usize {
    2
  }

  fn decode(&self, reader: &mut FieldReader) -> Result<RadiotapField<V>, RadiotapError> {
    // Decode the field
    let decoded = VhtField::from_reader((reader, 0))?.1;

    Ok(RadiotapField::Vht(decoded))
  }

  fn encode(&self, writer: &mut Writer<&mut FieldWriter>) -> Result<(), RadiotapError> {
    // Encode the field
    self.to_writer(writer, ())?;

    Ok(())
  }
}

/// VHT bandwidth
#[derive(Clone, Debug, Default, DekuRead, DekuWrite, PartialEq)]
#[deku(
  id_type = "u8",
  bits = "8",
  bit_order = "order",
  endian = "endian",
  ctx = "endian: deku::ctx::Endian, order: deku::ctx::Order"
)]
pub enum VhtBandwidth {
  /// 20 MHz channel
  #[default]
  #[deku(id = "0")]
  Bw20,

  /// 40 MHz channel
  #[deku(id = "1")]
  Bw40,

  /// 40 MHz channel with lower 20 MHz sidechannel
  #[deku(id = "2")]
  Bw40_20L,

  /// 40 MHz channel with upper 20 MHz sidechannel
  #[deku(id = "3")]
  Bw40_20U,

  /// 80 MHz channel
  #[deku(id = "4")]
  Bw80,

  /// 80 MHz channel with lower 40 MHz sidechannel
  #[deku(id = "5")]
  Bw80_40L,

  /// 80 MHz channel with upper 40 MHz sidechannel
  #[deku(id = "6")]
  Bw80_40U,

  /// 80 MHz channel with lower, lower 20 MHz sidechannel
  #[deku(id = "7")]
  Bw80_20LL,

  /// 80 MHz channel with lower, upper 20 MHz sidechannel
  #[deku(id = "8")]
  Bw80_20LU,

  /// 80 MHz channel with upper, lower 20 MHz sidechannel
  #[deku(id = "9")]
  Bw80_20UL,

  /// 80 MHz channel with upper, upper 20 MHz sidechannel
  #[deku(id = "10")]
  Bw80_20UU,

  /// 160 MHz channel
  #[deku(id = "11")]
  Bw160,

  /// 160 MHz channel with lower 80 MHz sidechannel
  #[deku(id = "12")]
  Bw160_80L,

  /// 160 MHz channel with upper 80 MHz sidechannel
  #[deku(id = "13")]
  Bw160_80U,

  /// 160 MHz channel with lower, lower 40 MHz sidechannel
  #[deku(id = "14")]
  Bw160_40LL,

  /// 160 MHz channel with lower, upper 40 MHz sidechannel
  #[deku(id = "15")]
  Bw160_40LU,

  /// 160 MHz channel with upper, lower 40 MHz sidechannel
  #[deku(id = "16")]
  Bw160_40UL,

  /// 160 MHz channel with upper, upper 40 MHz sidechannel
  #[deku(id = "17")]
  Bw160_40UU,

  /// 160 MHz channel with lower, lower, lower 20 MHz sidechannel
  #[deku(id = "18")]
  Bw160_20LLL,

  /// 160 MHz channel with lower, lower, upper 20 MHz sidechannel
  #[deku(id = "19")]
  Bw160_20LLU,

  /// 160 MHz channel with lower, upper, lower 20 MHz sidechannel
  #[deku(id = "20")]
  Bw160_20LUL,

  /// 160 MHz channel with lower, upper, upper 20 MHz sidechannel
  #[deku(id = "21")]
  Bw160_20LUU,

  /// 160 MHz channel with upper, lower, lower 20 MHz sidechannel
  #[deku(id = "22")]
  Bw160_20ULL,

  /// 160 MHz channel with upper, lower, upper 20 MHz sidechannel
  #[deku(id = "23")]
  Bw160_20ULU,

  /// 160 MHz channel with upper, upper, lower 20 MHz sidechannel
  #[deku(id = "24")]
  Bw160_20UUL,

  /// 160 MHz channel with upper, upper, upper 20 MHz sidechannel
  #[deku(id = "25")]
  Bw160_20UUU,
}

/// User Modulation and Coding Scheme (MCS) and Number of Spatial Streams (NSS) field
#[derive(Clone, Debug, Default, DekuRead, DekuWrite, PartialEq)]
#[deku(
  bit_order = "order",
  endian = "endian",
  ctx = "endian: deku::ctx::Endian, order: deku::ctx::Order"
)]
pub struct VhtUserMcsNssField {
  /// Number of spatial streams, range 1-8.
  #[deku(assert = "*nss <= 8", bits = "4")]
  pub nss: u8,

  /// MCS rate index, range 0-9.
  #[deku(assert = "*mcs <= 9 || *mcs == 15", bits = "4")]
  pub mcs: u8,
}

#[cfg(test)]
mod tests {
  use deku::{DekuContainerRead, DekuContainerWrite};
  use pretty_assertions::assert_eq;
  use rstest::rstest;

  use super::*;

  /// Encoding test cases
  const ENCODING_CASES: &[(&[u8], &VhtField)] = &[
    // No flags
    (
      &[
        0x00, 0x00, // Known
        0x00, // Flags
        0x09, // Bandwidth
        0x12, // MCS/NSS user 0
        0x34, // MCS/NSS user 1
        0x56, // MCS/NSS user 2
        0x78, // MCS/NSS user 3
        0x00, // Coding
        0x2A, // Group ID
        0x24, 0x00, // Partial AID
      ],
      &VhtField {
        known_stbc: false,
        known_txop_ps_na: false,
        known_gi: false,
        known_sgi_nsym_dis: false,
        known_ldpc_extra_ofdm_sym: false,
        known_beamformed: false,
        known_bandwidth: false,
        known_group_id: false,
        known_partial_aid: false,
        known_reserved_0: 0,
        flags_stbc: false,
        flags_txop_ps_na: false,
        flags_sgi: false,
        flags_sgi_nsym_dis: false,
        flags_ldpc_extra_ofdm_sym: false,
        flags_beamformed: false,
        flags_reserved_0: 0,
        bandwidth: VhtBandwidth::Bw80_20UL,
        mcs_ncss: [
          VhtUserMcsNssField { mcs: 1, nss: 2 },
          VhtUserMcsNssField { mcs: 3, nss: 4 },
          VhtUserMcsNssField { mcs: 5, nss: 6 },
          VhtUserMcsNssField { mcs: 7, nss: 8 },
        ],
        coding_ldpc_user0: false,
        coding_ldpc_user1: false,
        coding_ldpc_user2: false,
        coding_ldpc_user3: false,
        coding_reserved_0: 0,
        group_id: 42,
        partial_aid: 36,
      },
    ),
    // Alternating flags (A)
    (
      &[
        0x55, 0x01, // Known
        0x2A, // Flags
        0x09, // Bandwidth
        0x12, // MCS/NSS user 0
        0x34, // MCS/NSS user 1
        0x56, // MCS/NSS user 2
        0x78, // MCS/NSS user 3
        0x0A, // Coding
        0x2A, // Group ID
        0x24, 0x00, // Partial AID
      ],
      &VhtField {
        known_stbc: true,
        known_txop_ps_na: false,
        known_gi: true,
        known_sgi_nsym_dis: false,
        known_ldpc_extra_ofdm_sym: true,
        known_beamformed: false,
        known_bandwidth: true,
        known_group_id: false,
        known_partial_aid: true,
        known_reserved_0: 0,
        flags_stbc: false,
        flags_txop_ps_na: true,
        flags_sgi: false,
        flags_sgi_nsym_dis: true,
        flags_ldpc_extra_ofdm_sym: false,
        flags_beamformed: true,
        flags_reserved_0: 0,
        bandwidth: VhtBandwidth::Bw80_20UL,
        mcs_ncss: [
          VhtUserMcsNssField { mcs: 1, nss: 2 },
          VhtUserMcsNssField { mcs: 3, nss: 4 },
          VhtUserMcsNssField { mcs: 5, nss: 6 },
          VhtUserMcsNssField { mcs: 7, nss: 8 },
        ],
        coding_ldpc_user0: false,
        coding_ldpc_user1: true,
        coding_ldpc_user2: false,
        coding_ldpc_user3: true,
        coding_reserved_0: 0,
        group_id: 42,
        partial_aid: 36,
      },
    ),
    // Alternating flags (B)
    (
      &[
        0xAA, 0x00, // Known
        0x15, // Flags
        0x09, // Bandwidth
        0x12, // MCS/NSS user 0
        0x34, // MCS/NSS user 1
        0x56, // MCS/NSS user 2
        0x78, // MCS/NSS user 3
        0x05, // Coding
        0x2A, // Group ID
        0x24, 0x00, // Partial AID
      ],
      &VhtField {
        known_stbc: false,
        known_txop_ps_na: true,
        known_gi: false,
        known_sgi_nsym_dis: true,
        known_ldpc_extra_ofdm_sym: false,
        known_beamformed: true,
        known_bandwidth: false,
        known_group_id: true,
        known_partial_aid: false,
        known_reserved_0: 0,
        flags_stbc: true,
        flags_txop_ps_na: false,
        flags_sgi: true,
        flags_sgi_nsym_dis: false,
        flags_ldpc_extra_ofdm_sym: true,
        flags_beamformed: false,
        flags_reserved_0: 0,
        bandwidth: VhtBandwidth::Bw80_20UL,
        mcs_ncss: [
          VhtUserMcsNssField { mcs: 1, nss: 2 },
          VhtUserMcsNssField { mcs: 3, nss: 4 },
          VhtUserMcsNssField { mcs: 5, nss: 6 },
          VhtUserMcsNssField { mcs: 7, nss: 8 },
        ],
        coding_ldpc_user0: true,
        coding_ldpc_user1: false,
        coding_ldpc_user2: true,
        coding_ldpc_user3: false,
        coding_reserved_0: 0,
        group_id: 42,
        partial_aid: 36,
      },
    ),
  ];

  #[rstest]
  #[case::no_flags(ENCODING_CASES[0].0, ENCODING_CASES[0].1)]
  #[case::alternating_flags_a(ENCODING_CASES[1].0, ENCODING_CASES[1].1)]
  #[case::alternating_flags_b(ENCODING_CASES[2].0, ENCODING_CASES[2].1)]
  fn test_decode_field(#[case] encoded: &[u8], #[case] expected: &VhtField) {
    // Decode the bytes
    let decoded = VhtField::from_bytes((encoded, 0)).unwrap().1;

    // Check the result
    assert_eq!(decoded, *expected);
  }

  #[rstest]
  #[case::no_flags(ENCODING_CASES[0].1, ENCODING_CASES[0].0)]
  #[case::alternating_flags_a(ENCODING_CASES[1].1, ENCODING_CASES[1].0)]
  #[case::alternating_flags_b(ENCODING_CASES[2].1, ENCODING_CASES[2].0)]
  fn test_encode_field(#[case] field: &VhtField, #[case] expected: &[u8]) {
    // Encode the field
    let encoded = field.to_bytes().unwrap();

    // Check the result
    assert_eq!(encoded, expected);
  }
}
