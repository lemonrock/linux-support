// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.



/// Atomically add `val` to the content of `ptr` and return the result with 'HLE Acquire' memory ordering.
#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn __hle_acquire_add_fetch8(ptr: *mut u64, mut val: u64) -> u64
{
	let oldval = val;
	llvm_asm!(".byte 0xf2; lock; xadd %0,%1" : "+q"(val), "+m"(*ptr) : : "memory" : "volatile");
	val + oldval
}

/// Atomically subtract `val` from the content of `ptr` and return the result with 'HLE Acquire' memory ordering.
#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn __hle_acquire_sub_fetch8(ptr: *mut u64, mut val: u64) -> u64
{
	let oldval = val;
	val = !val;
	llvm_asm!(".byte 0xf2; lock; xadd %0,%1" : "+q"(val), "+m"(*ptr) : : "memory" : "volatile");
	val - oldval
}

/// Atomically add `val` to the content of `ptr` and return the result with 'HLE Release' memory ordering.
#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn __hle_release_add_fetch8(ptr: *mut u64, mut val: u64) -> u64
{
	let oldval = val;
	llvm_asm!(".byte 0xf3; lock; xadd %0,%1" : "+q"(val), "+m"(*ptr) : : "memory" : "volatile");
	val + oldval
}

/// Atomically subtract `val` from the content of `ptr` and return the result with 'HLE Release' memory ordering.
#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn __hle_release_sub_fetch8(ptr: *mut u64, mut val: u64) -> u64
{
	let oldval = val;
	val = !val;
	llvm_asm!(".byte 0xf3; lock; xadd %0,%1" : "+q"(val), "+m"(*ptr) : : "memory" : "volatile");
	val - oldval
}

/// Atomically add `val` to the content of `ptr` and return the previous content of `ptr` with 'HLE Acquire' memory ordering.
#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn __hle_acquire_fetch_add8(ptr: *mut u64, mut val: u64) -> u64
{
	llvm_asm!(".byte 0xf2; lock; xadd %0,%1" : "+q"(val), "+m"(*ptr) : : "memory" : "volatile");
	val
}

/// Atomically subtract `val` from the content of `ptr` and return the previous content of `ptr` with 'HLE Acquire' memory ordering.
#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn __hle_acquire_fetch_sub8(ptr: *mut u64, mut val: u64) -> u64
{
	val = !val;
	llvm_asm!(".byte 0xf2; lock; xadd %0,%1" : "+q"(val), "+m"(*ptr) : : "memory" : "volatile");
	val
}

/// Atomically add `val` to the content of `ptr` and return the previous content of `ptr` with 'HLE Release' memory ordering.
#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn __hle_release_fetch_add8(ptr: *mut u64, mut val: u64) -> u64
{
	llvm_asm!(".byte 0xf3; lock; xadd %0,%1" : "+q"(val), "+m"(*ptr) : : "memory" : "volatile");
	val
}

/// Atomically subtract `val` from the content of `ptr` and return the previous content of `ptr` with 'HLE Release' memory ordering.
#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn __hle_release_fetch_sub8(ptr: *mut u64, mut val: u64) -> u64
{
	val = !val;
	llvm_asm!(".byte 0xf3; lock; xadd %0,%1" : "+q"(val), "+m"(*ptr) : : "memory" : "volatile");
	val
}

/// Atomically exchange (swap) `val` for the content of `ptr` and return the previous content of `ptr` with 'HLE Acquire' memory ordering.
#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn __hle_acquire_exchange_n8(ptr: *mut u64, mut val: u64) -> u64
{
	llvm_asm!(".byte 0xf2; lock; xchg %0,%1" : "+q"(val), "+m"(*ptr) : : "memory" : "volatile");
	val
}

/// Atomically exchange (swap) 1 for the content of `ptr` and test the previous content of of `ptr` was 1 with 'HLE Acquire' memory ordering.
#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn __hle_acquire_test_and_set8(ptr: *mut u64) -> bool
{
 	 __hle_acquire_exchange_n8(ptr, 1) == 1
}

/// Atomically compare-and-swap with 'HLE Acquire' memory ordering.
/// This compares the content of `ptr` with the content of `oldp`:-
/// * If equal, the function atomically writes `newv` into `ptr` and returns `true`.
/// * If they are not equal, the function atomically writes the content of `ptr` into `oldp` and returns `false`
/// Since this is only for the x86_64 architecture, there is no weak variant.
#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn __hle_acquire_compare_exchange_n8(ptr: *mut u64, oldp: *mut u64, newv: u64) -> bool
{
	let res: u8;
	llvm_asm!(".byte 0xf2; lock; cmpxchg %3,%1; setz %2" : "+a"(*oldp), "+m"(*ptr), "=r"(res) : "r"(newv) : "memory" : "volatile");
	res == 1
}

/// Atomically exchange (swap) `val` for the content of `ptr` and return the previous content of of `ptr` with 'HLE Release' memory ordering.
#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn __hle_release_exchange_n8(ptr: *mut u64, mut val: u64) -> u64
{
	llvm_asm!(".byte 0xf3; lock; xchg %0,%1" : "+q"(val), "+m"(*ptr) : : "memory" : "volatile");
	val
}

/// Atomically exchange (swap) 1 for the content of `ptr` and test the previous content of of `ptr` was 1 with 'HLE Release' memory ordering.
#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn __hle_release_test_and_set8(ptr: *mut u64) -> bool
{
 	 __hle_release_exchange_n8(ptr, 1) == 1
}

