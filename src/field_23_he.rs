use deku::{DekuContainerRead, DekuRead, DekuWrite, DekuWriter, writer::Writer};

use crate::utils::{
  FieldReader, FieldWriter, RadiotapError, RadiotapField, RadiotapFieldTrait,
  RadiotapFieldTraitIdentifiers,
};

/// High Efficiency (HE) field
///
/// The presence of this field indicates that the frame was received or transmitted using the HE
/// PHY.
///
/// This field contains data mostly from the HE-SIG-A part that is common to all HE transmissions,
/// but some of the data might also be calculated (e.g. when included in the transmission of an
/// HE-TRIG type frame, some parameters are not included in the HE-SIG-A but were determined by the
/// trigger frame).
///
/// However, for Physical Protocol Data Units (PPDUs) of HE-MU format, it contains the data that
/// applies to the encoding of the PSDU, with more detailed information about the MU transmission
/// contained in the HE-MU field if needed, in many cases that field will not be needed as all the
/// data about the single user that was captured is already encoded here.
///
/// In the case of MU transmissions, the HE-MU-other-user field may also be present one or more
/// times in that case to capture extra users for which the data couldn’t be captured; if the data
/// was captured for more than one user then multiple packets must be written into the radiotap
/// capture.
///
/// See [www.radiotap.org/fields/HE.html](https://www.radiotap.org/fields/HE.html) for more
/// information.
#[derive(Clone, Debug, Default, DekuRead, DekuWrite, PartialEq)]
#[deku(endian = "little", bit_order = "lsb")]
pub struct HeField {
  /// Data 1 Physical Protocol Data Unit (PPDU) format
  pub data1_ppdu_format: HeData1PpduFormat,

  /// Data 1 Basic Service Set (BSS) color known
  #[deku(bits = "1")]
  pub data1_bss_color_known: bool,

  /// Data 1 beam change known
  #[deku(bits = "1")]
  pub data1_beam_change_known: bool,

  /// Data 1 Uplink (UL)/downlink (DL) known
  #[deku(bits = "1")]
  pub data1_ul_dl_known: bool,

  /// Data 1 Data Modulation and Coding Scheme (MCS) known
  #[deku(bits = "1")]
  pub data1_data_mcs_known: bool,

  /// Data 1 Data Dual Carrier Modulation (DCM) known
  #[deku(bits = "1")]
  pub data1_data_dcm_known: bool,

  /// Data 1 coding known
  #[deku(bits = "1")]
  pub data1_coding_known: bool,

  /// Data 1 Low Density Parity Check (LDPC) extra symbol segment known
  #[deku(bits = "1")]
  pub data1_ldpc_xsymseg_known: bool,

  /// Data 1 Space Time Block Coding (STBC) known
  #[deku(bits = "1")]
  pub data1_stbc_known: bool,

  /// Data 1 spatial reuse known (Spatial reuse 1 for High Efficiency Trigger (HE-TRIG) format
  /// known)
  #[deku(bits = "1")]
  pub data1_spatial_reuse_known: bool,

  /// Data 1 spatial reuse 2 for High Efficiency Trigger (HE-TRIG) format known or station ID known
  /// for High Efficiency Multi User (HE-MU) format
  #[deku(bits = "1")]
  pub data1_spatial_reuse2_known: bool,

  /// Data 1 spatial reuse 3 for High Efficiency Trigger (HE-TRIG) format known
  #[deku(bits = "1")]
  pub data1_spatial_reuse3_known: bool,

  /// Data 1 spatial reuse 4 for High Efficiency Trigger (HE-TRIG) format known
  #[deku(bits = "1")]
  pub data1_spatial_reuse4_known: bool,

  /// Data 1 data bandwidth/resource unit (RU) allocation known
  #[deku(bits = "1")]
  pub data1_data_bw_ru_alloc_known: bool,

  /// Data 1 doppler known
  #[deku(bits = "1")]
  pub data1_doppler_known: bool,

  /// Data 2 primary/secondary 80 MHz known
  #[deku(bits = "1")]
  pub data2_prisec_80_known: bool,

  /// Data 2 Guard Interval known
  #[deku(bits = "1")]
  pub data2_gi_known: bool,

