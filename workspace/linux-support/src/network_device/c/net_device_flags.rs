// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support; including this file; may be copied; modified; propagated; or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// Network device flags.
	///
	/// These can be set by drivers, the kernel and some can be triggered by userspace.
	/// Userspace can query and set these flags using userspace utilities but there is also a sysfs entry available for all dev flags which can be queried and set.
	/// These flags are shared for all types of net_devices.
	/// The sysfs entries are available via `/sys/class/net/<dev>/flags`.
	/// Flags which can be toggled through sysfs are annotated below, note that only a few flags can be toggled and some other flags are always preserved from the original flags even if you try to set them via sysfs.
	/// Flags which are always preserved are kept under the flag grouping `IFF_VOLATILE`.
	/// Flags which are volatile are annotated below as such.
	///
	/// Notes:-
	/// * The flags `IFF_LOOPBACK`, `IFF_BROADCAST` and `IFF_POINTOPOINT` can not be changed by userspace as they describe link media characteristics.
	/// * If a link does not have any of the flags `IFF_LOOPBACK`, `IFF_BROADCAST` and `IFF_POINTOPOINT` set then it is a non-broadcast multiple-access (NBMA) network device.
	/// * If a link has any of the  flags `IFF_LOOPBACK`, `IFF_BROADCAST` and `IFF_POINTOPOINT` set then it supports multicasts even if `IFF_MULTICAST` is not set.
	/// * Combination `IFF_BROADCAST | IFF_POINTOPOINT` is invalid.
	#[derive(Deserialize, Serialize)]
	pub struct net_device_flags: u32
	{
		/// Interface is up.
		///
		/// Can be toggled through sysfs.
		const UP = IFF_UP;
	
		/// Broadcast address is valid.
		///
		/// Is volatile.
		const BROADCAST = IFF_BROADCAST;
	
		/// Turn on debugging.
		/// 
		/// Can be toggled through sysfs.
		const DEBUG = IFF_DEBUG;
	
		/// Is a loopback net device.
		/// 
		/// Is volatile.
		const LOOPBACK = IFF_LOOPBACK;
	
		/// Interface is a point-to-point (P2P) link.
		///
		/// Is volatile.
		const POINTOPOINT = IFF_POINTOPOINT;
	
		/// Avoid use of trailers.
		///
		/// Can be toggled through sysfs.
		/// Is volatile.
		const NOTRAILERS = IFF_NOTRAILERS;
	
		/// Interface has RFC 2863 operational status `OPER_UP`.
		///
		/// Is volatile.
		const RUNNING = IFF_RUNNING;
	
		/// No ARP protocol.
		///
		/// Can be toggled through sysfs.
		/// Is volatile.
		const NOARP = IFF_NOARP;
	
		/// Receive all packets.
		///
		/// Can be toggled through sysfs.
		const PROMISC = IFF_PROMISC;
	
		/// Receive all multicast packets.
		///
		/// Can be toggled through sysfs.
		const ALLMULTI = IFF_ALLMULTI;
	
		/// Master of a load balancer.
		///
		/// Is volatile.
		const MASTER = IFF_MASTER;
	
		/// Slave of a load balancer.
		///
		/// Is volatile.
		const SLAVE = IFF_SLAVE;
	
		/// Means that this media uses special encapsulation for multicast frames.
		///
		/// Apparently, all `IFF_POINTOPOINT` and `IFF_BROADCAST` devices are able to use multicast too.
		///
		/// Can be toggled through sysfs.
		const MULTICAST = IFF_MULTICAST;
	
		/// Can set media type.
		///
		/// Can be toggled through sysfs.
		const PORTSEL = IFF_PORTSEL;
	
		/// Auto media select active.
		///
		/// Can be toggled through sysfs.
		const AUTOMEDIA = IFF_AUTOMEDIA;
	
		/// Dial-up device with changing addresses.
		///
		/// Can be toggled through sysfs.
		const DYNAMIC = IFF_DYNAMIC;
	
		/// Driver signals L1 up.
		///
		/// Is volatile.
		const LOWER_UP = IFF_LOWER_UP;
	
		/// Driver signals dormant.
		///
		/// Is volatile.
		const DORMANT = IFF_DORMANT;
	
		/// Echo sent packets.
		///
		/// Is volatile.
		const ECHO = IFF_ECHO;
		
		/// Volatile (combination of flags).
		const VOLATILE = IFF_VOLATILE;
	}
}

impl From<SettableLinkFlags> for net_device_flags
{
	#[inline(alwaus)]
	fn from(value: SettableLinkFlags) -> Self
	{
		net_device_flags::frrom_bits_truncate(value.bits() as u32)
	}
}

impl net_device_flags
{
	#[inline(always)]
	pub(crate) fn mask(&self) -> u32
	{
		!self.bits
	}
}
