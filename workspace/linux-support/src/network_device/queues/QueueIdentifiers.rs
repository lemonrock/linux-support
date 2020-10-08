// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// `BitSet` of `QueueIdentifier`.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct QueueIdentifiers(pub BitSet<QueueIdentifier>);

impl Deref for QueueIdentifiers
{
	type Target = BitSet<QueueIdentifier>;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl DerefMut for QueueIdentifiers
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		&mut self.0
	}
}

impl QueueIdentifiers
{
	/// For one queue identifier.
	#[inline(always)]
	pub fn for_one(queue_identifier: QueueIdentifier) -> Self
	{
		Self(BitSet::for_one(queue_identifier))
	}
}
