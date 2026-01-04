# radiotap-rs

[![continuous integration](https://github.com/Wakeful-Cloud/radiotap-rs/actions/workflows/continuous-integration.yml/badge.svg)](https://github.com/Wakeful-Cloud/radiotap-rs/actions/workflows/continuous-integration.yml)
[![continuous deployment](https://github.com/Wakeful-Cloud/radiotap-rs/actions/workflows/continuous-deployment.yml/badge.svg)](https://github.com/Wakeful-Cloud/radiotap-rs/actions/workflows/continuous-deployment.yml)

`no_std` compatible [radiotap](https://www.radiotap.org) header encoder and decoder for Rust.
Supports repeated fields, vendor-specific fields (both vendor-specific namespaces and vendor-
specific TLV fields), and most standard radiotap fields.

## Examples

See the [`examples`](examples) directory for and [`header_tests.rs`](src/header_tests.rs) for
usage examples.

## Documentation

### Supported Fields

The following fields are currently supported:

| Bit Index | Field Name                                                                      |
| --------- | ------------------------------------------------------------------------------- |
| `0`       | [TSFT](https://www.radiotap.org/fields/TSFT.html)                               |
| `1`       | [Flags](https://www.radiotap.org/fields/Flags.html)                             |
| `2`       | [Rate](https://www.radiotap.org/fields/Rate.html)                               |
| `3`       | [Channel](https://www.radiotap.org/fields/Channel.html)                         |
| `4`       | [FHSS](https://www.radiotap.org/fields/FHSS.html)                               |
| `5`       | [Antenna Signal](https://www.radiotap.org/fields/Antenna%20signal.html)         |
| `6`       | [Antenna Noise](https://www.radiotap.org/fields/Antenna%20noise.html)           |
| `7`       | [Lock Quality](https://www.radiotap.org/fields/Lock%20quality.html)             |
| `8`       | [TX Attenuation](https://www.radiotap.org/fields/TX%20attenuation.html)         |
| `9`       | [db TX Attenuation](https://www.radiotap.org/fields/dB%20TX%20attenuation.html) |
| `10`      | [dbm TX Power](https://www.radiotap.org/fields/dBm%20TX%20power.html)           |
| `11`      | [Antenna](https://www.radiotap.org/fields/Antenna.html)                         |
| `12`      | [dB Antenna Signal](https://www.radiotap.org/fields/dB%20antenna%20signal.html) |
| `13`      | [dB Antenna Noise](https://www.radiotap.org/fields/dB%20antenna%20noise.html)   |
| `14`      | [RX Flags](https://www.radiotap.org/fields/RX%20flags.html)                     |
| `14`      | [FCS in Header](https://www.radiotap.org/fields/FCS%20in%20header.html)         |
| `15`      | [TX Flags](https://www.radiotap.org/fields/TX%20flags.html)                     |
| `15`      | [Hardware Queue](https://www.radiotap.org/fields/hardware%20queue.html)         |
| `16`      | [RSSI](https://www.radiotap.org/fields/RSSI.html)                               |
| `16`      | [RTS Retries](https://www.radiotap.org/fields/RTS%20retries.html)               |
| `17`      | [Data Retries](https://www.radiotap.org/fields/data%20retries.html)             |
| `18`      | [XChannel](https://www.radiotap.org/fields/XChannel.html)                       |
| `19`      | [MCS](https://www.radiotap.org/fields/MCS.html)                                 |
| `20`      | [A-MPDU Status](https://www.radiotap.org/fields/A-MPDU%20status.html)           |
| `21`      | [VHT](https://www.radiotap.org/fields/VHT.html)                                 |
| `22`      | [Timestamp](https://www.radiotap.org/fields/timestamp.html)                     |
| `22`      | [Extended Flags](https://www.radiotap.org/fields/extended%20flags.html)         |
| `23`      | [HE](https://www.radiotap.org/fields/HE.html)                                   |
| `24`      | [HE-MU](https://www.radiotap.org/fields/HE-MU.html)                             |
| `25`      | [HE-MU-Other-User](https://www.radiotap.org/fields/HE-MU-other-user.html)       |
| `26`      | [0-Length-PSDU](https://www.radiotap.org/fields/0-length-PSDU.html)             |
| `27`      | [L-SIG](https://www.radiotap.org/fields/L-SIG.html)                             |
| `28`      | [TLV](https://www.radiotap.org/fields/TLV.html)                                 |
| `29`      | [Radiotap Namespace](https://www.radiotap.org/fields/Radiotap%20Namespace.html) |
| `30`      | [Vendor Namespace](https://www.radiotap.org/fields/Vendor%20Namespace.html)     |

Additional, vendor-specific fields are also supported. If you need support for non-vendor-specific
fields not listed above, please feel free to submit a pull request.

### Gotchas

This section contains some important notes for library consumers and contributors alike.

#### This Library

As a consequence of some of the quirks of the radiotap specification (See the next subsection for
details) and various other factors, you should note the following about this library:

1. Fields are represented as a list (rather than a struct) to support repeated fields.
2. Field order matters. Fields are encoded in the order in which they are stored in the list,
   however, the radiotap specification mandates non-Type-Length-Value (TLV) fields be ordered in
   ascending order by their bit index (for fields in the same namespace). Therefore, if you pass
   out-of-order fields (in the same namespace), the encoder will rectify this by splitting the
   fields across multiple namespaces, potentially resulting in a larger-than-necessary header.
   Unless you have a specific reason not to do so, you are encouraged to define fields in ascending
   order by their bit index.
3. Fields with a bit index < `28` in the first radiotap namespace are encoded using the older
   implicit-length encoding for backwards compatibility.
4. Vendor-specific fields are always encoded using TLV encoding. However, vendor-specific fields can
   be decoded from either implicit-length encoding (in vendor-specific namespaces) or TLV encoding
   (in the radiotap namespace).

#### Radiotap Specification

If you aren't familiar with the radiotap specification, it is suggested to read about [Type
Length Value (TLV) fields](https://www.radiotap.org/fields/TLV.html), the [Radiotap Namespace
pseudo-field](https://www.radiotap.org/fields/Radiotap%20Namespace.html), and the [Vendor
Namespace field](https://www.radiotap.org/fields/Vendor%20Namespace.html) in particular. Some
important observations about the radiotap specification itself are:

- Radiotap headers start with a subheader containing the header version, padding, and a sequence
  of bitmasks defining which fields are present immediately following the subheader.
- Every individual presence bitmask contains 3 special bit indices:
  - $n \cdot 32 - 3$: set to switch to the radiotap namespace (starting with the presence
    bitmask immediately following this one). This is a pseudo-field (i.e., it only exists within the
    bitmask and takes up no additional room).
  - $n \cdot 32 - 2$: set to switch to a vendor namespace (starting with the
    presence bitmask immediately following this one). This is a full field, containing the
    vendor Organizationally Unique Identifier (OUI), vendor-specific sub-namespace, and the
    length of the namespace.
  - $n \cdot 32 - 1$: set if another bitmask immediately follows this one. This is a
    pseudo-field (i.e., it only exists within the bitmask and takes up no additional room).
- Fields can be repeated by repeating their respective namespaces. For example, the presence
  bitmasks `[0xA0000001, 0x00000001]` define 2 TSFT fields back-to-back in 2 separate radiotap
  namespaces.
- Fields with a bit index < `28` can either use the older implicit-length encoding or the newer
  Type-Length-Value (TLV) encoding. Additionally, fields with a bit index >= `32` must use TLV
  encoding. TLV encoded fields are nested under the TLV field, which itself uses the older encoding.
- Vendor-specific fields can either use the older implicit-length encoding in vendor-specific
  namespaces or TLV encoding in the radiotap namespace.
- Not all field members are nicely aligned (e.g., the HE field's `data4` station ID member).
- Refer to the IEEE 802.11 specification anywhere the radiotap specification lacks details.

## Prior Art

- Rust
  - [`radiotap`](https://github.com/rossmacarthur/radiotap)
  - [`rtap`](https://github.com/Frostie314159/rtap)
- C/C++
  - [`kismet`](https://github.com/kismetwireless/kismet)
  - [`libpcap`](https://github.com/the-tcpdump-group/libpcap)
  - [`libtins`](https://github.com/mfontanini/libtins)
  - [`libwifi`](https://github.com/libwifi/libwifi)
  - [`linux`](https://github.com/torvalds/linux)
  - [`radiotap-library`](https://github.com/radiotap/radiotap-library)
  - [`wireshark`](https://github.com/wireshark/wireshark)
- Go
  - [`go.pkt`](https://github.com/ghedo/go.pkt)
  - [`gopacket-80211`](https://github.com/dutchcoders/gopacket-80211)
  - [`gopacket`](https://github.com/google/gopacket)
  - [`wanonpcap`](https://github.com/heistp/wanonpcap)
- JavaScript
  - [`node_pcap`](https://github.com/node-pcap/node_pcap)
  - [`radioparse`](https://github.com/arselzer/radioparse)
  - [`wiregasm`](https://github.com/good-tools/wiregasm)
- Python
  - [`dpkt`](https://github.com/kbandla/dpkt)
  - [`itamae`](https://github.com/wraith-wireless/itamae)
  - [`python-radiotap`](https://github.com/radiotap/python-radiotap)
  - [`scapy`](https://github.com/secdev/scapy)
  - [`wltrace`](https://github.com/jhshi/wltrace)