/// Atomically compare-and-swap with 'HLE Release' memory ordering.
/// This compares the content of `ptr` with the content of `oldp`:-
/// * If equal, the function atomically writes `newv` into `ptr` and returns `true`.
/// * If they are not equal, the function atomically writes the content of `ptr` into `oldp` and returns `false`
/// Since this is only for the x86_64 architecture, there is no weak variant.
#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn __hle_release_compare_exchange_n8(ptr: *mut u64, oldp: *mut u64, newv: u64) -> u8
{
	let res: u8;
	llvm_asm!(".byte 0xf3; lock; cmpxchg %3,%1; setz %2" : "+a"(*oldp), "+m"(*ptr), "=r"(res) : "r"(newv) : "memory" : "volatile");
	res
}

/// Atomically store (set) the content of `ptr` to `val` with 'HLE Acquire' memory ordering.
#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn __hle_acquire_store_n8(ptr: *mut u64, val: u32)
{
	llvm_asm!(".byte 0xf2; mov %1,%0" : "=m"(*ptr) : "q"(val) : "memory" : "volatile");
}

/// Atomically clear the content of `ptr`, ie set the content of `ptr` to zero with 'HLE Acquire' memory ordering.
#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn __hle_acquire_clear8(ptr: *mut u64)
{
	__hle_acquire_store_n8(ptr, 0)
}

/// Atomically store (set) the content of `ptr` to `val` with 'HLE Release' memory ordering.
#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn __hle_release_store_n8(ptr: *mut u64, val: u32)
{
	llvm_asm!(".byte 0xf3; mov %1,%0" : "=m"(*ptr) : "q"(val) : "memory" : "volatile");
}

/// Atomically clear the content of `ptr`, ie set the content of `ptr` to zero with 'HLE Release' memory ordering.
#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn __hle_release_clear8(ptr: *mut u64)
{
	__hle_release_store_n8(ptr, 0)
}

/// Atomically add `val` to the content of `ptr` to `val` with 'HLE Acquire' memory ordering.
#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn __hle_acquire_add8(ptr: *mut u64, val: u64)
{
	llvm_asm!(".byte 0xf2; lock; add %1,%0" : "+m"(*ptr) : "q"(val) : "memory" : "volatile");
}

/// Atomically subtract `val` from the content of `ptr` to `val` with 'HLE Acquire' memory ordering.
#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn __hle_acquire_sub8(ptr: *mut u64, val: u64)
{
	llvm_asm!(".byte 0xf2; lock; sub %1,%0" : "+m"(*ptr) : "q"(val) : "memory" : "volatile");
}

/// Atomically OR `val` with the content of `ptr` to `val` with 'HLE Acquire' memory ordering.
#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn __hle_acquire_or8(ptr: *mut u64, val: u64)
{
	llvm_asm!(".byte 0xf2; lock; or %1,%0" : "+m"(*ptr) : "q"(val) : "memory" : "volatile");
}

/// Atomically AND `val` with the content of `ptr` to `val` with 'HLE Acquire' memory ordering.
#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn __hle_acquire_and8(ptr: *mut u64, val: u64)
{
	llvm_asm!(".byte 0xf2; lock; and %1,%0" : "+m"(*ptr) : "q"(val) : "memory" : "volatile");
}

/// Atomically XOR `val` with the content of `ptr` to `val` with 'HLE Acquire' memory ordering.
#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn __hle_acquire_xor8(ptr: *mut u64, val: u64)
{
	llvm_asm!(".byte 0xf2; lock; xor %1,%0" : "+m"(*ptr) : "q"(val) : "memory" : "volatile");
}

/// Atomically add `val` to the content of `ptr` to `val` with 'HLE Release' memory ordering.
#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn __hle_release_add8(ptr: *mut u64, val: u64)
{
	llvm_asm!(".byte 0xf3; lock; add %1,%0" : "+m"(*ptr) : "q"(val) : "memory" : "volatile");
}

/// Atomically subtract `val` from the content of `ptr` to `val` with 'HLE Release' memory ordering.
#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn __hle_release_sub8(ptr: *mut u64, val: u64)
{
	llvm_asm!(".byte 0xf3; lock; sub %1,%0" : "+m"(*ptr) : "q"(val) : "memory" : "volatile");
}

/// Atomically OR `val` with the content of `ptr` to `val` with 'HLE Release' memory ordering.
#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn __hle_release_or8(ptr: *mut u64, val: u64)
{
	llvm_asm!(".byte 0xf3; lock; or %1,%0" : "+m"(*ptr) : "q"(val) : "memory" : "volatile");
}

/// Atomically AND `val` with the content of `ptr` to `val` with 'HLE Release' memory ordering.
#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn __hle_release_and8(ptr: *mut u64, val: u64)
{
	llvm_asm!(".byte 0xf3; lock; and %1,%0" : "+m"(*ptr) : "q"(val) : "memory" : "volatile");
}

/// Atomically XOR `val` with the content of `ptr` to `val` with 'HLE Release' memory ordering.
#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn __hle_release_xor8(ptr: *mut u64, val: u64)
{
	llvm_asm!(".byte 0xf3; lock; xor %1,%0" : "+m"(*ptr) : "q"(val) : "memory" : "volatile");
}

/// Atomically add `val` to the content of `ptr` and return the result with 'HLE Acquire' memory ordering.
#[inline(always)]
pub unsafe fn __hle_acquire_add_fetch4(ptr: *mut u32, mut val: u32) -> u32
{
	let oldval = val;
	llvm_asm!(".byte 0xf2; lock; xadd %0,%1" : "+q"(val), "+m"(*ptr) : : "memory" : "volatile");
	val + oldval
}

/// Atomically subtract `val` from the content of `ptr` and return the result with 'HLE Acquire' memory ordering.
#[inline(always)]
pub unsafe fn __hle_acquire_sub_fetch4(ptr: *mut u32, mut val: u32) -> u32
{
	let oldval = val;
	val = !val;
	llvm_asm!(".byte 0xf2; lock; xadd %0,%1" : "+q"(val), "+m"(*ptr) : : "memory" : "volatile");
	val - oldval
}

