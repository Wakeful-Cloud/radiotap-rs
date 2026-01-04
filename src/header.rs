use alloc::{boxed::Box, vec::Vec};
use deku::{DekuContainerRead, DekuContainerWrite, writer::Writer};
use hashbrown::HashMap;

use crate::{
  field_0_tsft::TsftField,
  field_1_flags::FlagsField,
  field_2_rate::RateField,
  field_3_channel::ChannelField,
  field_4_fhss::FhssField,
  field_5_dbm_antsignal::DbmAntSignalField,
  field_6_dbm_antnoise::DbmAntNoiseField,
  field_7_lock_quality::LockQualityField,
  field_8_tx_attenuation::TxAttenuationField,
  field_9_db_tx_attenuation::DbTxAttenuationField,
  field_10_dbm_tx_power::DbmTxPowerField,
  field_11_antenna::AntennaField,
  field_12_db_antsignal::DbAntSignalField,
  field_13_db_antnoise::DbAntNoiseField,
  field_14_rx_flags::RxFlagsField,
  field_15_tx_flags::TxFlagsField,
  field_16_rts_retries::RtsRetriesField,
  field_17_data_retries::DataRetriesField,
  field_18_xchannel::XChannelField,
  field_19_mcs::McsField,
  field_20_ampdu_status::AmpduStatusField,
  field_21_vht::VhtField,
  field_22_timestamp::TimestampField,
  field_23_he::HeField,
  field_24_he_mu::HeMuField,
  field_25_he_mu_other_user::HeMuOtherUserField,
  field_26_psdu::PsduField,
  field_27_lsig::LsigField,
  field_28_tlv::{TlvField, TlvFieldItem},
  field_30_vendor_namespace::VendorNamespaceField,
  sub_header::SubHeader,
  utils::{
    FieldReader, FieldWriter, RadiotapError, RadiotapField, RadiotapFieldTrait,
    RadiotapFieldTraitIdentifiers, VendorFieldTrait, VendorFieldTraitIdentifiers,
  },
};

/// Radiotap header
///
/// See https://www.radiotap.org/ for more information.
#[derive(Clone, Debug, PartialEq)]
pub struct RadiotapHeader<V>
where
  V: VendorFieldTrait<V>,
{
  /// Indicates which major version of the radiotap header is in use. Currently, this is always `0`.
  /// Adding support for additional radiotap fields does not change the version number.
  pub version: u8,

  /// Fields follow, as specified by the present bitmasks.
  pub fields: Vec<RadiotapField<V>>,
}

