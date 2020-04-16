// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(super) const STATX_TYPE: c_uint = 0x0001;
pub(super) const STATX_MODE: c_uint = 0x0002;
pub(super) const STATX_NLINK: c_uint = 0x0004;
pub(super) const STATX_UID: c_uint = 0x0008;
pub(super) const STATX_GID: c_uint = 0x0010;
pub(super) const STATX_ATIME: c_uint = 0x0020;
pub(super) const STATX_MTIME: c_uint = 0x0040;
pub(super) const STATX_CTIME: c_uint = 0x0080;
pub(super) const STATX_INO: c_uint = 0x0100;
pub(super) const STATX_SIZE: c_uint = 0x0200;
pub(super) const STATX_BLOCKS: c_uint = 0x0400;
pub(super) const STATX_BASIC_STATS: c_uint = 0x07FF;
pub(super) const STATX_BTIME: c_uint = 0x0800;
pub(super) const STATX_ALL: c_uint = 0x0FFF;
#[allow(dead_code)] pub(super) const STATX__RESERVED: c_uint = 0x80000000;
