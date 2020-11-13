// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Original environment of a process when it was started.
#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct Environment(HashMap<Box<[u8]>, Vec<Option<Box<[u8]>>>>);

impl Deref for Environment
{
	type Target = HashMap<Box<[u8]>, Vec<Option<Box<[u8]>>>>;

	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl FromBytes for Environment
{
	type Error = io::Error;

	#[inline(always)]
	fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>
	{
		match parse_ascii_nul_string_values::<HashMap<Box<[u8]>, Vec<Option<Box<[u8]>>>>, _>(bytes, & |collection, byte_string|
		{
			Self::parse_line(collection, byte_string);
			Ok(())
		})
		{
			Ok(collection) => Ok(Self(collection)),
			Err(reason) => Err(io_error_other(reason)),
		}
	}
}

impl Environment
{
	/// Get a value, securely (returns `None` if this is a setuid/setgid binary).
	#[inline(always)]
	pub fn get_value_securely<'a>(name: &CStr) -> Option<&'a CStr>
	{
		let pointer = unsafe { secure_getenv(name.as_ptr()) };
		if pointer.is_null()
		{
			None
		}
		else
		{
			Some(unsafe { CStr::from_ptr(pointer) })
		}
	}
	
	/// As passed to `main()` and held in the libc global static `environ`.
	///
	/// Returns `None` if this pointer is `NULL`.
	///
	/// This method *CAN NOT BE TRULLY THREAD SAFE* if another thread modifies the libc global static `environ`.
	#[inline(always)]
	pub fn from_environ_pointer() -> Option<Self>
	{
		let environ_copy = unsafe { environ };
		if unlikely!(environ_copy.is_null())
		{
			return None
		}

		let mut pointer = environ_copy as *const *const c_char;
		let mut collection = HashMap::default();
		loop
		{
			let c_str = unsafe { CStr::from_ptr(* pointer) };
			let byte_string = c_str.to_bytes();
			if unlikely!(byte_string.is_empty())
			{
				break
			}

			Self::parse_line(&mut collection, byte_string);
			pointer = unsafe { pointer.add(1) };
		}

		Some(Self(collection))
	}

	/// For self.
	///
	/// May not be the same as `Self::from_environ_pointer_to_main()`.
	#[inline(always)]
	pub fn original_for_self(proc_path: &ProcPath) -> io::Result<Self>
	{
		Self::original_for_process(proc_path, ProcessIdentifierChoice::Current)
	}

	/// For process.
	#[inline(always)]
	pub fn original_for_process(proc_path: &ProcPath, process_identifier: ProcessIdentifierChoice) -> io::Result<Self>
	{
		let bytes = proc_path.process_file_path(process_identifier, "environ").read_raw()?;
		Self::from_bytes(&bytes)
	}

	/// Contains a specific environment variable?
	///
	/// Ignores variables that have no value or are duplicates.
	#[inline(always)]
	pub fn contains_environment_variable(&self, variable_name: &[u8]) -> bool
	{
		self.0.contains_key(variable_name)
	}

	/// Get a specific environment variable's value.
	///
	/// Ignores variables that have no value or are duplicates (unlike libc `getenv()`).
	///
	/// If a variable has duplicates but the first has no value, then `None` is returned.
	#[inline(always)]
	pub fn get_environment_variable<'a>(&'a self, variable_name: &[u8]) -> Option<&'a [u8]>
	{
		match self.0.get(variable_name)
		{
			None => None,
			Some(list) =>
			{
				let length = list.len();
				debug_assert_ne!(length, 0);
				if likely!(length == 1)
				{
					let value = list.get_unchecked_safe(0);
					value.as_ref().map(|value| &value[..])
				}
				else
				{
					None
				}
			}
		}
	}

	#[inline(always)]
	pub(crate) fn to_environment_c_string_array(&self) -> NulTerminatedCStringArray
	{
		let c_strings_as_fragments_to_combine = self.iter().flat_map(|(name, list_of_values)| list_of_values.iter().map(move |value| (name, value)));

		NulTerminatedCStringArray::new(c_strings_as_fragments_to_combine, PageSize::current())
	}

	#[inline(always)]
	fn parse_line(collection: &mut HashMap<Box<[u8]>, Vec<Option<Box<[u8]>>>>, byte_string: &[u8])
	{
		let mut iterator = byte_string.split_bytes_n(2, b'=');
		let environment_variable_name = iterator.next().unwrap().to_vec().into_boxed_slice();
		let environment_variable_value = iterator.next().map(|value| value.to_vec().into_boxed_slice());

		collection.entry(environment_variable_name).or_insert_with(|| Vec::with_capacity(1)).push(environment_variable_value);
	}
}
