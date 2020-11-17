// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Type for use as a key in hash maps.
#[derive(Debug, Clone)]
pub(crate) enum EitherName<'message>
{
	Parsed(ParsedName<'message>),
	
	EfficientCaseFolded(EfficientCaseFoldedName),
}

impl<'message> PartialEq for EitherName<'message>
{
	#[inline(always)]
	fn eq(&self, right: &Self) -> bool
	{
		use self::EitherName::*;
		
		match (self, right)
		{
			(Parsed(left), Parsed(right)) => left.equals(right),
			(Parsed(left), EfficientCaseFolded(right)) => left.equals(right),
			(EfficientCaseFolded(left), EfficientCaseFolded(right)) => left.equals(right),
			(EfficientCaseFolded(left), Parsed(right)) => left.equals(right),
		}
	}
}

impl<'message> Eq for EitherName<'message>
{
}

impl<'message> PartialOrd for EitherName<'message>
{
	#[inline(always)]
	fn partial_cmp(&self, right: &Self) -> Option<Ordering>
	{
		use self::EitherName::*;
		
		match (self, right)
		{
			(Parsed(left), Parsed(right)) => left.partial_compare(right),
			(Parsed(left), EfficientCaseFolded(right)) => left.partial_compare(right),
			(EfficientCaseFolded(left), EfficientCaseFolded(right)) => left.partial_compare(right),
			(EfficientCaseFolded(left), Parsed(right)) => left.partial_compare(right),
		}
	}
}
impl<'message> Ord for EitherName<'message>
{
	#[inline(always)]
	fn cmp(&self, right: &Self) -> Ordering
	{
		use self::EitherName::*;
		
		match (self, right)
		{
			(Parsed(left), Parsed(right)) => left.compare(right),
			(Parsed(left), EfficientCaseFolded(right)) => left.compare(right),
			(EfficientCaseFolded(left), EfficientCaseFolded(right)) => left.compare(right),
			(EfficientCaseFolded(left), Parsed(right)) => left.compare(right),
		}
	}
}

impl<'message> Hash for EitherName<'message>
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		use self::EitherName::*;
		
		match self
		{
			Parsed(left) => left.hash_slice(state),
			EfficientCaseFolded(left) => left.hash_slice(state),
		}
	}
}
