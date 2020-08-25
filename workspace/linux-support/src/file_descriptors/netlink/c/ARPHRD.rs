// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// 'Device Type'.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(u16)]
pub enum ARPHRD
{
	/// from KA9Q: NET/ROM pseudo.
	#[serde(rename = "NETROM")] ARPHRD_NETROM = 0,
	
	/// Ethernet 10Mbps.
	#[serde(rename = "Ethernet 10Mpbs")] ARPHRD_ETHER = 1,
	
	/// Experimental Ethernet.
	#[serde(rename = "EETHER")] ARPHRD_EETHER = 2,
	
	/// AX.25 Level 2.
	#[serde(rename = "AX.25 level 2")] ARPHRD_AX25 = 3,
	
	/// PROnet token ring.
	#[serde(rename = "PROnet token ring")] ARPHRD_PRONET = 4,
	
	/// Chaosnet.
	#[serde(rename = "Chaosnet")] ARPHRD_CHAOS = 5,
	
	/// IEEE 802.2 Ethernet/TR/TB.
	#[serde(rename = "IEEE 802.2 Ethernet")] ARPHRD_IEEE802 = 6,
	
	/// ARCnet.
	#[serde(rename = "ARCnet")] ARPHRD_ARCNET = 7,
	
	/// APPLEtalk.
	#[serde(rename = "APPLEtalk")] ARPHRD_APPLETLK = 8,
	
	/// Frame Relay DLCI.
	#[serde(rename = "Frame Relay DLCI")] ARPHRD_DLCI = 15,
	
	/// ATM.
	#[serde(rename = "ATM")] ARPHRD_ATM = 19,
	
	/// Metricom STRIP (new IANA id).
	#[serde(rename = "Metricom STRIP")] ARPHRD_METRICOM = 23,
	
	/// IEEE 1394 IPv4 - RFC 2734.
	#[serde(rename = "IEEE 1394 IPv4")] ARPHRD_IEEE1394 = 24,
	
	/// EUI-64.
	#[serde(rename = "EUI-64")] ARPHRD_EUI64 = 27,
	
	/// InfiniBand.
	#[serde(rename = "InfiniBand")] ARPHRD_INFINIBAND = 32,
	
	/// .
	///
	/// Dummy type for non-ARP hardware.
	#[serde(rename = "SLIP")] ARPHRD_SLIP = 256,
	
	/// .
	///
	/// Dummy type for non-ARP hardware.
	#[serde(rename = "CSLIP")] ARPHRD_CSLIP = 257,
	
	/// .
	///
	/// Dummy type for non-ARP hardware.
	#[serde(rename = "SLIP6")] ARPHRD_SLIP6 = 258,
	
	/// .
	///
	/// Dummy type for non-ARP hardware.
	#[serde(rename = "CSLIP6")] ARPHRD_CSLIP6 = 259,
	
	/// Notional KISS type.
	///
	/// Dummy type for non-ARP hardware.
	#[serde(rename = "Notional KISS")] ARPHRD_RSRVD = 260,
	
	/// .
	///
	/// Dummy type for non-ARP hardware.
	#[serde(rename = "ADAPT")] ARPHRD_ADAPT = 264,
	
	/// .
	///
	/// Dummy type for non-ARP hardware.
	#[serde(rename = "ROSE")] ARPHRD_ROSE = 270,
	
	/// CCITT X.25.
	///
	/// Dummy type for non-ARP hardware.
	#[serde(rename = "CCITT X.25")] ARPHRD_X25 = 271,
	
	/// Boards with X.25 in firmware.
	///
	/// Dummy type for non-ARP hardware.
	#[serde(rename = "Hardware X.25")] ARPHRD_HWX25 = 272,
	
	/// Controller Area Network.
	///
	/// Dummy type for non-ARP hardware.
	#[serde(rename = "CAN")] ARPHRD_CAN = 280,
	
	/// .
	///
	/// Dummy type for non-ARP hardware.
	#[serde(rename = "PPP")] ARPHRD_PPP = 512,
	
