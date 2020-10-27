// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Label bytes in US-ASCII, but casing may be mixed and invalid characters may be present.
///
/// Maximum length is 63.
///
/// If empty (length of 0) then this represents the Root, terminal label.
#[derive(Debug, Copy, Clone)]
pub struct ParsedLabel<'message>(&'message [u8]);

impl<'message> PartialEq for ParsedLabel<'message>
{
	#[inline(always)]
	fn eq(&self, rhs: &Self) -> bool
	{
		self.equals(rhs)
	}
}

impl<'message> Eq for ParsedLabel<'message>
{
}

impl<'message> PartialOrd for ParsedLabel<'message>
{
	#[inline(always)]
	fn partial_cmp(&self, rhs: &Self) -> Option<Ordering>
	{
		self.partial_compare(rhs)
	}
}

impl<'message> Ord for ParsedLabel<'message>
{
	#[inline(always)]
	fn cmp(&self, rhs: &Self) -> Ordering
	{
		self.compare(rhs)
	}
}

impl<'message> Hash for ParsedLabel<'message>
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		self.hash_slice(state)
	}
}

// impl<'message, 'cache: 'message> PartialEq<CaseFoldedLabel<'cache>> for ParsedLabel<'cache>
// {
// 	#[inline(always)]
// 	fn eq(&self, rhs: &CaseFoldedLabel<'cache>) -> bool
// 	{
// 		self.equals(rhs)
// 	}
// }
//
// impl<'message, 'cache: 'message> PartialOrd<CaseFoldedLabel<'cache>> for ParsedLabel<'message>
// {
// 	#[inline(always)]
// 	fn partial_cmp(&self, rhs: &CaseFoldedLabel<'cache>) -> Option<Ordering>
// 	{
// 		self.partial_compare(rhs)
// 	}
// }

impl<'message> Label<'message> for ParsedLabel<'message>
{
	#[inline(always)]
	fn len(&self) -> u8
	{
		self.0.len() as u8
	}
	
	#[inline(always)]
	fn get_unchecked_case_folded_byte(&self, index: u8) -> u8
	{
		case_fold_byte(self.get_unchecked(index))
	}
	
	#[inline(always)]
	fn get_unchecked(&self, index: u8) -> &u8
	{
		unsafe { self.0.get_unchecked(index as usize) }
	}
}
