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
#![feature(core_intrinsics)]
#![feature(internal_uninit_const)]
#![feature(maybe_uninit_extra)]


//! #linux-support
//!
//! This library provides wrappers and additional functionality to make use of a panoply of miscellaneous Linux (and, sometimes, POSIX) features.
//!
//! See <https://github.com/lemonrock/linux-support> for far more detail.


use static_assertions::assert_cfg;
assert_cfg!(target_os = "linux");
assert_cfg!(target_pointer_width = "64");


use libc::c_char;
use likely::likely;
use likely::unlikely;
use memchr::memchr;
use serde::Deserialize;
use serde::Serialize;
use std::alloc::alloc_zeroed;
use std::alloc::AllocRef;
use std::alloc::dealloc;
use std::alloc::Global;
use std::alloc::Layout;
use std::array::TryFromSliceError;
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
use std::io;
use std::io::ErrorKind;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::mem::size_of;
use std::mem::transmute;
#[allow(deprecated)] use std::mem::uninitialized;
use std::num::NonZeroU8;
use std::ops::Deref;
use std::ops::DerefMut;
use std::ops::Index;
use std::ops::IndexMut;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::path::PathBuf;
use std::ptr::NonNull;
use std::ptr::null;
use std::ptr::write_bytes;


include!("LoadNonAtomically.rs");
include!("move_to_front_of_vec.rs");
include!("StaticInitializedOnce.rs");
include!("VariablySized.rs");


/// A set of types to support the use of bit sets in Linux APIs and files.
#[macro_use]
pub mod bit_set;


/// Internet protocol.
pub mod internet_protocol;


/// Path utilities.
pub mod path;


/// String utilities.
pub mod strings;
