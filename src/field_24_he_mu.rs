use deku::{DekuContainerRead, DekuRead, DekuWrite, DekuWriter, writer::Writer};

use crate::utils::{FieldReader, FieldWriter, RadiotapError, RadiotapField, RadiotapFieldTrait, RadiotapFieldTraitIdentifiers};

/// High-Efficiency Multi-User (HE-MU) field
///
/// See https://www.radiotap.org/fields/HE-MU.html for more information.
#[derive(Clone, Debug, Default, DekuRead, DekuWrite, PartialEq)]
#[deku(endian = "little", bit_order = "lsb")]
pub struct HeMuField {
  /// Flags 1 Signal field B (SIG-B) Modulation and Coding Scheme (MCS)
  #[deku(bits = "4")]
  pub flags1_sig_b_mcs: u8,

  /// Flags 1 Signal field B (SIG-B) Modulation and Coding Scheme (MCS) Known
  #[deku(bits = "1")]
  pub flags1_sig_b_mcs_known: bool,

  /// Flags 1 Dual Carrier Modulation (DCM)
  #[deku(bits = "1")]
  pub flags1_dcm: bool,

  /// Flags 1 Dual Carrier Modulation (DCM) Known
  #[deku(bits = "1")]
  pub flags1_dcm_known: bool,

  /// Flags 1 channel 2 center 26-tone Resource Unit (RU) bit known
  #[deku(bits = "1")]
  pub flags1_ch2_ctr_26t_ru_known: bool,

  /// Flags 1 channel 1 Resource Units (RUs) known
  #[deku(bits = "1")]
  pub flags1_ch1_ru_known: bool,

  /// Flags 1 channel 2 Resource Units (RUs) known
  #[deku(bits = "1")]
  pub flags1_ch2_ru_known: bool,

  /// Flags 1 reserved 0
  #[deku(bits = "2")]
  pub flags1_reserved0: u8,

  /// Flags 1 channel 1 center 26-tone Resource Unit (RU) bit known
  #[deku(bits = "1")]
  pub flags1_ch1_ctr_26t_ru_known: bool,

  /// Flags 1 channel 1 center 26-tone Resource Unit (RU) bit
  #[deku(bits = "1")]
  pub flags1_ch1_ctr_26t_ru: bool,

  /// Flags 1 Signal field B (SIG-B) compression known
  #[deku(bits = "1")]
  pub flags1_sig_b_comp_known: bool,

  /// Flags 1 number of HE Signal field B (SIG-B) symbols/Multiple User Multiple Input Multiple
  /// Output (MU-MIMO) users known
  #[deku(bits = "1")]
  pub flags1_sig_b_syms_users_known: bool,

  /// Flags 2 bandwidth from bandwidth field in HE Signal field A (SIG-A)
  pub flags2_bw_from_sig_a_bw: HeMuFlags2BwFromSigABw,

  /// Flags 2 bandwidth from bandwidth field in HE Signal field A (SIG-A) known
  #[deku(bits = "1")]
  pub flags2_bw_from_sig_a_bw_known: bool,

  /// Flags 2 Signal field B (SIG-B) compression
  #[deku(bits = "1")]
  pub flags2_sig_b_comp: bool,

  /// Flags 2 number of HE Signal field B (SIG-B) symbols/Multiple User Multiple Input Multiple
  /// Output (MU-MIMO) users
  #[deku(bits = "4")]
  pub flags2_sig_b_syms_users: u8,

  /// Flags 2 preamble puncturing from bandwidth field in HE Signal field A (SIG-A)
  pub flags2_punc_from_sig_a_bw: HeMuFlags2PuncFromSigABw,

  /// Flags 2 preamble puncturing from bandwidth field in HE Signal field A (SIG-A) known
  #[deku(bits = "1")]
  pub flags2_punc_from_sig_a_bw_known: bool,

  /// Flags 2 channel 2 center 26-tone Resource Unit (RU) bit
  #[deku(bits = "1")]
  pub flags2_ch2_ctr_26t_ru: bool,

  /// Flags 2 reserved 0
  #[deku(bits = "4")]
  pub flags2_reserved0: u8,

  /// Resource Unit (RU) allocation index for channel 1
  pub ru_channel1: [u8; 4],

  /// Resource Unit (RU) allocation index for channel 2
  pub ru_channel2: [u8; 4],
}

