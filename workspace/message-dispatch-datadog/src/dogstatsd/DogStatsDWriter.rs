// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug)]
struct DogStatsDWriter<'a>
{
	buffer_pointer: *mut u8,
	remaining_length: usize,
	original_length: usize,
	buffer: PhantomData<&'a [u8]>
}

impl<'a> DogStatsDWriter<'a>
{
	#[inline(always)]
	fn new(buffer: &mut [u8]) -> Self
	{
		let original_length = buffer.len();
		
		Self
		{
			buffer_pointer: buffer.as_mut_ptr(),
			remaining_length: original_length,
			original_length,
			buffer: PhantomData,
		}
	}
	
	#[inline(always)]
	fn written_length(self) -> usize
	{
		self.original_length - self.remaining_length
	}
	
	#[inline(always)]
	fn duration_to_string(duration: Duration) -> String
	{
		min(duration.as_millis(), i64::MAX as u128).to_string()
	}
	
	#[inline(always)]
	fn write_pipe(&mut self) -> Result<(), ()>
	{
		self.write_byte(b'|')
	}
	
	#[inline(always)]
	fn write_colon(&mut self) -> Result<(), ()>
	{
		self.write_byte(b':')
	}
	
	#[inline(always)]
	fn write_comma(&mut self) -> Result<(), ()>
	{
		self.write_byte(b',')
	}
	
	#[inline(always)]
	fn write_line_feed(&mut self) -> Result<(), ()>
	{
		self.write_byte(b'\n')
	}
	
	#[inline(always)]
	fn write_system_time(&mut self, system_time: &SystemTime) -> Result<(), ()>
	{
		let duration = system_time.duration_since(SystemTime::UNIX_EPOCH).unwrap();
		self.write_string(Self::duration_to_string(duration))
	}
	
	#[inline(always)]
	fn write_usize(&mut self, value: usize) -> Result<(), ()>
	{
		self.write_string(format!("{}", value))
	}
	
	#[inline(always)]
	fn write_string(&mut self, string: String) -> Result<(), ()>
	{
		self.write_bytes(string.as_bytes())
	}
	
	#[inline(always)]
	fn write_array_vec<A: Array<Item=u8> + Copy>(&mut self, array_vec: &ArrayVec<A>) -> Result<(), ()>
	{
		self.write_bytes(&array_vec[..])
	}
	
	#[inline(always)]
	fn write_array_string<A: Array<Item=u8> + Copy>(&mut self, array_string: &ArrayString<A>) -> Result<(), ()>
	{
		self.write_bytes(array_string.as_bytes())
	}
	
	#[inline(always)]
	fn write_byte(&mut self, byte: u8) -> Result<(), ()>
	{
		const RequiredLength: usize = 1;
		
		if unlikely!(RequiredLength > self.remaining_length)
		{
			return Err(())
		}
		
		unsafe { * self.buffer_pointer = byte };
		
		self.buffer_pointer = unsafe { self.buffer_pointer.add(RequiredLength) };
		self.remaining_length = self.remaining_length - RequiredLength;
		
		Ok(())
	}
	
	#[inline(always)]
	fn write_bytes(&mut self, bytes: &[u8]) -> Result<(), ()>
	{
		let required_length = bytes.len();
		
		if unlikely!(required_length > self.remaining_length)
		{
			return Err(())
		}
		
		unsafe { self.buffer_pointer.copy_from_nonoverlapping(bytes.as_ptr(), required_length) };
		
		self.buffer_pointer = unsafe { self.buffer_pointer.add(required_length) };
		self.remaining_length = self.remaining_length - required_length;
		
		Ok(())
	}

}
