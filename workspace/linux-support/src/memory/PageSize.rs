// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Regular page size in bytes for the current process.
///
/// Some MIPS processors supports 1Kb and 2Kb page sizes, but Linux does not.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[repr(u64)]
pub enum PageSize
{
	/// 4Kb page size.
	///
	/// The only page size possible on powerpc64, riscv64 and x86_64.
	/// mips64 but not Loongson 2EF or Loongson 64.
	/// aarch64 with the 4Kb translation granule.
	_4Kb = 4_096,

	/// 8Kb page size.
	///
	/// The only page size possible on sparc64.
	/// mips64 for Cavium Octeon cnMIPS processors only.
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))]
	_8Kb = 8_192,

	/// 16Kb page size.
	///
	/// mips64 and aarch64 as with 16Kb translation granule.
	#[cfg(any(target_arch = "aarch64", target_arch = "mips64"))]
	_16Kb = 16_384,

	/// 32Kb page size.
	///
	/// mips64 for Cavium Octeon cnMIPS processors only.
	#[cfg(target_arch = "mips64")]
	_32Kb = 32_768,

	/// 64Kb page size.
	///
	/// mips64 and aarch64 with 64Kb translation granule.
	#[cfg(any(target_arch = "aarch64", target_arch = "mips64"))]
	_64Kb = 65_536,
}

impl Into<NonZeroU64> for PageSize
{
	#[inline(always)]
	fn into(self) -> NonZeroU64
	{
		self.into_non_zero_u64()
	}
}

impl Into<u64> for PageSize
{
	#[inline(always)]
	fn into(self) -> u64
	{
		self.into_u64()
	}
}

impl Into<NonZeroUsize> for PageSize
{
	#[inline(always)]
	fn into(self) -> NonZeroUsize
	{
		self.into_non_zero_usize()
	}
}

impl Into<usize> for PageSize
{
	#[inline(always)]
	fn into(self) -> usize
	{
		self.into_usize()
	}
}

impl Into<NonZeroU32> for PageSize
{
	#[inline(always)]
	fn into(self) -> NonZeroU32
	{
		self.into_non_zero_u32()
	}
}

impl Into<u32> for PageSize
{
	#[inline(always)]
	fn into(self) -> u32
	{
		self.into_u32()
	}
}

impl TryFrom<NonZeroU64> for PageSize
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: NonZeroU64) -> Result<Self, Self::Error>
	{
		Self::from_non_zero_bytes(value).ok_or(ParseNumberError::OutOfRange)
	}
}

impl TryFrom<u64> for PageSize
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: u64) -> Result<Self, Self::Error>
	{
		Self::from_bytes(value).ok_or(ParseNumberError::OutOfRange)
	}
}

impl TryFrom<NonZeroUsize> for PageSize
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: NonZeroUsize) -> Result<Self, Self::Error>
	{
		Self::try_from(value.get())
	}
}

impl TryFrom<usize> for PageSize
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: usize) -> Result<Self, Self::Error>
	{
		Self::try_from(value as u64)
	}
}

impl Default for PageSize
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::default()
	}
}

impl PageSize
{
	/// Into a `NonZeroU64`.
	pub const fn into_non_zero_u64(self) -> NonZeroU64
	{
		new_non_zero_u64(self as u64)
	}
	
	/// Into an `u64`.
	pub const fn into_u64(self) -> u64
	{
		self as u64
	}
	
	/// Into a `NonZeroUsize`.
	pub const fn into_non_zero_usize(self) -> NonZeroUsize
	{
		new_non_zero_usize(self.into_usize())
	}
	
	/// Into an `usize`.
	pub const fn into_usize(self) -> usize
	{
		self.into_u64() as usize
	}
	
	/// Into a `NonZeroU32`.
	pub const fn into_non_zero_u32(self) -> NonZeroU32
	{
		new_non_zero_u32(self.into_u32())
	}
	
	/// Into an `u32`.
	pub const fn into_u32(self) -> u32
	{
		self.into_u64() as u32
	}
	
	/// Exact page size multiple?
	#[inline(always)]
	pub fn is_an_exact_page_size_multiple_of_current_usize(value: usize) -> bool
	{
		Self::default().is_an_exact_page_size_multiple_usize(value)
	}

	/// Exact page size multiple?
	#[inline(always)]
	pub fn is_an_exact_page_size_multiple_usize(self, value: usize) -> bool
	{
		value % self.size_in_bytes().get() as usize == 0
	}

	/// Size in kilobytes.
	#[inline(always)]
	pub const fn size_in_kilobytes(self) -> NonZeroKilobyte
	{
		new_non_zero_u64((self as u64) / 1_024)
	}

	/// Size in bytes.
	#[inline(always)]
	pub const fn size_in_bytes(self) -> NonZeroU64
	{
		new_non_zero_u64(self as u64)
	}

