// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![deny(missing_docs)]
#![deny(unconditional_recursion)]
#![deny(unreachable_patterns)]
#![feature(alloc_layout_extra)]
#![feature(allocator_api)]
#![feature(const_fn)]
#![feature(const_fn_fn_ptr_basics)]
#![feature(const_fn_transmute)]
#![feature(const_panic)]
#![feature(const_ptr_is_null)]
#![cfg_attr(not(debug_assertions), feature(const_unreachable_unchecked))]
#![feature(core_intrinsics)]
#![feature(extend_one)]
#![feature(llvm_asm)]
#![feature(maybe_uninit_extra)]
#![feature(asm)]


//! #linux-support
//!
//! This library provides wrappers and additional functionality to make use of a panoply of miscellaneous Linux (and, sometimes, POSIX) features.
//!
//! See <https://github.com/lemonrock/linux-support> for far more detail.


use static_assertions::assert_cfg;
assert_cfg!(target_pointer_width = "64");


use self::non_zero::new_non_null;
#[cfg(target_arch = "x86_64")] use std::arch::x86_64::_rdrand16_step;
#[cfg(target_arch = "x86_64")] use std::arch::x86_64::_rdrand32_step;
#[cfg(target_arch = "x86_64")] use std::arch::x86_64::_rdrand64_step;
use arrayvec::Array;
use libc::c_char;
use likely::likely;
use likely::unlikely;
use memchr::memchr;
use memchr::memrchr;
use serde::Deserialize;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::alloc::alloc_zeroed;
use std::alloc::dealloc;
use std::alloc::Layout;
use std::ascii::escape_default;
use std::array::TryFromSliceError;
use std::borrow::Borrow;
use std::cell::UnsafeCell;
use std::cmp::max;
use std::cmp::min;
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::borrow::Cow;
use std::error;
use std::ffi::CStr;
use std::ffi::CString;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::hash::Hash;
use std::hash::Hasher;
#[cfg(not(debug_assertions))] use std::hint::unreachable_unchecked;
use std::intrinsics::assert_uninit_valid;
use std::intrinsics::assert_zero_valid;
use std::io;
use std::io::ErrorKind;
use std::iter::FromIterator;
use std::marker::PhantomData;
use std::mem::size_of;
use std::mem::transmute;
use std::mem::MaybeUninit;
use std::num::NonZeroI128;
use std::num::NonZeroI16;
use std::num::NonZeroI32;
use std::num::NonZeroI64;
use std::num::NonZeroI8;
use std::num::NonZeroIsize;
use std::num::NonZeroU128;
use std::num::NonZeroU16;
use std::num::NonZeroU32;
use std::num::NonZeroU64;
use std::num::NonZeroU8;
use std::num::NonZeroUsize;
use std::ops::BitAnd;
use std::ops::BitOr;
use std::ops::BitXor;
use std::ops::Deref;
use std::ops::DerefMut;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Neg;
use std::ops::Rem;
use std::ops::RemAssign;
use std::ops::Sub;
use std::ops::SubAssign;
use std::ops::Index;
use std::ops::IndexMut;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::path::PathBuf;
use std::ptr::NonNull;
use std::ptr::null;
use std::ptr::write_bytes;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Acquire;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::Ordering::Release;
use std::time::Duration;
use std::time::SystemTime;


/// AVX512 support, including functions one might assume to be present as intrinsics provided by Intel but aren't.
#[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
pub mod avx512;


/// Big-endian definitions.
pub mod big_endian;


/// A set of types to support the use of bit sets in Linux APIs and files.
#[macro_use]
pub mod bit_set;


/// Allows the use of slice methods `get_unchecked()` and `get_unchecked_mut()` such that when compiling with debug assertions access is checked.
pub mod get_unchecked;


/// Error support.
pub mod error_support;


/// Exponents of 2.
pub mod exponents_of_two;


/// Fixed point arithmetic.
pub mod fixed_point_arithmetic;


/// Spin lock.
///
/// An Intel hardware-optimized spin lock that uses Hardware Lock Elision (HLE) and a non-CAS based spin lock (an OR lock) as a fast fallback.
/// The intel spin lock, `HardwareLockElisionSpinLock`, is only available on a `x86_64` targets.
/// To pick the best spin lock for the compilation target, use the type alias `BestForCompilationTargetSpinLock`.
pub mod hardware_optimized_spin_lock;


/// Wrappers around the current best choices for a `HashMap` and `HashSet`.
pub mod hash_map_and_hash_set;


/// Intel hardware lock elision.
///
/// From wikipedia: "Hardware Lock Elision (HLE) adds two new instruction prefixes, XACQUIRE and XRELEASE. These two prefixes reuse the opcodes of the existing REPNE / REPE prefixes (F2H / F3H). On processors that do not support TSX/TSX-NI, REPNE / REPE prefixes are ignored on instructions for which the XACQUIRE / XRELEASE are valid, thus enabling backward compatibility".
///
/// The naming of the intrinsics follows that in Andi Kleen's [tsx-tools](https://github.com/andikleen/tsx-tools).
/// Intrinsics are available for `u8`, `u16`, `u32`, and, for x86_64, `u64`.
/// These intrinsics can be thought of as providing additional memory orderings to Rust's `Relaxed`, `Release`, `Acquire` and `SeqCst`.
/// They closely model the intent of [GCC's built in atomic instrincs](https://gcc.gnu.org/onlinedocs/gcc/_005f_005fatomic-Builtins.html).
#[cfg(target_arch = "x86_64")]
pub mod intel_hardware_lock_elision;


/// Internet protocol.
pub mod internet_protocol;


/// Non zero numerics support.
pub mod non_zero;


/// Path utilities.
pub mod path;


/// Random.
pub mod random;


/// Split performance utilities.
pub mod split;


/// String utilities.
pub mod strings;


/// Time utilities.
pub mod time;


/// Unsafe initialization of memory.
pub mod unsafe_initialization;


include!("ConstArrayVec.rs");
include!("LoadNonAtomically.rs");
include!("move_to_front_of_vec.rs");
include!("StaticInitializedOnce.rs");
include!("unreachable_code.rs");
include!("unreachable_code_const.rs");
include!("VariablySized.rs");