/// Atomically add `val` to the content of `ptr` and return the result with 'HLE Release' memory ordering.
#[inline(always)]
pub unsafe fn __hle_release_add_fetch4(ptr: *mut u32, mut val: u32) -> u32
{
	let oldval = val;
	llvm_asm!(".byte 0xf3; lock; xadd %0,%1" : "+q"(val), "+m"(*ptr) : : "memory" : "volatile");
	val + oldval
}

/// Atomically subtract `val` from the content of `ptr` and return the result with 'HLE Release' memory ordering.
#[inline(always)]
pub unsafe fn __hle_release_sub_fetch4(ptr: *mut u32, mut val: u32) -> u32
{
	let oldval = val;
	val = !val;
	llvm_asm!(".byte 0xf3; lock; xadd %0,%1" : "+q"(val), "+m"(*ptr) : : "memory" : "volatile");
	val - oldval
}

/// Atomically add `val` to the content of `ptr` and return the previous content of `ptr` with 'HLE Acquire' memory ordering.
#[inline(always)]
pub unsafe fn __hle_acquire_fetch_add4(ptr: *mut u32, mut val: u32) -> u32
{
	llvm_asm!(".byte 0xf2; lock; xadd %0,%1" : "+q"(val), "+m"(*ptr) : : "memory" : "volatile");
	val
}

/// Atomically subtract `val` from the content of `ptr` and return the previous content of `ptr` with 'HLE Acquire' memory ordering.
#[inline(always)]
pub unsafe fn __hle_acquire_fetch_sub4(ptr: *mut u32, mut val: u32) -> u32
{
	val = !val;
	llvm_asm!(".byte 0xf2; lock; xadd %0,%1" : "+q"(val), "+m"(*ptr) : : "memory" : "volatile");
	val
}

/// Atomically add `val` to the content of `ptr` and return the previous content of `ptr` with 'HLE Release' memory ordering.
#[inline(always)]
pub unsafe fn __hle_release_fetch_add4(ptr: *mut u32, mut val: u32) -> u32
{
	llvm_asm!(".byte 0xf3; lock; xadd %0,%1" : "+q"(val), "+m"(*ptr) : : "memory" : "volatile");
	val
}

/// Atomically subtract `val` from the content of `ptr` and return the previous content of `ptr` with 'HLE Release' memory ordering.
#[inline(always)]
pub unsafe fn __hle_release_fetch_sub4(ptr: *mut u32, mut val: u32) -> u32
{
	val = !val;
	llvm_asm!(".byte 0xf3; lock; xadd %0,%1" : "+q"(val), "+m"(*ptr) : : "memory" : "volatile");
	val
}

/// Atomically exchange (swap) `val` for the content of `ptr` and return the previous content of `ptr` with 'HLE Acquire' memory ordering.
#[inline(always)]
pub unsafe fn __hle_acquire_exchange_n4(ptr: *mut u32, mut val: u32) -> u32
{
	llvm_asm!(".byte 0xf2; lock; xchg %0,%1" : "+q"(val), "+m"(*ptr) : : "memory" : "volatile");
	val
}

/// Atomically exchange (swap) 1 for the content of `ptr` and test the previous content of of `ptr` was 1 with 'HLE Acquire' memory ordering.
#[inline(always)]
pub unsafe fn __hle_acquire_test_and_set4(ptr: *mut u32) -> bool
{
 	 __hle_acquire_exchange_n4(ptr, 1) == 1
}

/// Atomically compare-and-swap with 'HLE Acquire' memory ordering.
/// This compares the content of `ptr` with the content of `oldp`:-
/// * If equal, the function atomically writes `newv` into `ptr` and returns `true`.
/// * If they are not equal, the function atomically writes the content of `ptr` into `oldp` and returns `false`
/// Since this is only for the x86 / x86_64 architecture, there is no weak variant.
#[inline(always)]
pub unsafe fn __hle_acquire_compare_exchange_n4(ptr: *mut u32, oldp: *mut u32, newv: u32) -> bool
{
	let res: u8;
	llvm_asm!(".byte 0xf2; lock; cmpxchg %3,%1; setz %2" : "+a"(*oldp), "+m"(*ptr), "=r"(res) : "r"(newv) : "memory" : "volatile");
	res == 1
}

/// Atomically exchange (swap) `val` for the content of `ptr` and return the previous content of of `ptr` with 'HLE Release' memory ordering.
#[inline(always)]
pub unsafe fn __hle_release_exchange_n4(ptr: *mut u32, mut val: u32) -> u32
{
	llvm_asm!(".byte 0xf3; lock; xchg %0,%1" : "+q"(val), "+m"(*ptr) : : "memory" : "volatile");
	val
}

/// Atomically exchange (swap) 1 for the content of `ptr` and test the previous content of of `ptr` was 1 with 'HLE Release' memory ordering.
#[inline(always)]
pub unsafe fn __hle_release_test_and_set4(ptr: *mut u32) -> bool
{
 	 __hle_release_exchange_n4(ptr, 1) == 1
}

/// Atomically compare-and-swap with 'HLE Release' memory ordering.
/// This compares the content of `ptr` with the content of `oldp`:-
/// * If equal, the function atomically writes `newv` into `ptr` and returns `true`.
/// * If they are not equal, the function atomically writes the content of `ptr` into `oldp` and returns `false`
/// Since this is only for the x86 / x86_64 architecture, there is no weak variant.
#[inline(always)]
pub unsafe fn __hle_release_compare_exchange_n4(ptr: *mut u32, oldp: *mut u32, newv: u32) -> bool
{
	let res: u8;
	llvm_asm!(".byte 0xf3; lock; cmpxchg %3,%1; setz %2" : "+a"(*oldp), "+m"(*ptr), "=r"(res) : "r"(newv) : "memory" : "volatile");
	res == 1
}

