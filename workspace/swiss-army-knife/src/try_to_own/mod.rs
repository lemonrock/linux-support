// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


use std::borrow::Borrow;
use std::borrow::Cow;
use std::cell::UnsafeCell;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::TryReserveError;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::hash::Hash;
use std::hash::Hasher;
use super::get_unchecked::GetUnchecked;


include!("MutableKey.rs");
include!("MutableKeyHashMap.rs");
include!("TryToOwn.rs");
include!("TryToOwned.rs");
include!("TryToOwnInPlace.rs");
