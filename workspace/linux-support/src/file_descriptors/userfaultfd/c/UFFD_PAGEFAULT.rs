// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// If this was a write fault.
pub(super) const UFFD_PAGEFAULT_FLAG_WRITE: u32 = 1 << 0;

/// If reason is `VM_UFFD_WP`.
pub(super) const UFFD_PAGEFAULT_FLAG_WP: u32 = 1 << 1;
