// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


impl SYS
{
	/// Raw syscall for 0 arguments.
	#[inline(always)]
	pub unsafe fn raw_syscall0(self) -> usize
	{
		let mut rax: usize = self as usize;
		asm!
		(
			"syscall"
			:
				"+{rax}"(rax)
			:
			:
				"rcx",
				"r11",
				"memory"
			:
				"volatile"
		);
		rax
	}

	/// Raw syscall for 1 argument.
	#[inline(always)]
	pub unsafe fn raw_syscall1(self, a: usize) -> usize
	{
		let mut rax = self as usize;
		asm!
		(
        	"syscall"
        	:
        		"+{rax}"(rax)
        	:
        		"{rdi}"(a)
			:
				"rcx",
				"r11",
				"memory"
			:
				"volatile"
		);
		rax
	}

	/// Raw syscall for 2 arguments.
	#[inline(always)]
	pub unsafe fn raw_syscall2(self, a: usize, b: usize) -> usize
	{
		let mut rax = self as usize;
		asm!
		(
			"syscall"
			:
				"+{rax}"(rax)
			:
				"{rdi}"(a),
				"{rsi}"(b)
			:
				"rcx",
				"r11",
				"memory"
			:
				"volatile"
		);
		rax
	}

	/// Raw syscall for 3 arguments.
	#[inline(always)]
	pub unsafe fn raw_syscall3(self, a: usize, b: usize, c: usize) -> usize
	{
		let mut rax = self as usize;
		asm!
		(
			"syscall"
			:
				"+{rax}"(rax)
			:
				"{rdi}"(a),
				"{rsi}"(b),
				"{rsi}"(c)
			:
				"rcx",
				"r11",
				"memory"
			:
				"volatile"
		);
		rax
	}

	/// Raw syscall for 4 arguments.
	#[inline(always)]
	pub unsafe fn raw_syscall4(self, a: usize, b: usize, c: usize, d: usize) -> usize
	{
		let mut rax = self as usize;
		asm!
		(
			"syscall"
			:
				"+{rax}"(rax)
			:
				"{rdi}"(a),
				"{rsi}"(b),
				"{rsi}"(c),
				"{r10}"(d)
			:
				"rcx",
				"r11",
				"memory"
			:
				"volatile"
		);
		rax
	}

	/// Raw syscall for 5 arguments.
	#[inline(always)]
	pub unsafe fn raw_syscall5(self, a: usize, b: usize, c: usize, d: usize, e: usize) -> usize
	{
		let mut rax = self as usize;
		asm!
		(
			"syscall"
			:
				"+{rax}"(rax)
			:
				"{rdi}"(a),
				"{rsi}"(b),
				"{rsi}"(c),
				"{r10}"(d),
				"{r8}"(e)
			:
				"rcx",
				"r11",
				"memory"
			:
				"volatile"
		);
		rax
	}

	/// Raw syscall for 6 arguments.
	#[inline(always)]
	pub unsafe fn raw_syscall6(self, a: usize, b: usize, c: usize, d: usize, e: usize, f: usize) -> usize
	{
		let mut rax = self as usize;
		asm!
		(
			"syscall"
			:
				"+{rax}"(rax)
			:
				"{rdi}"(a),
				"{rsi}"(b),
				"{rsi}"(c),
				"{r10}"(d),
				"{r8}"(e),
				"{r9}"(f)
			:
				"rcx",
				"r11",
				"memory"
			:
				"volatile"
		);
		rax
	}
}