  /// Data 2 number of Long Training Fields (LTFs) symbols known
  #[deku(bits = "1")]
  pub data2_num_ltf_syms_known: bool,

  /// Data 2 pre-Forward Error Correction (FEC) padding known
  #[deku(bits = "1")]
  pub data2_pre_fec_pad_known: bool,

  /// Data 2 Transmit (TX) beamforming (BF) known
  #[deku(bits = "1")]
  pub data2_txbf_known: bool,

  /// Data 2 Packet Extension (PE) disambiguity known
  #[deku(bits = "1")]
  pub data2_pe_disambig_known: bool,

  /// Data 2 Transmission Opportunity (TXOP) known
  #[deku(bits = "1")]
  pub data2_txop_known: bool,

  /// Data 2 midamble periodicity known
  #[deku(bits = "1")]
  pub data2_midamble_known: bool,

  /// Data 2 Resource Unit (RU) allocation offset
  #[deku(bits = "6")]
  pub data2_ru_offset: u8,

  /// Data 2 Resource Unit (RU) allocation offset known
  #[deku(bits = "1")]
  pub data2_ru_known: bool,

  /// Data 2 primary/secondary 80 MHz is secondary
  #[deku(bits = "1")]
  pub data2_prisec_80_sec: bool,

  /// Data 3 Basic Service Set (BSS) color
  #[deku(bits = "6")]
  pub data3_bss_color: u8,

  /// Data 3 beam change
  #[deku(bits = "1")]
  pub data3_beam_change: bool,

  /// Data 3 Uplink (UL)/downlink (DL)
  #[deku(bits = "1")]
  pub data3_ul_dl: bool,

  /// Data 3 Data Modulation and Coding Scheme (MCS)
  #[deku(bits = "4")]
  pub data3_mcs: u8,

  /// Data 3 Data Dual Carrier Modulation (DCM)
  #[deku(bits = "1")]
  pub data3_dcm: bool,

  /// Data 3 coding (0 = Binary Convolutional Coding (BCC), 1 = Low Density Parity Check (LDPC))
  #[deku(bits = "1")]
  pub data3_coding: bool,

  /// Data 3 Low Density Parity Check (LDPC) extra symbol segment
  #[deku(bits = "1")]
  pub data3_ldpc_xsymseg: bool,

  /// Data 3 Space Time Block Coding (STBC)
  #[deku(bits = "1")]
  pub data3_stbc: bool,

  /// Data 4
  pub data4: HeData4,

  /// Data 5 data bandwidth/resource unit (RU) allocation
  pub data5_bw_ru_alloc: HeData5BwRuAlloc,

  /// Data 5 Guard Interval (GI)
  pub data5_gi: HeData5Gi,

  /// Data 5 Long Training Fields (LTFs) symbol size
  pub data5_ltf_size: HeData5LtfSymbolSize,

  /// Data 5 number of Long Training Fields (LTFs) symbols
  #[deku(bits = "3")]
  pub data5_num_ltf_syms: u8,

  /// Data 5 reserved 0
  #[deku(bits = "1")]
  pub data5_reserved0: u8,

  /// Data 5 pre-Forward Error Correction (FEC) padding factor
  #[deku(bits = "2")]
  pub data5_pre_fec_pad: u8,

  /// Data 5 Transmit (TX) beamforming (BF)
  #[deku(bits = "1")]
  pub data5_txbf: bool,

  /// Data 5 Packet Extension (PE) disambiguity
  #[deku(bits = "1")]
  pub data5_pe_disambig: bool,

  /// Data 6 Number of spatial streams (NSTS)
  #[deku(bits = "4")]
  pub data6_nsts: u8,

  /// Data 6 doppler value
  #[deku(bits = "1")]
  pub data6_doppler: bool,

  /// Data 6 reserved 0
  #[deku(bits = "3")]
  pub data6_reserved0: u8,

  /// Data 6 Transmission Opportunity (TXOP)
  #[deku(bits = "7")]
  pub data6_txop: u8,

  /// Data 6 midamble periodicity
  #[deku(bits = "1")]
  pub data6_midamble: bool,
}

