// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Queue identifier.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct ExpressDataPathQueueIdentifier(u32);

impl From<QueueIdentifier> for ExpressDataPathQueueIdentifier
{
	#[inline(always)]
	fn from(value: QueueIdentifier) -> Self
	{
		Self::from_queue_identifier(value)
	}
}

impl ExpressDataPathQueueIdentifier
{
	#[inline(always)]
	pub(crate) const fn from_queue_identifier(queue_identifier: QueueIdentifier) -> Self
	{
		Self(queue_identifier.0 as u32)
	}
}
