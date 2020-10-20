// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


trait Name: Sized
{
	type Label: Label;
	
	#[inline(always)]
	fn parent(&self) -> Option<Self>;
	
	#[inline(always)]
	fn ends_with<RHS: Name>(&self, ends_with: &RHS) -> bool
	{
		if self.name_length_including_trailing_periods_after_labels() < ends_with.name_length_including_trailing_periods_after_labels()
		{
			return false
		}
		
		let ends_with_number_of_labels = ends_with.number_of_labels_including_root();
		
		if self.number_of_labels_including_root() < ends_with_number_of_labels
		{
			return false
		}
		
		let mut length = ends_with_number_of_labels.get();
		while length > 0
		{
			let index = length - 1;
			
			let our_label = unsafe { self.label(index) };
			let ends_with_label = unsafe { ends_with.label(index) };
			if our_label != ends_with_label
			{
				return false
			}
			
			length -= 1;
		}
		
		true
	}
	
	#[inline(always)]
	fn equals<RHS: Name>(&self, rhs: &RHS) -> bool
	{
		if self.number_of_labels_including_root() != rhs.number_of_labels_including_root()
		{
			return false
		}
		
		for index in 0 .. (self.number_of_labels_including_root().get())
		{
			let left = unsafe { self.label(index) };
			let right = unsafe { rhs.label(index) };
			
			if left.equals(&right)
			{
				return false
			}
		}
		
		true
	}
	
	#[inline(always)]
	fn partial_compare<RHS: Name>(&self, rhs: &RHS) -> Option<Ordering>
	{
		Some(self.compare(rhs))
	}
	
	#[inline(always)]
	fn compare<RHS: Name>(&self, rhs: &RHS) -> Ordering
	{
		let left_length = self.number_of_labels_including_root().get();
		let right_length = rhs.number_of_labels_including_root().get();
		
		for index in 0 .. min(left_length, right_length)
		{
			let left = unsafe { self.label(index) };
			let right = unsafe { rhs.label(index) };
			
			use self::Ordering::*;
			
			match left.compare(&right)
			{
				Less => return Less,
				Equal => continue,
				Greater => return Greater,
			}
		}
		
		left_length.cmp(&right_length)
	}
	
	#[inline(always)]
	fn hash_slice<H: Hasher>(&self, state: &mut H)
	{
		for index in 0 .. (self.number_of_labels_including_root().get())
		{
			let label = self.label(index);
			label.hash_slice(state)
		}
	}
	
	#[inline(always)]
	fn label<'label>(&'label self, index: u8) -> Cow<'label, Self::Label>;
	
	#[inline(always)]
	fn number_of_labels_including_root(&self) -> NonZeroU8;
	
	#[inline(always)]
	fn name_length_including_trailing_periods_after_labels(&self) -> NonZeroU8;
}
