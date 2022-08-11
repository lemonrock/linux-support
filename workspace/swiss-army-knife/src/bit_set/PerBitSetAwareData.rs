// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Data (`PerBitSetAware`) with an item per BitSetAware, such as a HyperThread, in use by the process.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
pub struct PerBitSetAwareData<BSA: BitSetAware, PerBitSetAware>
{
	data: Box<[Option<PerBitSetAware>]>,
	bit_set: BitSet<BSA>,
	#[serde(skip)] marker: PhantomData<BSA>,
}

impl<BSA: BitSetAware, PerBitSetAware> Deref for PerBitSetAwareData<BSA, PerBitSetAware>
{
	type Target = [Option<PerBitSetAware>];

	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.data
	}
}

impl<BSA: BitSetAware, PerBitSetAware> DerefMut for PerBitSetAwareData<BSA, PerBitSetAware>
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		&mut self.data
	}
}

impl<BSA: BitSetAware, PerBitSetAware> Index<BSA> for PerBitSetAwareData<BSA, PerBitSetAware>
{
	type Output = Option<PerBitSetAware>;

	#[inline(always)]
	fn index(&self, index: BSA) -> &Self::Output
	{
		let i: usize = index.into();
		self.data.index(i)
	}
}

impl<BSA: BitSetAware, PerBitSetAware> IndexMut<BSA> for PerBitSetAwareData<BSA, PerBitSetAware>
{
	#[inline(always)]
	fn index_mut(&mut self, index: BSA) -> &mut Self::Output
	{
		let i: usize = index.into();
		self.data.index_mut(i)
	}
}

impl<BSA: BitSetAware, PerBitSetAware> PerBitSetAwareData<BSA, PerBitSetAware>
{
	/// `constructor` is called for each defined BitSetAware in `bit_set`; it is passed the BitSetAware.
	#[inline(always)]
	pub fn new(bit_set: &BitSet<BSA>, mut constructor: impl FnMut(BSA) -> PerBitSetAware) -> Self
	{
		debug_assert!(!bit_set.is_empty(), "Must be at least one BitSetAware");

		let (capacity, bit_set) =
		{
			let mut bit_set = bit_set.clone();
			bit_set.shrink_to_fit();
			(bit_set.capacity(), bit_set)
		};

		Self
		{
			data:
			{
				let mut data = Vec::with_capacity(capacity);
				for bit_set_aware in bit_set.iterate_including_empty()
				{
					data.push(bit_set_aware.map(|bit_set_aware| constructor(bit_set_aware)))
				}
				data.into_boxed_slice()
			},
			bit_set,
			marker: PhantomData,
		}
	}
	
	/// `constructor` is called for each defined BitSetAware in `bit_set`; it is passed the BitSetAware.
	#[inline(always)]
	pub fn from_iterator<R>(mut constructor: impl Iterator<Item=Result<(BSA, PerBitSetAware), R>>) -> Result<Self, R>
	{
		let capacity = Self::capacity_required(&mut constructor);
		let mut bit_set = BitSet::new_set_length(capacity);
		let mut data = Vec::with_capacity(capacity);
		
		for result in constructor
		{
			let (bit_set_aware, per_bit_set_aware) = result?;
			bit_set.add(bit_set_aware);
			data.push(Some(per_bit_set_aware));
		}
		
		bit_set.shrink_to_fit();
		
		Ok
		(
			Self
			{
				bit_set,
				data: data.into_boxed_slice(),
				marker: PhantomData,
			}
		)
	}
	
	#[inline(always)]
	fn capacity_required(constructor: &mut impl Iterator) -> usize
	{
		let (minimum, maximum) = constructor.size_hint();
		if let Some(maximum) = maximum
		{
			let safety_threshold = BSA::LinuxExclusiveMaximum as usize;
			if maximum > minimum && maximum < safety_threshold
			{
				maximum
			}
			else
			{
				BSA::LinuxExclusiveMaximum as usize
			}
		}
		else
		{
			BSA::LinuxExclusiveMaximum as usize
		}
	}
	
	/// Gets the data for a particular BitSetAware.
	///
	/// If the BitSetAware does not exist (or does not have assigned data), returns None; this can happen on Linux if using the `SO_INCOMING_CPU` socket option, which can map to a CPU not assigned to the process.
	#[inline(always)]
	pub fn get(&self, bit_set_aware: BSA) -> Option<&PerBitSetAware>
	{
		let i: usize = bit_set_aware.into();
		if unlikely!(i >= self.data.len())
		{
			return None
		}
		self.data.get_unchecked_safe(i).as_ref()
	}

	/// Gets the data for a particular BitSetAware.
	#[inline(always)]
	pub fn get_unchecked_safe(&self, bit_set_aware: BSA) -> &PerBitSetAware
	{
		let i: usize = bit_set_aware.into();
		self.data.get_unchecked_safe(i).as_ref().unwrap()
	}

