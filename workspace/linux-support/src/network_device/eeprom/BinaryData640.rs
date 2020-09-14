// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Wrapper type.
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[repr(transparent)]
pub struct BinaryData640
{
	#[serde(with = "PluginModuleBigArray")] binary_data: [u8; BinaryData640::LargeSize]
}

impl<'a> From<&'a [u8]> for BinaryData640
{
	#[inline(always)]
	#[allow(deprecated)]
	fn from(binary_data: &'a [u8]) -> Self
	{
		let slice = &binary_data[.. BinaryData640::LargeSize];
		let mut binary_data: [u8; BinaryData640::LargeSize] = unsafe { uninitialized() };
		unsafe { binary_data.as_mut_ptr().copy_from_nonoverlapping(slice.as_ptr(), BinaryData640::LargeSize) }
		Self { binary_data }
	}
}

impl From<Vec<u8>> for BinaryData640
{
	#[inline(always)]
	#[allow(deprecated)]
	fn from(binary_data: Vec<u8>) -> Self
	{
		Self::from(&binary_data[.. BinaryData640::LargeSize])
	}
}

impl BinaryData640
{
	/// Large page size of 640 bytes.
	pub const LargeSize: usize = 640;
}
