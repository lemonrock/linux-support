// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(crate) struct VariablySizedEthtoolCommandWrapper<VSEC: VariablySizedEthtoolCommand>
{
	data: Vec<u8>,
	marker: PhantomData<VSEC>,
}

impl<VSEC: VariablySizedEthtoolCommand> Deref for VariablySizedEthtoolCommandWrapper<VSEC>
{
	type Target = VSEC;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		self.header()
	}
}

impl<VSEC: VariablySizedEthtoolCommand> DerefMut for VariablySizedEthtoolCommandWrapper<VSEC>
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		self.header_mut()
	}
}

impl<VSEC: VariablySizedEthtoolCommand> VariablySizedEthtoolCommandWrapper<VSEC>
{
	#[inline(always)]
	pub(crate) fn array_elements(&self) -> &[VSEC::ArrayElement]
	{
		unsafe { from_raw_parts(self.array_start(), self.array_length() as usize) }
	}
	
	#[inline(always)]
	pub(crate) fn array_elements_mut(&mut self) -> &mut [VSEC::ArrayElement]
	{
		unsafe { from_raw_parts_mut(self.array_start_mut(), self.array_length() as usize) }
	}
	
	#[inline(always)]
	fn header(&self) -> &VSEC
	{
		unsafe { & * self.start() }
	}
	
	#[inline(always)]
	fn header_mut(&mut self) -> &mut VSEC
	{
		unsafe { &mut * self.start_mut() }
	}
	
	#[inline(always)]
	fn array_start(&self) -> *const VSEC::ArrayElement
	{
		unsafe { self.start().add(1) as *const VSEC::ArrayElement }
	}
	
	#[inline(always)]
	fn array_start_mut(&mut self) -> *mut VSEC::ArrayElement
	{
		unsafe { self.start_mut().add(1) as *mut VSEC::ArrayElement }
	}
	
	#[inline(always)]
	fn start(&self) -> *const VSEC
	{
		self.data.as_ptr() as *const VSEC
	}
	
	#[inline(always)]
	fn start_mut(&mut self) -> *mut VSEC
	{
		self.data.as_mut_ptr() as *mut VSEC
	}
}

impl<VSEC: VariablySizedEthtoolCommand> EthtoolCommand for VariablySizedEthtoolCommandWrapper<VSEC>
{
	#[inline(always)]
	fn command(&self) -> u32
	{
		self.deref().command()
	}
}