	/// Cisco HDLC.
	///
	/// Dummy type for non-ARP hardware.
	#[serde(rename = "Cisco HDLC")] ARPHRD_CISCO = 513,
	
	/// LAPB.
	///
	/// Dummy type for non-ARP hardware.
	#[serde(rename = "LAPB")] ARPHRD_LAPB = 516,
	
	/// Digital's DDCMP protocol.
	#[serde(rename = "Digital DDCMP")] ARPHRD_DDCMP = 517,
	
	/// Raw HDLC.
	///
	/// Dummy type for non-ARP hardware.
	#[serde(rename = "Raw HDLC")] ARPHRD_RAWHDLC = 518,
	
	/// Raw IP.
	///
	/// Dummy type for non-ARP hardware.
	#[serde(rename = "Raw IP")] ARPHRD_RAWIP = 519,
	
	/// IPIP tunnel.
	///
	/// Dummy type for non-ARP hardware.
	#[serde(rename = "IPIP Tunnel")] ARPHRD_TUNNEL = 768,
	
	/// IP6IP6 tunnel.
	///
	/// Dummy type for non-ARP hardware.
	#[serde(rename = "IP6IP6 Tunnel")] ARPHRD_TUNNEL6 = 769,
	
	/// Frame Relay Access Device.
	///
	/// Dummy type for non-ARP hardware.
	#[serde(rename = "Frame Relay Access Device")] ARPHRD_FRAD = 770,
	
	/// SKIP vif.
	///
	/// Dummy type for non-ARP hardware.
	#[serde(rename = "SKIP vif")] ARPHRD_SKIP = 771,
	
	/// Loopback device.
	///
	/// Dummy type for non-ARP hardware.
	#[serde(rename = "Loopback device")] ARPHRD_LOOPBACK = 772,
	
	/// Localtalk device.
	///
	/// Dummy type for non-ARP hardware.
	#[serde(rename = "Localtalk")] ARPHRD_LOCALTLK = 773,
	
	/// Fiber Distributed Data Interface.
	///
	/// Dummy type for non-ARP hardware.
	#[serde(rename = "Fiber Distributed Data Interface")] ARPHRD_FDDI = 774,
	
	/// AP1000 BIF.
	///
	/// Dummy type for non-ARP hardware.
	#[serde(rename = "AP1000 BIF")] ARPHRD_BIF = 775,
	
	/// sit0 device - IPv6-in-IPv4.
	///
	/// Dummy type for non-ARP hardware.
	#[serde(rename = "SIT")] ARPHRD_SIT = 776,
	
	/// IP over DDP tunneller.
	///
	/// Dummy type for non-ARP hardware.
	#[serde(rename = "IP over DDP tunneller")] ARPHRD_IPDDP = 777,
	
	/// GRE over IP.
	///
	/// Dummy type for non-ARP hardware.
	#[serde(rename = "GRE over IP")] ARPHRD_IPGRE = 778,
	
	/// PIMSM register interface.
	///
	/// Dummy type for non-ARP hardware.
	#[serde(rename = "PIMSM register interface")] ARPHRD_PIMREG = 779,
	
	/// High Performance Parallel Interface.
	///
	/// Dummy type for non-ARP hardware.
	#[serde(rename = "High Performance Parallel Interface")] ARPHRD_HIPPI = 780,
	
	/// Nexus 64Mbps Ash.
	///
	/// Dummy type for non-ARP hardware.
	#[serde(rename = "Nexus 64Mbps Ash")] ARPHRD_ASH = 781,
	
	/// Acorn Econet.
	///
	/// Dummy type for non-ARP hardware.
	#[serde(rename = "Acorn Econet")] ARPHRD_ECONET = 782,
	
	/// Linux-IrDA.
	///
	/// Dummy type for non-ARP hardware.
	#[serde(rename = "Linux-IrDA")] ARPHRD_IRDA = 783,
	
	/// Point to point FibreChannel.
	///
	/// ARP works differently on different FibreChannel media.
	///
	/// Dummy type for non-ARP hardware.
	#[serde(rename = "Point to point FibreChannel")] ARPHRD_FCPP = 784,
	
