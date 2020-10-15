// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C, packed)]
struct QuerySectionEntryFooter
{
	/// A two octet code which specifies the type of the query.
	///
	/// The values for this field include all codes valid for a `TYPE` field, together with some more general codes which can match more than one type of Resource Record (RR).
	qtype: QueryTypeOrDataType,

	/// A two octet code that specifies the class of the query.
	///
	/// For example, the `QCLASS` field is `IN` for the Internet.
	qclass: BigEndianU16,
}

impl QuerySectionEntryFooter
{
	#[inline(always)]
	fn query_type_or_data_type(&self) -> QueryTypeOrDataType
	{
		self.qtype
	}

	#[inline(always)]
	fn query_class(&self) -> Result<QueryClass, DnsProtocolError>
	{
		let (upper, lower) = unsafe { transmute::<_, (u8, u8)>(self.qclass) };

		if likely!(upper == 0x00)
		{
			if likely!(lower == 0x01)
			{
				return Ok(QueryClass::Internet)
			}
		}

		Err(ClassIsReservedUnassignedOrObsolete(self.qclass))
	}
}
