// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


trait Label<'a>
{
	/// Is the terminal root label?
	#[inline(always)]
	fn is_root(&self) -> bool
	{
		self.len() == 0
	}
	
	#[inline(always)]
	fn equals<RHS: Label>(&self, rhs: &RHS) -> bool
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
	
	#[inline(always)]
	fn partial_compare<RHS: Label>(&self, rhs: &RHS) -> Option<Ordering>
	{
		Some(self.cmp(rhs))
	}
	
	#[inline(always)]
	fn compare<RHS: Label>(&self, rhs: &RHS) -> Ordering
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
	
	#[inline(always)]
	fn hash_slice<H: Hasher>(&self, state: &mut H)
	{
		for index in 0 .. self.len()
		{
			self.get_unchecked_case_folded_byte(index).hash(state)
		}
	}
	
	fn len(&self) -> u8;
	
	fn get_unchecked_case_folded_byte(&self, index: u8) -> u8;
	
	fn get_unchecked(&self, index: u8) -> &u8;
}
