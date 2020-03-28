// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A structure that can be stored in a bit set.
pub trait BitSetAware: Sized + Into<u16> + TryFrom<u16, Error=BitSetAwareTryFromU16Error>
{
	#[doc(hidden)]
	const LinuxMaximum: u16;

	/// Minimum.
	const InclusiveMinimum: Self;

	/// Maixmum.
	const InclusiveMaximum: Self;

	/// Converts item into set of item.
	#[inline(always)]
	fn into_bit_set(self) -> BitSet<Self>
	{
		let mut new = BitSet::empty();
		new.add(self);
		new
	}

	#[doc(hidden)]
	fn hydrate(value: u16) -> Self;
}
