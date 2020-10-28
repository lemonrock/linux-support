// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Location resource data using [WGS84](https://en.wikipedia.org/wiki/World_Geodetic_System#WGS84).
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
