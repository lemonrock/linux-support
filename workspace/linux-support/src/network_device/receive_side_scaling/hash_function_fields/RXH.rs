// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// RXH data field.
	#[allow(missing_docs)]
	#[derive(Deserialize, Serialize)]
	#[serde(deny_unknown_fields)]
	struct RXH: u64
	{
		/// Ethtool calls this `m` and `L2DA`.
		const EthernetDestinationAddress = RXH_L2DA;
		
		/// Ethtool calls this `v` and `VLAN tag`.
		const EthernetVirtualLocalAreaNetworkTag = RXH_VLAN;
		
		/// eg Transmission Control Protocol (TCP), User Datagram Protocol (UDP) or Stream Control Transmission Protocol (SCTP).
		///
		/// Ethtool calls this `t` and `L3 proto`.
		const Layer3ProtocolNumber = RXH_L3_PROTO;
		
		/// Ethtool calls this `s` and `IP SA`.
		const InternetProtocolVersion4OrInternetProtocolVersion6SourceAddress = RXH_IP_SRC;
		
		/// Ethtool calls this `d` and `IP DA`.
		const InternetProtocolVersion4OrInternetProtocolVersion6DestinationAddress = RXH_IP_DST;
		
		/// Also known as source port for Transmission Control Protocol (TCP), User Datagram Protocol (UDP) and Stream Control Transmission Protocol (SCTP).
		///
		/// First two bytes of four for IPSec Authentication Header or Encapsulating Security Payload's Security Parameter Index (SPI).
		///
		/// Ethtool calls this `f` and `L4 bytes 0 & 1 [TCP/UDP src port]`.
		const FirstTwoBytesOfLayer4Header = RXH_L4_B_0_1;
		
		/// Also known as destination port for Transmission Control Protocol (TCP), User Datagram Protocol (UDP) and Stream Control Transmission Protocol (SCTP).
		///
		/// Last two bytes of four for IPSec Authentication Header or Encapsulating Security Payload's Security Parameter Index (SPI).
		///
		/// Ethtool calls this `n` and `L4 bytes 2 & 3 [TCP/UDP dst port]`.
		const NextTwoBytesOfLayer4Header = RXH_L4_B_2_3;
		
		/// Discard action.
		///
		/// Ethtool calls this `r`.
		const Discard = RXH_DISCARD;
	}
}
