// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Is never empty.
///
/// Is usually 40 bytes but Intel i40e drivers use a 52 byte key.
///
/// For Intel ixgbevf, the key is 40 (`IXGBEVF_RSS_HASH_KEY_SIZE`) bytes.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct HashFunctionSeed(pub(crate) Vec<u8>);

impl Deref for HashFunctionSeed
{
	type Target = Vec<u8>;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl DerefMut for HashFunctionSeed
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		&mut self.0
	}
}

impl HashFunctionSeed
{
	#[inline(always)]
	fn copy_from_non_overlapping(&mut self, original_length: usize, copy: usize, length: usize)
	{
		unsafe { self.as_mut_ptr().add(original_length).add(copy * original_length).copy_from_nonoverlapping(self.as_ptr(), length) };
	}
	
	/// Resize.
	#[inline(always)]
	pub fn resize(&mut self, size: usize)
	{
		let original_length = self.len();
		
		if likely!(original_length == size)
		{
			return
		}
		else if likely!(original_length < size)
		{
			let additional_length = size - original_length;
			self.reserve_exact(additional_length);
			unsafe { self.set_len(size) };
			
			let number_of_copies = additional_length / original_length;
			
			for copy in 0 .. number_of_copies
			{
				self.copy_from_non_overlapping(original_length, copy, original_length)
			}
			
			let remainder = additional_length % original_length;
			self.copy_from_non_overlapping(original_length, number_of_copies, remainder)
		}
		else
		{
			self.truncate(size)
		}
	}
	
	/// Microsoft key, found at <http://www.ran-lifshitz.com/2014/08/28/symmetric-rss-receive-side-scaling/>.
	///
	/// Also the default key for:-
	///
	/// * Intel ixgbe.
	/// * Intel E1000.
	/// * Intel FM10K.
	/// * LiquidIO.
	/// * VMWare vmnet3.
	///
	/// Good distribution apparently.
	///
	/// 40 bytes.
	#[inline(always)]
	pub fn toeplitz_microsoft() -> Self
	{
		Self
		(
			vec!
			[
				0x6D, 0x5A, 0x56, 0xDA, 0x25, 0x5B, 0x0E, 0xC2,
				0x41, 0x67, 0x25, 0x3D, 0x43, 0xA3, 0x8F, 0xB0,
				0xD0, 0xCA, 0x2B, 0xCB, 0xAE, 0x7B, 0x30, 0xB4,
				0x77, 0xCB, 0x2D, 0xA3, 0x80, 0x30, 0xF2, 0x0C,
				0x6A, 0x42, 0xB7, 0x3B, 0xBE, 0xAC, 0x01, 0xFA,
			]
		)
	}
	
	/// Symmetric with good queue distribution, found at <http://www.ran-lifshitz.com/2014/08/28/symmetric-rss-receive-side-scaling/> and <https://galsagie.github.io/2015/02/26/dpdk-tips-1/>.
	///
	/// Essential when applying RSS to both sides of a TCP or UDP connection, eg if one is a man-in-the-middle.
	///
	/// 40 bytes.
	#[inline(always)]
	pub fn toeplitz_symmetric() -> Self
	{
		Self
		(
			vec!
			[
				0x6D, 0x5A, 0x6D, 0x5A, 0x6D, 0x5A, 0x6D, 0x5A,
				0x6D, 0x5A, 0x6D, 0x5A, 0x6D, 0x5A, 0x6D, 0x5A,
				0x6D, 0x5A, 0x6D, 0x5A, 0x6D, 0x5A, 0x6D, 0x5A,
				0x6D, 0x5A, 0x6D, 0x5A, 0x6D, 0x5A, 0x6D, 0x5A,
				0x6D, 0x5A, 0x6D, 0x5A, 0x6D, 0x5A, 0x6D, 0x5A,
			]
		)
	}
	
	/// Default Mellanox key.
	///
	/// 40 bytes.
	#[inline(always)]
	pub fn mellanox_default() -> Self
	{
		Self
		(
			vec!
			[
				0xD1, 0x81, 0xC6, 0x2C, 0xF7, 0xF4, 0xDB, 0x5B,
				0x19, 0x83, 0xA2, 0xFC, 0x94, 0x3E, 0x1A, 0xDB,
				0xD9, 0x38, 0x9E, 0x6B, 0xD1, 0x03, 0x9C, 0x2C,
				0xA7, 0x44, 0x99, 0xAD, 0x59, 0x3D, 0x56, 0xD9,
				0xF3, 0x25, 0x3C, 0x06, 0x2A, 0xDC, 0x1F, 0xFC,
			]
		)
	}
	
	/// Intel i40e default key.
	///
	/// 52 bytes.
	#[inline(always)]
	pub fn intel_i40e_default() -> Self
	{
		Self
		(
			vec!
			[
				0x44, 0x39, 0x79, 0x6B, 0xB5, 0x4C, 0x50, 0x23,
				0xB6, 0x75, 0xEA, 0x5B, 0x12, 0x4F, 0x9F, 0x30,
				0xB8, 0xA2, 0xC0, 0x3D, 0xDF, 0xDC, 0x4D, 0x02,
				0xA0, 0x8C, 0x9B, 0x33, 0x4A, 0xF6, 0x4A, 0x4C,
				0x05, 0xC6, 0xFA, 0x34, 0x39, 0x58, 0xD8, 0x55,
				0x7D, 0x99, 0x58, 0x3A, 0xE1, 0x38, 0xC9, 0x2E,
				0x81, 0x15, 0x03, 0x66,
			]
		)
	}
}
