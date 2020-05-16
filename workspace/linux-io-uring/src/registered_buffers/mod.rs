// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use linux_support::file_descriptors::CreationError;
use linux_support::memory::huge_pages::*;
use linux_support::memory::mapping::*;
use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;
use std::error;
use std::fmt;
use std::fmt::Formatter;
use std::fmt::Display;
use std::fmt::Debug;
use std::io;
use linux_support::io_uring::{RegisteredBufferIndex, IoUring};
use magic_ring_buffer::{LargeRingQueue, ReferenceCountedLargeRingQueueElement, ReferenceCountedLargeRingQueue};
use std::ptr::NonNull;
use std::rc::Rc;
use std::ops::{Deref, DerefMut};
use std::marker::PhantomData;
use std::alloc::AllocErr;


include!("RegisteredBuffer.rs");
include!("RegisteredBuffers.rs");
include!("RegisteredBuffersCreationError.rs");
include!("RegisteredBufferSetting.rs");
include!("RegisteredBufferSettings.rs");
include!("RegisteredBufferSource.rs");
include!("RegisteredBufferSubdivision.rs");