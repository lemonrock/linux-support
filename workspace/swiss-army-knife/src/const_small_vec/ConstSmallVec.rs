// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Deliberately designed to permit use in `const` contexts.
pub struct ConstSmallVec<T, const N: usize>
{
	length_of_stack_or_capacity_of_heap: usize,
	
	stack_without_length_or_heap: StackWithoutLengthOrHeap<T, N>
}

impl<T, const N: usize> Drop for ConstSmallVec<T, N>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		if self.has_spilled_to_heap()
		{
			let to_drop = self.stack_without_length_or_heap.heap_mut().slice_mut();
			unsafe { drop_in_place(to_drop) };
			
			self.deallocate_heap();
		}
		else
		{
			let length = self.length_of_stack();
			
			let to_drop = self.stack_without_length_or_heap.stack_without_length_mut().slice_mut(length);
			unsafe { drop_in_place(to_drop) }
		}
	}
}

impl<T, const N: usize> const From<[T; N]> for ConstSmallVec<T, N>
{
	#[inline(always)]
	fn from(array: [T; N]) -> Self
	{
		Self
		{
			length_of_stack_or_capacity_of_heap: Self::capacity_of_stack(),

			stack_without_length_or_heap: StackWithoutLengthOrHeap
			{
				stack_without_length: StackWithoutLength::from(MaybeUninit::new(array)),
			}
		}
	}
}

impl<T, const N: usize> From<Vec<T>> for ConstSmallVec<T, N>
{
	#[inline(always)]
	fn from(vec: Vec<T>) -> Self
	{
		let length = vec.len();
		if length <= Self::capacity_of_stack()
		{
			let mut stack_without_length: MaybeUninit<[T; N]> = MaybeUninit::uninit();
			
			let from = vec.as_ptr();
			let to = stack_without_length.as_mut_ptr().cast::<T>();
			let length_of_stack = length;
			unsafe { copy_nonoverlapping(from, to, length_of_stack) };
			
			let _forget = ManuallyDrop::new(vec);
			
			Self
			{
				length_of_stack_or_capacity_of_heap: length_of_stack,
				
				stack_without_length_or_heap: StackWithoutLengthOrHeap
				{
					stack_without_length: StackWithoutLength::from(stack_without_length),
				}
			}
		}
		else
		{
			Self
			{
				length_of_stack_or_capacity_of_heap: vec.capacity(),
				
				stack_without_length_or_heap: StackWithoutLengthOrHeap
				{
					heap: Heap::from_vec(vec),
				},
			}
		}
	}
}

impl<T, const N: usize> TryFrom<ConstSmallVec<T, N>> for Vec<T>
{
	type Error = TryReserveError;
	
	#[inline(always)]
	fn try_from(const_small_vec: ConstSmallVec<T, N>) -> Result<Self, Self::Error>
	{
		let vec = if const_small_vec.has_spilled_to_heap()
		{
			let x = const_small_vec.stack_without_length_or_heap.heap();
			let (pointer, length) = x.pointer_and_length();
			let capacity = const_small_vec.capacity_of_heap();
			unsafe { Vec::from_raw_parts(pointer.as_ptr(), length, capacity) }
		}
		else
		{
			let from = const_small_vec.stack_without_length_or_heap.stack_without_length().pointer();
			let length = const_small_vec.length_of_stack();
			
			let mut vec = Vec::new();
			vec.try_reserve(length)?;
			
			let to = vec.as_mut_ptr();
			unsafe
			{
				copy_nonoverlapping(from, to, length);
				vec.set_len(length);
			}
			vec
		};
		let _forget = ManuallyDrop::new(const_small_vec);
		Ok(vec)
	}
}

impl<T: Debug, const N: usize> const TryFrom<ConstSmallVec<T, N>> for [T; N]
{
	type Error = IntoArrayError<T, N>;
	
	#[inline(always)]
	fn try_from(mut const_small_vec: ConstSmallVec<T, N>) -> Result<Self, Self::Error>
	{
		use IntoArrayError::*;
		
		if const_small_vec.has_spilled_to_heap()
		{
			return Err(HasSpilledToHeap(const_small_vec))
		}
		let length = const_small_vec.length_of_stack();
		if length != N
		{
			return Err(StackTooShort(const_small_vec))
		}
		
		let array = const_small_vec.stack_without_length_or_heap.take_array();
		let _ = ManuallyDrop::new(const_small_vec);
		Ok(array)
	}
}

