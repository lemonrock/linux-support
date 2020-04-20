// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// User visible flags.
///
/// Only supported it seems by `cifs`, `jfs` and `nilfs2`.
///
/// For `ext2`, an internal constant, `EXT2_FL_USER_VISIBLE`, with a different value, `0x004BDFFF`, is used.
#[allow(unused)]
pub(super) const FS_FL_USER_VISIBLE: i32 = 0x0003DFFF;

/// User modifiable flags.
///
/// Only supported it seems by `ext2`, `jfs`, `nilfs2` and `ocfs2`.
#[allow(unused)]
pub(super) const FS_FL_USER_MODIFIABLE: i32 = 0x000380FF;
