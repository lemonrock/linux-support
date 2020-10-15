// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// See <https://www.iana.org/assignments/dns-parameters/dns-parameters.xhtml#dns-parameters-9>.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum IdentifierType
{
	/// The 1-octet `htype` followed by `hlen` octets of `chaddr` from a DHCPv4 client's `DHCPREQUEST`.
	htype_followed_by_hlen_octets_of_chaddr = 0x0000,

	/// The data octets (ie, the Type and Client-Identifier fields) from a DHCPv4 client's Client Identifier option.
	ClientIdentifier = 0x0001,

	/// The client's DUID (ie, the data octets of a DHCPv6 client's Client Identifier option or the `DUID` field from a DHCPv4 client's Client Identifier option).
	DUID = 0x0002,
}
