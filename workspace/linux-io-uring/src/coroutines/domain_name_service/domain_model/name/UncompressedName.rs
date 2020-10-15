// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub(crate) struct UncompressedName<Allocator: AllocRef>
{
	allocator: Allocator,
	pointer: NonNull<(UncompressedNameHeader, UpTo255Bytes)>,
}

impl<Allocator: AllocRef> Drop for UncompressedName<Allocator>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		let pointer = self.pointer.unsafe_cast_mut_non_null::<u8>();
		let layout = Self::layout(self.header_mut().name_length as usize);
		self.allocator.dealloc(pointer, layout)
	}
}

impl<'message, Allocator: AllocRef> IntoIterator for &'message UncompressedName<Allocator>
{
	type Item = LabelBytes<'message>;

	type IntoIter = WithoutCompressionParsedNameIterator<'message>;

	#[inline(always)]
	fn into_iter(&'message self) -> Self::IntoIter
	{
		WithoutCompressionParsedNameIterator::new(self.pointer_to_label())
	}
}

impl<Allocator: AllocRef> PartialEq for UncompressedName<Allocator>
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		if unlikely!(self.number_of_labels_u8() != other.number_of_labels_u8())
		{
			return false
		}

		self.raw_name_bytes_slice().eq(other.raw_name_bytes_slice())
	}
}

impl<'message, Allocator: AllocRef> PartialEq<WithoutCompressionParsedName<'message>> for UncompressedName<Allocator>
{
	#[inline(always)]
	fn eq(&self, other: &WithCompressionParsedName<'message>) -> bool
	{
		other.eq(self)
	}
}

impl<'message, Allocator: AllocRef> PartialEq<WithCompressionParsedName<'message>> for UncompressedName<Allocator>
{
	#[inline(always)]
	fn eq(&self, other: &WithCompressionParsedName<'message>) -> bool
	{
		other.eq(self)
	}
}

impl<Allocator: AllocRef> Eq for UncompressedName<Allocator>
{
}

impl<Allocator: AllocRef> Hash for UncompressedName<Allocator>
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		self.raw_name_bytes_slice().hash(state)
	}
}

impl<Allocator: AllocRef> UncompressedName<Allocator>
{
	// TODO: conversions from a dotted name, and from a dotted name with / without a trailing .
	// TODO: conversions from a relative name + domain name (which may not have a trailing .)
	// TODO: Consider validating that the label sizes are correct.
	/// Creates a new instance.
	#[inline(always)]
	pub fn new(allocator: &mut Allocator, name: &[u8], number_of_labels: NonZeroU8) -> Result<Option<Self>, AllocErr>
	{
		let number_of_labels = number_of_labels.get();
		debug_assert!((number_of_labels as usize) <= Label::MaximumNumber, "number_of_labels `{}` exceeds {}", number_of_labels, Label::MaximumNumber);

		let name_length_usize = name.len();
		debug_assert_ne!(name_length_usize, 0, "name is empty");
		debug_assert!(name_length_usize <= Name::MaximumSize, "name `{}` exceeds {} bytes", name_length_usize, Name::MaximumSize);
		debug_assert_eq!(name.get(name_length_usize - 1), 0x00, "final byte of name is not 0x00 (a root label)");

		let MemoryBlock { ptr, size } = allocator.alloc(Self::layout(name_length_usize))?;

		let this = Self
		{
			allocator,
			pointer: unsafe { transmute(non_null_u8) },
		};

		unsafe { write(this.header_mut(), UncompressedNameHeader::new(number_of_labels, name_length_usize as u8)) }
		unsafe { copy_nonoverlapping(name.as_ptr(), this.raw_name_bytes_mut() as *mut UpTo255Bytes as *mut u8, name_length_usize) }

		Ok(Some(this))
	}

	/// Obtains a name.
	#[inline(always)]
	pub fn name<'query>(&'query self) -> WithoutCompressionParsedName<'query>
	{
		WithoutCompressionParsedName::new(self.number_of_labels_u8(), self.name_length_u8(), self.pointer_to_label())
	}

	/// Validation of available buffer size is done before calling this.
	#[inline(always)]
	pub(crate) fn copy_non_overlapping_to(&self, pointer: usize) -> usize
	{
		let name_length = self.name_length_usize();
		unsafe { copy_nonoverlapping(self.pointer_to_label(),cast::<u8>().pointer.cast_mut::<u8>(), name_length) };
		pointer + name_length
	}

	#[inline(always)]
	fn number_of_labels_u8(&self) -> u8
	{
		self.header().number_of_labels
	}

	#[inline(always)]
	fn name_length_usize(&self) -> usize
	{
		self.name_length_u8() as usize
	}

	#[inline(always)]
	fn name_length_u8(&self) -> u8
	{
		self.header().name_length
	}

	#[inline(always)]
	fn pointer_to_label(&self) -> usize
	{
		self.raw_name_bytes().as_usize_pointer()
	}

	#[inline(always)]
	fn header(&self) -> &UncompressedNameHeader
	{
		&mut self.pointer().0
	}

	#[inline(always)]
	fn header_mut(&mut self) -> &mut UncompressedNameHeader
	{
		&mut self.pointer_mut().0
	}

	#[inline(always)]
	fn raw_name_bytes_slice(&self) -> &[u8]
	{
		self.raw_name_bytes().unsafe_cast_slice::<u8>(self.name_length_usize())
	}

	#[inline(always)]
	fn raw_name_bytes(&self) -> &UpTo255Bytes
	{
		&mut self.pointer().1
	}

	#[inline(always)]
	fn raw_name_bytes_mut(&mut self) -> &mut UpTo255Bytes
	{
		&mut self.pointer_mut().1
	}

	#[inline(always)]
	fn pointer(&self) -> &(UncompressedNameHeader, UpTo255Bytes)
	{
		unsafe { & * self.pointer.as_ptr() }
	}

	#[inline(always)]
	fn pointer_mut(&mut self) -> &mut (UncompressedNameHeader, UpTo255Bytes)
	{
		unsafe { &mut * self.pointer.as_ptr() }
	}

	#[inline(always)]
	fn layout(name_length_usize: usize) -> Layout
	{
		unsafe { Layout::from_size_align_unchecked(size_of::<UncompressedNameHeader>() + name_length_usize, align_of::<UncompressedNameHeader>()) }
	}
}
