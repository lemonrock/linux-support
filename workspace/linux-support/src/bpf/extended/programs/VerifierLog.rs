// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VerifierLog
{
	/// Logging level.
	level: VerifierLogLevel,
	
	/// Messages consisting of ASCII-NUL terminated C strings.
	///
	/// Must be at least 128 bytes and not more than x bytes.
	messages_buffer: Vec<c_char>,
}

impl VerifierLog
{
	const MinimumInclusiveBufferSize: usize = 128;
	
	const ExclusiveMaximumBufferSize: usize = (1 << 24) as usize;
	
	/// New instance.
	pub fn new(level: VerifierLogLevel, ideal_size: usize) -> Self
	{
		let buffer_size = if ideal_size < Self::MinimumInclusiveBufferSize
		{
			Self::MinimumInclusiveBufferSize
		}
		else if ideal_size >= Self::ExclusiveMaximumBufferSize
		{
			Self::ExclusiveMaximumBufferSize - 1
		}
		else
		{
			ideal_size
		};
		
		let mut messages_buffer: Vec<c_char> = Vec::with_capacity(buffer_size);
		unsafe { messages_buffer.set_len(buffer_size) };
		unsafe { *messages_buffer.get_unchecked_mut(0) = b'\0' as i8 };
		
		Self
		{
			level,
			messages_buffer,
		}
	}
	
	#[inline(always)]
	pub(crate) fn to_values_for_syscall(verifier_log: Option<&mut Self>) -> (u32, AlignedU64, u32)
	{
		match verifier_log
		{
			None =>
			(
				0,
				AlignedU64::Null,
				0
			),
			
			Some(this) =>
			(
				this.level as u32,
				AlignedU64::from(&mut this.messages_buffer[..]),
				this.messages_buffer.len() as u32,
			)
		}
	}
}