impl<V> RadiotapFieldTrait<V> for HeField {
  fn get_identifiers(&self) -> RadiotapFieldTraitIdentifiers {
    RadiotapFieldTraitIdentifiers { bit_index: 23 }
  }

  fn get_alignment(&self) -> usize {
    2
  }

  fn decode(&self, reader: &mut FieldReader) -> Result<RadiotapField<V>, RadiotapError> {
    // Decode the field
    let decoded = HeField::from_reader((reader, 0))?.1;

    Ok(RadiotapField::He(decoded))
  }

  fn encode(&self, writer: &mut Writer<&mut FieldWriter>) -> Result<(), RadiotapError> {
    // Encode the field
    self.to_writer(writer, ())?;

    Ok(())
  }
}

/// HE data 1 Physical Protocol Data Unit (PPDU) format
#[derive(Clone, Debug, Default, DekuRead, DekuWrite, PartialEq)]
#[deku(
  id_type = "u8",
  bits = "2",
  bit_order = "order",
  endian = "endian",
  ctx = "endian: deku::ctx::Endian, order: deku::ctx::Order"
)]
pub enum HeData1PpduFormat {
  /// High Efficiency Single User (HE-SU) format
  #[default]
  #[deku(id = "0")]
  HeSu,

  /// High Efficiency Extended Single User (HE-EXT-SU) format
  #[deku(id = "1")]
  HeExtSu,

  /// High Efficiency Multi User (HE-MU) format
  #[deku(id = "2")]
  HeMu,

  /// High Efficiency Trigger (HE-TRIG) format
  #[deku(id = "3")]
  HeTrig,
}

/// HE data 4
#[derive(Clone, Debug, Default, DekuRead, DekuWrite, PartialEq)]
#[deku(
  bit_order = "order",
  endian = "endian",
  ctx = "endian: deku::ctx::Endian, order: deku::ctx::Order"
)]
pub struct HeData4 {
  value: u16,
}

impl HeData4 {
  /// Get the High Efficiency Single User (HE-SU) and High Efficiency Extended Single User
  /// (HE-EXT-SU) spatial reuse value
  pub fn get_su_mu_sptl_reuse(&self) -> u8 {
    (self.value & 0x000F) as u8
  }

  /// Set the High Efficiency Single User (HE-SU) and High Efficiency Extended Single User
  /// (HE-EXT-SU) spatial reuse value
  pub fn set_su_mu_sptl_reuse(&mut self, value: u8) {
    self.value = (self.value & !0x000F) | (value & 0x0F) as u16;
  }

  /// Get the High Efficiency Multi User (HE-MU) station ID value
  pub fn get_mu_sta_id(&self) -> u16 {
    (self.value & 0x7FF0) >> 4
  }

  /// Set the High Efficiency Multi User (HE-MU) station ID value
  pub fn set_mu_sta_id(&mut self, value: u16) {
    self.value = (self.value & !0x7FF0) | ((value & 0x7FF) << 4);
  }

  /// Get the High Efficiency Trigger (HE-TRIG) spatial reuse 1 value
  pub fn get_tb_sptl_reuse1(&self) -> u8 {
    (self.value & 0x000F) as u8
  }

  /// Set the High Efficiency Trigger (HE-TRIG) spatial reuse 1 value
  pub fn set_tb_sptl_reuse1(&mut self, value: u8) {
    self.value = (self.value & !0x000F) | (value & 0x0F) as u16;
  }

  /// Get the High Efficiency Trigger (HE-TRIG) spatial reuse 2 value
  pub fn get_tb_sptl_reuse2(&self) -> u8 {
    ((self.value & 0x00F0) >> 4) as u8
  }

  /// Set the High Efficiency Trigger (HE-TRIG) spatial reuse 2 value
  pub fn set_tb_sptl_reuse2(&mut self, value: u8) {
    self.value = (self.value & !0x00F0) | (((value & 0x0F) as u16) << 4);
  }

  /// Get the High Efficiency Trigger (HE-TRIG) spatial reuse 3 value
  pub fn get_tb_sptl_reuse3(&self) -> u8 {
    ((self.value & 0x0F00) >> 8) as u8
  }