impl<T: Debug, const N: usize> Debug for ConstSmallVec<T, N>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
	{
		self.deref().fmt(f)
	}
}

impl<T: Clone, const N: usize> Clone for ConstSmallVec<T, N>
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		if self.has_spilled_to_heap()
		{
			let (from, current_length) = self.stack_without_length_or_heap.heap().pointer_and_length();
			let current_capacity = self.length_of_stack_or_capacity_of_heap;
			let allocator = Self::allocator();
			let current_layout = Self::current_layout(current_capacity);
			let allocation = match allocator.allocate(current_layout)
			{
				Err(_) => handle_alloc_error(current_layout),
				
				Ok(allocation) => allocation,
			};
			
			let from = from.as_ptr();
			let to = allocation.as_mut_ptr() as *mut T;
			unsafe { copy_nonoverlapping(from, to, current_length) };
			
			Self
			{
				length_of_stack_or_capacity_of_heap: current_capacity,
			
				stack_without_length_or_heap: StackWithoutLengthOrHeap
				{
					heap: Heap::from_pointer_and_length(allocation.as_non_null_ptr().cast(), current_capacity)
				}
			}
		}
		else
		{
			Self
			{
				length_of_stack_or_capacity_of_heap: self.length_of_stack_or_capacity_of_heap,
				stack_without_length_or_heap: self.stack_without_length_or_heap.clone(),
			}
		}
	}
}

impl<T: PartialEq, const N: usize> PartialEq for ConstSmallVec<T, N>
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		self.deref() == other.deref()
	}
}

impl<T: Eq + PartialEq, const N: usize> Eq for ConstSmallVec<T, N>
{
}

impl<T: PartialOrd + PartialEq, const N: usize> PartialOrd for ConstSmallVec<T, N>
{
	#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		self.deref().partial_cmp(other.deref())
	}
}

impl<T: Ord + PartialOrd + Eq + PartialEq, const N: usize> Ord for ConstSmallVec<T, N>
{
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering
	{
		self.deref().cmp(other.deref())
	}
}

impl<T: Hash, const N: usize> Hash for ConstSmallVec<T, N>
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		self.deref().hash(state)
	}
}

impl<T, const N: usize> const Default for ConstSmallVec<T, N>
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			length_of_stack_or_capacity_of_heap: 0,
		
			stack_without_length_or_heap: StackWithoutLengthOrHeap::default(),
		}
	}
}

impl<T, const N: usize> const Deref for ConstSmallVec<T, N>
{
	type Target = [T];
	
	#[inline(always)]
	fn deref(&self) -> &[T]
	{
		if self.has_spilled_to_heap()
		{
			self.stack_without_length_or_heap.heap().slice()
		}
		else
		{
			let length_of_stack = self.length_of_stack();
			self.stack_without_length_or_heap.stack_without_length().slice(length_of_stack)
		}
	}
}

impl<T, const N: usize> const DerefMut for ConstSmallVec<T, N>
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut [T]
	{
		if self.has_spilled_to_heap()
		{
			self.stack_without_length_or_heap.heap().slice_mut()
		}
		else
		{
			let length_of_stack = self.length_of_stack();
			self.stack_without_length_or_heap.stack_without_length_mut().slice_mut(length_of_stack)
		}
	}
}

impl<T, const N: usize> const AsRef<[T]> for ConstSmallVec<T, N>
{
	#[inline(always)]
	fn as_ref(&self) -> &[T]
	{
		self.deref()
	}
}

impl<T, const N: usize> const AsMut<[T]> for ConstSmallVec<T, N>
{
	#[inline(always)]
	fn as_mut(&mut self) -> &mut [T]
	{
		self.deref_mut()
	}
}

impl<T, const N: usize> const Borrow<[T]> for ConstSmallVec<T, N>
{
	#[inline(always)]
	fn borrow(&self) -> &[T]
	{
		self.deref()
	}
}

impl<T, const N: usize> const BorrowMut<[T]> for ConstSmallVec<T, N>
{
	#[inline(always)]
	fn borrow_mut(&mut self) -> &mut [T]
	{
		self.deref_mut()
	}
}

impl<T: Copy, const N: usize> ConstSmallVec<T, N>
{
	/// Will panic if `slice.len()` exceeds `N` (unless `T` is zero-sized).
	#[inline(always)]
	pub const fn from_panic_slice(slice: &[T]) -> Self
	{
		Self::from_panic(slice)
	}
}

impl<T: TryToOwn + TryToOwnInPlace, const N: usize> TryToOwn for ConstSmallVec<T, N>
{
	type TryToOwned = ConstSmallVec<T::TryToOwned, N>;
	