/// Atomically store (set) the content of `ptr` to `val` with 'HLE Acquire' memory ordering.
#[inline(always)]
pub unsafe fn __hle_acquire_store_n4(ptr: *mut u32, val: u32)
{
	llvm_asm!(".byte 0xf2; mov %1,%0" : "=m"(*ptr) : "q"(val) : "memory" : "volatile");
}

/// Atomically clear the content of `ptr`, ie set the content of `ptr` to zero with 'HLE Acquire' memory ordering.
#[inline(always)]
pub unsafe fn __hle_acquire_clear4(ptr: *mut u32)
{
	__hle_acquire_store_n4(ptr, 0)
}

/// Atomically store (set) the content of `ptr` to `val` with 'HLE Release' memory ordering.
#[inline(always)]
pub unsafe fn __hle_release_store_n4(ptr: *mut u32, val: u32)
{
	llvm_asm!(".byte 0xf3; mov %1,%0" : "=m"(*ptr) : "q"(val) : "memory" : "volatile");
}

/// Atomically clear the content of `ptr`, ie set the content of `ptr` to zero with 'HLE Release' memory ordering.
#[inline(always)]
pub unsafe fn __hle_release_clear4(ptr: *mut u32)
{
	__hle_release_store_n4(ptr, 0)
}

/// Atomically add `val` to the content of `ptr` to `val` with 'HLE Acquire' memory ordering.
#[inline(always)]
pub unsafe fn __hle_acquire_add4(ptr: *mut u32, val: u32)
{
	llvm_asm!(".byte 0xf2; lock; add %1,%0" : "+m"(*ptr) : "q"(val) : "memory" : "volatile");
}

/// Atomically subtract `val` from the content of `ptr` to `val` with 'HLE Acquire' memory ordering.
#[inline(always)]
pub unsafe fn __hle_acquire_sub4(ptr: *mut u32, val: u32)
{
	llvm_asm!(".byte 0xf2; lock; sub %1,%0" : "+m"(*ptr) : "q"(val) : "memory" : "volatile");
}

/// Atomically OR `val` with the content of `ptr` to `val` with 'HLE Acquire' memory ordering.
#[inline(always)]
pub unsafe fn __hle_acquire_or4(ptr: *mut u32, val: u32)
{
	llvm_asm!(".byte 0xf2; lock; or %1,%0" : "+m"(*ptr) : "q"(val) : "memory" : "volatile");
}

/// Atomically AND `val` with the content of `ptr` to `val` with 'HLE Acquire' memory ordering.
#[inline(always)]
pub unsafe fn __hle_acquire_and4(ptr: *mut u32, val: u32)
{
	llvm_asm!(".byte 0xf2; lock; and %1,%0" : "+m"(*ptr) : "q"(val) : "memory" : "volatile");
}

/// Atomically XOR `val` with the content of `ptr` to `val` with 'HLE Acquire' memory ordering.
#[inline(always)]
pub unsafe fn __hle_acquire_xor4(ptr: *mut u32, val: u32)
{
	llvm_asm!(".byte 0xf2; lock; xor %1,%0" : "+m"(*ptr) : "q"(val) : "memory" : "volatile");
}

/// Atomically add `val` to the content of `ptr` to `val` with 'HLE Release' memory ordering.
#[inline(always)]
pub unsafe fn __hle_release_add4(ptr: *mut u32, val: u32)
{
	llvm_asm!(".byte 0xf3; lock; add %1,%0" : "+m"(*ptr) : "q"(val) : "memory" : "volatile");
}

/// Atomically subtract `val` from the content of `ptr` to `val` with 'HLE Release' memory ordering.
#[inline(always)]
pub unsafe fn __hle_release_sub4(ptr: *mut u32, val: u32)
{
	llvm_asm!(".byte 0xf3; lock; sub %1,%0" : "+m"(*ptr) : "q"(val) : "memory" : "volatile");
}

/// Atomically OR `val` with the content of `ptr` to `val` with 'HLE Release' memory ordering.
#[inline(always)]
pub unsafe fn __hle_release_or4(ptr: *mut u32, val: u32)
{
	llvm_asm!(".byte 0xf3; lock; or %1,%0" : "+m"(*ptr) : "q"(val) : "memory" : "volatile");
}

/// Atomically AND `val` with the content of `ptr` to `val` with 'HLE Release' memory ordering.
#[inline(always)]
pub unsafe fn __hle_release_and4(ptr: *mut u32, val: u32)
{
	llvm_asm!(".byte 0xf3; lock; and %1,%0" : "+m"(*ptr) : "q"(val) : "memory" : "volatile");
}

/// Atomically XOR `val` with the content of `ptr` to `val` with 'HLE Release' memory ordering.
#[inline(always)]
pub unsafe fn __hle_release_xor4(ptr: *mut u32, val: u32)
{
	llvm_asm!(".byte 0xf3; lock; xor %1,%0" : "+m"(*ptr) : "q"(val) : "memory" : "volatile");
}

/// Atomically add `val` to the content of `ptr` and return the result with 'HLE Acquire' memory ordering.
#[inline(always)]
pub unsafe fn __hle_acquire_add_fetch2(ptr: *mut u16, mut val: u16) -> u16
{
	let oldval = val;
	llvm_asm!(".byte 0xf2; lock; xadd %0,%1" : "+q"(val), "+m"(*ptr) : : "memory" : "volatile");
	val + oldval
}