  /// Set the High Efficiency Trigger (HE-TRIG) spatial reuse 3 value
  pub fn set_tb_sptl_reuse3(&mut self, value: u8) {
    self.value = (self.value & !0x0F00) | (((value & 0x0F) as u16) << 8);
  }

  /// Get the High Efficiency Trigger (HE-TRIG) spatial reuse 4 value
  pub fn get_tb_sptl_reuse4(&self) -> u8 {
    ((self.value & 0xF000) >> 12) as u8
  }

  /// Set the High Efficiency Trigger (HE-TRIG) spatial reuse 4 value
  pub fn set_tb_sptl_reuse4(&mut self, value: u8) {
    self.value = (self.value & !0xF000) | (((value & 0x0F) as u16) << 12);
  }
}

/// HE data 5 bandwidth/resource unit (RU) allocation
#[derive(Clone, Debug, Default, DekuRead, DekuWrite, PartialEq)]
#[deku(
  id_type = "u8",
  bits = "4",
  bit_order = "order",
  endian = "endian",
  ctx = "endian: deku::ctx::Endian, order: deku::ctx::Order"
)]
pub enum HeData5BwRuAlloc {
  /// 20 MHz
  #[default]
  #[deku(id = "0")]
  Bw20MHz,

  /// 40 MHz
  #[deku(id = "1")]
  Bw40MHz,

  /// 80 MHz
  #[deku(id = "2")]
  Bw80MHz,

  /// 160 MHz
  #[deku(id = "3")]
  Bw160MHz,

  /// 26-tone RU
  #[deku(id = "4")]
  Bw26ToneRu,

  /// 52-tone RU
  #[deku(id = "5")]
  Bw52ToneRu,

  /// 106-tone RU
  #[deku(id = "6")]
  Bw106ToneRu,

  /// 242-tone RU
  #[deku(id = "7")]
  Bw242ToneRu,

  /// 484-tone RU
  #[deku(id = "8")]
  Bw484ToneRu,

  /// 996-tone RU
  #[deku(id = "9")]
  Bw996ToneRu,

  /// 2x996-tone RU
  #[deku(id = "10")]
  Bw2x996ToneRu,
}

/// HE data 5 Guard Interval (GI)
#[derive(Clone, Debug, Default, DekuRead, DekuWrite, PartialEq)]
#[deku(
  id_type = "u8",
  bits = "2",
  bit_order = "order",
  endian = "endian",
  ctx = "endian: deku::ctx::Endian, order: deku::ctx::Order"
)]
pub enum HeData5Gi {
  /// 0.8 us
  #[default]
  #[deku(id = "0")]
  Gi0_8,

  /// 1.6 us
  #[deku(id = "1")]
  Gi1_6,

  /// 3.2 us
  #[deku(id = "2")]
  Gi3_2,
}

/// HE data 5 Long Training Fields (LTFs) symbol size
#[derive(Clone, Debug, Default, DekuRead, DekuWrite, PartialEq)]
#[deku(
  id_type = "u8",
  bits = "2",
  bit_order = "order",
  endian = "endian",
  ctx = "endian: deku::ctx::Endian, order: deku::ctx::Order"
)]
pub enum HeData5LtfSymbolSize {
  /// Unknown number of symbols
  #[default]
  #[deku(id = "0")]
  Unknown,

  /// 1x symbol
  #[deku(id = "1")]
  Size1x,

  /// 2x symbol
  #[deku(id = "2")]
  Size2x,

  /// 4x symbol
  #[deku(id = "3")]
  Size4x,
}

#[cfg(test)]
mod tests {
  use deku::{DekuContainerRead, DekuContainerWrite};
  use pretty_assertions::assert_eq;
  use rstest::rstest;

  use super::*;

