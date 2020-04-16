// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(super) const STATX_ATTR_COMPRESSED: u64 = 0x0004;
pub(super) const STATX_ATTR_IMMUTABLE: u64 = 0x0010;
pub(super) const STATX_ATTR_APPEND: u64 = 0x0020;
pub(super) const STATX_ATTR_NODUMP: u64 = 0x0040;
pub(super) const STATX_ATTR_ENCRYPTED: u64 = 0x0800;
pub(super) const STATX_ATTR_AUTOMOUNT: u64 = 0x1000;
pub(super) const STATX_ATTR_VERITY: u64 = 0x00100000;
