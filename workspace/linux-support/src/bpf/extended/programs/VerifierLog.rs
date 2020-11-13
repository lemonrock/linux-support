// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Verifier log messages.
///
/// `Default` has sensible defaults.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VerifierLog
{
	/// Logging level.
	level: VerifierLogLevel,
	
	/// Messages consist of one ASCII NULL terminated C string.
	///
	/// Must be at least 128 bytes and not more than 1^24 bytes.
	messages_buffer: Vec<c_char>,
}

impl Default for VerifierLog
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::new(Self::DefaultLevel, Self::DefaultBufferSize)
	}
}

impl Into<CString> for VerifierLog
{
	#[inline(always)]
	fn into(self) -> CString
	{
		CString::from_fixed_length_buffer_ascii_nul_terminated(self.messages_buffer)
	}
}

impl VerifierLog
{
	const MinimumInclusiveBufferSize: usize = 128;
	
	const ExclusiveMaximumBufferSize: usize = (1 << 24) as usize;
	
	/// Defaults to `Notice` if `cfg!(debug_assertions)`, otherwise `Error`.
	#[cfg(debug_assertions)]
	pub const DefaultLevel: VerifierLogLevel = VerifierLogLevel::Notice;
	
	/// Defaults to `Notice` if `cfg!(debug_assertions)`, otherwise `Error`.
	#[cfg(not(debug_assertions))]
	pub const DefaultLevel: VerifierLogLevel = VerifierLogLevel::Error;
	
	/// 16Kb.
	pub const DefaultBufferSize: usize = 16 * 1024;
	
	/// New instance.
	pub fn new(level: VerifierLogLevel, preferred_buffer_size: usize) -> Self
	{
		let buffer_size = if preferred_buffer_size < Self::MinimumInclusiveBufferSize
		{
			Self::MinimumInclusiveBufferSize
		}
		else if preferred_buffer_size >= Self::ExclusiveMaximumBufferSize
		{
			Self::ExclusiveMaximumBufferSize - 1
		}
		else
		{
			preferred_buffer_size
		};
		
		let mut messages_buffer: Vec<c_char> = Vec::with_capacity(buffer_size);
		unsafe { messages_buffer.set_len(buffer_size) };
		messages_buffer.set_unchecked_mut_safe(0u8, b'\0' as i8);
		
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
			
			Some(verifier_log) =>
			(
				verifier_log.level as u32,
				AlignedU64::from(&mut verifier_log.messages_buffer[..]),
				verifier_log.messages_buffer.len() as u32,
			)
		}
	}
}