  /// Encoding test cases
  const ENCODING_CASES: &[(&[u8], &HeField)] = &[
    // No flags
    (
      &[
        0x02, 0x00, // Data 1
        0x00, 0x0C, // Data 2
        0x22, 0x05, // Data 3
        0x76, 0x98, // Data 4
        0x93, 0x37, // Data 5
        0x04, 0x38, // Data 6
      ],
      &HeField {
        data1_ppdu_format: HeData1PpduFormat::HeMu,
        data1_bss_color_known: false,
        data1_beam_change_known: false,
        data1_ul_dl_known: false,
        data1_data_mcs_known: false,
        data1_data_dcm_known: false,
        data1_coding_known: false,
        data1_ldpc_xsymseg_known: false,
        data1_stbc_known: false,
        data1_spatial_reuse_known: false,
        data1_spatial_reuse2_known: false,
        data1_spatial_reuse3_known: false,
        data1_spatial_reuse4_known: false,
        data1_data_bw_ru_alloc_known: false,
        data1_doppler_known: false,
        data2_prisec_80_known: false,
        data2_gi_known: false,
        data2_num_ltf_syms_known: false,
        data2_pre_fec_pad_known: false,
        data2_txbf_known: false,
        data2_pe_disambig_known: false,
        data2_txop_known: false,
        data2_midamble_known: false,
        data2_ru_offset: 12,
        data2_ru_known: false,
        data2_prisec_80_sec: false,
        data3_bss_color: 34,
        data3_beam_change: false,
        data3_ul_dl: false,
        data3_mcs: 5,
        data3_dcm: false,
        data3_coding: false,
        data3_ldpc_xsymseg: false,
        data3_stbc: false,
        data4: HeData4 { value: 0x9876 },
        data5_bw_ru_alloc: HeData5BwRuAlloc::Bw160MHz,
        data5_gi: HeData5Gi::Gi1_6,
        data5_ltf_size: HeData5LtfSymbolSize::Size2x,
        data5_num_ltf_syms: 7,
        data5_reserved0: 0,
        data5_pre_fec_pad: 3,
        data5_txbf: false,
        data5_pe_disambig: false,
        data6_nsts: 4,
        data6_doppler: false,
        data6_reserved0: 0,
        data6_txop: 56,
        data6_midamble: false,
      },
    ),
    // Alternating flags (A)
    (
      &[
        0x56, 0x55, // Data 1
        0x55, 0x4C, // Data 2
        0x62, 0x55, // Data 3
        0x76, 0x98, // Data 4
        0x93, 0x77, // Data 5
        0x14, 0x38, // Data 6
      ],
      &HeField {
        data1_ppdu_format: HeData1PpduFormat::HeMu,
        data1_bss_color_known: true,
        data1_beam_change_known: false,
        data1_ul_dl_known: true,
        data1_data_mcs_known: false,
        data1_data_dcm_known: true,
        data1_coding_known: false,
        data1_ldpc_xsymseg_known: true,
        data1_stbc_known: false,
        data1_spatial_reuse_known: true,
        data1_spatial_reuse2_known: false,
        data1_spatial_reuse3_known: true,
        data1_spatial_reuse4_known: false,
        data1_data_bw_ru_alloc_known: true,
        data1_doppler_known: false,
        data2_prisec_80_known: true,
        data2_gi_known: false,
        data2_num_ltf_syms_known: true,
        data2_pre_fec_pad_known: false,
        data2_txbf_known: true,
        data2_pe_disambig_known: false,
        data2_txop_known: true,
        data2_midamble_known: false,
        data2_ru_offset: 12,
        data2_ru_known: true,
        data2_prisec_80_sec: false,
        data3_bss_color: 34,
        data3_beam_change: true,
        data3_ul_dl: false,
        data3_mcs: 5,
        data3_dcm: true,
        data3_coding: false,
        data3_ldpc_xsymseg: true,
        data3_stbc: false,
        data4: HeData4 { value: 0x9876 },
        data5_bw_ru_alloc: HeData5BwRuAlloc::Bw160MHz,
        data5_gi: HeData5Gi::Gi1_6,
        data5_ltf_size: HeData5LtfSymbolSize::Size2x,
        data5_num_ltf_syms: 7,
        data5_reserved0: 0,
        data5_pre_fec_pad: 3,
        data5_txbf: true,
        data5_pe_disambig: false,
        data6_nsts: 4,
        data6_doppler: true,
        data6_reserved0: 0,
        data6_txop: 56,
        data6_midamble: false,
      },
    ),
    // Alternating flags (B)
    (
      &[
        0xAA, 0xAA, // Data 1
        0xAA, 0x8C, // Data 2
        0xA2, 0xA5, // Data 3
        0x76, 0x98, // Data 4
        0x93, 0xB7, // Data 5
        0x04, 0xB8, // Data 6
      ],
      &HeField {
        data1_ppdu_format: HeData1PpduFormat::HeMu,
        data1_bss_color_known: false,
        data1_beam_change_known: true,
        data1_ul_dl_known: false,
        data1_data_mcs_known: true,
        data1_data_dcm_known: false,
        data1_coding_known: true,
        data1_ldpc_xsymseg_known: false,
        data1_stbc_known: true,
        data1_spatial_reuse_known: false,
        data1_spatial_reuse2_known: true,
        data1_spatial_reuse3_known: false,
        data1_spatial_reuse4_known: true,
        data1_data_bw_ru_alloc_known: false,
        data1_doppler_known: true,
        data2_prisec_80_known: false,
        data2_gi_known: true,
        data2_num_ltf_syms_known: false,
        data2_pre_fec_pad_known: true,
        data2_txbf_known: false,
        data2_pe_disambig_known: true,
        data2_txop_known: false,
        data2_midamble_known: true,
        data2_ru_offset: 12,
        data2_ru_known: false,
        data2_prisec_80_sec: true,
        data3_bss_color: 34,
        data3_beam_change: false,
        data3_ul_dl: true,
        data3_mcs: 5,
        data3_dcm: false,
        data3_coding: true,
        data3_ldpc_xsymseg: false,
        data3_stbc: true,
        data4: HeData4 { value: 0x9876 },
        data5_bw_ru_alloc: HeData5BwRuAlloc::Bw160MHz,
        data5_gi: HeData5Gi::Gi1_6,
        data5_ltf_size: HeData5LtfSymbolSize::Size2x,
        data5_num_ltf_syms: 7,
        data5_reserved0: 0,
        data5_pre_fec_pad: 3,
        data5_txbf: false,
        data5_pe_disambig: true,
        data6_nsts: 4,
        data6_doppler: false,
        data6_reserved0: 0,
        data6_txop: 56,
        data6_midamble: true,
      },
    ),
  ];