/// Atomically subtract `val` from the content of `ptr` and return the result with 'HLE Acquire' memory ordering.
#[inline(always)]
pub unsafe fn __hle_acquire_sub_fetch2(ptr: *mut u16, mut val: u16) -> u16
{
	let oldval = val;
	val = !val;
	llvm_asm!(".byte 0xf2; lock; xadd %0,%1" : "+q"(val), "+m"(*ptr) : : "memory" : "volatile");
	val - oldval
}

/// Atomically add `val` to the content of `ptr` and return the result with 'HLE Release' memory ordering.
#[inline(always)]
pub unsafe fn __hle_release_add_fetch2(ptr: *mut u16, mut val: u16) -> u16
{
	let oldval = val;
	llvm_asm!(".byte 0xf3; lock; xadd %0,%1" : "+q"(val), "+m"(*ptr) : : "memory" : "volatile");
	val + oldval
}

/// Atomically subtract `val` from the content of `ptr` and return the result with 'HLE Release' memory ordering.
#[inline(always)]
pub unsafe fn __hle_release_sub_fetch2(ptr: *mut u16, mut val: u16) -> u16
{
	let oldval = val;
	val = !val;
	llvm_asm!(".byte 0xf3; lock; xadd %0,%1" : "+q"(val), "+m"(*ptr) : : "memory" : "volatile");
	val - oldval
}

/// Atomically add `val` to the content of `ptr` and return the previous content of `ptr` with 'HLE Acquire' memory ordering.
#[inline(always)]
pub unsafe fn __hle_acquire_fetch_add2(ptr: *mut u16, mut val: u16) -> u16
{
	llvm_asm!(".byte 0xf2; lock; xadd %0,%1" : "+q"(val), "+m"(*ptr) : : "memory" : "volatile");
	val
}

/// Atomically subtract `val` from the content of `ptr` and return the previous content of `ptr` with 'HLE Acquire' memory ordering.
#[inline(always)]
pub unsafe fn __hle_acquire_fetch_sub2(ptr: *mut u16, mut val: u16) -> u16
{
	val = !val;
	llvm_asm!(".byte 0xf2; lock; xadd %0,%1" : "+q"(val), "+m"(*ptr) : : "memory" : "volatile");
	val
}

/// Atomically add `val` to the content of `ptr` and return the previous content of `ptr` with 'HLE Release' memory ordering.
#[inline(always)]
pub unsafe fn __hle_release_fetch_add2(ptr: *mut u16, mut val: u16) -> u16
{
	llvm_asm!(".byte 0xf3; lock; xadd %0,%1" : "+q"(val), "+m"(*ptr) : : "memory" : "volatile");
	val
}

/// Atomically subtract `val` from the content of `ptr` and return the previous content of `ptr` with 'HLE Release' memory ordering.
#[inline(always)]
pub unsafe fn __hle_release_fetch_sub2(ptr: *mut u16, mut val: u16) -> u16
{
	val = !val;
	llvm_asm!(".byte 0xf3; lock; xadd %0,%1" : "+q"(val), "+m"(*ptr) : : "memory" : "volatile");
	val
}

/// Atomically exchange (swap) `val` for the content of `ptr` and return the previous content of `ptr` with 'HLE Acquire' memory ordering.
#[inline(always)]
pub unsafe fn __hle_acquire_exchange_n2(ptr: *mut u16, mut val: u16) -> u16
{
	llvm_asm!(".byte 0xf2; lock; xchg %0,%1" : "+q"(val), "+m"(*ptr) : : "memory" : "volatile");
	val
}

/// Atomically exchange (swap) 1 for the content of `ptr` and test the previous content of of `ptr` was 1 with 'HLE Acquire' memory ordering.
#[inline(always)]
pub unsafe fn __hle_acquire_test_and_set2(ptr: *mut u16) -> bool
{
 	 __hle_acquire_exchange_n2(ptr, 1) == 1
}

/// Atomically compare-and-swap with 'HLE Acquire' memory ordering.
/// This compares the content of `ptr` with the content of `oldp`:-
/// * If equal, the function atomically writes `newv` into `ptr` and returns `true`.
/// * If they are not equal, the function atomically writes the content of `ptr` into `oldp` and returns `false`
/// Since this is only for the x86 / x86_64 architecture, there is no weak variant.
#[inline(always)]
pub unsafe fn __hle_acquire_compare_exchange_n2(ptr: *mut u16, oldp: *mut u16, newv: u16) -> bool
{
	let res: u8;
	llvm_asm!(".byte 0xf2; lock; cmpxchg %3,%1; setz %2" : "+a"(*oldp), "+m"(*ptr), "=r"(res) : "r"(newv) : "memory" : "volatile");
	res == 1
}

/// Atomically exchange (swap) `val` for the content of `ptr` and return the previous content of of `ptr` with 'HLE Release' memory ordering.
#[inline(always)]
pub unsafe fn __hle_release_exchange_n2(ptr: *mut u16, mut val: u16) -> u16
{
	llvm_asm!(".byte 0xf3; lock; xchg %0,%1" : "+q"(val), "+m"(*ptr) : : "memory" : "volatile");
	val
}

/// Atomically exchange (swap) 1 for the content of `ptr` and test the previous content of of `ptr` was 1 with 'HLE Release' memory ordering.
#[inline(always)]
pub unsafe fn __hle_release_test_and_set2(ptr: *mut u16) -> bool
{
 	 __hle_release_exchange_n2(ptr, 1) == 1
}