impl<V> RadiotapFieldTrait<V> for HeMuField {
  fn get_identifiers(&self) -> RadiotapFieldTraitIdentifiers {
    RadiotapFieldTraitIdentifiers { bit_index: 24 }
  }

  fn get_alignment(&self) -> usize {
    2
  }

  fn decode(&self, reader: &mut FieldReader) -> Result<RadiotapField<V>, RadiotapError> {
    // Decode the field
    let decoded = HeMuField::from_reader((reader, 0))?.1;

    Ok(RadiotapField::HeMu(decoded))
  }

  fn encode(&self, writer: &mut Writer<&mut FieldWriter>) -> Result<(), RadiotapError> {
    // Encode the field
    self.to_writer(writer, ())?;

    Ok(())
  }
}

/// HE-MU flags 2 bandwidth from bandwidth field in HE Signal field A (SIG-A)
#[derive(Clone, Debug, Default, DekuRead, DekuWrite, PartialEq)]
#[deku(
  id_type = "u8",
  bits = "2",
  bit_order = "order",
  endian = "endian",
  ctx = "endian: deku::ctx::Endian, order: deku::ctx::Order"
)]
pub enum HeMuFlags2BwFromSigABw {
  /// 20 MHz
  #[default]
  #[deku(id = "0")]
  Bw20,

  /// 40 MHz
  #[deku(id = "1")]
  Bw40,

  /// 80 MHz
  #[deku(id = "2")]
  Bw80,

  /// 160/80+80 MHz
  #[deku(id = "3")]
  Bw160,
}

/// HE-MU flags 2 preamble puncturing from bandwidth field in HE Signal field A (SIG-A)
#[derive(Clone, Debug, Default, DekuRead, DekuWrite, PartialEq)]
#[deku(
  id_type = "u8",
  bits = "2",
  bit_order = "order",
  endian = "endian",
  ctx = "endian: deku::ctx::Endian, order: deku::ctx::Order"
)]
pub enum HeMuFlags2PuncFromSigABw {
  /// Non-puncturing
  #[default]
  #[deku(id = "0")]
  NonPuncturing,

  /// Preamble puncturing in 80 MHz, where in the preamble the only punctured subchannel is the
  /// secondary 20 MHz channel
  #[deku(id = "1")]
  Punc80MHzSecondary20MHz,

  /// Preamble puncturing in 80 MHz, where in the preamble the only punctured subchannel is one of
  /// the two 20 MHz subchannels in the secondary 40 MHz channel
  #[deku(id = "2")]
  Punc80MHzSecondary40MHz20MHz,
}

#[cfg(test)]
mod tests {
  use deku::{DekuContainerRead, DekuContainerWrite};
  use pretty_assertions::assert_eq;
  use rstest::rstest;

  use super::*;