	/// Non-zero number of pages from non-zero number of bytes, rounded up.
	#[inline(always)]
	pub fn non_zero_number_of_pages_from_non_zero_number_of_bytes_rounded_up(self, number_of_bytes: NonZeroU64) -> NonZeroNumberOfPages
	{
		new_non_zero_u64(self.number_of_pages_from_number_of_bytes_rounded_up(number_of_bytes.get()))
	}

	/// Number of pages from number of bytes, rounded up.
	#[inline(always)]
	pub fn number_of_pages_from_number_of_bytes_rounded_up(self, number_of_bytes: u64) -> NumberOfPages
	{
		let size_in_bytes = self.size_in_bytes().get();
		(number_of_bytes + size_in_bytes - 1) / size_in_bytes
	}

	/// Non-zero number of bytes rounded up to number of pages.
	#[inline(always)]
	pub fn non_zero_number_of_bytes_rounded_up_to_multiple_of_page_size(self, number_of_bytes: NonZeroU64) -> NonZeroU64
	{
		new_non_zero_u64(self.number_of_bytes_rounded_up_to_multiple_of_page_size(number_of_bytes.get()))
	}

	/// Number of bytes rounded up to number of pages.
	#[inline(always)]
	pub fn number_of_bytes_rounded_up_to_multiple_of_page_size(self, number_of_bytes: u64) -> u64
	{
		let size_in_bytes = self.size_in_bytes().get();
		((number_of_bytes + size_in_bytes - 1) / size_in_bytes) * size_in_bytes
	}

	/// Is this considered a gigantic huge page?
	#[inline(always)]
	pub fn can_have_a_dynamic_huge_page_pool(self) -> bool
	{
		false
	}

	/// Is this considered a gigantic huge page?
	#[inline(always)]
	pub fn is_a_gigantic_huge_page(self) -> bool
	{
		false
	}

	/// A very slightly faster function to get page size than `sysconf(_SC_PAGESIZE)` on musl libc systems.
	///
	/// On powerpc64, riscv64, sparc64 and x86_64, the value is truly constant; otherwise, it is effectively constant and is derived from data passed when an executable is first loaded.
	#[cfg(any(target_arch = "powerpc64", target_arch = "riscv64", target_arch = "x86_64"))]
	#[inline(always)]
	pub const fn default() -> Self
	{
		Self::_4Kb
	}

	/// A very slightly faster function to get page size than `sysconf(_SC_PAGESIZE)` on musl libc systems.
	///
	/// On powerpc64, sparc64 and x86_64, the value is truly constant; otherwise, it is effectively constant and is derived from data passed when an executable is first loaded.
	#[cfg(target_arch = "sparc64")]
	#[inline(always)]
	pub const fn default() -> Self
	{
		Self::_8Kb
	}

	/// A very slightly faster function to get page size than `sysconf(_SC_PAGESIZE)` on musl libc systems.
	///
	/// On powerpc64, sparc64 and x86_64, the value is truly constant; otherwise, it is effectively constant and is derived from data passed when an executable is first loaded.
	#[cfg(not(any(target_arch = "powerpc64", target_arch = "riscv64", target_arch = "sparc64", target_arch = "x86_64")))]
	#[inline(always)]
	pub fn default() -> Self
	{
		// `getpagesize()` is faster than `sysconf(_SC_PAGESIZE)` on musl libc systems.
		unsafe { transmute(getpagesize() as usize) }
	}
	
	#[inline(always)]
	pub(crate) fn from_non_zero_bytes(bytes: NonZeroU64) -> Option<Self>
	{
		Self::from_bytes(bytes.get())
	}
	
	#[inline(always)]
	pub(crate) fn from_bytes(bytes: u64) -> Option<Self>
	{
		use self::PageSize::*;
		
		match bytes
		{
			4_096 => Some(_4Kb),
			
			#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] 8_192 => Some(_8Kb),
			
			#[cfg(any(target_arch = "aarch64", target_arch = "mips64"))] 16_384 => Some(_16Kb),
			
			#[cfg(target_arch = "mips64")] 32_768 => Some(_32Kb),
			
			#[cfg(any(target_arch = "aarch64", target_arch = "mips64"))] 65_536 => Some(_64Kb),
			
			_ => None,
		}
	}
	
	
	/// From kilobytes (non-zero).
	#[inline(always)]
	pub fn from_non_zero_kilobytes(kilobytes: NonZeroU64) -> Option<Self>
	{
		Self::from_kilobytes(kilobytes.get())
	}
	
	/// From kilobytes.
	#[inline(always)]
	pub fn from_kilobytes(kilobytes: u64) -> Option<Self>
	{
		use self::PageSize::*;
		
		match kilobytes
		{
			4 => Some(_4Kb),
			
			#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] 8 => Some(_8Kb),
			
			#[cfg(any(target_arch = "aarch64", target_arch = "mips64"))] 16 => Some(_16Kb),
			
			#[cfg(target_arch = "mips64")] 32 => Some(_32Kb),
			
			#[cfg(any(target_arch = "aarch64", target_arch = "mips64"))] 64 => Some(_64Kb),
			
			_ => None,
		}
	}

}
