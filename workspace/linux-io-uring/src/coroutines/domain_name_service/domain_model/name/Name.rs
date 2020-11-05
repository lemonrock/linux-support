// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Name.
pub trait Name<'label>: HasTypeEquality + Sized + Clone + Debug + Display + PartialEq + Eq + PartialOrd + Ord + Hash
{
	#[doc(hidden)]
	type Label: Label<'label, TypeEquality=Self::TypeEquality>;
	
	#[inline(always)]
	fn last_label(&'label self) -> Option<Self::Label>
	{
		if unlikely!(self.is_root())
		{
			None
		}
		else
		{
			Some(self.label(self.number_of_labels_including_root().get() - 1))
		}
	}
	
	/// Display or debug.
	#[inline(always)]
	fn display(&self, f: &mut Formatter) -> fmt::Result
	{
		for index in (0 .. self.number_of_labels_including_root().get()).rev()
		{
			let label = self.label(index);
			label.display(f)?;
			write!(f, ".")?;
		}
		Ok(())
	}
	
	/// Parent.
	fn parent(&self) -> Option<Self>;
	
	#[doc(hidden)]
	#[inline(always)]
	fn copy_non_overlapping_to_without_case_folding(&'label self, start_of_name_pointer: usize) -> usize
	{
		let mut next_label_starts_at_pointer = start_of_name_pointer;
		for index in 0 .. self.number_of_labels_including_root().get()
		{
			let label = self.label(index);
			next_label_starts_at_pointer = label.copy_non_overlapping_to_without_case_folding(next_label_starts_at_pointer);
		}
		
		next_label_starts_at_pointer
	}
	
	/// Ends with name?
	#[inline(always)]
	fn ends_with<'ends_with_label, RHS: Name<'ends_with_label>>(&'label self, ends_with: &'ends_with_label RHS) -> bool
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
			
			let our_label = self.label(index);
			let ends_with_label = ends_with.label(index);
			if !our_label.deref().equals(ends_with_label.deref())
			{
				return false
			}
			
			length -= 1;
		}
		
		true
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn equals<'rhs_label, RHS: Name<'rhs_label>>(&'label self, rhs: &'rhs_label RHS) -> bool
	{
		if self.number_of_labels_including_root() != rhs.number_of_labels_including_root()
		{
			return false
		}
		
		for index in 0 .. (self.number_of_labels_including_root().get())
		{
			let left = self.label(index);
			let right = rhs.label(index);
			
			if left.deref().equals(right.deref())
			{
				return false
			}
		}
		
		true
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn partial_compare<'rhs_label, RHS: Name<'rhs_label>>(&'label self, rhs: &'rhs_label RHS) -> Option<Ordering>
	{
		Some(self.compare(rhs))
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn compare<'rhs_label, RHS: Name<'rhs_label>>(&'label self, rhs: &'rhs_label RHS) -> Ordering
	{
		let left_length = self.number_of_labels_including_root().get();
		let right_length = rhs.number_of_labels_including_root().get();
		
		for index in 0 .. min(left_length, right_length)
		{
			let left = self.label(index);
			let right = rhs.label(index);
			
			use self::Ordering::*;
			
			match left.deref().compare(right.deref())
			{
				Less => return Less,
				Equal => continue,
				Greater => return Greater,
			}
		}
		
		left_length.cmp(&right_length)
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn hash_slice<H: Hasher>(&'label self, state: &mut H)
	{
		for index in 0 .. (self.number_of_labels_including_root().get())
		{
			let label = self.label(index);
			label.hash_slice(state)
		}
	}
	
	#[doc(hidden)]
	fn label(&'label self, index: u8) -> Self::Label;
	
	/// Number of labels.
	fn number_of_labels_including_root(&self) -> NonZeroU8;
	
	/// Length of name.
	fn name_length_including_trailing_periods_after_labels(&self) -> NonZeroU8;
	
	/// Is root?
	///
	/// ie `.`.
	#[inline(always)]
	fn is_root(&self) -> bool
	{
		self.number_of_labels_including_root().get() == 1
	}
	
	/// Is top level?
	///
	/// eg `.net.`.
	#[inline(always)]
	fn is_top_level(&self) -> bool
	{
		self.number_of_labels_including_root().get() == 2
	}
	
	/// Is second level?
	///
	/// eg `.example.net.`.
	#[inline(always)]
	fn is_second_level(&self) -> bool
	{
		self.number_of_labels_including_root().get() == 3
	}
	
	/// Is third level?
	///
	/// eg `.www.example.net.`.
	#[inline(always)]
	fn is_third_level(&self) -> bool
	{
		self.number_of_labels_including_root().get() == 4
	}
	
	/// Is fourth level?
	///
	/// eg `.www.example.co.uk.`.
	#[inline(always)]
	fn is_fourth_level(&self) -> bool
	{
		self.number_of_labels_including_root().get() == 5
	}
	
	/// Pushing a child will always succeed.
	///
	/// ie is a root, top level or second level name.
	#[inline(always)]
	fn pushing_a_child_will_always_succeed(&self) -> bool
	{
		self.number_of_labels_including_root().get() <= 3
	}
}