/// Atomically compare-and-swap with 'HLE Release' memory ordering.
/// This compares the content of `ptr` with the content of `oldp`:-
/// * If equal, the function atomically writes `newv` into `ptr` and returns `true`.
/// * If they are not equal, the function atomically writes the content of `ptr` into `oldp` and returns `false`
/// Since this is only for the x86 / x86_64 architecture, there is no weak variant.
#[inline(always)]
pub unsafe fn __hle_release_compare_exchange_n2(ptr: *mut u16, oldp: *mut u16, newv: u16) -> bool
{
	let res: u8;
	llvm_asm!(".byte 0xf3; lock; cmpxchg %3,%1; setz %2" : "+a"(*oldp), "+m"(*ptr), "=r"(res) : "r"(newv) : "memory" : "volatile");
	res == 1
}

/// Atomically store (set) the content of `ptr` to `val` with 'HLE Acquire' memory ordering.
#[inline(always)]
pub unsafe fn __hle_acquire_store_n2(ptr: *mut u16, val: u32)
{
	llvm_asm!(".byte 0xf2; mov %1,%0" : "=m"(*ptr) : "q"(val) : "memory" : "volatile");
}

/// Atomically clear the content of `ptr`, ie set the content of `ptr` to zero with 'HLE Acquire' memory ordering.
#[inline(always)]
pub unsafe fn __hle_acquire_clear2(ptr: *mut u16)
{
	__hle_acquire_store_n2(ptr, 0)
}

/// Atomically store (set) the content of `ptr` to `val` with 'HLE Release' memory ordering.
#[inline(always)]
pub unsafe fn __hle_release_store_n2(ptr: *mut u16, val: u32)
{
	llvm_asm!(".byte 0xf3; mov %1,%0" : "=m"(*ptr) : "q"(val) : "memory" : "volatile");
}

/// Atomically clear the content of `ptr`, ie set the content of `ptr` to zero with 'HLE Release' memory ordering.
#[inline(always)]
pub unsafe fn __hle_release_clear2(ptr: *mut u16)
{
	__hle_release_store_n2(ptr, 0)
}

/// Atomically add `val` to the content of `ptr` to `val` with 'HLE Acquire' memory ordering.
#[inline(always)]
pub unsafe fn __hle_acquire_add2(ptr: *mut u16, val: u16)
{
	llvm_asm!(".byte 0xf2; lock; add %1,%0" : "+m"(*ptr) : "q"(val) : "memory" : "volatile");
}

/// Atomically subtract `val` from the content of `ptr` to `val` with 'HLE Acquire' memory ordering.
#[inline(always)]
pub unsafe fn __hle_acquire_sub2(ptr: *mut u16, val: u16)
{
	llvm_asm!(".byte 0xf2; lock; sub %1,%0" : "+m"(*ptr) : "q"(val) : "memory" : "volatile");
}

/// Atomically OR `val` with the content of `ptr` to `val` with 'HLE Acquire' memory ordering.
#[inline(always)]
pub unsafe fn __hle_acquire_or2(ptr: *mut u16, val: u16)
{
	llvm_asm!(".byte 0xf2; lock; or %1,%0" : "+m"(*ptr) : "q"(val) : "memory" : "volatile");
}

/// Atomically AND `val` with the content of `ptr` to `val` with 'HLE Acquire' memory ordering.
#[inline(always)]
pub unsafe fn __hle_acquire_and2(ptr: *mut u16, val: u16)
{
	llvm_asm!(".byte 0xf2; lock; and %1,%0" : "+m"(*ptr) : "q"(val) : "memory" : "volatile");
}

/// Atomically XOR `val` with the content of `ptr` to `val` with 'HLE Acquire' memory ordering.
#[inline(always)]
pub unsafe fn __hle_acquire_xor2(ptr: *mut u16, val: u16)
{
	llvm_asm!(".byte 0xf2; lock; xor %1,%0" : "+m"(*ptr) : "q"(val) : "memory" : "volatile");
}

/// Atomically add `val` to the content of `ptr` to `val` with 'HLE Release' memory ordering.
#[inline(always)]
pub unsafe fn __hle_release_add2(ptr: *mut u16, val: u16)
{
	llvm_asm!(".byte 0xf3; lock; add %1,%0" : "+m"(*ptr) : "q"(val) : "memory" : "volatile");
}

/// Atomically subtract `val` from the content of `ptr` to `val` with 'HLE Release' memory ordering.
#[inline(always)]
pub unsafe fn __hle_release_sub2(ptr: *mut u16, val: u16)
{
	llvm_asm!(".byte 0xf3; lock; sub %1,%0" : "+m"(*ptr) : "q"(val) : "memory" : "volatile");
}

/// Atomically OR `val` with the content of `ptr` to `val` with 'HLE Release' memory ordering.
#[inline(always)]
pub unsafe fn __hle_release_or2(ptr: *mut u16, val: u16)
{
	llvm_asm!(".byte 0xf3; lock; or %1,%0" : "+m"(*ptr) : "q"(val) : "memory" : "volatile");
}

/// Atomically AND `val` with the content of `ptr` to `val` with 'HLE Release' memory ordering.
#[inline(always)]
pub unsafe fn __hle_release_and2(ptr: *mut u16, val: u16)
{
	llvm_asm!(".byte 0xf3; lock; and %1,%0" : "+m"(*ptr) : "q"(val) : "memory" : "volatile");
}

/// Atomically XOR `val` with the content of `ptr` to `val` with 'HLE Release' memory ordering.
#[inline(always)]
pub unsafe fn __hle_release_xor2(ptr: *mut u16, val: u16)
{
	llvm_asm!(".byte 0xf3; lock; xor %1,%0" : "+m"(*ptr) : "q"(val) : "memory" : "volatile");
}

/// Atomically add `val` to the content of `ptr` and return the result with 'HLE Acquire' memory ordering.
#[inline(always)]
pub unsafe fn __hle_acquire_add_fetch1(ptr: *mut u8, mut val: u8) -> u8
{
	let oldval = val;
	llvm_asm!(".byte 0xf2; lock; xadd %0,%1" : "+q"(val), "+m"(*ptr) : : "memory" : "volatile");
	val + oldval
}