	/// Gets the data for a particular BitSetAware; if no datum for that gets a datum for `default_bit_set_aware`.
	///
	/// If the BitSetAware does not exist (or does not have assigned data), returns None; this can happen on Linux if using the` SO_INCOMING_CPU` socket option with `HyperThreads`, which can return an index for a CPU not assigned to the process.
	///
	/// Does not validate `default_bit_set_aware` has a datum unless compiled as a debug build.
	#[inline(always)]
	pub fn get_or(&self, bit_set_aware: BSA, default_bit_set_aware: BSA) -> (&PerBitSetAware, BSA)
	{
		let i: usize = bit_set_aware.into();
		if unlikely!(i >= self.data.len())
		{
			return self.get_or_default_bit_set_aware(default_bit_set_aware)
		}
		
		if let Some(datum) = self.data.get_unchecked_safe(i).as_ref()
		{
			return (datum, bit_set_aware)
		}
		
		self.get_or_default_bit_set_aware(default_bit_set_aware)
	}
	
	#[inline(always)]
	fn get_or_default_bit_set_aware(&self, default_bit_set_aware: BSA) -> (&PerBitSetAware, BSA)
	{
		let datum = if cfg!(debug_assertions)
		{
			self.get(default_bit_set_aware).unwrap()
		}
		else
		{
			self.get_unchecked_safe(default_bit_set_aware)
		};
		(datum, default_bit_set_aware)
	}

	/// Sets the current value, discarding the old one.
	#[inline(always)]
	pub fn set(&mut self, bit_set_aware: BSA, value: PerBitSetAware)
	{
		let i: usize = bit_set_aware.into();
		self.data[i] = Some(value)
	}

	/// Takes the data for a particular `BitSetAware`.
	///
	/// If the BitSetAware does not exist (or does not have assigned data), returns None; this can happen on Linux if using the SO_INCOMING_CPU socket option with `PerBitSetAwareData<HyperThread>`, which can map to a HyperThread not assigned to the process.
	#[inline(always)]
	pub fn take(&mut self, bit_set_aware: BSA) -> Option<PerBitSetAware>
	{
		let i: usize = bit_set_aware.into();
		self.data[i].take()
	}

	/// Replaces the current value, returning the old one.
	#[inline(always)]
	pub fn replace(&mut self, bit_set_aware: BSA, value: PerBitSetAware) -> Option<PerBitSetAware>
	{
		let i: usize = bit_set_aware.into();
		self.data[i].replace(value)
	}

	/// Iterates over all indices.
	#[inline(always)]
	pub fn bit_set_aware_indices<'a>(&'a self) -> impl Iterator<Item=BSA> + 'a
	{
		self.bit_set.iterate()
	}

	/// Iterates over all entries.
	#[inline(always)]
	pub fn bit_set_aware_items<'a>(&'a self) -> impl Iterator<Item=(BSA, &'a PerBitSetAware)> + 'a
	{
		self.bit_set.iterate().map(move |bit_set_aware|
		{
			let i: usize = bit_set_aware.into();
			let element = self.data.get_unchecked_safe(i);
			(bit_set_aware, element.as_ref().unwrap())
		})
	}

	/// Maps from `PerBitSetAware` to `NewPerBitSetAware`.
	#[inline(always)]
	pub fn map_ref<'a, NewPerBitSetAware: 'a>(&'a self, mut mapper: impl FnMut(BSA, &'a PerBitSetAware) -> NewPerBitSetAware) -> PerBitSetAwareData<BSA, NewPerBitSetAware>
	{
		let mut mapped_data = Vec::with_capacity(self.data.len());

		for bit_set_aware in self.bit_set.iterate_including_empty()
		{
			let new_per_bit_set_aware = match bit_set_aware
			{
				None => None,
				Some(bit_set_aware) =>
				{
					Some(mapper(bit_set_aware, self.get_unchecked_safe(bit_set_aware)))
				}
			};
			mapped_data.push(new_per_bit_set_aware);
		}

		PerBitSetAwareData
		{
			data: mapped_data.into_boxed_slice(),
			bit_set: self.bit_set.clone(),
			marker: PhantomData,
		}
	}

	/// Maps from `PerBitSetAware` to `NewPerBitSetAware`.
	#[inline(always)]
	pub fn map<NewPerBitSetAware>(mut self, mut mapper: impl FnMut(BSA, PerBitSetAware) -> NewPerBitSetAware) -> PerBitSetAwareData<BSA, NewPerBitSetAware>
	{
		let mut mapped_data = Vec::with_capacity(self.data.len());

		for bit_set_aware in self.bit_set.iterate_including_empty()
		{
			let new_per_bit_set_aware = match bit_set_aware
			{
				None => None,
				Some(bit_set_aware) =>
				{
					let i: usize = bit_set_aware.into();
					let x = self.data.get_unchecked_mut_safe(i);
					Some(mapper(bit_set_aware, x.take().unwrap()))
				}
			};
			mapped_data.push(new_per_bit_set_aware);
		}

		PerBitSetAwareData
		{
			data: mapped_data.into_boxed_slice(),
			bit_set: self.bit_set,
			marker: PhantomData,
		}
	}
}