	#[inline(always)]
	fn try_to_own(mut self) -> Result<Self::TryToOwned, TryReserveError>
	{
		self.try_to_own_in_place()?;
		let copy = unsafe { transmute_copy(&mut self) };
		let _ = ManuallyDrop::new(self);
		Ok(copy)
	}
}

impl<T: TryToOwnInPlace, const N: usize> TryToOwnInPlace for ConstSmallVec<T, N>
{
	#[inline(always)]
	fn try_to_own_in_place(&mut self) -> Result<(), TryReserveError>
	{
		self.deref_mut().try_to_own_in_place()
	}
}

impl<T, const N: usize> ConstSmallVec<T, N>
{
	/// Will panic if `M` exceeds `N` (unless `T` is zero-sized).
	#[inline(always)]
	pub const fn from_panic_array<const M: usize>(array: [T; M]) -> Self
	{
		let this = Self::from_panic(array.as_slice());
		let _forget = ManuallyDrop::new(array);
		this
	}
	
	/// Tries to turn into an array of length `M`.
	/// Fails if:-
	/// * Has spilled to heap (`self.len() > N`).
	/// * `M > N` or, if `size_of::<T>() == 0`, `M > usize::MAX`.
	/// * `M > self.len()`.
	#[inline(always)]
	pub fn try_into_array<const M: usize>(mut self) -> Result<[T; M], IntoArrayMError>
	{
		use IntoArrayMError::*;
		
		let m_array = if self.has_spilled_to_heap()
		{
			let (pointer, length_of_heap) = self.stack_without_length_or_heap.heap().pointer_and_length();
			
			if M > length_of_heap
			{
				return Err(MExceedsLength)
			}
			
			let mut m_array: MaybeUninit<[T; M]> = MaybeUninit::uninit();
			
			let heap_pointer = pointer.as_ptr();
			
			{
				let to = m_array.as_mut_ptr().cast::<T>();
				unsafe { copy_nonoverlapping(heap_pointer as *const T, to, M) }
			}
			
			{
				let from = unsafe { heap_pointer.add(M) };
				let range_to_drop = unsafe { from_raw_parts_mut(from, length_of_heap - M) };
				unsafe { drop_in_place(range_to_drop) };
			}
			
			self.deallocate_heap();
			
			unsafe { m_array.assume_init() }
		}
		else
		{
			let length_of_stack = self.length_of_stack();
			if M > length_of_stack
			{
				return Err(MExceedsLength)
			}
			
			let stack_without_length = self.stack_without_length_or_heap.stack_without_length_mut();
			let stack_pointer = stack_without_length.pointer_mut();
			
			{
				let from = unsafe { stack_pointer.add(M) };
				let range_to_drop = unsafe { from_raw_parts_mut(from, length_of_stack - M) };
				unsafe { drop_in_place(range_to_drop) };
			}
			
			{
				let m_array_pointer = stack_pointer.cast::<[T; M]>();
				unsafe { read(m_array_pointer) }
			}
		};
		let _ = ManuallyDrop::new(self);
		Ok(m_array)
	}

	#[inline(always)]
	const fn from_panic(slice: &[T]) -> Self
	{
		let length_of_stack = slice.len();
		if length_of_stack > Self::capacity_of_stack()
		{
			panic!("slice is too large to allocate on the stack, and heap allocation is not possible at build time")
		}
		
		let mut stack_without_length = MaybeUninit::uninit();
		let to = stack_without_length.as_mut_ptr() as *mut T;
		let from = slice.as_ptr();
		unsafe { copy_nonoverlapping(from, to, length_of_stack) };
		
		Self
		{
			length_of_stack_or_capacity_of_heap: length_of_stack,
			
			stack_without_length_or_heap: StackWithoutLengthOrHeap
			{
				stack_without_length: StackWithoutLength::from(stack_without_length),
			},
		}
	}
	
	/// Constant function suitable for use at compile time.
	///
	/// Can fail with an `Err()` if there is not enough memory to allocate on the stack, i.e. the length (`len()`) is already equal to or greater than `N`.
	#[inline(always)]
	pub const fn try_stack_push(&mut self, element: T) -> Result<(), T>
	{
		if self.has_spilled_to_heap_or_stack_is_full()
		{
			return Err(element)
		}
		
		let length = self.length_of_stack();
		{
			let pointer = self.stack_without_length_or_heap.stack_without_length_mut().pointer_mut();
			unsafe { pointer.add(length).write(element) }
		}
		*self.length_of_stack_ref_mut() = length + 1;
		
		Ok(())
	}
	
