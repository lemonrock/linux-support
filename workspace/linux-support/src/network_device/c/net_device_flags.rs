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
	/// Flags which can be toggled through sysfs are annotated below, note that only a few flags can be toggled and some other flags are always preserved from the original net_device flags even if you try to set them via sysfs.
	/// Flags which are always preserved are kept under the flag grouping `IFF_VOLATILE`.
	/// Flags which are volatile are annotated below as such.
	///
	/// Notes:-
	/// * The flags `IFF_LOOPBACK`, `IFF_BROADCAST` and `IFF_POINTOPOINT` can not be changed by userspace as they describe link media characteristics.
	/// * If a link does not have any of the flags `IFF_LOOPBACK`, `IFF_BROADCAST` and `IFF_POINTOPOINT` set then it is a non-broadcast multiple-access (NBMA) network device.
	/// * If a link has any of the  flags `IFF_LOOPBACK`, `IFF_BROADCAST` and `IFF_POINTOPOINT` set then it supports multicasts even if `IFF_MULTICAST` is not set.
	/// * Combination `IFF_BROADCAST | IFF_POINTOPOINT` is invalid.
	pub struct net_device_flags: u32
	{
		/// Interface is up.
		///
		/// Can be toggled through sysfs.
		const IFF_UP = 1 << 0;
	
		/// Broadcast address is valid.
		///
		/// Is volatile.
		const IFF_BROADCAST = 1 << 1;
	
		/// Turn on debugging.
		/// 
		/// Can be toggled through sysfs.
		const IFF_DEBUG = 1 << 2;
	
		/// Is a loopback net device.
		/// 
		/// Is volatile.
		const IFF_LOOPBACK = 1 << 3;
	
		/// Interface is a point-to-point (P2P) link.
		///
		/// Is volatile.
		const IFF_POINTOPOINT = 1 << 4;
	
		/// Avoid use of trailers.
		///
		/// Can be toggled through sysfs.
		/// Is volatile.
		const IFF_NOTRAILERS = 1 << 5;
	
		/// Interface has RFC 2863 operational status `OPER_UP`.
		///
		/// Is volatile.
		const IFF_RUNNING = 1 << 6;
	
		/// No ARP protocol.
		///
		/// Can be toggled through sysfs.
		/// Is volatile.
		const IFF_NOARP = 1 << 7;
	
		/// Receive all packets.
		///
		/// Can be toggled through sysfs.
		const IFF_PROMISC = 1 << 8;
	
		/// Receive all multicast packets.
		///
		/// Can be toggled through sysfs.
		const IFF_ALLMULTI = 1 << 9;
	
		/// Master of a load balancer.
		///
		/// Is volatile.
		const IFF_MASTER = 1 << 10;
	
		/// Slave of a load balancer.
		///
		/// Is volatile.
		const IFF_SLAVE = 1 << 11;
	
		/// Means that this media uses special encapsulation for multicast frames.
		///
		/// Apparently, all `IFF_POINTOPOINT` and `IFF_BROADCAST` devices are able to use multicast too.
		///
		/// Can be toggled through sysfs.
		const IFF_MULTICAST = 1 << 12;
	
		/// Can set media type.
		///
		/// Can be toggled through sysfs.
		const IFF_PORTSEL = 1 << 13;
	
		/// Auto media select active.
		///
		/// Can be toggled through sysfs.
		const IFF_AUTOMEDIA = 1 << 14;
	
		/// Dial-up device with changing addresses.
		///
		/// Can be toggled through sysfs.
		const IFF_DYNAMIC = 1 << 15;
	
		/// Driver signals L1 up.
		///
		/// Is volatile.
		const IFF_LOWER_UP = 1 << 16;
	
		/// Driver signals dormant.
		///
		/// Is volatile.
		const IFF_DORMANT = 1 << 17;
	
		/// Echo sent packets.
		///
		/// Is volatile.
		const IFF_ECHO = 1 << 18;
		
		const IFF_VOLATILE = net_device_flags::IFF_LOOPBACK.bits | net_device_flags::IFF_POINTOPOINT.bits | net_device_flags::IFF_BROADCAST.bits | net_device_flags::IFF_ECHO.bits | net_device_flags::IFF_MASTER.bits | net_device_flags::IFF_SLAVE.bits | net_device_flags::IFF_RUNNING.bits | net_device_flags::IFF_LOWER_UP.bits | net_device_flags::IFF_DORMANT.bits;
	}
}
