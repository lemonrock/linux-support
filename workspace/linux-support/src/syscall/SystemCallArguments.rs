// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Exists to permit implementation of a Rust equivalent to the libc `long syscall(long n, ...)` function.
pub trait SystemCallArguments: Default + Debug + Copy + Eq + Ord + Hash
{
	/// Make a system call using the `system_call_number` and arguments `self`.
	unsafe fn system_call(self, system_call_number: SystemCallNumber) -> SystemCallResult;
}

impl SystemCallArguments for ()
{
	#[inline(always)]
	unsafe fn system_call(self, system_call_number: SystemCallNumber) -> SystemCallResult
	{
		system_call_number.system_call_0()
	}
}

impl SystemCallArguments for (usize,)
{
	#[inline(always)]
	unsafe fn system_call(self, system_call_number: SystemCallNumber) -> SystemCallResult
	{
		system_call_number.system_call_1(self.0)
	}
}

impl SystemCallArguments for (usize, usize)
{
	#[inline(always)]
	unsafe fn system_call(self, system_call_number: SystemCallNumber) -> SystemCallResult
	{
		system_call_number.system_call_2(self.0, self.1)
	}
}

impl SystemCallArguments for (usize, usize, usize)
{
	#[inline(always)]
	unsafe fn system_call(self, system_call_number: SystemCallNumber) -> SystemCallResult
	{
		system_call_number.system_call_3(self.0, self.1, self.2)
	}
}

impl SystemCallArguments for (usize, usize, usize, usize)
{
	#[inline(always)]
	unsafe fn system_call(self, system_call_number: SystemCallNumber) -> SystemCallResult
	{
		system_call_number.system_call_4(self.0, self.1, self.2, self.3)
	}
}

impl SystemCallArguments for (usize, usize, usize, usize, usize)
{
	#[inline(always)]
	unsafe fn system_call(self, system_call_number: SystemCallNumber) -> SystemCallResult
	{
		system_call_number.system_call_5(self.0, self.1, self.2, self.3, self.4)
	}
}

impl SystemCallArguments for (usize, usize, usize, usize, usize, usize)
{
	#[inline(always)]
	unsafe fn system_call(self, system_call_number: SystemCallNumber) -> SystemCallResult
	{
		system_call_number.system_call_6(self.0, self.1, self.2, self.3, self.4, self.5)
	}
}