  #[rstest]
  #[case::no_flags(ENCODING_CASES[0].0, ENCODING_CASES[0].1)]
  #[case::alternating_flags_a(ENCODING_CASES[1].0, ENCODING_CASES[1].1)]
  #[case::alternating_flags_b(ENCODING_CASES[2].0, ENCODING_CASES[2].1)]
  fn test_decode_field(#[case] encoded: &[u8], #[case] expected: &HeField) {
    // Decode the bytes
    let decoded = HeField::from_bytes((encoded, 0)).unwrap().1;

    // Check the result
    assert_eq!(decoded, *expected);
  }

  #[rstest]
  #[case::no_flags(ENCODING_CASES[0].1, ENCODING_CASES[0].0)]
  #[case::alternating_flags_a(ENCODING_CASES[1].1, ENCODING_CASES[1].0)]
  #[case::alternating_flags_b(ENCODING_CASES[2].1, ENCODING_CASES[2].0)]
  fn test_encode_field(#[case] field: &HeField, #[case] expected: &[u8]) {
    // Encode the field
    let encoded = field.to_bytes().unwrap();

    // Check the result
    assert_eq!(encoded, expected);
  }

  /// Data 4 tests
  #[rstest]
  fn test_he_data4_getters_setters() {
    let mut data4 = HeData4 { value: 0 };

    data4.set_su_mu_sptl_reuse(0x0A);
    assert_eq!(data4.get_su_mu_sptl_reuse(), 0x0A);

    data4.set_mu_sta_id(0x3FF);
    assert_eq!(data4.get_mu_sta_id(), 0x3FF);

    data4.set_tb_sptl_reuse1(0x05);
    assert_eq!(data4.get_tb_sptl_reuse1(), 0x05);

    data4.set_tb_sptl_reuse2(0x06);
    assert_eq!(data4.get_tb_sptl_reuse2(), 0x06);

    data4.set_tb_sptl_reuse3(0x07);
    assert_eq!(data4.get_tb_sptl_reuse3(), 0x07);

    data4.set_tb_sptl_reuse4(0x08);
    assert_eq!(data4.get_tb_sptl_reuse4(), 0x08);
  }
}