	/// Optimized reservation followed by push.
	#[inline(always)]
	pub fn try_reserve_push(&mut self, element: T) -> Result<(), TryReserveError>
	{
		let (memory, length, length_ref_mut) = self.try_reserve::<ExactNewCapacityCalculator>(1)?;
		
		let pointer = memory.as_mut_ptr();
		unsafe { pointer.add(length).write(element) };
		*length_ref_mut = length + 1;
		
		Ok(())
	}
	
	/// Returns a slice of the memory available, including additional and possibly more; not all of it will be valid to dereference.
	#[inline(always)]
	pub fn try_reserve<NGC: NewCapacityCalculator>(&mut self, additional: usize) -> Result<(NonNull<[T]>, usize, &mut usize), TryReserveError>
	{
		if self.has_spilled_to_heap()
		{
			self.try_reserve_heap::<NGC>(additional)
		}
		else
		{
			self.try_reserve_stack::<NGC>(additional)
		}
	}
	
	#[inline(always)]
	fn try_reserve_stack<NGC: NewCapacityCalculator>(&mut self, additional: usize) -> Result<(NonNull<[T]>, usize, &mut usize), TryReserveError>
	{
		let current_length = self.length_of_stack();
		let current_capacity = Self::capacity_of_stack();
		let required_capacity = required_capacity!(current_length, current_capacity, additional, (NonNull::slice_from_raw_parts(self.stack_without_length_or_heap.stack_without_length_mut().non_null_pointer(), current_capacity), current_length, self.length_of_stack_ref_mut()));
		
		let new_capacity = NGC::calculate::<T>(current_capacity, required_capacity)?;
		let new_layout = Self::new_layout(new_capacity)?;
		let allocator = Self::allocator();
		
		let allocation = match allocator.allocate(new_layout)
		{
			Err(alloc_error) => return Self::alloc_error(alloc_error, new_layout),
			
			Ok(allocation) => allocation,
		};
		
		let pointer = self.set_capacity_of_heap_to_new_capacity(allocation, new_capacity).cast();
		
		{
			let from = self.stack_without_length_or_heap.stack_without_length().pointer();
			let to = pointer.as_ptr();
			unsafe { copy_nonoverlapping(from, to, current_length) };
		}
		
		self.stack_without_length_or_heap.set_heap(Heap::from_pointer_and_length(pointer, current_length));
		let length_ref_mut = self.stack_without_length_or_heap.heap_mut().length_ref_mut();
		
		Ok((NonNull::slice_from_raw_parts(pointer, new_capacity), current_length, length_ref_mut))
	}
	
	#[inline(always)]
	fn try_reserve_heap<NGC: NewCapacityCalculator>(&mut self, additional: usize) -> Result<(NonNull<[T]>, usize, &mut usize), TryReserveError>
	{
		let current_capacity = self.capacity_of_heap();
		
		let current_length = self.stack_without_length_or_heap.heap().length();
		let heap_pointer = self.stack_without_length_or_heap.heap().non_null_pointer();
		let required_capacity = required_capacity!(current_length, current_capacity, additional, (NonNull::slice_from_raw_parts(heap_pointer, current_capacity), current_length, self.stack_without_length_or_heap.heap_mut().length_ref_mut()));
		
		let new_capacity = NGC::calculate::<T>(current_capacity, required_capacity)?;
		let new_layout = Self::new_layout(new_capacity)?;
		let current_pointer = heap_pointer.cast();
		let current_layout = Self::current_layout(current_capacity);
		let allocator = Self::allocator();
		match unsafe { allocator.grow(current_pointer, current_layout, new_layout) }
		{
			Ok(allocation) =>
			{
				let pointer = self.set_capacity_of_heap_to_new_capacity(allocation, new_capacity).cast();
				let heap = self.stack_without_length_or_heap.heap_mut();
				heap.set_pointer(pointer);
				Ok((NonNull::slice_from_raw_parts(pointer, new_capacity), current_length, heap.length_ref_mut()))
			},
			
			Err(alloc_error) => Self::alloc_error(alloc_error, new_layout),
		}
	}
	
	#[inline(always)]
	fn set_capacity_of_heap_to_new_capacity(&mut self, allocation: NonNull<[u8]>, new_capacity: usize) -> NonNull<u8>
	{
		debug_assert!((allocation.len() >= (new_capacity * size_of::<T>())));
		self.length_of_stack_or_capacity_of_heap = new_capacity;
		allocation.as_non_null_ptr()
	}
	
