// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Location resource data using [WGS84](https://en.wikipedia.org/wiki/World_Geodetic_System#WGS84).
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(C, packed)]
pub struct LocationBodyVersion0
{
	/// Size of a sphere at this location.
	pub size: LocationCompressedCentimetres,
	
	/// Horizontal precision.
	pub horizontal_precision: LocationCompressedCentimetres,
	
	/// Vertical precision.
	pub vertical_precision: LocationCompressedCentimetres,
	
	/// The latitude of the center of the sphere described by `size`, expressed as a 32-bit integer, most significant octet first (network standard byte order), in thousandths of a second of arc.
	///
	/// 2^31 represents the equator; numbers above that are north latitude.
	pub unsigned_latitude: BigEndianU32,
	
	/// The longitude of the center of the sphere described by `size`, expressed as a 32-bit integer, most significant octet first (network standard byte order), in thousandths of a second of arc.
	///
	/// 2^31 represents the prime meridian; numbers above that are east longitude.
	pub unsigned_longitude: BigEndianU32,
	
	/// The altitude of the center of the sphere described by `size`, expressed as a 32-bit integer, most significant octet first (network standard byte order), in centimeters, from a base of 100,000m below the WGS 84 reference spheroid used by GPS.
	pub unsigned_altitude: BigEndianU32,
}

impl<'message> ParsedRecord for &'message LocationBodyVersion0
{
	type OrderPriorityAndWeight = ();
	
	type OwnedRecord = Self;
	
	#[inline(always)]
	fn into_owned_records(records: OwnerNameToParsedRecordsValue<Self>) -> <Self::OwnedRecord as OwnedRecord>::OwnedRecords
	{
		let mut parsed_records = records.records();
		
		let length = parsed_records.len();
		let mut owned_records = Vec::with_capacity(length);
		unsafe{ owned_records.set_len(length) };
		
		let mut index = 0;
		for (parsed_record, _) in parsed_records
		{
			let owned_record: LocationBodyVersion0 = parsed_record.clone();
			unsafe { owned_records.as_mut_ptr().add(index).write(owned_record) }
			index + 1;
		}
		
		owned_records.sort_unstable();
		MultipleUnsortedRecords::new(owned_records)
	}
}

impl OwnedRecord for LocationBodyVersion0
{
	type OwnedRecords = MultipleUnsortedRecords<Self>;
	
	#[inline(always)]
	fn retrieve(query_types_cache: &mut QueryTypesCache) -> &Option<QueryTypeCache<Self::OwnedRecords>>
	{
		&query_types_cache.LOC_version_0
	}
	
	#[inline(always)]
	fn retrieve_mut(query_types_cache: &mut QueryTypesCache) -> &mut Option<QueryTypeCache<Self::OwnedRecords>>
	{
		&mut query_types_cache.LOC_version_0
	}
}
