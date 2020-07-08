// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// License.
///
/// Defaults to `GPLv2`.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum BpfProgramLicense
{
	/// This is a GPL-compatible license.
	GPL,
	
	/// This is a GPL-compatible license.
	GPLv2,
	
	/// This is a GPL-compatible license.
	GPLAndAdditionalRights,
	
	/// This is a GPL-compatible license.
	DualBSDAndGPL,
	
	/// This is a GPL-compatible license.
	DualMITAndGPL,
	
	/// This is a GPL-compatible license.
	DualMPLAndGPL,

	/// Any other license kind.
	Other(CString),
}

impl Default for BpfProgramLicense
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::GPLv2
	}
}

impl BpfProgramLicense
{
	#[inline(always)]
	pub(crate) fn as_aligned_u64(&self) -> AlignedU64
	{
		use self::BpfProgramLicense::*;
		
		let pointer = match self
		{
			GPL => b"GPL\0".as_ptr(),
			
			GPLv2 => b"GPL v2\0".as_ptr(),
			
			GPLAndAdditionalRights => b"GPL and additional rights\0".as_ptr(),
			
			DualBSDAndGPL => b"Dual BSD/GPL\0".as_ptr(),
			
			DualMITAndGPL => b"Dual MIT/GPL\0".as_ptr(),
			
			DualMPLAndGPL => b"Dual MPL/GPL\0".as_ptr(),
			
			Other(other) => other.as_ptr() as *const u8
		};
		AlignedU64::from(pointer)
	}
	
}
