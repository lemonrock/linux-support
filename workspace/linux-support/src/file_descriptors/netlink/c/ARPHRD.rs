// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// 'Device Type'.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u16)]
pub enum ARPHRD
{
	/* ARP protocol HARDWARE identifiers. */
	/// from KA9Q: NET/ROM pseudo.
	ARPHRD_NETROM = 0,
	
	/// Ethernet 10Mbps.
	ARPHRD_ETHER = 1,
	
	/// Experimental Ethernet.
	ARPHRD_EETHER = 2,
	
	/// AX.25 Level 2.
	ARPHRD_AX25 = 3,
	
	/// PROnet token ring.
	ARPHRD_PRONET = 4,
	
	/// Chaosnet.
	ARPHRD_CHAOS = 5,
	
	/// IEEE 802.2 Ethernet/TR/TB.
	ARPHRD_IEEE802 = 6,
	
	/// ARCnet.
	ARPHRD_ARCNET = 7,
	
	/// APPLEtalk.
	ARPHRD_APPLETLK = 8,
	
	/// Frame Relay DLCI.
	ARPHRD_DLCI = 15,
	
	/// ATM .
	ARPHRD_ATM = 19,
	
	/// Metricom STRIP (new IANA id).
	ARPHRD_METRICOM = 23,
	
	/// IEEE 1394 IPv4 - RFC 2734.
	ARPHRD_IEEE1394 = 24,
	
	/// EUI-64.
	ARPHRD_EUI64 = 27,
	
	/// InfiniBand.
	ARPHRD_INFINIBAND = 32,
	
	/// .
	///
	/// Dummy type for non-ARP hardware.
	ARPHRD_SLIP = 256,
	
	/// .
	///
	/// Dummy type for non-ARP hardware.
	ARPHRD_CSLIP = 257,
	
	/// .
	///
	/// Dummy type for non-ARP hardware.
	ARPHRD_SLIP6 = 258,
	
	/// .
	///
	/// Dummy type for non-ARP hardware.
	ARPHRD_CSLIP6 = 259,
	
	/// Notional KISS type.
	///
	/// Dummy type for non-ARP hardware.
	ARPHRD_RSRVD = 260,
	
	/// .
	///
	/// Dummy type for non-ARP hardware.
	ARPHRD_ADAPT = 264,
	
	/// .
	///
	/// Dummy type for non-ARP hardware.
	ARPHRD_ROSE = 270,
	
	/// CCITT X.25.
	///
	/// Dummy type for non-ARP hardware.
	ARPHRD_X25 = 271,
	
	/// Boards with X.25 in firmware.
	///
	/// Dummy type for non-ARP hardware.
	ARPHRD_HWX25 = 272,
	
	/// Controller Area Network.
	///
	/// Dummy type for non-ARP hardware.
	ARPHRD_CAN = 280,
	
	/// .
	///
	/// Dummy type for non-ARP hardware.
	ARPHRD_PPP = 512,
	
	/// Cisco HDLC.
	///
	/// Dummy type for non-ARP hardware.
	ARPHRD_CISCO = 513,
	
	/// LAPB.
	///
	/// Dummy type for non-ARP hardware.
	ARPHRD_LAPB = 516,
	
	/// Digital's DDCMP protocol.
	ARPHRD_DDCMP = 517,
	
	/// Raw HDLC.
	///
	/// Dummy type for non-ARP hardware.
	ARPHRD_RAWHDLC = 518,
	
	/// Raw IP.
	///
	/// Dummy type for non-ARP hardware.
	ARPHRD_RAWIP = 519,
	
	/// IPIP tunnel.
	///
	/// Dummy type for non-ARP hardware.
	ARPHRD_TUNNEL = 768,
	
	/// IP6IP6 tunnel.
	///
	/// Dummy type for non-ARP hardware.
	ARPHRD_TUNNEL6 = 769,
	
	/// Frame Relay Access Device.
	///
	/// Dummy type for non-ARP hardware.
	ARPHRD_FRAD = 770,
	
	/// SKIP vif.
	///
	/// Dummy type for non-ARP hardware.
	ARPHRD_SKIP = 771,
	
	/// Loopback device.
	///
	/// Dummy type for non-ARP hardware.
	ARPHRD_LOOPBACK = 772,
	
	/// Localtalk device.
	///
	/// Dummy type for non-ARP hardware.
	ARPHRD_LOCALTLK = 773,
	
	/// Fiber Distributed Data Interface.
	///
	/// Dummy type for non-ARP hardware.
	ARPHRD_FDDI = 774,
	
