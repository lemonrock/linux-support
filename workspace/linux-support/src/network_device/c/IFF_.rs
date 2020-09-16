// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Interface is up.
///
/// Can be toggled through sysfs.
pub(crate) const IFF_UP: u32 = 1 << 0;

/// Broadcast address is valid.
///
/// Is volatile.
pub(crate) const IFF_BROADCAST: u32 = 1 << 1;

/// Turn on debugging.
/// 
/// Can be toggled through sysfs.
pub(crate) const IFF_DEBUG: u32 = 1 << 2;

/// Is a loopback net device.
/// 
/// Is volatile.
pub(crate) const IFF_LOOPBACK: u32 = 1 << 3;

/// Interface is a point-to-point (P2P) link.
///
/// Is volatile.
pub(crate) const IFF_POINTOPOINT: u32 = 1 << 4;

/// Avoid use of trailers.
///
/// Can be toggled through sysfs.
/// Is volatile.
pub(crate) const IFF_NOTRAILERS: u32 = 1 << 5;

/// Interface has RFC 2863 operational status `OPER_UP`.
///
/// Is volatile.
pub(crate) const IFF_RUNNING: u32 = 1 << 6;

/// No ARP protocol.
///
/// Can be toggled through sysfs.
/// Is volatile.
pub(crate) const IFF_NOARP: u32 = 1 << 7;

/// Receive all packets.
///
/// Can be toggled through sysfs.
pub(crate) const IFF_PROMISC: u32 = 1 << 8;

/// Receive all multicast packets.
///
/// Can be toggled through sysfs.
pub(crate) const IFF_ALLMULTI: u32 = 1 << 9;

/// Master of a load balancer.
///
/// Is volatile.
pub(crate) const IFF_MASTER: u32 = 1 << 10;

/// Slave of a load balancer.
///
/// Is volatile.
pub(crate) const IFF_SLAVE: u32 = 1 << 11;

/// Means that this media uses special encapsulation for multicast frames.
///
/// Apparently, all `IFF_POINTOPOINT` and `IFF_BROADCAST` devices are able to use multicast too.
///
/// Can be toggled through sysfs.
pub(crate) const IFF_MULTICAST: u32 = 1 << 12;

/// Can set media type.
///
/// Can be toggled through sysfs.
pub(crate) const IFF_PORTSEL: u32 = 1 << 13;

/// Auto media select active.
///
/// Can be toggled through sysfs.
pub(crate) const IFF_AUTOMEDIA: u32 = 1 << 14;

/// Dial-up device with changing addresses.
///
/// Can be toggled through sysfs.
pub(crate) const IFF_DYNAMIC: u32 = 1 << 15;

/// Driver signals L1 up.
///
/// Is volatile.
pub(crate) const IFF_LOWER_UP: u32 = 1 << 16;

/// Driver signals dormant.
///
/// Is volatile.
pub(crate) const IFF_DORMANT: u32 = 1 << 17;

/// Echo sent packets.
///
/// Is volatile.
pub(crate) const IFF_ECHO: u32 = 1 << 18;

/// Volatile (combination of flags).
pub(crate) const IFF_VOLATILE: u32 = IFF_LOOPBACK | IFF_POINTOPOINT | IFF_BROADCAST | IFF_ECHO | IFF_MASTER | IFF_SLAVE | IFF_RUNNING | IFF_LOWER_UP | IFF_DORMANT;
