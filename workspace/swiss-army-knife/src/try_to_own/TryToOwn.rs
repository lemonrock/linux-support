// This file is part of swiss-army-knife. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/swiss-army-knife/master/COPYRIGHT. No part of swiss-army-knife, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of swiss-army-knife. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/swiss-army-knife/master/COPYRIGHT.


/// Try to own a Cow-backed object.
pub trait TryToOwn: TryToOwnInPlace
{
	/// Instance of `Self` with a `'static` lifetime.
	///
	/// Rust does not provide a way to express this restriction.
	type TryToOwned: 'static + TryToOwn;
	
	/// Try to own a Cow-backed object that might require allocation that could fail.
	fn try_to_own(self) -> Result<Self::TryToOwned, TryReserveError>;
}
