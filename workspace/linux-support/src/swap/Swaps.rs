// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Swaps.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct Swaps(Vec<SwapLine>);

impl Deref for Swaps
{
	type Target = Vec<SwapLine>;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl FromBytes for Swaps
{
	type Error = io::Error;
	
	fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>
	{
		let index = memchr(b'\n', &bytes[..]).ok_or(io_error_invalid_data("No header line"))?;
		let bytes_without_header_line = &bytes[(index + 1) .. ];
		
		let mut swap_lines = Vec::new();
		for line in bytes_without_header_line.split_bytes(b'\n')
		{
			swap_lines.push(SwapLine::from_bytes(line)?);
		}
		Ok(Self(swap_lines))
	}
}

impl Swaps
{
	/// Equivalent to `swapoff -a`.
	pub fn swap_off_all(&self) -> io::Result<()>
	{
		for swap_line in self.iter()
		{
			swap_line.swap_off()?;
		}
		Ok(())
	}
	
	/// Parse.
	#[inline(always)]
	pub fn parse(proc_path: &ProcPath) -> io::Result<Self>
	{
		let bytes = proc_path.file_path("swaps").read_raw()?;
		Self::from_bytes(&bytes[..])
	}
}
