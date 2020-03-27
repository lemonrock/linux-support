// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


include!("SYS.constants.rs");
#[cfg(target_arch = "aarch64")] include!("SYS.raw_syscall.aarch64.rs");
#[cfg(target_arch = "x86_64")] include!("SYS.raw_syscall.x86_64.rs");

impl SYS
{
	/// Syscall for 0 arguments.
	#[cfg(not(target_arch = "powerpc64"))]
	#[inline(always)]
	pub fn syscall0(self) -> isize
	{
		Self::raw_syscall_result_to_syscall_result(unsafe { self.raw_syscall0() })
	}

	/// Syscall for 1 argument.
	#[cfg(not(target_arch = "powerpc64"))]
	#[inline(always)]
	pub fn syscall1(self, a: usize) -> isize
	{
		Self::raw_syscall_result_to_syscall_result(unsafe { self.raw_syscall1(a) })
	}

	/// Syscall for 2 arguments.
	#[cfg(not(target_arch = "powerpc64"))]
	#[inline(always)]
	pub fn syscall2(self, a: usize, b: usize) -> isize
	{
		Self::raw_syscall_result_to_syscall_result(unsafe { self.raw_syscall2(a, b) })
	}

	/// Syscall for 3 arguments.
	#[cfg(not(target_arch = "powerpc64"))]
	#[inline(always)]
	pub fn syscall3(self, a: usize, b: usize, c: usize) -> isize
	{
		Self::raw_syscall_result_to_syscall_result(unsafe { self.raw_syscall3(a, b, c) })
	}

	/// Syscall for 4 arguments.
	#[cfg(not(target_arch = "powerpc64"))]
	#[inline(always)]
	pub fn syscall4(self, a: usize, b: usize, c: usize, d: usize) -> isize
	{
		Self::raw_syscall_result_to_syscall_result(unsafe { self.raw_syscall4(a, b, c, d) })
	}

	/// Syscall for 5 arguments.
	#[inline(always)]
	pub fn syscall5(self, a: usize, b: usize, c: usize, d: usize, e: usize) -> isize
	{
		Self::raw_syscall_result_to_syscall_result(unsafe { self.raw_syscall5(a, b, c, d, e) })
	}

	/// Syscall for 6 arguments.
	#[cfg(not(target_arch = "s390x"))]
	#[inline(always)]
	pub fn syscall6(self, a: usize, b: usize, c: usize, d: usize, e: usize, f: usize) -> isize
	{
		Self::raw_syscall_result_to_syscall_result(unsafe { self.raw_syscall6(a, b, c, d, e, f) })
	}

	#[inline(always)]
	fn raw_syscall_result_to_syscall_result(result: usize) -> isize
	{
		// Linux returns errors in the range -1 to -4095 inclusive; `-4096isize as usize` is 0xFFFF_FFFF_FFFF_F000.
		let result_is_error_code = result > -4096isize as usize;
		if unlikely!(result_is_error_code)
		{
			set_errno(Errno(result as isize as i32));
			-1
		}
		else
		{
			result as isize
		}
	}
}
