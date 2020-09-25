// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// Settable link flags are those that can be specified through ioctl or sysfs.
	#[derive(Deserialize, Serialize)]
	pub struct SettableLinkFlags: u16
	{
		/// Interface is up.
		///
		/// `ip link` calls this `up` and `down` (for the flag not set).
		const Up = IFF_UP as u16;
	
		/// Turn on debugging.
		///
		/// Not settable with `ip link`.
		const Debug = IFF_DEBUG as u16;
	
		/// Avoid use of trailers.
		///
		/// `ip link` calls this `trailers off` and `trailers on` (for the flag not set).
		const NoTraliers = IFF_NOTRAILERS as u16;
	
		/// No ARP protocol.
		///
		/// `ip link` calls this `arp off` and `arp on` (for the flag not set).
		const NoArpProtocol = IFF_NOARP as u16;
		
		/// Promiscuous
		///
		/// `ip link` calls this `promisc on` and `promisc off`.
		const Promiscuous = IFF_PROMISC as u16;
	
		/// Receive all multicast packets.
		///
		/// `ip link` calls this `allmulticast on` and `allmulticast off` (for the flag not set).
		const ReceiveAllMulticast = IFF_ALLMULTI as u16;
	
		/// Means that this media uses special encapsulation for multicast frames.
		///
		/// `ip link` calls this `multicast on` and `multicast off` (for the flag not set).
		const MulticastSpecialEncapsulation = IFF_MULTICAST as u16;
	
		/// Can set port media type.
		///
		/// Not settable with `ip link`.
		const PortSelectionEnabled = IFF_PORTSEL as u16;
	
		/// Auto media select active.
		///
		/// Not settable with `ip link`.
		const AutoMedia = IFF_AUTOMEDIA as u16;
	
		/// Dial-up device with changing addresses.
		///
		/// `ip link` calls this `dynamic on` and `dynamic off` (for the flag not set).
		const DialUpDeviceWithDynamicallyChangingAddresses = IFF_DYNAMIC as u16;
	}
}

impl From<net_device_flags> for SettableLinkFlags
{
	#[inline(always)]
	fn from(value: net_device_flags) -> Self
	{
		SettableLinkFlags::from_bits_truncate(value.bits() as u16)
	}
}

impl SettableLinkFlags
{
	#[inline(always)]
	fn to_net_device_flags_mask(self) -> u32
	{
		let value: net_device_flags = self.into();
		value.mask()
	}
	
	#[inline(always)]
	fn to_net_device_flags_bits(self) -> u32
	{
		let value: net_device_flags = self.into();
		value.bits()
	}
}
