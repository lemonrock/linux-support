// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Ring queue depth.
///
/// Must be a power of two.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(u32)]
pub enum RingQueueDepth
{
	/// `1`.
	#[serde(rename = "1")] _1 = 1,
	
	/// `2`.
	#[serde(rename = "2")] _2 = 2,
	
	/// `4`.
	#[serde(rename = "4")] _4 = 4,
	
	/// `8`.
	#[serde(rename = "8")] _8 = 8,
	
	/// `16`.
	#[serde(rename = "16")] _16 = 16,
	
	/// `32`.
	#[serde(rename = "32")] _32 = 32,
	
	/// `64`.
	#[serde(rename = "64")] _64 = 64,
	
	/// `128`.
	#[serde(rename = "128")] _128 = 128,
	
	/// `256`.
	#[serde(rename = "256")] _256 = 256,
	
	/// `512`.
	#[serde(rename = "512")] _512 = 512,
	
	/// `1,024`.
	#[serde(rename = "1_024")] _1_024 = 1_024,
	
	/// `2,048`.
	///
	/// This is the same as `libbpf`'s `XSK_RING_PROD__DEFAULT_NUM_DESCS` and `XSK_RING_CONS__DEFAULT_NUM_DESCS`.
	#[serde(rename = "2_048")] _2_048 = 2_048,
	
	/// `4,096`.
	#[serde(rename = "4_096")] _4_096 = 4_096,
	
	/// `8.192`.
	#[serde(rename = "8_192")] _8_192 = 8_192,
	
	/// `16,384`.
	#[serde(rename = "16_384")] _16_384 = 16_384,
	
	/// `32,768`.
	#[serde(rename = "32_768")] _32_768 = 32_768,
	/// `65,536`.
	#[serde(rename = "65_536")] _65_536 = 65_536,
	
	/// `131,072`.
	#[serde(rename = "131_072")] _131_072 = 131_072,
	
	/// `262,144`.
	#[serde(rename = "262_144")] _262_144 = 262_144,
	
	/// `524,288`.
	#[serde(rename = "524_288")] _524_288 = 524_288,
	
	/// `1,048,57`.
	#[serde(rename = "1_048_576")] _1_048_576 = 1_048_576,
	
	/// `2,097,15`.
	#[serde(rename = "2_097_152")] _2_097_152 = 2_097_152,
	
	/// `4,194,30`.
	#[serde(rename = "4_194_304")] _4_194_304 = 4_194_304,
	
	/// `8,388,608`.
	#[serde(rename = "8_388_608")] _8_388_608 = 8_388_608,

	/// `16,777,216`.
	#[serde(rename = "16_777_216")] _16_777_216 = 16_777_216,
	
	/// `33,554,432`.
	#[serde(rename = "33_554_432")] _33_554_432 = 33_554_432,
	
	/// `67,108,864`.
	#[serde(rename = "67_108_864")] _67_108_864 = 67_108_864,
	
	/// `134,217,728`.
	#[serde(rename = "134_217_728")] _134_217_728 = 134_217_728,
	
	/// `268,435,456`.
	#[serde(rename = "268_435_456")] _268_435_456 = 268_435_456,
	
	/// `536,870,912`.
	#[serde(rename = "536_870_912")] _536_870_912 = 536_870_912,
	
	/// `1,073,741,824`.
	#[serde(rename = "1_073_741_824")] _1_073_741_824 = 1_073_741_824,
	
	/// `2,147,483,648`.
	#[serde(rename = "2_147_483_648")] _2_147_483_648 = 2_147_483_648,
}

impl Default for RingQueueDepth
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::_1
	}
}

impl Into<NonZeroU32> for RingQueueDepth
{
	#[inline(always)]
	fn into(self) -> NonZeroU32
	{
		unsafe { transmute(self )}
	}
}

impl RingQueueDepth
{
	#[inline(always)]
	pub(crate) const fn memory_length<D: Descriptor>(self) -> usize
	{
		(self as u32 as usize) * size_of::<D>()
	}
	
	#[inline(always)]
	fn mask(self) -> u32
	{
		(self as u32) - 1
	}
}
