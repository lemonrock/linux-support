// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
pub struct RegisteredBuffers
{
	_4Kb: Box<[RegisteredBuffer<[u8; RegisteredBuffers::_4Kb]>]>,
	_16Kb: Box<[RegisteredBuffer<[u8; RegisteredBuffers::_16Kb]>]>,
	_64Kb: Box<[RegisteredBuffer<[u8; RegisteredBuffers::_64Kb]>]>,
	_256Kb: Box<[RegisteredBuffer<[u8; RegisteredBuffers::_256Kb]>]>,
	_1Mb: Box<[RegisteredBuffer<[u8; RegisteredBuffers::_1Mb]>]>,
	_4Mb: Box<[RegisteredBuffer<[u8; RegisteredBuffers::_4Mb]>]>,
	_16Mb: Box<[RegisteredBuffer<[u8; RegisteredBuffers::_16Mb]>]>,
	_64Mb: Box<[RegisteredBuffer<[u8; RegisteredBuffers::_64Mb]>]>,
	_256Mb: Box<[RegisteredBuffer<[u8; RegisteredBuffers::_256Mb]>]>,
	_1Gb: Box<[RegisteredBuffer<[u8; RegisteredBuffers::_1Gb]>]>,
	buffers_count: u16,
}

impl RegisteredBuffers
{
	const _4Kb: usize = 4 * 1024;
	
	const _16Kb: usize = 16 * 1024;
	
	const _64Kb: usize = 64 * 1024;
	
	const _256Kb: usize = 256 * 1024;
	
	const _1Mb: usize = 1024 * 1024;
	
	const _4Mb: usize = 4 * 1024 * 1024;
	
	const _16Mb: usize = 16 * 1024 * 1024;
	
	const _64Mb: usize = 64 * 1024 * 1024;
	
	const _256Mb: usize = 256 * 1024 * 1024;
	
	const _1Gb: usize = 1024 * 1024 * 1024;
	
	/// New instance.
	#[inline(always)]
	pub fn new(settings: &RegisteredBufferSettings, defaults: &DefaultPageSizeAndHugePageSizes) -> Result<Self, RegisteredBuffersCreationError>
	{
		let mut buffers_count = 0;
		
		Ok
		(
			Self
			{
				_4Kb: settings._4Kb.create_buffers(&mut buffers_count)?,
				_16Kb: settings._16Kb.create_buffers(&mut buffers_count)?,
				_64Kb: settings._64Kb.create_buffers(&mut buffers_count)?,
				_256Kb: settings._256Kb.create_buffers(&mut buffers_count)?,
				_1Mb: settings._1Mb.create_buffers(&mut buffers_count)?,
				_4Mb: settings._4Mb.create_buffers(&mut buffers_count)?,
				_16Mb: settings._16Mb.create_buffers(&mut buffers_count)?,
				_64Mb: settings._64Mb.create_buffers(&mut buffers_count)?,
				_256Mb: settings._256Mb.create_buffers(&mut buffers_count)?,
				_1Gb: settings._1Gb.create_buffers(&mut buffers_count)?,
				buffers_count,
			}
		)
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn next_buffer_4Kb(&self) -> Result<RegisteredBufferSource<[u8; Self::_4Kb]>, ()>
	{
		Self::next_buffer(&self._4Kb)
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn next_buffer_16Kb(&self) -> Result<RegisteredBufferSource<[u8; Self::_16Kb]>, ()>
	{
		Self::next_buffer(&self._16Kb)
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn next_buffer_64Kb(&self) -> Result<RegisteredBufferSource<[u8; Self::_64Kb]>, ()>
	{
		Self::next_buffer(&self._64Kb)
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn next_buffer_256Kb(&self) -> Result<RegisteredBufferSource<[u8; Self::_256Kb]>, ()>
	{
		Self::next_buffer(&self._256Kb)
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn next_buffer_1Mb(&self) -> Result<RegisteredBufferSource<[u8; Self::_1Mb]>, ()>
	{
		Self::next_buffer(&self._1Mb)
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn next_buffer_4Mb(&self) -> Result<RegisteredBufferSource<[u8; Self::_4Mb]>, ()>
	{
		Self::next_buffer(&self._4Mb)
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn next_buffer_16Mb(&self) -> Result<RegisteredBufferSource<[u8; Self::_16Mb]>, ()>
	{
		Self::next_buffer(&self._16Mb)
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn next_buffer_64Mb(&self) -> Result<RegisteredBufferSource<[u8; Self::_64Mb]>, ()>
	{
		Self::next_buffer(&self._64Mb)
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn next_buffer_256Mb(&self) -> Result<RegisteredBufferSource<[u8; Self::_256Mb]>, ()>
	{
		Self::next_buffer(&self._256Mb)
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn next_buffer_1Gb(&self) -> Result<RegisteredBufferSource<[u8; Self::_1Gb]>, ()>
	{
		Self::next_buffer(&self._1Gb)
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	fn next_buffer<BufferSize: Sized>(field: &Box<[RegisteredBuffer<BufferSize>]>) -> Result<RegisteredBufferSource<BufferSize>, ()>
	{
		for buffer in field.iter()
		{
			if let Ok(has) = buffer.next_buffer()
			{
				return Ok(has)
			}
		}
		Err(())
	}
	
	#[inline(always)]
	pub(crate) fn register(&self, io_uring: &IoUring) -> io::Result<()>
	{
		let buffers_to_register = Vec::with_capacity(self.buffers_count as usize);
		
		Self::register_buffers(&self._4Kb, &mut buffers_to_register);
		Self::register_buffers(&self._16Kb, &mut buffers_to_register);
		Self::register_buffers(&self._64Kb, &mut buffers_to_register);
		Self::register_buffers(&self._256Kb, &mut buffers_to_register);
		Self::register_buffers(&self._1Mb, &mut buffers_to_register);
		Self::register_buffers(&self._4Mb, &mut buffers_to_register);
		Self::register_buffers(&self._16Mb, &mut buffers_to_register);
		Self::register_buffers(&self._64Mb, &mut buffers_to_register);
		Self::register_buffers(&self._256Mb, &mut buffers_to_register);
		Self::register_buffers(&self._1Gb, &mut buffers_to_register);
		
		io_uring.register_buffers(buffers_to_register.as_slice())
	}
	
	#[inline(always)]
	fn register_buffers<BufferSize: Sized>(buffers: &Box<[RegisteredBuffer<BufferSize>]>, buffers_to_register: &mut Vec<&mut [u8]>)
	{
		for buffer in buffers
		{
			buffers_to_register.push(buffers.ring_queue.raw_backing_memory_slice())
		}
	}
}