	#[inline(always)]
	fn alloc_error<'a>(alloc_error: AllocError, layout: Layout) -> Result<(NonNull<[T]>, usize, &'a mut usize), TryReserveError>
	{
		let _ = alloc_error;
		Err(TryReserveErrorKind::AllocError { layout, non_exhaustive: () }.into())
	}
	
	#[inline(always)]
	fn deallocate_heap(&mut self)
	{
		let pointer = self.stack_without_length_or_heap.heap().non_null_pointer().cast::<u8>();
		let current_layout = Self::current_layout(self.capacity_of_heap());
		let allocator = Self::allocator();
		unsafe { allocator.deallocate(pointer, current_layout) };
	}
	
	/// We need to guarantee we don't ever allocate `> isize::MAX` bytes.
	///
	/// * On 64-bit: Trying to allocate `> isize::MAX` bytes will always fail
	/// * On 32-bit and 16-bit: We need to add an extra guard for this in case we're running on a platform which can use all 4GB in user-space, e.g., PAE or x32.
	#[inline(always)]
	fn guard_allocation_does_exceed_isize_max_on_16_bit_and_32_bit_platforms(new_layout: Layout) -> Result<Layout, TryReserveError>
	{
		if cfg!(not(target_pointer_width = "64"))
		{
			if new_layout.size() > (isize::MAX as usize)
			{
				return Err(TryReserveErrorKind::CapacityOverflow.into())
			}
		}
		Ok(new_layout)
	}
	
	#[inline(always)]
	fn new_layout(new_capacity: usize) -> Result<Layout, TryReserveError>
	{
		match Layout::array::<T>(new_capacity)
		{
			Err(_layout_error) => Err(TryReserveErrorKind::CapacityOverflow.into()),
			
			Ok(new_layout) => Self::guard_allocation_does_exceed_isize_max_on_16_bit_and_32_bit_platforms(new_layout),
		}
	}
	
	/// Equivalent to `new_layout()` but does not raise an error.
	///
	/// Internally, `Layout::array()` calls the same logic as is below but with a checked multiplication.
	#[inline(always)]
	const fn current_layout(current_capacity: usize) -> Layout
	{
		let size = size_of::<T>() * current_capacity;
		let align = align_of::<T>();
		unsafe { Layout::from_size_align_unchecked(size, align) }
	}
	
	#[inline(always)]
	fn required_capacity(current_length: usize, additional: usize) -> Result<usize, TryReserveError>
	{
		match current_length.checked_add(additional)
		{
			None => Err(TryReserveErrorKind::CapacityOverflow.into()),
			
			Some(required_capacity) => Ok(required_capacity),
		}
	}
	
	#[inline(always)]
	const fn length_of_stack(&self) -> usize
	{
		if cfg!(debug_assertions)
		{
			if self.has_spilled_to_heap()
			{
				panic!("Has spilled to heap")
			}
		}
		
		self.length_of_stack_or_capacity_of_heap
	}
	
	#[inline(always)]
	const fn length_of_stack_ref_mut(&mut self) -> &mut usize
	{
		if cfg!(debug_assertions)
		{
			if self.has_spilled_to_heap()
			{
				panic!("Has spilled to heap")
			}
		}
		
		&mut self.length_of_stack_or_capacity_of_heap
	}
	
	#[inline(always)]
	const fn has_spilled_to_heap(&self) -> bool
	{
		self.length_of_stack_or_capacity_of_heap > Self::capacity_of_stack()
	}
	
	#[inline(always)]
	const fn has_spilled_to_heap_or_stack_is_full(&self) -> bool
	{
		self.length_of_stack_or_capacity_of_heap >= Self::capacity_of_stack()
	}
	
	#[inline(always)]
	const fn capacity_of_stack() -> usize
	{
		let elements_are_zero_sized = size_of::<T>() == 0;
		
		if elements_are_zero_sized
		{
			usize::MAX
		}
		else
		{
			N
		}
	}
	
	#[inline(always)]
	const fn capacity_of_heap(&self) -> usize
	{
		if cfg!(debug_assertions)
		{
			if !self.has_spilled_to_heap()
			{
				panic!("Has not spilled to heap")
			}
		}
		
		self.length_of_stack_or_capacity_of_heap
	}
	
	#[inline(always)]
	fn allocator() -> impl Allocator
	{
		Global
	}
}
