// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


use new_capacity_calculator::ExactNewCapacityCalculator;
use new_capacity_calculator::NewCapacityCalculator;
use std::alloc::Allocator;
use std::alloc::AllocError;
use std::alloc::Global;
use std::alloc::handle_alloc_error;
use std::alloc::Layout;
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::collections::TryReserveError;
use std::collections::TryReserveErrorKind;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::hash::Hash;
use std::hash::Hasher;
use std::mem::align_of;
use std::mem::ManuallyDrop;
use std::mem::size_of;
use std::mem::transmute_copy;
use std::mem::MaybeUninit;
use std::ops::Deref;
use std::ops::DerefMut;
use std::ptr::NonNull;
use std::ptr::copy_nonoverlapping;
use std::ptr::drop_in_place;
use std::ptr::null_mut;
use std::slice::from_raw_parts;
use std::slice::from_raw_parts_mut;
use super::non_zero::new_non_null;


include!("required_capacity.rs");


/// Calculators for new capacity when trying to reserve memory.
pub mod new_capacity_calculator;


include!("ConstSmallVec.rs");
include!("Heap.rs");
include!("StackWithoutLength.rs");
include!("StackWithoutLengthOrHeap.rs");
