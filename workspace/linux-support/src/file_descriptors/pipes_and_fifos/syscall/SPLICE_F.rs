// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Attempt to move pages instead of copying.
///
/// This is only a hint to the kernel: pages may still be copied if the kernel cannot move the pages from the pipe, or if the pipe buffers don't refer to full pages.
///
/// The initial implementation of this flag was buggy: therefore starting in Linux 2.6.21 it is a no-op (but is still permitted in a `splice(`) call); in the future, a correct implementation may be restored.
pub(crate) const SPLICE_F_MOVE: c_uint = 1;

/// Do not block on I/O.
///
/// This makes the splice pipe operations nonblocking, but `splice()` may nevertheless block because the file descriptors that are spliced to/from may block (unless they have the `O_NONBLOCK` flag set).
pub(crate) const SPLICE_F_NONBLOCK: c_uint = 2;

/// More data will be coming in a subsequent splice.
///
/// This is a helpful hint when the `fd_out` refers to a socket; see also the description of `MSG_MORE` in `man 2 send()`, and the description of `TCP_CORK` in `man 7 tcp`.
pub(crate) const SPLICE_F_MORE: c_uint = 4;

/// Unused for splice(); see vmsplice(2).
pub(crate) const SPLICE_F_GIFT: c_uint = 8;
