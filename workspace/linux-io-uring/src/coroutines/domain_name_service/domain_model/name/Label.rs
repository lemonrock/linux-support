// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub trait Label<'label>
{
	/// Is the terminal root label?
	#[inline(always)]
	fn is_root(&self) -> bool
	{
		self.len() == 0
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn equals<'rhs_label, RHS: Label<'rhs_label>>(&self, rhs: &RHS) -> bool
	{
		if self.len() != rhs.len()
		{
			return false
		}
		
		for index in 0 .. self.len()
		{
			let left = self.get_unchecked_case_folded_byte(index);
			let right = rhs.get_unchecked_case_folded_byte(index);
			
			if left != right
			{
				return false
			}
		}
		
		true
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn partial_compare<'rhs_label, RHS: Label<'rhs_label>>(&self, rhs: &RHS) -> Option<Ordering>
	{
		Some(self.cmp(rhs))
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn compare<'rhs_label, RHS: Label<'rhs_label>>(&self, rhs: &RHS) -> Ordering
	{
		let left_length = self.len();
		let right_length = rhs.len();
		
		for index in 0 .. min(left_length, right_length)
		{
			let left = self.get_unchecked_case_folded_byte(index);
			let right = rhs.get_unchecked_case_folded_byte(index);
			
			use self::Ordering::*;
			match left.cmp(&right)
			{
				Less => return Less,
				Equal => continue,
				Greater => return Greater,
			}
		}
		
		return left_length.cmp(&right_length)
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn hash_slice<H: Hasher>(&self, state: &mut H)
	{
		for index in 0 .. self.len()
		{
			self.get_unchecked_case_folded_byte(index).hash(state)
		}
	}
	
	#[doc(hidden)]
	fn len(&self) -> u8;
	
	/// Length including trailing period.
	#[inline(always)]
	fn length_including_trailing_period(&self) -> NonZeroU8
	{
		unsafe { NonZeroU8::new_unchecked(self.len() + ParsedNameParser::SizeOfTrailingPeriod) }
	}
	
	/// Is probably an internationalized domain name (IDN)?
	///
	/// See RFC 3490; only by applying the `ToUnicode` alogrithm can one be certain.
	///
	/// It is not obviously if the empty root label is permitted (ie is the minimum length 4 or 5)?; we assume it is not.
	#[inline(always)]
	fn is_probably_an_internationalized_domain_name(&self) -> bool
	{
		let length = self.len();
		if length >= 5
		{
			*self.get_unchecked(0) == b'x' && *self.get_unchecked(1) == b'n' && *self.get_unchecked(2) == b'-' && *self.get_unchecked(3) == b'-'
		}
		else
		{
			false
		}
	}
	
	#[doc(hidden)]
	fn get_unchecked_case_folded_byte(&self, index: u8) -> u8;
	
	#[doc(hidden)]
	fn get_unchecked(&self, index: u8) -> &u8;
}
