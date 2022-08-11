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
			"syscall",
			in("rax") self.0,
			out("rcx") _, // Address of the instruction following `SYSCALL`.
			out("r11") _, // Saved `RFLAGS`.
			lateout("rax") result,
		);
		SystemCallResult(result)
	}
	
	#[inline(always)]
	unsafe fn system_call_1(self, a: usize) -> SystemCallResult
	{
		let result: usize;
		asm!
		(
			"syscall",
			in("rax") self.0,
			in("rdi") a,
			out("rcx") _, // Address of the instruction following `SYSCALL`.
			out("r11") _, // Saved `RFLAGS`.
			lateout("rax") result,
		);
		SystemCallResult(result)
	}
	
	#[inline(always)]
	unsafe fn system_call_2(self, a: usize, b: usize) -> SystemCallResult
	{
		let result: usize;
		asm!
		(
			"syscall",
			in("rax") self.0,
			in("rdi") a,
			in("rsi") b,
			out("rcx") _, // Address of the instruction following `SYSCALL`.
			out("r11") _, // Saved `RFLAGS`.
			lateout("rax") result,
		);
		SystemCallResult(result)
	}
	
	#[inline(always)]
	unsafe fn system_call_3(self, a: usize, b: usize, c: usize) -> SystemCallResult
	{
		let result: usize;
		asm!
		(
			"syscall",
			in("rax") self.0,
			in("rdi") a,
			in("rsi") b,
			in("rdx") c,
			out("rcx") _, // Address of the instruction following `SYSCALL`.
			out("r11") _, // Saved `RFLAGS`.
			lateout("rax") result,
		);
		SystemCallResult(result)
	}
	
	#[inline(always)]
	unsafe fn system_call_4(self, a: usize, b: usize, c: usize, d: usize) -> SystemCallResult
	{
		let result: usize;
		asm!
		(
			"syscall",
			in("rax") self.0,
			in("rdi") a,
			in("rsi") b,
			in("rdx") c,
			in("r10") d,
			out("rcx") _, // Address of the instruction following `SYSCALL`.
			out("r11") _, // Saved `RFLAGS`.
			lateout("rax") result,
		);
		SystemCallResult(result)
	}
	
	#[inline(always)]
	unsafe fn system_call_5(self, a: usize, b: usize, c: usize, d: usize, e: usize) -> SystemCallResult
	{
		let result: usize;
		asm!
		(
			"syscall",
			in("rax") self.0,
			in("rdi") a,
			in("rsi") b,
			in("rdx") c,
			in("r10") d,
			in("r8") e,
			out("rcx") _, // Address of the instruction following `SYSCALL`.
			out("r11") _, // Saved `RFLAGS`.
			lateout("rax") result,
		);
		SystemCallResult(result)
	}
	
	#[inline(always)]
	unsafe fn system_call_6(self, a: usize, b: usize, c: usize, d: usize, e: usize, f: usize) -> SystemCallResult
	{
		let result: usize;
		asm!
		(
			"syscall",
			in("rax") self.0,
			in("rdi") a,
			in("rsi") b,
			in("rdx") c,
			in("r10") d,
			in("r8") e,
			in("r9") f,
			out("rcx") _, // Address of the instruction following `SYSCALL`.
			out("r11") _, // Saved `RFLAGS`.
			lateout("rax") result,
		);
		SystemCallResult(result)
	}
}