  /// Encoding test cases
  const ENCODING_CASES: &[(&[u8], &HeMuField)] = &[
    // No flags
    (
      &[
        0x0C, 0x00, // Flags 1
        0x63, 0x01, // Flags 2
        0x0C, 0x22, 0x38, 0x4E, // RU Channel 1
        0x57, 0x41, 0x2B, 0x15, // RU Channel 2
      ],
      &HeMuField {
        flags1_sig_b_mcs: 12,
        flags1_sig_b_mcs_known: false,
        flags1_dcm: false,
        flags1_dcm_known: false,
        flags1_ch2_ctr_26t_ru_known: false,
        flags1_ch1_ru_known: false,
        flags1_ch2_ru_known: false,
        flags1_reserved0: 0,
        flags1_ch1_ctr_26t_ru_known: false,
        flags1_ch1_ctr_26t_ru: false,
        flags1_sig_b_comp_known: false,
        flags1_sig_b_syms_users_known: false,
        flags2_bw_from_sig_a_bw: HeMuFlags2BwFromSigABw::Bw160,
        flags2_bw_from_sig_a_bw_known: false,
        flags2_sig_b_comp: false,
        flags2_sig_b_syms_users: 6,
        flags2_punc_from_sig_a_bw: HeMuFlags2PuncFromSigABw::Punc80MHzSecondary20MHz,
        flags2_punc_from_sig_a_bw_known: false,
        flags2_ch2_ctr_26t_ru: false,
        flags2_reserved0: 0,
        ru_channel1: [12, 34, 56, 78],
        ru_channel2: [87, 65, 43, 21],
      },
    ),
    // Alternating flags (A)
    (
      &[
        0x5C, 0x51, // Flags 1
        0x67, 0x05, // Flags 2
        0x0C, 0x22, 0x38, 0x4E, // RU Channel 1
        0x57, 0x41, 0x2B, 0x15, // RU Channel 2
      ],
      &HeMuField {
        flags1_sig_b_mcs: 12,
        flags1_sig_b_mcs_known: true,
        flags1_dcm: false,
        flags1_dcm_known: true,
        flags1_ch2_ctr_26t_ru_known: false,
        flags1_ch1_ru_known: true,
        flags1_ch2_ru_known: false,
        flags1_reserved0: 0,
        flags1_ch1_ctr_26t_ru_known: true,
        flags1_ch1_ctr_26t_ru: false,
        flags1_sig_b_comp_known: true,
        flags1_sig_b_syms_users_known: false,
        flags2_bw_from_sig_a_bw: HeMuFlags2BwFromSigABw::Bw160,
        flags2_bw_from_sig_a_bw_known: true,
        flags2_sig_b_comp: false,
        flags2_sig_b_syms_users: 6,
        flags2_punc_from_sig_a_bw: HeMuFlags2PuncFromSigABw::Punc80MHzSecondary20MHz,
        flags2_punc_from_sig_a_bw_known: true,
        flags2_ch2_ctr_26t_ru: false,
        flags2_reserved0: 0,
        ru_channel1: [12, 34, 56, 78],
        ru_channel2: [87, 65, 43, 21],
      },
    ),
    // Alternating flags (B)
    (
      &[
        0xAC, 0xA2, // Flags 1
        0x6B, 0x09, // Flags 2
        0x0C, 0x22, 0x38, 0x4E, // RU Channel 1
        0x57, 0x41, 0x2B, 0x15, // RU Channel 2
      ],
      &HeMuField {
        flags1_sig_b_mcs: 12,
        flags1_sig_b_mcs_known: false,
        flags1_dcm: true,
        flags1_dcm_known: false,
        flags1_ch2_ctr_26t_ru_known: true,
        flags1_ch1_ru_known: false,
        flags1_ch2_ru_known: true,
        flags1_reserved0: 0,
        flags1_ch1_ctr_26t_ru_known: false,
        flags1_ch1_ctr_26t_ru: true,
        flags1_sig_b_comp_known: false,
        flags1_sig_b_syms_users_known: true,
        flags2_bw_from_sig_a_bw: HeMuFlags2BwFromSigABw::Bw160,
        flags2_bw_from_sig_a_bw_known: false,
        flags2_sig_b_comp: true,
        flags2_sig_b_syms_users: 6,
        flags2_punc_from_sig_a_bw: HeMuFlags2PuncFromSigABw::Punc80MHzSecondary20MHz,
        flags2_punc_from_sig_a_bw_known: false,
        flags2_ch2_ctr_26t_ru: true,
        flags2_reserved0: 0,
        ru_channel1: [12, 34, 56, 78],
        ru_channel2: [87, 65, 43, 21],
      },
    ),
  ];

  #[rstest]
  #[case::no_flags(ENCODING_CASES[0].0, ENCODING_CASES[0].1)]
  #[case::alternating_flags_a(ENCODING_CASES[1].0, ENCODING_CASES[1].1)]
  #[case::alternating_flags_b(ENCODING_CASES[2].0, ENCODING_CASES[2].1)]
  fn test_decode_field(#[case] encoded: &[u8], #[case] expected: &HeMuField) {
    // Decode the bytes
    let decoded = HeMuField::from_bytes((encoded, 0)).unwrap().1;

    // Check the result
    assert_eq!(decoded, *expected);
  }

  #[rstest]
  #[case::no_flags(ENCODING_CASES[0].1, ENCODING_CASES[0].0)]
  #[case::alternating_flags_a(ENCODING_CASES[1].1, ENCODING_CASES[1].0)]
  #[case::alternating_flags_b(ENCODING_CASES[2].1, ENCODING_CASES[2].0)]
  fn test_encode_field(#[case] field: &HeMuField, #[case] expected: &[u8]) {
    // Encode the field
    let encoded = field.to_bytes().unwrap();

    // Check the result
    assert_eq!(encoded, expected);
  }
}
