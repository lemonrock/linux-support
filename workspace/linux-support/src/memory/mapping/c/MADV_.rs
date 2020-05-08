// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(super) const MADV_NORMAL: i32 = 0;
pub(super) const MADV_RANDOM: i32 = 1;
pub(super) const MADV_SEQUENTIAL: i32 = 2;
pub(super) const MADV_WILLNEED: i32 = 3;
pub(super) const MADV_DONTNEED: i32 = 4;
pub(super) const MADV_FREE: i32 = 8;
pub(super) const MADV_REMOVE: i32 = 9;
pub(super) const MADV_DONTFORK: i32 = 10;
pub(super) const MADV_DOFORK: i32 = 11;
pub(super) const MADV_MERGEABLE: i32 = 12;
pub(super) const MADV_UNMERGEABLE: i32 = 13;
pub(super) const MADV_HUGEPAGE: i32 = 14;
pub(super) const MADV_NOHUGEPAGE: i32 = 15;
pub(super) const MADV_DONTDUMP: i32 = 16;
pub(super) const MADV_DODUMP: i32 = 17;
pub(super) const MADV_WIPEONFORK: i32 = 18;
pub(super) const MADV_KEEPONFORK: i32 = 19;
pub(super) const MADV_COLD: i32 = 20;
pub(super) const MADV_PAGEOUT: i32 = 21;
pub(super) const MADV_HWPOISON: i32 = 100;
pub(super) const MADV_SOFT_OFFLINE: i32 = 101;
