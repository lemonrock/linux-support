// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(super) const FIEMAP_EXTENT_LAST: u32 = 1;
pub(super) const FIEMAP_EXTENT_UNKNOWN: u32 = 2;
pub(super) const FIEMAP_EXTENT_DELALLOC: u32 = 4;
pub(super) const FIEMAP_EXTENT_ENCODED: u32 = 8;
pub(super) const FIEMAP_EXTENT_DATA_ENCRYPTED: u32 = 128;
pub(super) const FIEMAP_EXTENT_NOT_ALIGNED: u32 = 256;
pub(super) const FIEMAP_EXTENT_DATA_INLINE: u32 = 512;
pub(super) const FIEMAP_EXTENT_DATA_TAIL: u32 = 1024;
pub(super) const FIEMAP_EXTENT_UNWRITTEN: u32 = 2048;
pub(super) const FIEMAP_EXTENT_MERGED: u32 = 4096;
pub(super) const FIEMAP_EXTENT_SHARED: u32 = 8192;