impl<V> RadiotapHeader<V>
where
  V: VendorFieldTrait<V>,
{
  /// Decode a radiotap header from raw bytes, returning the header and the total size of the header
  /// in bytes. Note that setting `error_on_unknown_fields` to `false` may still yield an unknown
  /// field error for non-TLV fields (because non-TLV fields lack explicit lengths and thus this
  /// decoder is unable to decode subsequent fields, even if they are known fields).
  pub fn from_bytes(
    bytes: &[u8],
    config: &RadiotapHeaderDecodingConfig<V>,
    error_on_unknown_fields: bool,
  ) -> Result<(Self, usize), RadiotapError> {
    // Decode the sub header
    let sub_header = SubHeader::from_bytes((bytes, 0))?.1;

    // Calculate the header end offset
    let header_end_offset = sub_header.length as usize;

    // Decode the header
    let mut header = RadiotapHeader {
      version: sub_header.version,
      fields: Vec::new(),
    };

    // Decode the fields
    let mut header_current_offset = 4 + (sub_header.present_bitmasks.len() * 4);
    let mut bit_offset: isize = 0;
    let mut namespace: Option<([u8; 3], u8)> = None;

    for presence_bitmask in sub_header.present_bitmasks {
      for bit in 0..32 {
        // Check if the bit is set
        if presence_bitmask & (1 << bit) == 0 {
          continue;
        }

        // Calculate the bit index
        let bit_index = bit_offset + bit;

        match (&namespace, bit_index) {
          // TLV field
          (None, 28) => {
            // Skip alignment padding (4 bytes)
            header_current_offset = header_current_offset.div_ceil(4) * 4;

            // Decode the TLV field
            let ((rest, _), tlv_field) =
              TlvField::from_bytes((&bytes[header_current_offset..header_end_offset], 0))?;

            // Decode the field items
            for item in tlv_field.items {
              match item {
                // Vendor namespace fields
                TlvFieldItem::Vendor {
                  length: _,
                  oui,
                  sub_namespace,
                  presence,
                  value,
                } => {
                  // Look up the field
                  let field = config.vendor_fields.get(&VendorFieldTraitIdentifiers {
                    oui,
                    sub_namespace,
                    bit_index: presence,
                  });

                  if let Some(field) = field {
                    // Decode the field
                    let mut reader = FieldReader::new(&value, true);
                    let decoded_field = field.decode(&mut reader)?;

                    // Add the field to the header
                    header.fields.push(RadiotapField::Vendor(decoded_field));
                  } else if error_on_unknown_fields {
                    return Err(RadiotapError::UnknownField);
                  }
                }

                // Padding
                TlvFieldItem::Radiotap {
                  r#type: 28,
                  length: _,
                  value: _,
                } => {}

                // Radiotap namespace fields
                TlvFieldItem::Radiotap {
                  r#type,
                  length: _,
                  value,
                } => {
                  // Look up the field
                  let field = config
                    .radiotap_fields
                    .get(&RadiotapFieldTraitIdentifiers { bit_index: r#type });

                  if let Some(field) = field {
                    // Decode the field
                    let mut reader = FieldReader::new(&value, true);
                    let decoded_field = field.decode(&mut reader)?;

                    // Check that the field matches the expected type and value
                    if let RadiotapField::Vendor { .. } = decoded_field {
                      return Err(RadiotapError::FieldTypeValueMismatch);
                    }

                    // Add the field to the header
                    header.fields.push(decoded_field);
                  } else if error_on_unknown_fields {
                    return Err(RadiotapError::UnknownField);
                  }
                }
              }
            }

            // Update the header current offset
            header_current_offset = header_end_offset - rest.len();
          }

          // Radiotap namespace field
          (None, bit_index) if bit_index % 32 == 29 => {
            // Switch to the radiotap namespace
            bit_offset = -32;
            namespace = None;
            break;
          }

          // Vendor namespace field
          (None, bit_index) if bit_index % 32 == 30 => {
            // Skip alignment padding (2 bytes)
            header_current_offset = header_current_offset.div_ceil(2) * 2;

            // Decode the vendor namespace field
            let ((rest, _), vendor_namespace) = VendorNamespaceField::from_bytes((
              &bytes[header_current_offset..header_end_offset],
              0,
            ))?;

            // Switch to the vendor namespace
            bit_offset = -32;
            namespace = Some((vendor_namespace.oui, vendor_namespace.sub_namespace));

            // Update the header current offset
            header_current_offset = header_end_offset - rest.len();

            break;
          }

          // Extended present bitmask
          (_, bit_index) if bit_index % 32 == 31 => {}

          // Normal fields
          (None, bit_index) => {
            // Decode the field
            if let Some((decoded_field, decoded_field_size)) = match &namespace {
              Some((oui, sub_namespace)) => {
                match config.vendor_fields.get(&VendorFieldTraitIdentifiers {
                  oui: *oui,
                  sub_namespace: *sub_namespace,
                  bit_index: bit_index as u16,
                }) {
                  Some(field) => {
                    // Get the field alignment
                    let field_alignment = field.get_alignment();

                    // Skip alignment padding
                    header_current_offset =
                      header_current_offset.div_ceil(field_alignment) * field_alignment;

                    // Decode the field
                    let mut reader =
                      FieldReader::new(&bytes[header_current_offset..header_end_offset], false);
                    let decoded_field = field.decode(&mut reader)?;

                    Some((RadiotapField::Vendor(decoded_field), reader.position()))
                  }
                  None => None,
                }
              }
              None => match config.radiotap_fields.get(&RadiotapFieldTraitIdentifiers {
                bit_index: bit_index as u16,
              }) {
                Some(field) => {
                  // Get the field alignment
                  let field_alignment = field.get_alignment();

                  // Skip alignment padding
                  header_current_offset =
                    header_current_offset.div_ceil(field_alignment) * field_alignment;

                  // Decode the field
                  let mut reader =
                    FieldReader::new(&bytes[header_current_offset..header_end_offset], false);
                  let decoded_field = field.decode(&mut reader)?;

                  Some((decoded_field, reader.position()))
                }
                None => None,
              },
            } {
              // Add the field to the header
              header.fields.push(decoded_field);

              // Update the header current offset
              header_current_offset += decoded_field_size;
            } else {
              // We must return an error because we cannot decode subsequent fields due to
              // implicit-length encoding
              return Err(RadiotapError::UnknownField);
            }
          }

          // Vendor namespace field
          (Some((oui, sub_namespace)), bit_index) => {
            // Decode the field
            if let Some(field) = config.vendor_fields.get(&VendorFieldTraitIdentifiers {
              oui: *oui,
              sub_namespace: *sub_namespace,
              bit_index: bit_index as u16,
            }) {
              // Get the field alignment
              let field_alignment = field.get_alignment();

              // Skip alignment padding
              header_current_offset =
                header_current_offset.div_ceil(field_alignment) * field_alignment;

              // Decode the field
              let mut reader =
                FieldReader::new(&bytes[header_current_offset..header_end_offset], false);
              let decoded_field = field.decode(&mut reader)?;

              // Add the field to the header
              header.fields.push(RadiotapField::Vendor(decoded_field));

              // Update the header current offset
              header_current_offset += reader.position();
            } else {
              // We must return an error because we cannot decode subsequent fields due to
              // implicit-length encoding
              return Err(RadiotapError::UnknownField);
            }
          }
        }
      }

      // Advance the bit offset
      bit_offset += 32;
    }

    Ok((header, header_end_offset))
  }

  /// Encode a radiotap header to bytes. Note that vendor fields are always encoded as TLV fields
  /// for simplicity.
  pub fn to_bytes(&self) -> Result<Vec<u8>, RadiotapError> {
    // Partition fields by encoding type
    let (implicit_fields, tlv_fields): (Vec<&RadiotapField<V>>, Vec<&RadiotapField<V>>) = self
      .fields
      .iter()
      .partition(|field| !matches!(field, RadiotapField::Vendor { .. }));

    // Encode the implicit-length fields
    let mut bit_offset = 0;
    let mut present_bitmask = 0;
    let mut present_bitmasks = Vec::new();
    let mut encoded_fields = Vec::new();

    for field in implicit_fields {
      // Initialize a field writer
      let mut buffer = Vec::new();
      let mut field_writer = FieldWriter::new(&mut buffer);
      let mut writer = Writer::new(&mut field_writer);

      // Encode the field
      let (identifiers, field_alignment) = match field {
        RadiotapField::Tsft(field) => {
          RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
          (
            RadiotapFieldTrait::<V>::get_identifiers(field),
            RadiotapFieldTrait::<V>::get_alignment(field),
          )
        }
        RadiotapField::Flags(field) => {
          RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
          (
            RadiotapFieldTrait::<V>::get_identifiers(field),
            RadiotapFieldTrait::<V>::get_alignment(field),
          )
        }
        RadiotapField::Rate(field) => {
          RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
          (
            RadiotapFieldTrait::<V>::get_identifiers(field),
            RadiotapFieldTrait::<V>::get_alignment(field),
          )
        }
        RadiotapField::Channel(field) => {
          RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
          (
            RadiotapFieldTrait::<V>::get_identifiers(field),
            RadiotapFieldTrait::<V>::get_alignment(field),
          )
        }
        RadiotapField::Fhss(field) => {
          RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
          (
            RadiotapFieldTrait::<V>::get_identifiers(field),
            RadiotapFieldTrait::<V>::get_alignment(field),
          )
        }
        RadiotapField::DbmAntSignal(field) => {
          RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
          (
            RadiotapFieldTrait::<V>::get_identifiers(field),
            RadiotapFieldTrait::<V>::get_alignment(field),
          )
        }
        RadiotapField::DbmAntNoise(field) => {
          RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
          (
            RadiotapFieldTrait::<V>::get_identifiers(field),
            RadiotapFieldTrait::<V>::get_alignment(field),
          )
        }
        RadiotapField::LockQuality(field) => {
          RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
          (
            RadiotapFieldTrait::<V>::get_identifiers(field),
            RadiotapFieldTrait::<V>::get_alignment(field),
          )
        }
        RadiotapField::TxAttenuation(field) => {
          RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
          (
            RadiotapFieldTrait::<V>::get_identifiers(field),
            RadiotapFieldTrait::<V>::get_alignment(field),
          )
        }
        RadiotapField::DbTxAttenuation(field) => {
          RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
          (
            RadiotapFieldTrait::<V>::get_identifiers(field),
            RadiotapFieldTrait::<V>::get_alignment(field),
          )
        }
        RadiotapField::DbmTxPower(field) => {
          RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
          (
            RadiotapFieldTrait::<V>::get_identifiers(field),
            RadiotapFieldTrait::<V>::get_alignment(field),
          )
        }
        RadiotapField::Antenna(field) => {
          RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
          (
            RadiotapFieldTrait::<V>::get_identifiers(field),
            RadiotapFieldTrait::<V>::get_alignment(field),
          )
        }
        RadiotapField::DbAntSignal(field) => {
          RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
          (
            RadiotapFieldTrait::<V>::get_identifiers(field),
            RadiotapFieldTrait::<V>::get_alignment(field),
          )
        }
        RadiotapField::DbAntNoise(field) => {
          RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
          (
            RadiotapFieldTrait::<V>::get_identifiers(field),
            RadiotapFieldTrait::<V>::get_alignment(field),
          )
        }
        RadiotapField::FcsInHeader(field) => {
          RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
          (
            RadiotapFieldTrait::<V>::get_identifiers(field),
            RadiotapFieldTrait::<V>::get_alignment(field),
          )
        }
        RadiotapField::RxFlags(field) => {
          RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
          (
            RadiotapFieldTrait::<V>::get_identifiers(field),
            RadiotapFieldTrait::<V>::get_alignment(field),
          )
        }
        RadiotapField::HardwareQueue(field) => {
          RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
          (
            RadiotapFieldTrait::<V>::get_identifiers(field),
            RadiotapFieldTrait::<V>::get_alignment(field),
          )
        }
        RadiotapField::TxFlags(field) => {
          RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
          (
            RadiotapFieldTrait::<V>::get_identifiers(field),
            RadiotapFieldTrait::<V>::get_alignment(field),
          )
        }
        RadiotapField::Rssi(field) => {
          RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
          (
            RadiotapFieldTrait::<V>::get_identifiers(field),
            RadiotapFieldTrait::<V>::get_alignment(field),
          )
        }
        RadiotapField::RtsRetries(field) => {
          RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
          (
            RadiotapFieldTrait::<V>::get_identifiers(field),
            RadiotapFieldTrait::<V>::get_alignment(field),
          )
        }
        RadiotapField::DataRetries(field) => {
          RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
          (
            RadiotapFieldTrait::<V>::get_identifiers(field),
            RadiotapFieldTrait::<V>::get_alignment(field),
          )
        }
        RadiotapField::XChannel(field) => {
          RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
          (
            RadiotapFieldTrait::<V>::get_identifiers(field),
            RadiotapFieldTrait::<V>::get_alignment(field),
          )
        }
        RadiotapField::Mcs(field) => {
          RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
          (
            RadiotapFieldTrait::<V>::get_identifiers(field),
            RadiotapFieldTrait::<V>::get_alignment(field),
          )
        }
        RadiotapField::AmpduStatus(field) => {
          RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
          (
            RadiotapFieldTrait::<V>::get_identifiers(field),
            RadiotapFieldTrait::<V>::get_alignment(field),
          )
        }
        RadiotapField::Vht(field) => {
          RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
          (
            RadiotapFieldTrait::<V>::get_identifiers(field),
            RadiotapFieldTrait::<V>::get_alignment(field),
          )
        }
        RadiotapField::ExtendedFlags(field) => {
          RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
          (
            RadiotapFieldTrait::<V>::get_identifiers(field),
            RadiotapFieldTrait::<V>::get_alignment(field),
          )
        }
        RadiotapField::Timestamp(field) => {
          RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
          (
            RadiotapFieldTrait::<V>::get_identifiers(field),
            RadiotapFieldTrait::<V>::get_alignment(field),
          )
        }
        RadiotapField::He(field) => {
          RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
          (
            RadiotapFieldTrait::<V>::get_identifiers(field),
            RadiotapFieldTrait::<V>::get_alignment(field),
          )
        }
        RadiotapField::HeMu(field) => {
          RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
          (
            RadiotapFieldTrait::<V>::get_identifiers(field),
            RadiotapFieldTrait::<V>::get_alignment(field),
          )
        }
        RadiotapField::HeMuOtherUser(field) => {
          RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
          (
            RadiotapFieldTrait::<V>::get_identifiers(field),
            RadiotapFieldTrait::<V>::get_alignment(field),
          )
        }
        RadiotapField::Psdu(field) => {
          RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
          (
            RadiotapFieldTrait::<V>::get_identifiers(field),
            RadiotapFieldTrait::<V>::get_alignment(field),
          )
        }
        RadiotapField::Lsig(field) => {
          RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
          (
            RadiotapFieldTrait::<V>::get_identifiers(field),
            RadiotapFieldTrait::<V>::get_alignment(field),
          )
        }
        RadiotapField::Vendor(..) => unreachable!(),
      };

      // Add a new radiotap namespace if necessary
      if identifiers.bit_index < bit_offset + 32
        && (present_bitmask & (1 << (identifiers.bit_index - bit_offset))) != 0
      {
        present_bitmasks.push(present_bitmask | 0x20000000);
        present_bitmask = 0;
        bit_offset = 0;
      }

      // Advance to the correct present bitmask
      while identifiers.bit_index >= bit_offset + 32 {
        present_bitmasks.push(present_bitmask);
        present_bitmask = 0;
        bit_offset += 32;
      }

      // Update the present bitmask
      present_bitmask |= 1 << (identifiers.bit_index - bit_offset);

      // Calculate the current length
      let length = 4 + (present_bitmasks.len() * 4) + encoded_fields.len();

      // Generate alignment padding
      let field_padding =
        vec![0u8; (field_alignment - (length % field_alignment)) % field_alignment];

      // Add to encoded fields
      encoded_fields.extend(field_padding);
      encoded_fields.extend(buffer);
    }

    // Encode TLV fields
    if !tlv_fields.is_empty() {
      // Update the present bitmask
      if bit_offset == 0 {
        present_bitmasks.push(present_bitmask | 0x10000000);
      } else {
        present_bitmasks.push(0x30000000);
      }

      // Encode the TLV fields
      let items = tlv_fields
        .iter()
        .map(|field| {
          // Initialize a field writer
          let mut buffer = Vec::new();
          let mut field_writer = FieldWriter::new(&mut buffer);
          let mut writer = Writer::new(&mut field_writer);

          // Encode the field
          let identifiers = match field {
            RadiotapField::Tsft(field) => {
              RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
              RadiotapFieldTrait::<V>::get_identifiers(field)
            }

            RadiotapField::Flags(field) => {
              RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
              RadiotapFieldTrait::<V>::get_identifiers(field)
            }
            RadiotapField::Rate(field) => {
              RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
              RadiotapFieldTrait::<V>::get_identifiers(field)
            }
            RadiotapField::Channel(field) => {
              RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
              RadiotapFieldTrait::<V>::get_identifiers(field)
            }
            RadiotapField::Fhss(field) => {
              RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
              RadiotapFieldTrait::<V>::get_identifiers(field)
            }
            RadiotapField::DbmAntSignal(field) => {
              RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
              RadiotapFieldTrait::<V>::get_identifiers(field)
            }
            RadiotapField::DbmAntNoise(field) => {
              RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
              RadiotapFieldTrait::<V>::get_identifiers(field)
            }
            RadiotapField::LockQuality(field) => {
              RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
              RadiotapFieldTrait::<V>::get_identifiers(field)
            }
            RadiotapField::TxAttenuation(field) => {
              RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
              RadiotapFieldTrait::<V>::get_identifiers(field)
            }
            RadiotapField::DbTxAttenuation(field) => {
              RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
              RadiotapFieldTrait::<V>::get_identifiers(field)
            }
            RadiotapField::DbmTxPower(field) => {
              RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
              RadiotapFieldTrait::<V>::get_identifiers(field)
            }
            RadiotapField::Antenna(field) => {
              RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
              RadiotapFieldTrait::<V>::get_identifiers(field)
            }
            RadiotapField::DbAntSignal(field) => {
              RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
              RadiotapFieldTrait::<V>::get_identifiers(field)
            }
            RadiotapField::DbAntNoise(field) => {
              RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
              RadiotapFieldTrait::<V>::get_identifiers(field)
            }
            RadiotapField::FcsInHeader(field) => {
              RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
              RadiotapFieldTrait::<V>::get_identifiers(field)
            }
            RadiotapField::RxFlags(field) => {
              RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
              RadiotapFieldTrait::<V>::get_identifiers(field)
            }
            RadiotapField::HardwareQueue(field) => {
              RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
              RadiotapFieldTrait::<V>::get_identifiers(field)
            }
            RadiotapField::TxFlags(field) => {
              RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
              RadiotapFieldTrait::<V>::get_identifiers(field)
            }
            RadiotapField::Rssi(field) => {
              RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
              RadiotapFieldTrait::<V>::get_identifiers(field)
            }
            RadiotapField::RtsRetries(field) => {
              RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
              RadiotapFieldTrait::<V>::get_identifiers(field)
            }
            RadiotapField::DataRetries(field) => {
              RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
              RadiotapFieldTrait::<V>::get_identifiers(field)
            }
            RadiotapField::XChannel(field) => {
              RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
              RadiotapFieldTrait::<V>::get_identifiers(field)
            }
            RadiotapField::Mcs(field) => {
              RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
              RadiotapFieldTrait::<V>::get_identifiers(field)
            }
            RadiotapField::AmpduStatus(field) => {
              RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
              RadiotapFieldTrait::<V>::get_identifiers(field)
            }
            RadiotapField::Vht(field) => {
              RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
              RadiotapFieldTrait::<V>::get_identifiers(field)
            }
            RadiotapField::ExtendedFlags(field) => {
              RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
              RadiotapFieldTrait::<V>::get_identifiers(field)
            }
            RadiotapField::Timestamp(field) => {
              RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
              RadiotapFieldTrait::<V>::get_identifiers(field)
            }
            RadiotapField::He(field) => {
              RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
              RadiotapFieldTrait::<V>::get_identifiers(field)
            }
            RadiotapField::HeMu(field) => {
              RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
              RadiotapFieldTrait::<V>::get_identifiers(field)
            }
            RadiotapField::HeMuOtherUser(field) => {
              RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
              RadiotapFieldTrait::<V>::get_identifiers(field)
            }
            RadiotapField::Psdu(field) => {
              RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
              RadiotapFieldTrait::<V>::get_identifiers(field)
            }
            RadiotapField::Lsig(field) => {
              RadiotapFieldTrait::<V>::encode(field, &mut writer)?;
              RadiotapFieldTrait::<V>::get_identifiers(field)
            }
            RadiotapField::Vendor(vendor_field) => {
              // Get the field identifiers
              let identifiers = VendorFieldTrait::<V>::get_identifiers(vendor_field);

              // Encode the field
              VendorFieldTrait::<V>::encode(vendor_field, &mut writer)?;

              return Ok::<TlvFieldItem, RadiotapError>(TlvFieldItem::Vendor {
                length: 8 + buffer.len() as u16,
                oui: identifiers.oui,
                sub_namespace: identifiers.sub_namespace,
                presence: identifiers.bit_index,
                value: buffer,
              });
            }
          };

          Ok(TlvFieldItem::Radiotap {
            r#type: identifiers.bit_index,
            length: buffer.len() as u16,
            value: buffer,
          })
        })
        .collect::<Result<Vec<_>, _>>()?;

      // Encode the TLV field
      let encoded_tlv_field = TlvField { items }.to_bytes()?;

      // Calculate the current length
      let length = 4 + (present_bitmasks.len() * 4) + encoded_fields.len();

      // Generate alignment padding for TLV fields (4 bytes)
      let tlv_field_padding = vec![0u8; (4 - (length % 4)) % 4];

      // Add to encoded fields
      encoded_fields.extend(tlv_field_padding);
      encoded_fields.extend(encoded_tlv_field);
    } else {
      // Add the last present bitmask
      present_bitmasks.push(present_bitmask);
    }

    // Set bit 31 on all but the last present bitmasks
    let last_index = present_bitmasks.len() - 1;
    for present_bitmask in &mut present_bitmasks[..last_index] {
      *present_bitmask |= 0x80000000;
    }

    // Calculate the total length
    let length = 4 + (present_bitmasks.len() * 4) + encoded_fields.len();

    // Encode the sub header
    let encoded_sub_header = SubHeader {
      version: self.version,
      length: length as u16,
      present_bitmasks,
    }
    .to_bytes()?;

    // Combine the encoded sub-header and fields
    let encoded = [encoded_sub_header, encoded_fields].concat();

    // Check the length
    if encoded.len() != length {
      return Err(RadiotapError::UnexpectedHeaderLength);
    }

    Ok(encoded)
  }
}

/// Radiotap header decoding configuration
pub struct RadiotapHeaderDecodingConfig<V> {
  /// Radiotap fields indexed by bit index
  radiotap_fields: HashMap<RadiotapFieldTraitIdentifiers, Box<dyn RadiotapFieldTrait<V>>>,

  /// Vendor fields indexed by OUI, sub-namespace, and bit index
  vendor_fields: HashMap<VendorFieldTraitIdentifiers, Box<dyn VendorFieldTrait<V>>>,
}

impl<V> RadiotapHeaderDecodingConfig<V> {
  /// Reserved bit indices that cannot be used for fields
  const RESERVED_BIT_INDICES: [u16; 3] = [29, 30, 31];

  /// Standard radiotap header decoding configuration (Defined + suggested fields only)
  pub fn standard_config() -> Result<Self, RadiotapError> {
    let mut config = RadiotapHeaderDecodingConfig::default();

    // Add standard radiotap fields
    config.set_radiotap_field(Box::new(TsftField::default()))?;
    config.set_radiotap_field(Box::new(FlagsField::default()))?;
    config.set_radiotap_field(Box::new(RateField::default()))?;
    config.set_radiotap_field(Box::new(ChannelField::default()))?;
    config.set_radiotap_field(Box::new(FhssField::default()))?;
    config.set_radiotap_field(Box::new(DbmAntSignalField::default()))?;
    config.set_radiotap_field(Box::new(DbmAntNoiseField::default()))?;
    config.set_radiotap_field(Box::new(LockQualityField::default()))?;
    config.set_radiotap_field(Box::new(TxAttenuationField::default()))?;
    config.set_radiotap_field(Box::new(DbTxAttenuationField::default()))?;
    config.set_radiotap_field(Box::new(DbmTxPowerField::default()))?;
    config.set_radiotap_field(Box::new(AntennaField::default()))?;
    config.set_radiotap_field(Box::new(DbAntSignalField::default()))?;
    config.set_radiotap_field(Box::new(DbAntNoiseField::default()))?;
    config.set_radiotap_field(Box::new(RxFlagsField::default()))?;
    config.set_radiotap_field(Box::new(TxFlagsField::default()))?;
    config.set_radiotap_field(Box::new(RtsRetriesField::default()))?;
    config.set_radiotap_field(Box::new(DataRetriesField::default()))?;
    config.set_radiotap_field(Box::new(XChannelField::default()))?;
    config.set_radiotap_field(Box::new(McsField::default()))?;
    config.set_radiotap_field(Box::new(AmpduStatusField::default()))?;
    config.set_radiotap_field(Box::new(VhtField::default()))?;
    config.set_radiotap_field(Box::new(TimestampField::default()))?;
    config.set_radiotap_field(Box::new(HeField::default()))?;
    config.set_radiotap_field(Box::new(HeMuField::default()))?;
    config.set_radiotap_field(Box::new(HeMuOtherUserField::default()))?;
    config.set_radiotap_field(Box::new(PsduField::default()))?;
    config.set_radiotap_field(Box::new(LsigField::default()))?;

    Ok(config)
  }

  /// Set a radiotap field
  pub fn set_radiotap_field(
    &mut self,
    field: Box<dyn RadiotapFieldTrait<V>>,
  ) -> Result<(), RadiotapError> {
    // Get the bit index
    let bit_index = field.get_identifiers().bit_index;

    // Validate the bit index
    if Self::RESERVED_BIT_INDICES.contains(&(bit_index % 32)) || bit_index == 28 {
      return Err(RadiotapError::InvalidFieldBitIndex);
    }

    // Add the field
    self.radiotap_fields.insert(field.get_identifiers(), field);

    Ok(())
  }

  /// Set a vendor field
  pub fn set_vendor_field(
    &mut self,
    field: Box<dyn VendorFieldTrait<V>>,
  ) -> Result<(), RadiotapError> {
    // Get the bit index
    let bit_index = field.get_identifiers().bit_index;

    // Validate the bit index
    if Self::RESERVED_BIT_INDICES.contains(&(bit_index % 32)) {
      return Err(RadiotapError::InvalidFieldBitIndex);
    }

    // Add the field
    self.vendor_fields.insert(field.get_identifiers(), field);

    Ok(())
  }
}

impl<V> Default for RadiotapHeaderDecodingConfig<V> {
  /// Empty radiotap header decoding configuration
  fn default() -> Self {
    RadiotapHeaderDecodingConfig {
      radiotap_fields: HashMap::new(),
      vendor_fields: HashMap::new(),
    }
  }
}

#[cfg(test)]
#[path = "header_tests.rs"]
mod tests;
