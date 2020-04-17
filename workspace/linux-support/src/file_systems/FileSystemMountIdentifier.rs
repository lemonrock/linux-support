// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Key to a row in `/proc/self/mountinfo`; it is the first field.
///
/// Opening with `O_PATH` the field in the fifth column of the matched row yield a file descriptor that needs to then be passed to `open_by_handle_at()`.
///
/// This is not a persistent value; it is reused when file systems are unmounted and mounted.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FileSystemMountIdentifier(pub(crate) i32);
