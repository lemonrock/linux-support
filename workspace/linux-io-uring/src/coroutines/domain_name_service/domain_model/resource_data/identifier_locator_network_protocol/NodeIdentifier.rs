// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents a `NodeID`.
///
/// Used in a `NID` record.
///
/// Node identifiers have associated `L32`, `L64` and `LP` records; these are a bit like `A` and `AAAA` records.
///
/// Complies with the syntactic rules of Internet Protocol version 6 interface identifiers (RFC 4291, Section 2.5.1), but has slightly different semantics.
/// Unlike Internet Protocol version 6 interface identifiers, which are bound to a specific *interface* of a specific node, node identifier values are bound to a specific *node*, and they **may** be used with *any interface* of that node.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct NodeIdentifier(pub u64);

impl Display for NodeIdentifier
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "{:04x}:", (self.0 >> 48) as u16)?;
		write!(f, "{:04x}:", ((self.0 & 0x0000_FFFF_0000_0000) >> 32) as u16)?;
		write!(f, "{:04x}:", ((self.0 & 0x0000_0000_FFFF_0000) >> 16) as u16)?;
		write!(f, "{:04x}", (self.0 & 0x0000_0000_0000_FFFF) as u16)?;
		Ok(())
	}
}
