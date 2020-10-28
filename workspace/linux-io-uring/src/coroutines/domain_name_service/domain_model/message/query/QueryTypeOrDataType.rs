// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[repr(C, packed)]
#[derive(Copy, Clone)]
pub(crate) union QueryTypeOrDataType
{
	query_type: QueryType,
	data_type: DataType,
	bytes: BigEndianU16,
}

impl Debug for QueryTypeOrDataType
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		unsafe { self.bytes.fmt(f) }
	}
}

impl PartialEq for QueryTypeOrDataType
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		unsafe { self.bytes == other.bytes }
	}
}

impl Eq for QueryTypeOrDataType
{
}

impl PartialOrd for QueryTypeOrDataType
{
	#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		unsafe { self.bytes.partial_cmp(&other.bytes) }
	}
}

impl Ord for QueryTypeOrDataType
{
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering
	{
		unsafe { self.bytes.cmp(&other.bytes) }
	}
}

impl Hash for QueryTypeOrDataType
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		unsafe { self.bytes.hash(state) }
	}
}

impl QueryTypeOrDataType
{
	#[inline(always)]
	pub(crate) fn data_type(self) -> DataType
	{
		unsafe { self.data_type }
	}
}
