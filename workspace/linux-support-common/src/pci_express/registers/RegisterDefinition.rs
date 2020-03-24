// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A register definition.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RegisterDefinition<R: Register<RS>, RS: RegisterSize>
{
	indexing: Indexing,
	marker: PhantomData<(R, RS)>,
}

impl<R: Register<RS>, RS: RegisterSize> RegisterDefinition<R, RS>
{
	/// Singleton.
	#[inline(always)]
	pub const fn singleton(offset: usize) -> Self
	{
		Self::new(Indexing::singleton(offset))
	}

	/// Array.
	#[inline(always)]
	pub const fn array(offset: usize, step: usize, inclusive_minimum: usize, inclusive_maximum: usize) -> Self
	{
		Self::new(Indexing::array(offset, step, inclusive_minimum, inclusive_maximum))
	}

	/// Split array.
	#[inline(always)]
	pub const fn split_array(offset0: usize, step0: usize, inclusive_minimum0: usize, inclusive_maximum0: usize, offset1: usize, step1: usize, inclusive_minimum1: usize, inclusive_maximum1: usize) -> Self
	{
		Self::new(Indexing::split_array(offset0, step0, inclusive_minimum0, inclusive_maximum0, offset1, step1, inclusive_minimum1, inclusive_maximum1))
	}

	#[inline(always)]
	const fn new(indexing: Indexing) -> Self
	{
		Self
		{
			indexing,
			marker: PhantomData,
		}
	}

	/// Instantiate a register.
	#[inline(always)]
	pub fn instantiate(&self, base_pointer: NonNull<u8>) -> Box<[R]>
	{
		use self::Indexing::*;

		match self.indexing
		{
			Singleton { offset } => vec![R::new(base_pointer, offset)].into_boxed_slice(),

			Array(ref array) =>
			{
				let capacity = array.capacity();
				debug_assert!(capacity > 1);

				let mut vec = Vec::with_capacity(capacity);
				array.iterate(|offset| vec.push(R::new(base_pointer, offset)));

				vec.into_boxed_slice()
			}

			SplitArray(ref array0, ref array1) =>
			{
				let capacity0 = array0.capacity();
				debug_assert_ne!(capacity0, 0);
				let capacity1 = array1.capacity();
				debug_assert_ne!(capacity1, 0);
				let capacity = capacity0 + capacity1;
				debug_assert!(capacity > 1);

				let mut vec = Vec::with_capacity(capacity);
				array0.iterate(|offset| vec.push(R::new(base_pointer, offset)));
				array1.iterate(|offset| vec.push(R::new(base_pointer, offset)));

				vec.into_boxed_slice()
			}
		}
	}
}
