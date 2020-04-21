// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(crate) const FIEMAP_FLAG_SYNC: u32 = 1;
pub(crate) const FIEMAP_FLAG_XATTR: u32 = 2;

/// Linux source: "request caching of the extents".
///
/// Only supported by `ext4` and `f2fs`.
pub(crate) const FIEMAP_FLAG_CACHE: u32 = 4;
