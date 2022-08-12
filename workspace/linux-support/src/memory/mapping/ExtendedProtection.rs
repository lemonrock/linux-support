// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Memory protection.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[repr(i32)]
pub enum ExtendedProtection
{
	/// No access allowed.
	Inaccessible = PROT_NONE,

	/// Readable.
	Read = PROT_READ,

	/// Readable and Writable.
	ReadWrite = PROT_READ | PROT_WRITE,

	/// Readable and Executable (eg for binaries).
	ReadExecutable = PROT_READ | PROT_EXEC,

	/// Readable, Writable and Executable (eg for generated machine code).
	ReadWriteExecutable = PROT_READ | PROT_WRITE | PROT_EXEC,

	/// Grows up.
	///
	/// This flag cannot be used securely because using it means that collisions with other allocations cannot be avoided.
	///
	/// It is unclear what happens if this flag is applied to memory mmap-ed with `MAP_GROWSDOWN` or previously protected with `Self::GrowsDown`.
	GrowsUp = PROT_GROWSUP,

	/// Grows down.
	///
	/// This flag cannot be used securely because using it means that collisions with other allocations cannot be avoided.
	///
	/// It is unclear what happens if this flag is applied to memory mmap-ed with `MAP_GROWSDOWN` or previously protected with `Self::GrowsUp`.
	GrowsDown = PROT_GROWSDOWN,

	/// Since Linux 2.6.26.
	#[cfg(target_arch = "powerpc64")] StrongAccessOrdering = Self::PROT_SAO,
}

impl From<Protection> for ExtendedProtection
{
	#[inline(always)]
	fn from(protection: Protection) -> Self
	{
		unsafe { transmute(protection) }
	}
}

impl Into<Option<Protection>> for ExtendedProtection
{
	#[inline(always)]
	fn into(self) -> Option<Protection>
	{
		use self::ExtendedProtection::*;

		match self
		{
			Inaccessible => Some(Protection::Inaccessible),
			Read => Some(Protection::Read),
			ReadWrite => Some(Protection::ReadWrite),
			ReadExecutable => Some(Protection::ReadExecutable),
			ReadWriteExecutable => Some(Protection::ReadWriteExecutable),

			GrowsUp | GrowsDown => None,
			#[cfg(target_arch = "powerpc64")] StrongAccessOrdering => None,
		}
	}
}

impl Default for ExtendedProtection
{
	#[inline(always)]
	fn default() -> Self
	{
		ExtendedProtection::ReadWrite
	}
}

impl ExtendedProtection
{
	#[cfg(target_arch = "powerpc64")]
	const PROT_SAO: i32 = 0x10;
}
