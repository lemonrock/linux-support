// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Original environment of a process when it was started.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandLine(Vec<Box<[u8]>>);

impl Deref for CommandLine
{
	type Target = Vec<Box<[u8]>>;

	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl FromBytes for CommandLine
{
	type Error = io::Error;

	#[inline(always)]
	fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>
	{
		match parse_ascii_nul_string_values::<Vec<Box<[u8]>>, _>(bytes, & |collection, ascii_string|
		{
			collection.push(ascii_string.to_vec().into_boxed_slice());
			Ok(())
		})
		{
			Ok(collection) => Ok(Self(collection)),
			Err(reason) => Err(io::Error::new(ErrorKind::Other, reason)),
		}
	}
}

impl CommandLine
{
	/// For self.
	#[inline(always)]
	pub fn for_self(proc_path: &ProcPath) -> io::Result<Self>
	{
		Self::for_process(proc_path, ProcessIdentifierChoice::Current)
	}

	/// For process.
	#[inline(always)]
	pub fn for_process(proc_path: &ProcPath, process_identifier: ProcessIdentifierChoice) -> io::Result<Self>
	{
		let bytes = proc_path.process_file_path(process_identifier, "cmdline").read_raw()?;
		Self::from_bytes(&bytes)
	}
}
