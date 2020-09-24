// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Queue identifier.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct QueueIdentifier(pub u32);

impl QueueIdentifier
{
	/// Minimum.
	pub const InclusiveMinimum: Self = QueueCount::InclusiveMinimum.to_queue_identifier();
	
	/// Maximum.
	pub const InclusiveMaximum: Self = QueueCount::InclusiveMaximum.to_queue_identifier();
}