	/// FibreChannel arbitrated loop.
	///
	/// ARP works differently on different FibreChannel media.
	///
	/// Dummy type for non-ARP hardware.
	#[serde(rename = "FibreChannel arbitrated loop")] ARPHRD_FCAL = 785,
	
	/// FibreChannel public loop.
	///
	/// ARP works differently on different FibreChannel media.
	///
	/// Dummy type for non-ARP hardware.
	#[serde(rename = "FibreChannel public loop")] ARPHRD_FCPL = 786,
	
	/// FibreChannel fabric.
	///
	/// ARP works differently on different FibreChannel media.
	///
	/// Dummy type for non-ARP hardware.
	#[serde(rename = "FibreChannel fabric")] ARPHRD_FCFABRIC = 787,
	
	// 788 ..= 799 are reserved for FibreChannel media types.
	
	/// Magic type ident for TR.
	///
	/// Dummy type for non-ARP hardware.
	#[serde(rename = "IEEE 802 TR")] ARPHRD_IEEE802_TR = 800,
	
	/// IEEE 802.11.
	///
	/// Dummy type for non-ARP hardware.
	#[serde(rename = "IEEE 802.11")] ARPHRD_IEEE80211 = 801,
	
	/// IEEE 802.11 + Prism2 header.
	///
	/// Dummy type for non-ARP hardware.
	#[serde(rename = "IEEE 802.11 + Prism2 header")] ARPHRD_IEEE80211_PRISM = 802,
	
	/// IEEE 802.11 + radiotap header.
	///
	/// Dummy type for non-ARP hardware.
	#[serde(rename = "IEEE 802.11 + radiotap header")] ARPHRD_IEEE80211_RADIOTAP = 803,
	
	/// .
	///
	/// Dummy type for non-ARP hardware.
	#[serde(rename = "IEEE 802.15.4")] ARPHRD_IEEE802154 = 804,
	
	/// IEEE 802.15.4 network monitor.
	///
	/// Dummy type for non-ARP hardware.
	#[serde(rename = "IEEE 802.15.4 network monitor")] ARPHRD_IEEE802154_MONITOR = 805,
	
	/// PhoNet media type.
	///
	/// Dummy type for non-ARP hardware.
	#[serde(rename = "PhoNet")] ARPHRD_PHONET = 820,
	
	/// PhoNet pipe header.
	///
	/// Dummy type for non-ARP hardware.
	#[serde(rename = "PhoNet pipe header")] ARPHRD_PHONET_PIPE = 821,
	
	/// CAIF media type.
	///
	/// Dummy type for non-ARP hardware.
	#[serde(rename = "CAIF media type")] ARPHRD_CAIF = 822,
	
	/// GRE over IPv6.
	///
	/// Dummy type for non-ARP hardware.
	#[serde(rename = "GRE over IPv6")] ARPHRD_IP6GRE = 823,
	
	/// Netlink header.
	///
	/// Dummy type for non-ARP hardware.
	#[serde(rename = "Netlink header")] ARPHRD_NETLINK = 824,
	
	/// IPv6 over LoWPAN.
	///
	/// Dummy type for non-ARP hardware.
	#[serde(rename = "IPv6 over LoWPAN")] ARPHRD_6LOWPAN = 825,
	
	/// Vsock monitor header.
	///
	/// Dummy type for non-ARP hardware.
	#[serde(rename = "Vsock monitor header")] ARPHRD_VSOCKMON = 826,
	
	/// Zero header length.
	///
	/// Special type.
	#[serde(rename = "Zero header length")] ARPHRD_NONE = 0xFFFE,
	
	/// Void type, nothing is known.
	///
	/// Special type.
	#[serde(rename = "Void")] ARPHRD_VOID = 0xFFFF,
}

impl ARPHRD
{
	/// .
	///
	/// Dummy type for non-ARP hardware.
	pub const ARPHRD_HDLC: Self = ARPHRD::ARPHRD_CISCO;
}
