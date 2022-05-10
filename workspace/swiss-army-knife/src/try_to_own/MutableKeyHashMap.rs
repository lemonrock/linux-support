// This file is part of swiss-army-knife. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/swiss-army-knife/master/COPYRIGHT. No part of swiss-army-knife, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of swiss-army-knife. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/swiss-army-knife/master/COPYRIGHT.


/// A HashMap that needs to support `TryToOwn` for keys.
pub type MutableKeyHashMap<K, V> = HashMap<MutableKey<K>, V>;

impl<K: TryToOwnInPlace, V: TryToOwnInPlace> TryToOwnInPlace for MutableKeyHashMap<K, V>
{
	#[inline(always)]
	fn try_to_own_in_place(&mut self) -> Result<(), TryReserveError>
	{
		for (key, value) in self.iter_mut()
		{
			key.try_to_own_in_place()?;
			value.try_to_own_in_place()?;
		}
		Ok(())
	}
}
