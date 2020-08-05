// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Choice of lock down state.
///
/// Once set, can only be made harder until reboot, ie if `Integrity` is set, then it so no longer possible to use `Off`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum LockDownState
{
	Off,

	Integrity,

	Confidentiality,
}

impl Default for LockDownState
{
	#[inline(always)]
	fn default() -> Self
	{
		LockDownState::Off
	}
}

impl LockDownState
{
	/// Value of `/sys/kernel/security/lockdown`.
	///
	/// Only readable by root.
	pub fn current(sys_path: &SysPath) -> io::Result<Self>
	{
		assert_effective_user_id_is_root("read from /sys/kernel/security/lockdown");
		
		let file_path = Self::file_path(sys_path);
		
		// eg `none integrity [confidentiality]`.
		let bytes = file_path.read_raw_without_line_feed()?;
		let mut iterator = bytes.split_bytes_n(3, b' ');
		for raw_bytes in iterator
		{
			let length = raw_bytes.len();
			if length >= 2
			{
				let last_index = length - 1;
				let first = unsafe { *raw_bytes.get_unchecked(0) };
				let last = unsafe { *raw_bytes.get_unchecked(last_index) };
				if first == b'[' && last == b']'
				{
					use self::LockDownState::*;
					
					return match &raw_bytes[1 .. last_index]
					{
						b"none" => Ok(Off),
						b"integrity" => Ok(Integrity),
						b"confidentiality" => Ok(Confidentiality),
						inner @ _ => Err(io::Error::new(ErrorKind::InvalidData, format!("Did not expect lock down state `{:?}`", inner))),
					}
				}
			}
		}
		
		Err(io::Error::new(ErrorKind::InvalidData, "No active lock down state"))
	}
	
	/// Set value of `/sys/kernel/security/lockdown`.
	///
	/// Only writable by root.
	///
	/// Will not make any changes, or read from the file system as root, if the value is `Off`.
	/// Will not make any changes if the value has already been set to the same or a higher value.
	pub fn set(self, sys_path: &SysPath) -> io::Result<()>
	{
		if self == LockDownState::Off
		{
			return Ok(())
		}
		
		let current = Self::current(sys_path)?;
		if current >= self
		{
			return Ok(())
		}
		
		assert_effective_user_id_is_root("write to /sys/kernel/security/lockdown");
		
		let file_path = Self::file_path(sys_path);
		if file_path.exists()
		{
			file_path.write_value(self.to_bytes_for_writing())
		}
		else
		{
			Ok(())
		}
	}
	
	#[inline(always)]
	fn to_bytes_for_writing(self) -> &'static [u8]
	{
		use self::LockDownState::*;
		
		match self
		{
			Off => unreachable!("We should never be writing this value"),
			
			Integrity => b"integrity\n" as &[u8],
			
			Confidentiality => b"confidentiality\n" as &[u8],
		}
	}
	
	#[inline(always)]
	fn file_path(sys_path: &SysPath) -> PathBuf
	{
		sys_path.kernel_security_file_path("lockdown")
	}
}