	/// AP1000 BIF.
	///
	/// Dummy type for non-ARP hardware.
	ARPHRD_BIF = 775,
	
	/// sit0 device - IPv6-in-IPv4.
	///
	/// Dummy type for non-ARP hardware.
	ARPHRD_SIT = 776,
	
	/// IP over DDP tunneller.
	///
	/// Dummy type for non-ARP hardware.
	ARPHRD_IPDDP = 777,
	
	/// GRE over IP.
	///
	/// Dummy type for non-ARP hardware.
	ARPHRD_IPGRE = 778,
	
	/// PIMSM register interface.
	///
	/// Dummy type for non-ARP hardware.
	ARPHRD_PIMREG = 779,
	
	/// High Performance Parallel Interface.
	///
	/// Dummy type for non-ARP hardware.
	ARPHRD_HIPPI = 780,
	
	/// Nexus 64Mbps Ash.
	///
	/// Dummy type for non-ARP hardware.
	ARPHRD_ASH = 781,
	
	/// Acorn Econet.
	///
	/// Dummy type for non-ARP hardware.
	ARPHRD_ECONET = 782,
	
	/// Linux-IrDA.
	///
	/// Dummy type for non-ARP hardware.
	ARPHRD_IRDA = 783,
	
	/// Point to point fibrechannel.
	///
	/// ARP works differently on different FibreChannel media.
	///
	/// Dummy type for non-ARP hardware.
	ARPHRD_FCPP = 784,
	
	/// Fibrechannel arbitrated loop.
	///
	/// ARP works differently on different FibreChannel media.
	///
	/// Dummy type for non-ARP hardware.
	ARPHRD_FCAL = 785,
	
	/// Fibrechannel public loop.
	///
	/// ARP works differently on different FibreChannel media.
	///
	/// Dummy type for non-ARP hardware.
	ARPHRD_FCPL = 786,
	
	/// Fibrechannel fabric.
	///
	/// ARP works differently on different FibreChannel media.
	///
	/// Dummy type for non-ARP hardware.
	ARPHRD_FCFABRIC = 787,
	
	// 788 ..= 799 are reserved for fibrechannel media types.
	
	/// Magic type ident for TR.
	///
	/// Dummy type for non-ARP hardware.
	ARPHRD_IEEE802_TR = 800,
	
	/// IEEE 802.11.
	///
	/// Dummy type for non-ARP hardware.
	ARPHRD_IEEE80211 = 801,
	
	/// IEEE 802.11 + Prism2 header.
	///
	/// Dummy type for non-ARP hardware.
	ARPHRD_IEEE80211_PRISM = 802,
	
	/// IEEE 802.11 + radiotap header.
	///
	/// Dummy type for non-ARP hardware.
	ARPHRD_IEEE80211_RADIOTAP = 803,
	
	/// .
	///
	/// Dummy type for non-ARP hardware.
	ARPHRD_IEEE802154 = 804,
	
	/// IEEE 802.15.4 network monitor.
	///
	/// Dummy type for non-ARP hardware.
	ARPHRD_IEEE802154_MONITOR = 805,
	
	/// PhoNet media type.
	///
	/// Dummy type for non-ARP hardware.
	ARPHRD_PHONET = 820,
	
	/// PhoNet pipe header.
	///
	/// Dummy type for non-ARP hardware.
	ARPHRD_PHONET_PIPE = 821,
	
	/// CAIF media type.
	///
	/// Dummy type for non-ARP hardware.
	ARPHRD_CAIF = 822,
	
	/// GRE over IPv6.
	///
	/// Dummy type for non-ARP hardware.
	ARPHRD_IP6GRE = 823,
	
	/// Netlink header.
	///
	/// Dummy type for non-ARP hardware.
	ARPHRD_NETLINK = 824,
	
	/// IPv6 over LoWPAN.
	///
	/// Dummy type for non-ARP hardware.
	ARPHRD_6LOWPAN = 825,
	
	/// Vsock monitor header.
	///
	/// Dummy type for non-ARP hardware.
	ARPHRD_VSOCKMON = 826,
	
	/// Zero header length.
	///
	/// Special type.
	ARPHRD_NONE = 0xFFFE,
	
	/// Void type, nothing is known.
	///
	/// Special type.
	ARPHRD_VOID = 0xFFFF,
}

impl ARPHRD
{
	/// .
	///
	/// Dummy type for non-ARP hardware.
	pub const ARPHRD_HDLC: Self = ARPHRD::ARPHRD_CISCO;
}
