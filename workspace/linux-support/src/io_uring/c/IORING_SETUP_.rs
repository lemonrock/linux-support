// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


// io_uring_setup() flags.

/// io_context is polled.
const IORING_SETUP_IOPOLL: u32 = 1 << 0;

/// SQ poll thread.
const IORING_SETUP_SQPOLL: u32 = 1 << 1;

/// sq_thread_cpu is valid.
const IORING_SETUP_SQ_AFF: u32 = 1 << 2;

/// app defines CQ size.
const IORING_SETUP_CQSIZE: u32 = 1 << 3;

/// clamp SQ/CQ ring sizes.
const IORING_SETUP_CLAMP: u32 = 1 << 4;

/// attach to existing wq.
const IORING_SETUP_ATTACH_WQ: u32 = 1 << 5;
