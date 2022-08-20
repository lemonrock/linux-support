// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


impl SystemCallNumber
{
	/// Make a syscall with 0 arguments.
	#[inline(always)]
	unsafe fn system_call_0(self) -> SystemCallResult
	{
		let result: usize;
		asm!
		(
			"svc 0",
			in("x8") (self as usize),
			lateout("x0") result,
		);
		SystemCallResult(result)
	}
	
	/// Make a syscall with 1 argument.
	#[inline(always)]
	unsafe fn system_call_1(self, a: usize) -> SystemCallResult
	{
		let result: usize;
		asm!
		(
			"svc 0",
			in("x8") (self as usize),
			in("x0") a,
			lateout("x0") result,
		);
		SystemCallResult(result)
	}
	
	/// Make a syscall with 2 arguments.
	#[inline(always)]
	unsafe fn system_call_2(self, a: usize, b: usize) -> SystemCallResult
	{
		let result: usize;
		asm!
		(
			"svc 0",
			in("x8") (self as usize),
			in("x0") a,
			in("x1") b,
			lateout("x0") result,
		);
		SystemCallResult(result)
	}
	
	/// Make a syscall with 3 arguments.
	#[inline(always)]
	unsafe fn system_call_3(self, a: usize, b: usize, c: usize) -> SystemCallResult
	{
		let result: usize;
		asm!
		(
			"svc 0",
			in("x8") (self as usize),
			in("x0") a,
			in("x1") b,
			in("x2") c,
			lateout("x0") result,
		);
		SystemCallResult(result)
	}
	
	/// Make a syscall with 4 arguments.
	#[inline(always)]
	unsafe fn system_call_4(self, a: usize, b: usize, c: usize, d: usize) -> SystemCallResult
	{
		let result: usize;
		asm!
		(
			"svc 0",
			in("x8") (self as usize),
			in("x0") a,
			in("x1") b,
			in("x2") c,
			in("x3") d,
			lateout("x0") result,
		);
		SystemCallResult(result)
	}
	
	/// Make a syscall with 5 arguments.
	#[inline(always)]
	unsafe fn system_call_5(self, a: usize, b: usize, c: usize, d: usize, e: usize) -> SystemCallResult
	{
		let result: usize;
		asm!
		(
			"svc 0",
			in("x8") (self as usize),
			in("x0") a,
			in("x1") b,
			in("x2") c,
			in("x3") d,
			in("x4") e,
			lateout("x0") result,
		);
		SystemCallResult(result)
	}
	
	/// Make a syscall with 6 arguments.
	#[inline(always)]
	unsafe fn system_call_6(self, a: usize, b: usize, c: usize, d: usize, e: usize, f: usize) -> SystemCallResult
	{
		let result: usize;
		asm!
		(
			"svc 0",
			in("x8") (self as usize),
			in("x0") a,
			in("x1") b,
			in("x2") c,
			in("x3") d,
			in("x4") e,
			in("x5") f,
			lateout("x0") result,
		);
		SystemCallResult(result)
	}
}
