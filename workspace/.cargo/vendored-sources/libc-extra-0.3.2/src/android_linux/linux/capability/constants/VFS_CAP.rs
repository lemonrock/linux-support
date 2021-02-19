// This file is part of libc-extra. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT. No part of libc-extra, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of libc-extra. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT.


pub const VFS_CAP_REVISION_MASK: c_uint = 4278190080;
pub const VFS_CAP_REVISION_SHIFT: c_uchar = 24;

pub const VFS_CAP_FLAGS_MASK: c_longlong = -4278190081;
pub const VFS_CAP_FLAGS_EFFECTIVE: c_uchar = 1;

pub const VFS_CAP_REVISION_1: c_uint = 16777216;
pub const VFS_CAP_U32_1: c_uchar = 1;

pub const VFS_CAP_REVISION_2: c_uint = 33554432;
pub const VFS_CAP_U32_2: c_uchar = 2;

pub const VFS_CAP_REVISION: c_uint = VFS_CAP_REVISION_2;
pub const VFS_CAP_U32: c_uchar = VFS_CAP_U32_2;
