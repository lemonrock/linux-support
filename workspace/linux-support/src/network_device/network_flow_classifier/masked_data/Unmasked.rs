// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Unmasked.
pub trait Unmasked: Default + Copy + Clone + PartialEq + Eq + PartialOrd + Ord + Hash + DeserializeOwned
{
	#[doc(hidden)]
	const UnderlyingZero: Self::Underlying;
	
	#[doc(hidden)]
	type Underlying: Default + Debug + Copy + Clone + PartialEq + Eq + PartialOrd + Ord + Hash + Serialize + DeserializeOwned;
	
	#[doc(hidden)]
	fn into_mask(self) -> Masked<Self>;
	
	#[doc(hidden)]
	fn from_underlying(underlying: Self::Underlying) -> Self;
	
	#[doc(hidden)]
	fn from_underlying_inverted(underlying_inverted: Self::Underlying) -> Self;
	
	#[doc(hidden)]
	fn underlying(&self) -> Self::Underlying;
}
