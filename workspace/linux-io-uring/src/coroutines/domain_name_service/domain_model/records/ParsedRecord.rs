// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A parsed record.
pub(crate) trait ParsedRecord: Sized + Debug
{
	/// Any Order, Priority or Weight (as a tuple).
	type OrderPriorityAndWeight: Sized;
	
	type OwnedRecord: OwnedRecord;
	
	#[inline(always)]
	fn into_owned_record(self) -> Self::OwnedRecord
	{
		self
	}
}

impl ParsedRecord for Ipv4Addr
{
	type OrderPriorityAndWeight = ();
	
	type OwnedRecord = Self;
	
	#[inline(always)]
	fn into_owned_record(self) -> Self::OwnedRecord
	{
		self
	}
}

impl ParsedRecord for Ipv6Addr
{
	type OrderPriorityAndWeight = ();
	
	type OwnedRecord = Self;
	
	#[inline(always)]
	fn into_owned_record(self) -> Self::OwnedRecord
	{
		self
	}
}
