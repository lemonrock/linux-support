// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Based on the maximum label size in DNS of 63 bytes, not DataDog documentation.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HostNameLabel(ArrayVec<[u8; Self::Length]>);

impl HostNameLabel
{
	const Length: usize = 63;
	
	/// New instance.
	#[inline(always)]
	pub fn from_linux_kernel_host_name(label: &LinuxKernelHostName) -> Result<Self, String>
	{
		// ASCII letters a through z (in a case-insensitive manner), the digits 0 through 9, and the hyphen-minus character ('-').
		
		let bytes = &label[..];
		
		let length = bytes.len();
		
		if unlikely!(length == 0)
		{
			return Err("Can not be empty".to_string())
		}
		
		if unlikely!(length > Self::Length)
		{
			return Err(format!("Can not be more than `{}` bytes long (`{:?}` is invalid)", Self::Length, label))
		}
		
		let mut digits_count = 0;
		
		match unsafe { * bytes.get_unchecked(0) }
		{
			b'0' ..= b'9' => digits_count += 1,
			b'A' ..= b'Z' => (),
			b'a' ..= b'z' => (),
			first_byte @ _ => return Err(format!("First byte can not be '0x{:02X}' in `{:?}`", first_byte, label))
		}
		
		let final_byte_index = length - 1;
		
		let subsequent_bytes = &bytes[1 ..final_byte_index];
		for subsequent_byte in subsequent_bytes
		{
			match *subsequent_byte
			{
				b'0' ..= b'9' => digits_count += 1,
				b'A' ..= b'Z' => (),
				b'a' ..= b'z' => (),
				b'-' => (),
				subsequent_byte @ _ => return Err(format!("Subsequent byte can not be '0x{:02X}' in `{:?}`", subsequent_byte, label))
			}
		}
		
		match unsafe { * bytes.get_unchecked(final_byte_index) }
		{
			b'0' ..= b'9' => digits_count += 1,
			b'A' ..= b'Z' => (),
			b'a' ..= b'z' => (),
			final_byte @ _ => return Err(format!("Final byte can not be '0x{:02X}' in `{:?}`", final_byte, label))
		}
		
		if unlikely!(digits_count == length)
		{
			return Err(format!("Label `{:?}` can not be all digits", label))
		}
		
		let host_name_label =
		{
			let mut host_name_label = ArrayVec::new();
			let pointer: *mut u8 = host_name_label.as_mut_ptr();
			unsafe
			{
				pointer.copy_from_nonoverlapping(bytes.as_ptr(), length);
				host_name_label.set_len(length)
			}
			host_name_label
		};
		
		Ok(Self(host_name_label))
	}
	
	#[inline(always)]
	fn dog_stats_d_write(&self, dog_stats_d_writer: &mut DogStatsDWriter) -> Result<(), ()>
	{
		dog_stats_d_writer.write_array_vec(&self.0)
	}
}
