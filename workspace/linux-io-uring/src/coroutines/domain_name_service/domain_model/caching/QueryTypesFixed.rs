// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Populated at creation time for known records and potentially from `/etc/hostname`, `/etc/hosts` et al.
#[derive(Debug)]
pub(crate) struct QueryTypesFixed
{
	pub(crate) A: Option<MultipleSortedRecords<Ipv4Addr>>,
	
	pub(crate) AAAA: Option<MultipleSortedRecords<Ipv6Addr>>,
	
	pub(crate) PTR: Option<MultipleSortedRecords<PointerName<AliasOrDomainTarget>>>,
}

impl Default for QueryTypesFixed
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			A: None,
			AAAA: None,
			PTR: None,
		}
	}
}

impl QueryTypesFixed
{
	#[inline(always)]
	pub(crate) fn new(internet_protocol_address: IpAddr) -> Self
	{
		use self::IpAddr::*;
		
		let mut query_types_fixed = QueryTypesFixed::default();
		match internet_protocol_address
		{
			V4(v4) => query_types_fixed.A = Some(MultipleSortedRecords::single(v4)),
			
			V6(v6) => query_types_fixed.AAAA = Some(MultipleSortedRecords::single(v6)),
		}
		query_types_fixed
	}
	
	#[inline(always)]
	pub(crate) fn new_local_internet_protocol_addresses() -> Self
	{
		Self
		{
			A: Some(MultipleSortedRecords::single(Ipv4Addr::LOCALHOST)),
			AAAA: Some(MultipleSortedRecords::single(Ipv6Addr::LOCALHOST)),
			PTR: None,
		}
	}
	
	#[inline(always)]
	pub(crate) fn add_internet_protocol_address(&mut self, internet_protocol_address: IpAddr)
	{
		use self::IpAddr::*;
		
		match internet_protocol_address
		{
			V4(v4) => if self.A.is_none()
			{
				self.A = Some(MultipleSortedRecords::single(v4));
				return
			}
			else
			{
				self.A.as_mut().unwrap().add(v4)
			},
			
			V6(v6) =>if self.AAAA.is_none()
			{
				self.AAAA = Some(MultipleSortedRecords::single(v6));
				return
			}
			else
			{
				self.AAAA.as_mut().unwrap().add(v6)
			},
		}
	}
}
