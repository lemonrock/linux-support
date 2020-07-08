// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// License.
///
/// Defaults to `GPLv2`.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct BpfProgramLicense(ConstCStr);

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
	/// This is a GPL-compatible license.
	pub const GPL: Self = Self(ConstCStr(b"GPL\0"));
	
	/// This is a GPL-compatible license.
	pub const GPLv2: Self = Self(ConstCStr(b"GPL v2\0"));
	
	/// This is a GPL-compatible license.
	pub const GPLAndAdditionalRights: Self = Self(ConstCStr(b"GPL and additional rights\0"));
	
	/// This is a GPL-compatible license.
	pub const DualBSDAndGPL: Self = Self(ConstCStr(b"Dual BSD/GPL\0"));
	
	/// This is a GPL-compatible license.
	pub const DualMITAndGPL: Self = Self(ConstCStr(b"Dual MIT/GPL\0"));
	
	/// This is a GPL-compatible license.
	pub const DualMPLAndGPL: Self = Self(ConstCStr(b"Dual MPL/GPL\0"));
}
