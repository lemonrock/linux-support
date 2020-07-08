// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Program name.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct ProgramName(pub CommandName);

impl Deref for ProgramName
{
	type Target = CommandName;

	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl AsRef<CStr> for ProgramName
{
	#[inline(always)]
	fn as_ref(&self) -> &CStr
	{
		self.0.as_ref()
	}
}

impl AsRef<[u8]> for ProgramName
{
	#[inline(always)]
	fn as_ref(&self) -> &[u8]
	{
		self.0.deref()
	}
}

impl ToString for ProgramName
{
	#[inline(always)]
	fn to_string(&self) -> String
	{
		self.0.to_string()
	}
}

impl Default for ProgramName
{
	#[inline(always)]
	fn default() -> Self
	{
		Self(CommandName::new_from_bytes_excluding_ascii_nul(b"").unwrap())
	}
}

impl ProgramName
{
	#[inline(always)]
	fn prog_name(&self) -> [c_char; BPF_OBJ_NAME_LEN]
	{
		debug_assert_eq!(BPF_OBJ_NAME_LEN, CommandName::MaximumCommandNameLengthIncludingAsciiNul);
		
		let array_vec = self.0.clone().into();
		let const_array_vec: ConstArrayVec<[u8; BPF_OBJ_NAME_LEN]> = unsafe { transmute(array_vec) };
		unsafe { transmute(const_array_vec.xs) }
	}
}
