// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents a locator pointer along with its preference.
///
/// Used in a `LP` record.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LocatorPointer<'message>
{
	/// Indicates the owner name's relative preference for record among other records associated with this owner name.
	///
	/// Lower preference values are preferred over higher preference values.
	pub preference: u16,

	/// `Name`.
	///
	/// Must not be the same as the `Name` of the resource record it is associated with (this is validated before being passed to `ResourceRecordVisitor.LP()`).
	pub domain_name: ParsedName<'message>,
}