/// Atomically subtract `val` from the content of `ptr` and return the result with 'HLE Acquire' memory ordering.
#[inline(always)]
pub unsafe fn __hle_acquire_sub_fetch1(ptr: *mut u8, mut val: u8) -> u8
{
	let oldval = val;
	val = !val;
	llvm_asm!(".byte 0xf2; lock; xadd %0,%1" : "+q"(val), "+m"(*ptr) : : "memory" : "volatile");
	val - oldval
}

/// Atomically add `val` to the content of `ptr` and return the result with 'HLE Release' memory ordering.
#[inline(always)]
pub unsafe fn __hle_release_add_fetch1(ptr: *mut u8, mut val: u8) -> u8
{
	let oldval = val;
	llvm_asm!(".byte 0xf3; lock; xadd %0,%1" : "+q"(val), "+m"(*ptr) : : "memory" : "volatile");
	val + oldval
}

/// Atomically subtract `val` from the content of `ptr` and return the result with 'HLE Release' memory ordering.
#[inline(always)]
pub unsafe fn __hle_release_sub_fetch1(ptr: *mut u8, mut val: u8) -> u8
{
	let oldval = val;
	val = !val;
	llvm_asm!(".byte 0xf3; lock; xadd %0,%1" : "+q"(val), "+m"(*ptr) : : "memory" : "volatile");
	val - oldval
}

/// Atomically add `val` to the content of `ptr` and return the previous content of `ptr` with 'HLE Acquire' memory ordering.
#[inline(always)]
pub unsafe fn __hle_acquire_fetch_add1(ptr: *mut u8, mut val: u8) -> u8
{
	llvm_asm!(".byte 0xf2; lock; xadd %0,%1" : "+q"(val), "+m"(*ptr) : : "memory" : "volatile");
	val
}

/// Atomically subtract `val` from the content of `ptr` and return the previous content of `ptr` with 'HLE Acquire' memory ordering.
#[inline(always)]
pub unsafe fn __hle_acquire_fetch_sub1(ptr: *mut u8, mut val: u8) -> u8
{
	val = !val;
	llvm_asm!(".byte 0xf2; lock; xadd %0,%1" : "+q"(val), "+m"(*ptr) : : "memory" : "volatile");
	val
}

/// Atomically add `val` to the content of `ptr` and return the previous content of `ptr` with 'HLE Release' memory ordering.
#[inline(always)]
pub unsafe fn __hle_release_fetch_add1(ptr: *mut u8, mut val: u8) -> u8
{
	llvm_asm!(".byte 0xf3; lock; xadd %0,%1" : "+q"(val), "+m"(*ptr) : : "memory" : "volatile");
	val
}

/// Atomically subtract `val` from the content of `ptr` and return the previous content of `ptr` with 'HLE Release' memory ordering.
#[inline(always)]
pub unsafe fn __hle_release_fetch_sub1(ptr: *mut u8, mut val: u8) -> u8
{
	val = !val;
	llvm_asm!(".byte 0xf3; lock; xadd %0,%1" : "+q"(val), "+m"(*ptr) : : "memory" : "volatile");
	val
}

/// Atomically exchange (swap) `val` for the content of `ptr` and return the previous content of `ptr` with 'HLE Acquire' memory ordering.
#[inline(always)]
pub unsafe fn __hle_acquire_exchange_n1(ptr: *mut u8, mut val: u8) -> u8
{
	llvm_asm!(".byte 0xf2; lock; xchg %0,%1" : "+q"(val), "+m"(*ptr) : : "memory" : "volatile");
	val
}

/// Atomically exchange (swap) 1 for the content of `ptr` and test the previous content of of `ptr` was 1 with 'HLE Acquire' memory ordering.
#[inline(always)]
pub unsafe fn __hle_acquire_test_and_set1(ptr: *mut u8) -> bool
{
 	 __hle_acquire_exchange_n1(ptr, 1) == 1
}

/// Atomically compare-and-swap with 'HLE Acquire' memory ordering.
/// This compares the content of `ptr` with the content of `oldp`:-
/// * If equal, the function atomically writes `newv` into `ptr` and returns `true`.
/// * If they are not equal, the function atomically writes the content of `ptr` into `oldp` and returns `false`
/// Since this is only for the x86 / x86_64 architecture, there is no weak variant.
#[inline(always)]
pub unsafe fn __hle_acquire_compare_exchange_n1(ptr: *mut u8, oldp: *mut u8, newv: u8) -> bool
{
	let res: u8;
	llvm_asm!(".byte 0xf2; lock; cmpxchg %3,%1; setz %2" : "+a"(*oldp), "+m"(*ptr), "=r"(res) : "r"(newv) : "memory" : "volatile");
	res == 1
}

/// Atomically exchange (swap) `val` for the content of `ptr` and return the previous content of of `ptr` with 'HLE Release' memory ordering.
#[inline(always)]
pub unsafe fn __hle_release_exchange_n1(ptr: *mut u8, mut val: u8) -> u8
{
	llvm_asm!(".byte 0xf3; lock; xchg %0,%1" : "+q"(val), "+m"(*ptr) : : "memory" : "volatile");
	val
}

/// Atomically exchange (swap) 1 for the content of `ptr` and test the previous content of of `ptr` was 1 with 'HLE Release' memory ordering.
#[inline(always)]
pub unsafe fn __hle_release_test_and_set1(ptr: *mut u8) -> bool
{
 	 __hle_release_exchange_n1(ptr, 1) == 1
}

