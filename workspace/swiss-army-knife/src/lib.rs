// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![deny(missing_docs)]
#![deny(unconditional_recursion)]
#![deny(unreachable_patterns)]
#![feature(alloc_layout_extra)]
#![feature(allocator_api)]
#![feature(asm)]
#![feature(const_fn_fn_ptr_basics)]
#![feature(const_fn_trait_bound)]
#![feature(const_mut_refs)]
#![feature(const_ptr_is_null)]
#![feature(const_ptr_offset_from)]
#![cfg_attr(not(debug_assertions), feature(const_unreachable_unchecked))]
#![feature(core_intrinsics)]
#![feature(extend_one)]
#![feature(llvm_asm)]
#![feature(maybe_uninit_extra)]
#![feature(maybe_uninit_slice)]
#![feature(maybe_uninit_uninit_array)]
#![feature(slice_index_methods)]
#![feature(maybe_uninit_array_assume_init)]
#![cfg_attr(all(target_arch = "x86_64", target_feature = "sse2"), feature(stdarch))]
#![cfg_attr(all(target_arch = "x86_64", target_feature = "avx512f"), feature(stdsimd))]


//! #linux-support
//!
//! This library provides wrappers and additional functionality to make use of a panoply of miscellaneous Linux (and, sometimes, POSIX) features.
//!
//! See <https://github.com/lemonrock/linux-support> for far more detail.


use static_assertions::assert_cfg;
assert_cfg!(target_pointer_width = "64");


use self::non_zero::new_non_null;
use arrayvec::ArrayVec;
use arrayvec::CapacityError;
use libc::c_char;
use likely::likely;
use likely::unlikely;
use memchr::memchr;
use memchr::memrchr;
use serde::Deserialize;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::alloc::alloc_zeroed;
use std::alloc::AllocError;
use std::alloc::dealloc;
use std::alloc::Layout;
#[cfg(target_arch = "x86_64")] use std::arch::x86_64::_rdrand16_step;
#[cfg(target_arch = "x86_64")] use std::arch::x86_64::_rdrand32_step;
#[cfg(target_arch = "x86_64")] use std::arch::x86_64::_rdrand64_step;
use std::array::TryFromSliceError;
use std::ascii::escape_default;
use std::borrow::Borrow;
use std::cmp::max;
use std::cmp::min;
use std::cmp::Ordering;
use std::collections::TryReserveError;
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
use std::ptr;
use std::ptr::NonNull;
use std::ptr::null;
use std::ptr::write_bytes;
use std::slice::from_raw_parts_mut;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Acquire;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::Ordering::Release;
use std::time::Duration;
use std::time::SystemTime;


/// Big-endian definitions.
pub mod big_endian;


/// SIMD-accelerated for byte swapping large areas of memory.
pub mod byte_swap;


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
/// To pick the best spin lock for the compilation target, use the type alias `BestForCompilationTargetSpinLock`.
pub mod hardware_optimized_spin_lock;


/// Wrappers around the current best choices for a `HashMap` and `HashSet`.
pub mod hash_map_and_hash_set;


/// Internet protocol.
pub mod internet_protocol;


/// Non zero numerics support.
pub mod non_zero;


/// Path utilities.
pub mod path;


/// Random.
pub mod random;


/// Compatible implementations of SIMD intrinsics either not written by Intel (but seem as if they should exist) or backwards-compatible implementations of SIMD intrinsics present in later CPU target features (eg a 8-bit popcnt).
pub mod simd_compatibility;


/// Split performance utilities.
pub mod split;


/// String utilities.
pub mod strings;


/// Time utilities.
pub mod time;


/// Unsafe initialization of memory.
pub mod unsafe_initialization;


/// `Vec` extensions.
pub mod vec;


include!("LoadNonAtomically.rs");
include!("move_to_front_of_vec.rs");
include!("StaticInitializedOnce.rs");
include!("unreachable_code.rs");
include!("unreachable_code_const.rs");
include!("VariablySized.rs");
