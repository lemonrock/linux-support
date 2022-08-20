// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


impl SystemCallNumber
{
	#[inline(always)]
	unsafe fn system_call_0(self) -> SystemCallResult
	{
		let result: usize;
		asm!
		(
			"ecall",
			in("a7") (self as usize),
			lateout("a0") result,
		);
		SystemCallResult(result)
	}
	
	#[inline(always)]
	unsafe fn system_call_1(self, a: usize) -> SystemCallResult
	{
		let result: usize;
		asm!
		(
			"ecall",
			in("a7") (self as usize),
			in("a0") a,
			lateout("a0") result,
		);
		SystemCallResult(result)
	}
	
	#[inline(always)]
	unsafe fn system_call_2(self, a: usize, b: usize) -> SystemCallResult
	{
		let result: usize;
		asm!
		(
			"ecall",
			in("a7") (self as usize),
			in("a0") a,
			in("a1") b,
			lateout("a0") result,
		);
		SystemCallResult(result)
	}
	
	#[inline(always)]
	unsafe fn system_call_3(self, a: usize, b: usize, c: usize) -> SystemCallResult
	{
		let result: usize;
		asm!
		(
			"ecall",
			in("a7") (self as usize),
			in("a0") a,
			in("a1") b,
			in("a2") b,
			lateout("a0") result,
		);
		SystemCallResult(result)
	}
	
	#[inline(always)]
	unsafe fn system_call_4(self, a: usize, b: usize, c: usize, d: usize) -> SystemCallResult
	{
		let result: usize;
		asm!
		(
			"ecall",
			in("a7") (self as usize),
			in("a0") a,
			in("a1") b,
			in("a2") b,
			in("a3") b,
			lateout("a0") result,
		);
		SystemCallResult(result)
	}
	
	#[inline(always)]
	unsafe fn system_call_5(self, a: usize, b: usize, c: usize, d: usize, e: usize) -> SystemCallResult
	{
		let result: usize;
		asm!
		(
			"ecall",
			in("a7") (self as usize),
			in("a0") a,
			in("a1") b,
			in("a2") b,
			in("a3") b,
			in("a4") b,
			lateout("a0") result,
		);
		SystemCallResult(result)
	}
	
	#[inline(always)]
	unsafe fn system_call_6(self, a: usize, b: usize, c: usize, d: usize, e: usize, f: usize) -> SystemCallResult
	{
		let result: usize;
		asm!
		(
			"ecall",
			in("a7") (self as usize),
			in("a0") a,
			in("a1") b,
			in("a2") b,
			in("a3") b,
			in("a4") b,
			in("a5") b,
			lateout("a0") result,
		);
		SystemCallResult(result)
	}
}