/// Atomically compare-and-swap with 'HLE Release' memory ordering.
/// This compares the content of `ptr` with the content of `oldp`:-
/// * If equal, the function atomically writes `newv` into `ptr` and returns `true`.
/// * If they are not equal, the function atomically writes the content of `ptr` into `oldp` and returns `false`
/// Since this is only for the x86 / x86_64 architecture, there is no weak variant.
#[inline(always)]
pub unsafe fn __hle_release_compare_exchange_n1(ptr: *mut u8, oldp: *mut u8, newv: u8) -> bool
{
	let res: u8;
	llvm_asm!(".byte 0xf3; lock; cmpxchg %3,%1; setz %2" : "+a"(*oldp), "+m"(*ptr), "=r"(res) : "r"(newv) : "memory" : "volatile");
	res == 1
}

/// Atomically store (set) the content of `ptr` to `val` with 'HLE Acquire' memory ordering.
#[inline(always)]
pub unsafe fn __hle_acquire_store_n1(ptr: *mut u8, val: u32)
{
	llvm_asm!(".byte 0xf2; mov %1,%0" : "=m"(*ptr) : "q"(val) : "memory" : "volatile");
}

/// Atomically clear the content of `ptr`, ie set the content of `ptr` to zero with 'HLE Acquire' memory ordering.
#[inline(always)]
pub unsafe fn __hle_acquire_clear1(ptr: *mut u8)
{
	__hle_acquire_store_n1(ptr, 0)
}

/// Atomically store (set) the content of `ptr` to `val` with 'HLE Release' memory ordering.
#[inline(always)]
pub unsafe fn __hle_release_store_n1(ptr: *mut u8, val: u32)
{
	llvm_asm!(".byte 0xf3; mov %1,%0" : "=m"(*ptr) : "q"(val) : "memory" : "volatile");
}

/// Atomically clear the content of `ptr`, ie set the content of `ptr` to zero with 'HLE Release' memory ordering.
#[inline(always)]
pub unsafe fn __hle_release_clear1(ptr: *mut u8)
{
	__hle_release_store_n1(ptr, 0)
}

/// Atomically add `val` to the content of `ptr` to `val` with 'HLE Acquire' memory ordering.
#[inline(always)]
pub unsafe fn __hle_acquire_add1(ptr: *mut u8, val: u8)
{
	llvm_asm!(".byte 0xf2; lock; add %1,%0" : "+m"(*ptr) : "q"(val) : "memory" : "volatile");
}

/// Atomically subtract `val` from the content of `ptr` to `val` with 'HLE Acquire' memory ordering.
#[inline(always)]
pub unsafe fn __hle_acquire_sub1(ptr: *mut u8, val: u8)
{
	llvm_asm!(".byte 0xf2; lock; sub %1,%0" : "+m"(*ptr) : "q"(val) : "memory" : "volatile");
}

/// Atomically OR `val` with the content of `ptr` to `val` with 'HLE Acquire' memory ordering.
#[inline(always)]
pub unsafe fn __hle_acquire_or1(ptr: *mut u8, val: u8)
{
	llvm_asm!(".byte 0xf2; lock; or %1,%0" : "+m"(*ptr) : "q"(val) : "memory" : "volatile");
}

/// Atomically AND `val` with the content of `ptr` to `val` with 'HLE Acquire' memory ordering.
#[inline(always)]
pub unsafe fn __hle_acquire_and1(ptr: *mut u8, val: u8)
{
	llvm_asm!(".byte 0xf2; lock; and %1,%0" : "+m"(*ptr) : "q"(val) : "memory" : "volatile");
}

/// Atomically XOR `val` with the content of `ptr` to `val` with 'HLE Acquire' memory ordering.
#[inline(always)]
pub unsafe fn __hle_acquire_xor1(ptr: *mut u8, val: u8)
{
	llvm_asm!(".byte 0xf2; lock; xor %1,%0" : "+m"(*ptr) : "q"(val) : "memory" : "volatile");
}

/// Atomically add `val` to the content of `ptr` to `val` with 'HLE Release' memory ordering.
#[inline(always)]
pub unsafe fn __hle_release_add1(ptr: *mut u8, val: u8)
{
	llvm_asm!(".byte 0xf3; lock; add %1,%0" : "+m"(*ptr) : "q"(val) : "memory" : "volatile");
}

/// Atomically subtract `val` from the content of `ptr` to `val` with 'HLE Release' memory ordering.
#[inline(always)]
pub unsafe fn __hle_release_sub1(ptr: *mut u8, val: u8)
{
	llvm_asm!(".byte 0xf3; lock; sub %1,%0" : "+m"(*ptr) : "q"(val) : "memory" : "volatile");
}

/// Atomically OR `val` with the content of `ptr` to `val` with 'HLE Release' memory ordering.
#[inline(always)]
pub unsafe fn __hle_release_or1(ptr: *mut u8, val: u8)
{
	llvm_asm!(".byte 0xf3; lock; or %1,%0" : "+m"(*ptr) : "q"(val) : "memory" : "volatile");
}

/// Atomically AND `val` with the content of `ptr` to `val` with 'HLE Release' memory ordering.
#[inline(always)]
pub unsafe fn __hle_release_and1(ptr: *mut u8, val: u8)
{
	llvm_asm!(".byte 0xf3; lock; and %1,%0" : "+m"(*ptr) : "q"(val) : "memory" : "volatile");
}

/// Atomically XOR `val` with the content of `ptr` to `val` with 'HLE Release' memory ordering.
#[inline(always)]
pub unsafe fn __hle_release_xor1(ptr: *mut u8, val: u8)
{
	llvm_asm!(".byte 0xf3; lock; xor %1,%0" : "+m"(*ptr) : "q"(val) : "memory" : "volatile");
}
