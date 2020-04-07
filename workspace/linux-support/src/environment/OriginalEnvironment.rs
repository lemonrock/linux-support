// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Original environment of a process when it was started.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OriginalEnvironment(HashMap<Box<[u8]>, Box<[u8]>>);

impl FromBytes for OriginalEnvironment
{
	type Error = io::Error;

	#[inline(always)]
	fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>
	{
		let length = bytes.len();

		if unlikely!(length == 0)
		{
			return Ok(Self(HashMap::default()))
		}

		const AsciiNul: u8 = b'\0';

		let final_byte_index = length - 1;
		let final_byte = unsafe { *bytes.get_unchecked(final_byte_index) };
		if unlikely!(final_byte != AsciiNul)
		{
			return Err(io::Error::new(ErrorKind::Other, "/proc/self/environ must end with an Ascii NUL"));
		}

		let mut map = HashMap::new();
		for pair in bytes[0 .. final_byte_index].split(|byte| *byte == AsciiNul)
		{
			let mut iterator = pair.splitn(2, |byte| *byte == b'=');
			let environment_variable_name = iterator.next().unwrap().to_vec().into_boxed_slice();
			let environment_variable_value = iterator.next().ok_or_else(|| io::Error::new(ErrorKind::Other, "/proc/self/environ value must have '='"))?.to_vec().into_boxed_slice();
			if map.insert(environment_variable_name, environment_variable_value).is_some()
			{
				return Err(io::Error::new(ErrorKind::Other, "/proc/self/environ key duplicated"))
			}
		}
		Ok(Self(map))
	}
}

impl OriginalEnvironment
{
	/// Contains a specific environment variable?
	#[inline(always)]
	pub fn contains_environment_variable(&self, variable_name: &[u8]) -> bool
	{
		self.0.contains_key(variable_name)
	}

	/// Get a specific environment variable's value.
	#[inline(always)]
	pub fn get_environment_variable<'a>(&'a self, variable_name: &[u8]) -> Option<&'a Box<[u8]>>
	{
		self.0.get(variable_name)
	}

	/// Iterate.
	#[inline(always)]
	pub fn iterate(&self) -> impl Iterator<Item=(&Box<[u8]>, &Box<[u8]>)>
	{
		self.0.iter()
	}
}
