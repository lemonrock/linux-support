// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Registered file descriptor index.
///
/// Currently limited to an inclusive maximum value of 32,767.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct RegisteredFileDescriptorIndex(u32);

impl RegisteredFileDescriptorIndex
{
	const IORING_FILE_TABLE_SHIFT: u32 = 9;
	
	const IORING_MAX_FILES_TABLE: u32 =	1 << Self::IORING_FILE_TABLE_SHIFT;
	
	const IORING_MAX_FIXED_FILES: u32 = 64 * Self::IORING_MAX_FILES_TABLE;

	/// Inclusive maximum.
	pub const InclusiveMaximum: Self = RegisteredFileDescriptorIndex(Self::IORING_MAX_FIXED_FILES - 1);

	/// Exclusive maximum.
	pub const ExclusiveMaximum: NonZeroU64 = unsafe { NonZeroU64::new_unchecked(Self::IORING_MAX_FIXED_FILES as u64) };
}
