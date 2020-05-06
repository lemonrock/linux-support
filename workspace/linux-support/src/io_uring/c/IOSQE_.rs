// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


// `sqe->flags`.

/// use fixed fileset.
pub(super) const IOSQE_FIXED_FILE: u8 = 1 << (IOSQE::IOSQE_FIXED_FILE_BIT as u8);

/// issue after inflight IO.
pub(super) const IOSQE_IO_DRAIN: u8 = 1 << (IOSQE::IOSQE_IO_DRAIN_BIT as u8);

/// links next SQE.
pub(super) const IOSQE_IO_LINK: u8 = 1 << (IOSQE::IOSQE_IO_LINK_BIT as u8);

/// like LINK, but stronger.
pub(super) const IOSQE_IO_HARDLINK: u8 = 1 << (IOSQE::IOSQE_IO_HARDLINK_BIT as u8);

/// always go async.
pub(super) const IOSQE_ASYNC: u8 = 1 << (IOSQE::IOSQE_ASYNC_BIT as u8);

/// select buffer from `sqe->buf_group`.
pub(super) const IOSQE_BUFFER_SELECT: u8 = 1 << (IOSQE::IOSQE_BUFFER_SELECT_BIT as u8);
